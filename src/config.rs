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

use crate::logging::log_message;
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
/// Result of config parsing and validation
#[derive(Debug, Default)]
pub struct ConfigResult {
    pub configs: HashMap<String, ProcessConfig>,
    pub constants: ConfigConstants,
    pub constants_count: usize,
    pub aliases_count: usize,
    pub groups_count: usize,
    pub group_members_count: usize,
    pub process_rules_count: usize,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ConfigResult {
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
            println!("✓ Parsed {} process rules", self.process_rules_count);
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

/// Reads and validates the main config file.
/// Returns configs, constants, and validation results.
/// Supports:
/// - `@CONSTANT = value` - Define a constant
/// - `*alias = cpu_spec` - Define a CPU alias
/// - `&group { ... },priority,affinity,...` - Define a process group with inline rule
/// - `name,priority,affinity,cpuset,prime,io,memory` - Process config (only first 3 required)
pub fn read_config<P: AsRef<Path>>(path: P) -> ConfigResult {
    let mut result = ConfigResult::default();

    let file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => {
            result.errors.push(format!("Cannot open config file: {}", e));
            return result;
        }
    };

    let reader = io::BufReader::new(file);
    let mut cpu_aliases: HashMap<String, Vec<u32>> = HashMap::new();

    // Collect all lines for multi-line block parsing
    let lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();
    let mut i = 0;

    while i < lines.len() {
        let line_number = i + 1;
        let line = lines[i].trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            i += 1;
            continue;
        }

        // Process group definition: &group_name { ... },rule
        if line.starts_with('&') && line.contains('{') {
            let brace_start = line.find('{').unwrap();
            let group_name = line[1..brace_start].trim().to_lowercase();

            if group_name.is_empty() {
                result.errors.push(format!("Line {}: Empty group name", line_number));
                i += 1;
                continue;
            }

            let mut members: Vec<String> = Vec::new();
            let mut inline_rule_suffix: Option<String> = None;

            // Check if it's a single-line definition: &group { a.exe, b.exe },rule
            if let Some(brace_end) = line.find('}') {
                let content = &line[brace_start + 1..brace_end];
                for item in content.split(',') {
                    let item = item.trim().to_lowercase();
                    if !item.is_empty() && !item.starts_with('#') {
                        members.push(item);
                    }
                }
                // Check for inline rule suffix after }
                let after_close = line[brace_end + 1..].trim();
                if after_close.starts_with(',') {
                    inline_rule_suffix = Some(after_close[1..].to_string());
                }
                i += 1;
            } else {
                // Multi-line block: collect until closing }
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

                    // Check for closing brace
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
                            // Check for inline rule suffix after }
                            let after_close = block_line[pos + 1..].trim();
                            if after_close.starts_with(',') {
                                inline_rule_suffix = Some(after_close[1..].to_string());
                            }
                        }
                        found_closing = true;
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

                if !found_closing {
                    result
                        .errors
                        .push(format!("Line {}: Unclosed group block '&{}' - missing closing brace", line_number, group_name));
                    continue;
                }
            }

            if members.is_empty() {
                result.warnings.push(format!("Line {}: Group '&{}' has no members", line_number, group_name));
                continue;
            }

            result.groups_count += 1;
            result.group_members_count += members.len();

            // Process inline rule suffix - this is now required for groups
            if let Some(ref suffix) = inline_rule_suffix {
                let parts: Vec<&str> = suffix.split(',').collect();
                if parts.len() >= 2 {
                    let priority = ProcessPriority::from_str(parts[0]);

                    // Validate priority
                    let priority_str = parts[0].trim();
                    if priority == ProcessPriority::None && !priority_str.eq_ignore_ascii_case("none") {
                        result
                            .warnings
                            .push(format!("Line {}: Unknown priority '{}' - will be treated as 'none'", line_number, priority_str));
                    }

                    let affinity_def = parts[1].trim();
                    let affinity_cpus = if affinity_def.starts_with('*') {
                        let alias = affinity_def.trim_start_matches('*').to_lowercase();
                        if !cpu_aliases.contains_key(&alias) {
                            result.errors.push(format!("Line {}: Undefined alias '*{}' in affinity field", line_number, alias));
                        }
                        cpu_aliases.get(&alias).cloned().unwrap_or_default()
                    } else {
                        parse_cpu_spec(affinity_def)
                    };

                    let cpuset_def = if parts.len() >= 3 { parts[2].trim() } else { "0" };
                    let cpu_set_cpus = if cpuset_def.starts_with('*') {
                        let alias = cpuset_def.trim_start_matches('*').to_lowercase();
                        if !cpu_aliases.contains_key(&alias) {
                            result.errors.push(format!("Line {}: Undefined alias '*{}' in cpuset field", line_number, alias));
                        }
                        cpu_aliases.get(&alias).cloned().unwrap_or_default()
                    } else {
                        parse_cpu_spec(cpuset_def)
                    };

                    let prime_def = if parts.len() >= 4 { parts[3].trim() } else { "0" };
                    let prime_cpus = if prime_def.starts_with('*') {
                        let alias = prime_def.trim_start_matches('*').to_lowercase();
                        if !cpu_aliases.contains_key(&alias) {
                            result.errors.push(format!("Line {}: Undefined alias '*{}' in prime_cpus field", line_number, alias));
                        }
                        cpu_aliases.get(&alias).cloned().unwrap_or_default()
                    } else {
                        parse_cpu_spec(prime_def)
                    };

                    let io_priority = if parts.len() >= 5 {
                        let io_str = parts[4].trim();
                        let io_p = IOPriority::from_str(io_str);
                        if io_p == IOPriority::None && !io_str.eq_ignore_ascii_case("none") {
                            result
                                .warnings
                                .push(format!("Line {}: Unknown IO priority '{}' - will be treated as 'none'", line_number, io_str));
                        }
                        io_p
                    } else {
                        IOPriority::None
                    };

                    let memory_priority = if parts.len() >= 6 {
                        let mem_str = parts[5].trim();
                        let mem_p = MemoryPriority::from_str(mem_str);
                        if mem_p == MemoryPriority::None && !mem_str.eq_ignore_ascii_case("none") {
                            result
                                .warnings
                                .push(format!("Line {}: Unknown memory priority '{}' - will be treated as 'none'", line_number, mem_str));
                        }
                        mem_p
                    } else {
                        MemoryPriority::None
                    };

                    for name in &members {
                        result.configs.insert(
                            name.clone(),
                            ProcessConfig {
                                name: name.clone(),
                                priority: priority.clone(),
                                affinity_cpus: affinity_cpus.clone(),
                                cpu_set_cpus: cpu_set_cpus.clone(),
                                prime_cpus: prime_cpus.clone(),
                                io_priority: io_priority.clone(),
                                memory_priority: memory_priority.clone(),
                            },
                        );
                    }
                    result.process_rules_count += members.len();
                    log_message(&format!("Config: Group '&{}' with {} processes", group_name, members.len()));
                } else {
                    result.errors.push(format!(
                        "Line {}: Group '&{}' has invalid rule suffix - expected at least priority,affinity",
                        line_number, group_name
                    ));
                }
            } else {
                result.errors.push(format!(
                    "Line {}: Group '&{}' missing rule suffix - use }},priority,affinity,...",
                    line_number, group_name
                ));
            }
            continue;
        }

        // Constant definition: @NAME=VALUE
        if line.starts_with('@') {
            if let Some(eq_pos) = line.find('=') {
                let const_name = line[1..eq_pos].trim().to_uppercase();
                let const_value = line[eq_pos + 1..].trim();
                match const_name.as_str() {
                    "HYSTERESIS_RATIO" => {
                        if let Ok(v) = const_value.parse::<f64>() {
                            result.constants.hysteresis_ratio = v;
                            result.constants_count += 1;
                            log_message(&format!("Config: HYSTERESIS_RATIO = {}", v));
                        } else {
                            result.errors.push(format!(
                                "Line {}: Invalid value '{}' for constant '{}' - expected a number",
                                line_number, const_value, const_name
                            ));
                        }
                    }
                    "KEEP_THRESHOLD" => {
                        if let Ok(v) = const_value.parse::<f64>() {
                            result.constants.keep_threshold = v;
                            result.constants_count += 1;
                            log_message(&format!("Config: KEEP_THRESHOLD = {}", v));
                        } else {
                            result.errors.push(format!(
                                "Line {}: Invalid value '{}' for constant '{}' - expected a number",
                                line_number, const_value, const_name
                            ));
                        }
                    }
                    "ENTRY_THRESHOLD" => {
                        if let Ok(v) = const_value.parse::<f64>() {
                            result.constants.entry_threshold = v;
                            result.constants_count += 1;
                            log_message(&format!("Config: ENTRY_THRESHOLD = {}", v));
                        } else {
                            result.errors.push(format!(
                                "Line {}: Invalid value '{}' for constant '{}' - expected a number",
                                line_number, const_value, const_name
                            ));
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

        // CPU alias definition: *NAME=SPEC
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

        // Reject standalone group references (no longer supported)
        if name.starts_with('&') {
            result.errors.push(format!(
                "Line {}: Standalone group reference '{}' not supported - use inline rule syntax: &group {{ ... }},rule",
                line_number, name
            ));
            i += 1;
            continue;
        }

        let priority = ProcessPriority::from_str(parts[1]);
        let priority_str = parts[1].trim();
        if priority == ProcessPriority::None && !priority_str.eq_ignore_ascii_case("none") {
            result
                .warnings
                .push(format!("Line {}: Unknown priority '{}' - will be treated as 'none'", line_number, priority_str));
        }

        // Parse affinity
        let affinity_def = parts[2].trim();
        let affinity_cpus = if affinity_def.starts_with('*') {
            let alias = affinity_def.trim_start_matches('*').to_lowercase();
            if !cpu_aliases.contains_key(&alias) {
                result.errors.push(format!("Line {}: Undefined alias '*{}' in affinity field", line_number, alias));
            }
            cpu_aliases.get(&alias).cloned().unwrap_or_default()
        } else {
            parse_cpu_spec(affinity_def)
        };

        // Parse CPU set
        let cpuset_def = if parts.len() >= 4 { parts[3].trim() } else { "0" };
        let cpu_set_cpus = if cpuset_def.starts_with('*') {
            let alias = cpuset_def.trim_start_matches('*').to_lowercase();
            if !cpu_aliases.contains_key(&alias) {
                result.errors.push(format!("Line {}: Undefined alias '*{}' in cpuset field", line_number, alias));
            }
            cpu_aliases.get(&alias).cloned().unwrap_or_default()
        } else {
            parse_cpu_spec(cpuset_def)
        };

        // Parse prime CPU
        let prime_def = if parts.len() >= 5 { parts[4].trim() } else { "0" };
        let prime_cpus = if prime_def.starts_with('*') {
            let alias = prime_def.trim_start_matches('*').to_lowercase();
            if !cpu_aliases.contains_key(&alias) {
                result.errors.push(format!("Line {}: Undefined alias '*{}' in prime_cpus field", line_number, alias));
            }
            cpu_aliases.get(&alias).cloned().unwrap_or_default()
        } else {
            parse_cpu_spec(prime_def)
        };

        let io_priority = if parts.len() >= 6 {
            let io_str = parts[5].trim();
            let io_p = IOPriority::from_str(io_str);
            if io_p == IOPriority::None && !io_str.eq_ignore_ascii_case("none") {
                result
                    .warnings
                    .push(format!("Line {}: Unknown IO priority '{}' - will be treated as 'none'", line_number, io_str));
            }
            io_p
        } else {
            IOPriority::None
        };

        let memory_priority = if parts.len() >= 7 {
            let mem_str = parts[6].trim();
            let mem_p = MemoryPriority::from_str(mem_str);
            if mem_p == MemoryPriority::None && !mem_str.eq_ignore_ascii_case("none") {
                result
                    .warnings
                    .push(format!("Line {}: Unknown memory priority '{}' - will be treated as 'none'", line_number, mem_str));
            }
            mem_p
        } else {
            MemoryPriority::None
        };

        result.configs.insert(
            name.to_lowercase(),
            ProcessConfig {
                name: name.to_lowercase(),
                priority,
                affinity_cpus,
                cpu_set_cpus,
                prime_cpus,
                io_priority,
                memory_priority,
            },
        );
        result.process_rules_count += 1;

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
