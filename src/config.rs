//! Configuration parsing and management.
//!
//! Handles reading config.ini files with process configurations,
//! affinity aliases, and scheduler constants.
//!
//! ## CPU Specification Format
//!
//! Supports both legacy hex masks (≤64 cores) and new range syntax (unlimited cores):
//!
//! - Legacy hex: `0xFF`, `0xFFFF`
//! - Single CPU: `5`
//! - Range: `0-7`
//! - Multiple ranges: `0-7;64-71`
//! - Mixed: `0-3;8;9;64-67`

use crate::logging::{log_message, log_to_find};
use crate::priority::{IOPriority, MemoryPriority, ProcessPriority};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::Path;

/// Configuration for a single process, parsed from config.ini.
/// Format: `name,priority,affinity,cpu_set,prime_cpu,io_priority,memory_priority`
#[derive(Debug, Clone)]
pub struct ProcessConfig {
    pub name: String,
    pub priority: ProcessPriority,
    /// CPU indices for legacy affinity mask (SetProcessAffinityMask, ≤64 cores)
    pub affinity_cpus: Vec<u32>,
    /// CPU indices for CPU sets (SetProcessDefaultCpuSets, unlimited cores)
    pub cpu_set_cpus: Vec<u32>,
    /// CPU indices for prime thread scheduling (high-priority threads)
    pub prime_cpus: Vec<u32>,
    pub io_priority: IOPriority,
    pub memory_priority: MemoryPriority,
}

/// Constants used by the Prime Thread Scheduler algorithm.
///
/// These control how threads are promoted/demoted to prime cores based on
/// their CPU activity levels.
#[derive(Debug, Clone)]
pub struct ConfigConstants {
    /// Multiplier for hysteresis between promotion and demotion thresholds
    pub hysteresis_ratio: f64,
    /// Fraction of max cycles below which a prime thread is demoted (default: 69%)
    pub keep_threshold: f64,
    /// Fraction of max cycles above which a thread becomes prime candidate (default: 42%)
    pub entry_threshold: f64,
}

impl Default for ConfigConstants {
    fn default() -> Self {
        ConfigConstants {
            hysteresis_ratio: 1.259,
            keep_threshold: 0.69,
            entry_threshold: 0.42,
        }
    }
}

/// Parses a CPU specification string into a vector of CPU indices.
///
/// Supports multiple formats:
/// - Legacy hex mask (≤64 cores): `0xFF`, `0xFFFF`
/// - Single CPU index: `5`
/// - Range: `0-7`
/// - Multiple ranges with semicolons: `0-7;64-71`
/// - Mixed: `0-3;8;9;64-67`
///
/// Note: Decimal masks are NOT supported to avoid confusion with single core indices.
/// Use hex format (0xFF) for masks or the new range format (0-7) instead.
///
/// # Examples
/// ```
/// parse_cpu_spec("0xFF")       // -> [0, 1, 2, 3, 4, 5, 6, 7]
/// parse_cpu_spec("0-7;64-71")  // -> [0, 1, 2, 3, 4, 5, 6, 7, 64, 65, 66, 67, 68, 69, 70, 71]
/// parse_cpu_spec("0;4;8")      // -> [0, 4, 8]
/// parse_cpu_spec("7")          // -> [7] (single core, not a mask)
/// ```
pub fn parse_cpu_spec(s: &str) -> Vec<u32> {
    let s = s.trim();
    if s.is_empty() || s == "0" {
        return Vec::new();
    }

    // Legacy hex mask format: 0xFF, 0xFFFF, etc.
    if s.starts_with("0x") || s.starts_with("0X") {
        if let Ok(mask) = u64::from_str_radix(&s[2..], 16) {
            return mask_to_cpu_indices(mask);
        }
        return Vec::new();
    }

    // New range/individual format: "0-7", "0-7;64-71", "0;4;8", "7"
    let mut cpus = Vec::new();
    for part in s.split(';') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        if let Some(dash_pos) = part.find('-') {
            // Range format: "0-7"
            let start: u32 = part[..dash_pos].trim().parse().unwrap_or(0);
            let end: u32 = part[dash_pos + 1..].trim().parse().unwrap_or(start);
            for cpu in start..=end {
                if !cpus.contains(&cpu) {
                    cpus.push(cpu);
                }
            }
        } else if let Ok(cpu) = part.parse::<u32>() {
            // Single CPU index: "7"
            if !cpus.contains(&cpu) {
                cpus.push(cpu);
            }
        }
    }
    cpus.sort();
    cpus
}

fn mask_to_cpu_indices(mask: u64) -> Vec<u32> {
    (0..64).filter(|i| (mask >> i) & 1 == 1).collect()
}

/// Converts a vector of CPU indices to a legacy affinity mask.
/// Only works correctly for indices < 64.
pub fn cpu_indices_to_mask(cpus: &[u32]) -> usize {
    let mut mask: usize = 0;
    for &cpu in cpus {
        if cpu < 64 {
            mask |= 1usize << cpu;
        }
    }
    mask
}

/// Formats CPU indices as a compact string for logging.
/// E.g., [0, 1, 2, 3, 8, 9, 10] -> "0-3,8-10"
pub fn format_cpu_indices(cpus: &[u32]) -> String {
    if cpus.is_empty() {
        return String::from("none");
    }

    let mut sorted: Vec<u32> = cpus.to_vec();
    sorted.sort();

    let mut result = String::new();
    let mut i = 0;
    while i < sorted.len() {
        let start = sorted[i];
        let mut end = start;

        // Find contiguous range
        while i + 1 < sorted.len() && sorted[i + 1] == sorted[i] + 1 {
            end = sorted[i + 1];
            i += 1;
        }

        if !result.is_empty() {
            result.push(',');
        }
        if start == end {
            result.push_str(&start.to_string());
        } else {
            result.push_str(&format!("{}-{}", start, end));
        }
        i += 1;
    }
    result
}

/// Reads a configuration file and returns process configs and scheduler constants.
///
/// ## Supported line formats:
/// - `# comment` - Lines starting with `#` are ignored
/// - `@CONSTANT=value` - Set scheduler constants (KEEP_THRESHOLD, ENTRY_THRESHOLD)
/// - `*alias=spec` - Define a reusable CPU spec alias (e.g., `*pcore=0-7;64-71`)
/// - `&group { ... }` - Define a process group (multi-line with `{}`)
/// - `&group,priority,affinity,...` - Apply config to all processes in a group
/// - `name,priority,affinity,cpuset,prime,io,memory` - Process config (only first 3 required)
pub fn read_config<P: AsRef<Path>>(path: P) -> io::Result<(HashMap<String, ProcessConfig>, ConfigConstants)> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut configs = HashMap::new();
    let mut cpu_aliases: HashMap<String, Vec<u32>> = HashMap::new();
    let mut process_groups: HashMap<String, Vec<String>> = HashMap::new();
    let mut constants = ConfigConstants::default();

    // Collect all lines for multi-line block parsing
    let lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            i += 1;
            continue;
        }

        // Process group definition: &group_name { ... }
        // Check if this is a group definition (has { before any comma, or no comma at all)
        if line.starts_with('&') {
            let brace_pos = line.find('{');
            let comma_pos = line.find(',');
            let is_group_def = match (brace_pos, comma_pos) {
                (Some(b), Some(c)) => b < c, // { comes before ,
                (Some(_), None) => true,     // has { but no ,
                _ => false,                  // no { means it's a group reference
            };

            if is_group_def {
                if let Some(brace_start) = brace_pos {
                    let group_name = line[1..brace_start].trim().to_lowercase();
                    let mut members: Vec<String> = Vec::new();

                    // Check if it's a single-line definition: &group { a.exe, b.exe }
                    if let Some(brace_end) = line.find('}') {
                        // Single line: &group { a.exe, b.exe }
                        let content = &line[brace_start + 1..brace_end];
                        for item in content.split(',') {
                            let item = item.trim().to_lowercase();
                            if !item.is_empty() && !item.starts_with('#') {
                                members.push(item);
                            }
                        }
                        i += 1;
                    } else {
                        // Multi-line block: collect until closing }
                        // First, check for content after { on the same line
                        let after_brace = line[brace_start + 1..].trim();
                        if !after_brace.is_empty() && !after_brace.starts_with('#') {
                            for item in after_brace.split(',') {
                                let item = item.trim().to_lowercase();
                                if !item.is_empty() && !item.starts_with('#') {
                                    members.push(item);
                                }
                            }
                        }

                        i += 1;
                        while i < lines.len() {
                            let block_line = lines[i].trim();

                            // Check for closing brace
                            if block_line.contains('}') {
                                // Check for content before }
                                if let Some(pos) = block_line.find('}') {
                                    let before_brace = block_line[..pos].trim();
                                    if !before_brace.is_empty() && !before_brace.starts_with('#') {
                                        for item in before_brace.split(',') {
                                            let item = item.trim().to_lowercase();
                                            if !item.is_empty() && !item.starts_with('#') {
                                                members.push(item);
                                            }
                                        }
                                    }
                                }
                                i += 1;
                                break;
                            }

                            // Process line content
                            if !block_line.is_empty() && !block_line.starts_with('#') {
                                for item in block_line.split(',') {
                                    let item = item.trim().to_lowercase();
                                    if !item.is_empty() && !item.starts_with('#') {
                                        members.push(item);
                                    }
                                }
                            }
                            i += 1;
                        }
                    }

                    if !group_name.is_empty() && !members.is_empty() {
                        log_message(&format!("Config: Group '&{}' with {} processes", group_name, members.len()));
                        process_groups.insert(group_name, members);
                    }
                    continue;
                }
            }
            // If no brace found, fall through to process as potential group reference
        }

        // Constant definition: @NAME=VALUE
        if line.starts_with('@') {
            if let Some(eq_pos) = line.find('=') {
                let const_name = line[1..eq_pos].trim().to_uppercase();
                let const_value = line[eq_pos + 1..].trim();
                match const_name.as_str() {
                    "HYSTERESIS_RATIO" => {
                        if let Ok(v) = const_value.parse::<f64>() {
                            constants.hysteresis_ratio = v;
                            log_message(&format!("Config: HYSTERESIS_RATIO = {}", v));
                        }
                    }
                    "KEEP_THRESHOLD" => {
                        if let Ok(v) = const_value.parse::<f64>() {
                            constants.keep_threshold = v;
                            log_message(&format!("Config: KEEP_THRESHOLD = {}", v));
                        }
                    }
                    "ENTRY_THRESHOLD" => {
                        if let Ok(v) = const_value.parse::<f64>() {
                            constants.entry_threshold = v;
                            log_message(&format!("Config: ENTRY_THRESHOLD = {}", v));
                        }
                    }
                    _ => {
                        log_to_find(&format!("Unknown constant: {}", const_name));
                    }
                }
            }
            i += 1;
            continue;
        }

        // CPU alias definition: *NAME=SPEC
        if line.starts_with('*') {
            if let Some(eq_pos) = line.find('=') {
                let alias_name = line[1..eq_pos].trim().to_lowercase();
                let alias_value = line[eq_pos + 1..].trim();
                let cpus = parse_cpu_spec(alias_value);
                cpu_aliases.insert(alias_name, cpus);
            }
            i += 1;
            continue;
        }

        // Process configuration line (or group reference)
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() >= 3 {
            let name_or_group = parts[0].trim();

            // Check if this is a group reference: &group_name,priority,affinity,...
            let process_names: Vec<String> = if name_or_group.starts_with('&') {
                let group_name = name_or_group[1..].to_lowercase();
                process_groups.get(&group_name).cloned().unwrap_or_else(|| {
                    log_to_find(&format!("Unknown process group: &{}", group_name));
                    Vec::new()
                })
            } else {
                vec![name_or_group.to_lowercase()]
            };

            let priority = ProcessPriority::from_str(parts[1]);

            // Parse affinity
            let affinity_def = parts[2].trim();
            let affinity_cpus = if affinity_def.starts_with('*') {
                cpu_aliases.get(&affinity_def.trim_start_matches('*').to_lowercase()).cloned().unwrap_or_default()
            } else {
                parse_cpu_spec(affinity_def)
            };

            // Parse CPU set
            let cpuset_def = if parts.len() >= 4 { parts[3].trim() } else { "0" };
            let cpu_set_cpus = if cpuset_def.starts_with('*') {
                cpu_aliases.get(&cpuset_def.trim_start_matches('*').to_lowercase()).cloned().unwrap_or_default()
            } else {
                parse_cpu_spec(cpuset_def)
            };

            // Parse prime CPU
            let prime_def = if parts.len() >= 5 { parts[4].trim() } else { "0" };
            let prime_cpus = if prime_def.starts_with('*') {
                cpu_aliases.get(&prime_def.trim_start_matches('*').to_lowercase()).cloned().unwrap_or_default()
            } else {
                parse_cpu_spec(prime_def)
            };

            let io_priority = if parts.len() >= 6 { IOPriority::from_str(parts[5]) } else { IOPriority::None };
            let memory_priority = if parts.len() >= 7 {
                MemoryPriority::from_str(parts[6])
            } else {
                MemoryPriority::None
            };

            // Insert config for each process name (single or from group)
            for name in process_names {
                configs.insert(
                    name.clone(),
                    ProcessConfig {
                        name,
                        priority: priority.clone(),
                        affinity_cpus: affinity_cpus.clone(),
                        cpu_set_cpus: cpu_set_cpus.clone(),
                        prime_cpus: prime_cpus.clone(),
                        io_priority: io_priority.clone(),
                        memory_priority: memory_priority.clone(),
                    },
                );
            }
        }

        i += 1;
    }

    Ok((configs, constants))
}

/// Result of config validation
#[derive(Debug, Default)]
pub struct ConfigValidationResult {
    pub constants_count: usize,
    pub aliases_count: usize,
    pub groups_count: usize,
    pub group_members_count: usize,
    pub configs_count: usize,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ConfigValidationResult {
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn print_report(&self) {
        if self.is_valid() {
            println!("✓ Parsed {} constants", self.constants_count);
            println!("✓ Parsed {} CPU aliases", self.aliases_count);
            if self.groups_count > 0 {
                println!("✓ Parsed {} process groups ({} processes)", self.groups_count, self.group_members_count);
            }
            println!("✓ Parsed {} process rules", self.configs_count);
            if !self.warnings.is_empty() {
                for warning in &self.warnings {
                    println!("⚠ {}", warning);
                }
            }
            println!("✓ Config is valid!");
        } else {
            for error in &self.errors {
                println!("✗ {}", error);
            }
            for warning in &self.warnings {
                println!("⚠ {}", warning);
            }
            println!();
            println!("Found {} error(s). Fix them before running.", self.errors.len());
        }
    }
}

/// Validates a config file without applying any changes.
/// Returns detailed information about parsing results and any errors found.
pub fn validate_config<P: AsRef<Path>>(path: P) -> ConfigValidationResult {
    let mut result = ConfigValidationResult::default();

    let file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => {
            result.errors.push(format!("Cannot open config file: {}", e));
            return result;
        }
    };

    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();

    let mut cpu_aliases: HashMap<String, Vec<u32>> = HashMap::new();
    let mut process_groups: HashMap<String, Vec<String>> = HashMap::new();
    let mut i = 0;

    while i < lines.len() {
        let line_number = i + 1;
        let line = lines[i].trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            i += 1;
            continue;
        }

        // Process group definition: &group_name { ... }
        // Check if this is a group definition (has { before any comma, or no comma at all)
        if line.starts_with('&') {
            let brace_pos = line.find('{');
            let comma_pos = line.find(',');
            let is_group_def = match (brace_pos, comma_pos) {
                (Some(b), Some(c)) => b < c, // { comes before ,
                (Some(_), None) => true,     // has { but no ,
                _ => false,                  // no { means it's a group reference
            };

            if is_group_def {
                if let Some(brace_start) = brace_pos {
                    let group_name = line[1..brace_start].trim().to_lowercase();
                    let mut members: Vec<String> = Vec::new();

                    if group_name.is_empty() {
                        result.errors.push(format!("Line {}: Empty group name", line_number));
                        i += 1;
                        continue;
                    }

                    // Check if it's a single-line definition
                    if let Some(brace_end) = line.find('}') {
                        let content = &line[brace_start + 1..brace_end];
                        for item in content.split(',') {
                            let item = item.trim().to_lowercase();
                            if !item.is_empty() && !item.starts_with('#') {
                                members.push(item);
                            }
                        }
                        i += 1;
                    } else {
                        // Multi-line block
                        let after_brace = line[brace_start + 1..].trim();
                        if !after_brace.is_empty() && !after_brace.starts_with('#') {
                            for item in after_brace.split(',') {
                                let item = item.trim().to_lowercase();
                                if !item.is_empty() && !item.starts_with('#') {
                                    members.push(item);
                                }
                            }
                        }

                        i += 1;
                        let mut found_closing = false;
                        while i < lines.len() {
                            let block_line = lines[i].trim();

                            if block_line.contains('}') {
                                if let Some(pos) = block_line.find('}') {
                                    let before_brace = block_line[..pos].trim();
                                    if !before_brace.is_empty() && !before_brace.starts_with('#') {
                                        for item in before_brace.split(',') {
                                            let item = item.trim().to_lowercase();
                                            if !item.is_empty() && !item.starts_with('#') {
                                                members.push(item);
                                            }
                                        }
                                    }
                                }
                                found_closing = true;
                                i += 1;
                                break;
                            }

                            if !block_line.is_empty() && !block_line.starts_with('#') {
                                for item in block_line.split(',') {
                                    let item = item.trim().to_lowercase();
                                    if !item.is_empty() && !item.starts_with('#') {
                                        members.push(item);
                                    }
                                }
                            }
                            i += 1;
                        }

                        if !found_closing {
                            result
                                .errors
                                .push(format!("Line {}: Unclosed group block '&{}' - missing closing braces", line_number, group_name));
                        }
                    }

                    if members.is_empty() {
                        result.warnings.push(format!("Line {}: Group '&{}' has no members", line_number, group_name));
                    } else {
                        result.groups_count += 1;
                        result.group_members_count += members.len();
                        process_groups.insert(group_name, members);
                    }
                    continue;
                }
            }
            // If no brace found, fall through to process as potential group reference
        }

        // Constant definition
        if line.starts_with('@') {
            if let Some(eq_pos) = line.find('=') {
                let const_name = line[1..eq_pos].trim().to_uppercase();
                let const_value = line[eq_pos + 1..].trim();

                match const_name.as_str() {
                    "HYSTERESIS_RATIO" | "KEEP_THRESHOLD" | "ENTRY_THRESHOLD" => {
                        if const_value.parse::<f64>().is_err() {
                            result.errors.push(format!(
                                "Line {}: Invalid value '{}' for constant '{}' - expected a number",
                                line_number, const_value, const_name
                            ));
                        } else {
                            result.constants_count += 1;
                        }
                    }
                    _ => {
                        result
                            .warnings
                            .push(format!("Line {}: Unknown constant '{}' - will be ignored", line_number, const_name));
                    }
                }
            } else {
                result
                    .errors
                    .push(format!("Line {}: Invalid constant definition '{}' - expected '@NAME = value'", line_number, line));
            }
            i += 1;
            continue;
        }

        // CPU alias definition
        if line.starts_with('*') {
            if let Some(eq_pos) = line.find('=') {
                let alias_name = line[1..eq_pos].trim().to_lowercase();
                let alias_value = line[eq_pos + 1..].trim();

                if alias_name.is_empty() {
                    result.errors.push(format!("Line {}: Empty alias name", line_number));
                } else {
                    let cpus = parse_cpu_spec(alias_value);
                    if cpus.is_empty() && alias_value != "0" {
                        result
                            .warnings
                            .push(format!("Line {}: Alias '*{}' has empty CPU set from '{}'", line_number, alias_name, alias_value));
                    }
                    cpu_aliases.insert(alias_name, cpus);
                    result.aliases_count += 1;
                }
            } else {
                result
                    .errors
                    .push(format!("Line {}: Invalid alias definition '{}' - expected '*name = cpu_spec'", line_number, line));
            }
            i += 1;
            continue;
        }

        // Process configuration line
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 3 {
            result.errors.push(format!(
                "Line {}: Too few fields ({}) - expected at least 3 (name,priority,affinity)",
                line_number,
                parts.len()
            ));
            i += 1;
            continue;
        }

        let name = parts[0].trim();
        if name.is_empty() {
            result.errors.push(format!("Line {}: Empty process name", line_number));
            i += 1;
            continue;
        }

        // Check for group reference
        if name.starts_with('&') {
            let group_name = name[1..].to_lowercase();
            if !process_groups.contains_key(&group_name) {
                result.errors.push(format!("Line {}: Undefined process group '&{}'", line_number, group_name));
            }
        }

        // Validate priority
        let priority_str = parts[1].trim();
        if ProcessPriority::from_str(priority_str) == ProcessPriority::None && !priority_str.eq_ignore_ascii_case("none") {
            result
                .warnings
                .push(format!("Line {}: Unknown priority '{}' - will be treated as 'none'", line_number, priority_str));
        }

        // Validate CPU specs (affinity, cpuset, prime)
        for (field_idx, field_name) in [(2, "affinity"), (3, "cpuset"), (4, "prime_cpus")].iter() {
            if parts.len() > *field_idx {
                let spec = parts[*field_idx].trim();
                if spec.starts_with('*') {
                    let alias = spec.trim_start_matches('*').to_lowercase();
                    if !cpu_aliases.contains_key(&alias) {
                        result
                            .errors
                            .push(format!("Line {}: Undefined alias '*{}' in {} field", line_number, alias, field_name));
                    }
                }
            }
        }

        // Validate IO priority
        if parts.len() >= 6 {
            let io_str = parts[5].trim();
            if IOPriority::from_str(io_str) == IOPriority::None && !io_str.eq_ignore_ascii_case("none") {
                result
                    .warnings
                    .push(format!("Line {}: Unknown IO priority '{}' - will be treated as 'none'", line_number, io_str));
            }
        }

        // Validate Memory priority
        if parts.len() >= 7 {
            let mem_str = parts[6].trim();
            if MemoryPriority::from_str(mem_str) == MemoryPriority::None && !mem_str.eq_ignore_ascii_case("none") {
                result
                    .warnings
                    .push(format!("Line {}: Unknown memory priority '{}' - will be treated as 'none'", line_number, mem_str));
            }
        }

        result.configs_count += 1;
        i += 1;
    }

    result
}

/// Reads a list of process names from a file (for blacklist).
pub fn read_list<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    Ok(reader
        .lines()
        .filter_map(|l| l.ok())
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty() && !s.starts_with('#'))
        .collect())
}

/// Reads a UTF-16 LE encoded file (Process Lasso format).
pub fn read_utf16le_file(path: &str) -> io::Result<String> {
    let bytes = fs::read(path)?;
    let utf16: Vec<u16> = bytes.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
    Ok(String::from_utf16_lossy(&utf16))
}

/// Parses a CPU mask string in Process Lasso format.
/// Supports semicolon-separated core IDs and ranges: `"0;2;4-7"` -> cores 0, 2, 4, 5, 6, 7
#[allow(dead_code)]
pub fn parse_mask(s: &str) -> usize {
    let cpus = parse_cpu_spec(s);
    cpu_indices_to_mask(&cpus)
}

/// Converts Process Lasso configuration (UTF-16LE) to this tool's format.
/// Extracts `DefaultPriorities` and `DefaultAffinitiesEx` entries.
pub fn convert(in_file: Option<String>, out_file: Option<String>) {
    let in_path = match in_file {
        Some(p) => p,
        None => {
            println!("Error: -in <file> is required for -convert");
            return;
        }
    };
    let out_path = match out_file {
        Some(p) => p,
        None => {
            println!("Error: -out <file> is required for -convert");
            return;
        }
    };

    let content = match read_utf16le_file(&in_path) {
        Ok(c) => c,
        Err(e) => {
            println!("Failed to read {}: {}", in_path, e);
            return;
        }
    };

    let mut output_lines: Vec<String> = Vec::new();
    output_lines.push("# Converted from Process Lasso config".to_string());
    output_lines.push("# Format: process_name,priority,affinity,cpuset,prime,io_priority,memory_priority".to_string());
    output_lines.push(String::new());

    let mut priorities: HashMap<String, String> = HashMap::new();
    let mut affinities: HashMap<String, String> = HashMap::new();

    // Parse DefaultPriorities section
    let mut in_priorities = false;
    let mut in_affinities = false;

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("[DefaultPriorities]") {
            in_priorities = true;
            in_affinities = false;
            continue;
        }
        if line.starts_with("[DefaultAffinitiesEx]") {
            in_priorities = false;
            in_affinities = true;
            continue;
        }
        if line.starts_with('[') {
            in_priorities = false;
            in_affinities = false;
            continue;
        }

        if in_priorities {
            if let Some(eq_pos) = line.find('=') {
                let name = line[..eq_pos].trim().to_lowercase();
                let value = line[eq_pos + 1..].trim();
                priorities.insert(name, value.to_string());
            }
        }
        if in_affinities {
            if let Some(eq_pos) = line.find('=') {
                let name = line[..eq_pos].trim().to_lowercase();
                let value = line[eq_pos + 1..].trim();
                affinities.insert(name, value.to_string());
            }
        }
    }

    // Merge priorities and affinities
    let mut all_processes: std::collections::HashSet<String> = priorities.keys().cloned().collect();
    all_processes.extend(affinities.keys().cloned());

    for name in all_processes {
        let priority = priorities.get(&name).map(|s| s.as_str()).unwrap_or("none");
        let affinity = affinities.get(&name).map(|s| s.as_str()).unwrap_or("0");

        // Convert Process Lasso priority to our format
        let priority_str = match priority {
            "1" => "idle",
            "2" => "below normal",
            "3" => "normal",
            "4" => "above normal",
            "5" => "high",
            "6" => "real time",
            _ => "none",
        };

        output_lines.push(format!("{},{},{},0,0,none,none", name, priority_str, affinity));
    }

    // Write output file
    let mut out = match File::create(&out_path) {
        Ok(f) => f,
        Err(e) => {
            println!("Failed to create {}: {}", out_path, e);
            return;
        }
    };

    for line in output_lines {
        if writeln!(out, "{}", line).is_err() {
            println!("Failed to write to {}", out_path);
            return;
        }
    }

    println!("Converted {} to {}", in_path, out_path);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cpu_spec_hex() {
        assert_eq!(parse_cpu_spec("0xFF"), vec![0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(parse_cpu_spec("0x0F"), vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_parse_cpu_spec_ranges() {
        assert_eq!(parse_cpu_spec("0-3"), vec![0, 1, 2, 3]);
        assert_eq!(parse_cpu_spec("0-7;64-71"), vec![0, 1, 2, 3, 4, 5, 6, 7, 64, 65, 66, 67, 68, 69, 70, 71]);
    }

    #[test]
    fn test_parse_cpu_spec_individual() {
        assert_eq!(parse_cpu_spec("0;4;8"), vec![0, 4, 8]);
    }

    #[test]
    fn test_parse_cpu_spec_single_core() {
        // Single number should be interpreted as a single core, not a mask
        assert_eq!(parse_cpu_spec("7"), vec![7]);
        assert_eq!(parse_cpu_spec("15"), vec![15]);
    }

    #[test]
    fn test_parse_cpu_spec_mixed() {
        assert_eq!(parse_cpu_spec("0-3;8;9;64-67"), vec![0, 1, 2, 3, 8, 9, 64, 65, 66, 67]);
    }

    #[test]
    fn test_parse_cpu_spec_empty() {
        assert_eq!(parse_cpu_spec(""), Vec::<u32>::new());
        assert_eq!(parse_cpu_spec("0"), Vec::<u32>::new());
    }

    #[test]
    fn test_cpu_indices_to_mask() {
        assert_eq!(cpu_indices_to_mask(&[0, 1, 2, 3]), 0xF);
        assert_eq!(cpu_indices_to_mask(&[0, 1, 2, 3, 4, 5, 6, 7]), 0xFF);
    }

    #[test]
    fn test_format_cpu_indices() {
        assert_eq!(format_cpu_indices(&[0, 1, 2, 3]), "0-3");
        assert_eq!(format_cpu_indices(&[0, 1, 2, 3, 8, 9, 10]), "0-3,8-10");
        assert_eq!(format_cpu_indices(&[0, 2, 4]), "0,2,4");
    }
}
