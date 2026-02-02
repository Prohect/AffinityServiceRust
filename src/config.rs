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

use crate::{
    cli::get_config_help_lines,
    log,
    logging::{log_message, log_to_find},
    priority::{IOPriority, MemoryPriority, ProcessPriority, ThreadPriority},
};

use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, BufRead, Write},
    path::Path,
};

/// Represents a module prefix filter for prime thread scheduling with optional CPU set override and thread priority.
#[derive(Debug, Clone)]
pub struct PrimePrefix {
    /// Module name prefix to match (e.g., "cs2.exe", "nvwgf2umx.dll")
    pub prefix: String,
    /// Optional CPU set for threads matching this prefix (falls back to prime_threads_cpus if None)
    pub cpus: Option<Vec<u32>>,
    /// Optional thread priority for threads matching this prefix (uses boost_one if None)
    pub thread_priority: ThreadPriority,
}

/// Configuration for a single process, parsed from config.ini.
/// Format: `name:priority:affinity:cpu_set:prime_cpu:io_priority:memory_priority`
#[derive(Debug, Clone)]

pub struct ProcessConfig {
    pub name: String,
    pub priority: ProcessPriority,
    /// CPU indices for legacy affinity mask (SetProcessAffinityMask, ≤64 cores)
    pub affinity_cpus: Vec<u32>,
    /// CPU indices for CPU sets (SetProcessDefaultCpuSets, unlimited cores)
    pub cpu_set_cpus: Vec<u32>,
    /// CPU indices for prime thread scheduling (high-priority threads)
    pub prime_threads_cpus: Vec<u32>,
    /// Module name prefixes for filtering prime threads with optional CPU overrides
    pub prime_threads_prefixes: Vec<PrimePrefix>,
    pub io_priority: IOPriority,
    pub memory_priority: MemoryPriority,
}

/// Constants used by the Prime Thread Scheduler algorithm.
///
/// These control how threads are promoted/demoted to prime cores based on
/// their CPU activity levels.
#[derive(Debug, Clone)]
pub struct ConfigConstants {
    /// Minimum consecutive active intervals before a thread can be promoted (default: 2)
    pub min_active_streak: u8,
    /// Fraction of max cycles below which a prime thread is demoted (default: 69%)
    pub keep_threshold: f64,
    /// Fraction of max cycles above which a thread becomes prime candidate (default: 42%)
    pub entry_threshold: f64,
}

impl Default for ConfigConstants {
    fn default() -> Self {
        ConfigConstants {
            min_active_streak: 2,
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
        return String::from("0");
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
            if self.groups_count > 0 {
                log!("Parsed {} process groups ({} processes)", self.groups_count, self.group_members_count);
            }
            log!("Parsed {} process rules", self.process_rules_count);
            if !self.warnings.is_empty() {
                for warning in &self.warnings {
                    log_to_find(&format!("⚠ {}", warning));
                }
            }
        } else {
            for error in &self.errors {
                log_to_find(&format!("✗ {}", error));
            }
            for warning in &self.warnings {
                log_to_find(&format!("⚠ {}", warning));
            }
            log_to_find(&format!("Found {} error(s). Fix them before running.", self.errors.len()));
        }
    }
}

/// Helper function to resolve a CPU spec field (handles aliases and direct specs).
fn resolve_cpu_spec(spec: &str, field_name: &str, line_number: usize, cpu_aliases: &HashMap<String, Vec<u32>>, errors: &mut Vec<String>) -> Vec<u32> {
    let spec = spec.trim();
    if spec.starts_with('*') {
        let alias = spec.trim_start_matches('*').to_lowercase();
        if !cpu_aliases.contains_key(&alias) {
            errors.push(format!("Line {}: Undefined alias '*{}' in {} field", line_number, alias, field_name));
        }
        cpu_aliases.get(&alias).cloned().unwrap_or_default()
    } else {
        parse_cpu_spec(spec)
    }
}

/// Collects process names from comma-separated text into a vector.
fn collect_members(text: &str, members: &mut Vec<String>) {
    for item in text.split(':') {
        let item = item.trim().to_lowercase();
        if !item.is_empty() && !item.starts_with('#') {
            members.push(item);
        }
    }
}

/// Parses a constant definition and updates the result.
fn parse_constant(name: &str, value: &str, line_number: usize, result: &mut ConfigResult) {
    match name {
        "MIN_ACTIVE_STREAK" => {
            if let Ok(v) = value.parse::<u8>() {
                result.constants.min_active_streak = v;
                log_message(&format!("Config: MIN_ACTIVE_STREAK = {}", v));
                result.constants_count += 1;
            } else {
                result
                    .errors
                    .push(format!("Line {}: Invalid constant value '{}' for '{}' (expected u8)", line_number, value, name));
            }
        }
        "KEEP_THRESHOLD" | "ENTRY_THRESHOLD" => {
            if let Ok(v) = value.parse::<f64>() {
                match name {
                    "KEEP_THRESHOLD" => {
                        result.constants.keep_threshold = v;
                        log_message(&format!("Config: KEEP_THRESHOLD = {}", v));
                    }
                    "ENTRY_THRESHOLD" => {
                        result.constants.entry_threshold = v;
                        log_message(&format!("Config: ENTRY_THRESHOLD = {}", v));
                    }
                    _ => {}
                }
                result.constants_count += 1;
            } else {
                result.errors.push(format!("Line {}: Invalid constant value '{}' for '{}'", line_number, value, name));
            }
        }
        _ => {
            result.warnings.push(format!("Line {}: Unknown constant '{}' - will be ignored", line_number, name));
        }
    }
}

/// Parses a CPU alias definition and adds it to the aliases map.
fn parse_alias(name: &str, value: &str, line_number: usize, cpu_aliases: &mut HashMap<String, Vec<u32>>, result: &mut ConfigResult) {
    if name.is_empty() {
        result.errors.push(format!("Line {}: Empty alias name", line_number));
    } else {
        let cpus = parse_cpu_spec(value);
        if cpus.is_empty() && value != "0" {
            result
                .warnings
                .push(format!("Line {}: Alias '*{}' has empty CPU set from '{}'", line_number, name, value));
        }
        cpu_aliases.insert(name.to_string(), cpus);
        result.aliases_count += 1;
    }
}

/// Collects group members from lines until closing brace is found.
/// Returns (members, rule_suffix, next_line_index) or None if unclosed.
fn collect_group_block(lines: &[String], start_index: usize, first_line_content: &str) -> Option<(Vec<String>, Option<String>, usize)> {
    let mut members = Vec::new();
    let mut i = start_index;

    // Collect from first line (content after '{')
    if !first_line_content.is_empty() && !first_line_content.starts_with('#') {
        collect_members(first_line_content, &mut members);
    }

    // Continue to subsequent lines
    while i < lines.len() {
        let block_line = lines[i].trim();

        if let Some(pos) = block_line.find('}') {
            // Found closing brace
            let before = block_line[..pos].trim();
            if !before.is_empty() && !before.starts_with('#') {
                collect_members(before, &mut members);
            }
            let after = block_line[pos + 1..].trim();
            let suffix = if after.starts_with(':') { Some(after[1..].to_string()) } else { None };
            return Some((members, suffix, i + 1));
        }

        // Regular content line
        if !block_line.is_empty() && !block_line.starts_with('#') {
            collect_members(block_line, &mut members);
        }
        i += 1;
    }

    None // Unclosed block
}

/// Parses rule fields and creates ProcessConfig entries for all members.
/// This is the unified logic for both group definitions and single-line process rules.
///
/// # Arguments
/// * `members` - Process names to create configs for (single element for normal lines, multiple for groups)
/// * `rule_parts` - The rule fields: [priority, affinity, cpuset, prime, io, memory]
/// * `line_number` - For error reporting
/// * `cpu_aliases` - Resolved CPU aliases
/// * `result` - ConfigResult to add configs, errors, and warnings to
fn parse_and_insert_rules(members: &[String], rule_parts: &[&str], line_number: usize, cpu_aliases: &HashMap<String, Vec<u32>>, result: &mut ConfigResult) {
    if rule_parts.len() < 2 {
        result.errors.push(format!(
            "Line {}: Too few fields ({}) - expected at least 2 (priority,affinity)",
            line_number,
            rule_parts.len()
        ));
        return;
    }

    // Parse priority
    let priority_str = rule_parts[0].trim();
    let priority = ProcessPriority::from_str(priority_str);
    if priority == ProcessPriority::None && !priority_str.eq_ignore_ascii_case("none") {
        result
            .warnings
            .push(format!("Line {}: Unknown priority '{}' - will be treated as 'none'", line_number, priority_str));
    }

    // Parse affinity
    let affinity_cpus = resolve_cpu_spec(rule_parts[1], "affinity", line_number, cpu_aliases, &mut result.errors);

    // Parse cpuset (optional, defaults to "0")
    let cpu_set_cpus = if rule_parts.len() >= 3 {
        resolve_cpu_spec(rule_parts[2], "cpuset", line_number, cpu_aliases, &mut result.errors)
    } else {
        Vec::new()
    };

    // Parse prime_cpus (optional, defaults to "0") and prime_threads_prefixes (optional, defaults to "")
    // Supports multiple segments: *alias1@prefix1;prefix2;*alias2@prefix3!priority;...
    let (prime_threads_cpus, prime_threads_prefixes) = if rule_parts.len() >= 4 {
        let prime_spec = rule_parts[3].trim();
        if let Some(_) = prime_spec.find('@') {
            // Parse segments separated by '*' (each segment can have its own CPU alias)
            let mut all_prefixes: Vec<PrimePrefix> = Vec::new();
            let mut base_cpus: Vec<u32> = Vec::new();

            // Split by '*' to get segments, but keep track of positions
            let mut segments: Vec<&str> = Vec::new();

            if prime_spec.starts_with('*') {
                // First segment starts with '*'
                segments.push("");
            }

            for (idx, part) in prime_spec.split('*').enumerate() {
                if idx == 0 && !prime_spec.starts_with('*') {
                    // Handle case where spec doesn't start with '*'
                    segments.push(part);
                } else if !part.is_empty() {
                    segments.push(part);
                }
            }

            // Process each segment
            for segment in segments {
                if segment.is_empty() {
                    continue;
                }

                // Each segment format: alias@prefix1[!priority];prefix2[!priority];...
                if let Some(at_pos) = segment.find('@') {
                    let alias = segment[..at_pos].trim();

                    // Find the end of this segment's prefix list (before next '*' or end)
                    let remaining = &segment[at_pos + 1..];

                    // Resolve CPU alias for this segment (stored without '*' prefix, lowercase)
                    let segment_cpus = if alias.is_empty() {
                        Vec::new()
                    } else {
                        let alias_lower = alias.to_lowercase();
                        if let Some(alias_cpus) = cpu_aliases.get(alias_lower.as_str()) {
                            alias_cpus.clone()
                        } else {
                            result
                                .errors
                                .push(format!("Line {}: Unknown CPU alias '*{}' in prime specification", line_number, alias));
                            Vec::new()
                        }
                    };

                    // Use first segment's CPUs as base
                    if base_cpus.is_empty() {
                        base_cpus = segment_cpus.clone();
                    }

                    // Parse prefixes for this segment
                    for prefix_str in remaining.split(';') {
                        let prefix_str = prefix_str.trim();
                        if prefix_str.is_empty() {
                            continue;
                        }

                        // Check for !priority suffix
                        if let Some(bang_pos) = prefix_str.find('!') {
                            let prefix = prefix_str[..bang_pos].to_string();
                            let prio_str = &prefix_str[bang_pos + 1..];
                            let thread_prio = ThreadPriority::from_str(prio_str.trim());
                            if thread_prio == ThreadPriority::None && !prio_str.trim().eq_ignore_ascii_case("none") {
                                result.warnings.push(format!(
                                    "Line {}: Unknown thread priority '{}' in prefix - will be treated as 'none' (auto-boost)",
                                    line_number, prio_str
                                ));
                            }
                            all_prefixes.push(PrimePrefix {
                                prefix,
                                cpus: Some(segment_cpus.clone()),
                                thread_priority: thread_prio,
                            });
                        } else {
                            // No priority suffix, just prefix name
                            all_prefixes.push(PrimePrefix {
                                prefix: prefix_str.to_string(),
                                cpus: Some(segment_cpus.clone()),
                                thread_priority: ThreadPriority::None,
                            });
                        }
                    }
                }
            }

            if all_prefixes.is_empty() {
                all_prefixes.push(PrimePrefix {
                    prefix: "".to_string(),
                    cpus: None,
                    thread_priority: ThreadPriority::None,
                });
            }

            (base_cpus, all_prefixes)
        } else {
            // No '@' found, treat as simple CPU spec
            let cpus = resolve_cpu_spec(prime_spec, "prime_cpus", line_number, cpu_aliases, &mut result.errors);
            (
                cpus,
                vec![PrimePrefix {
                    prefix: "".to_string(),
                    cpus: None,
                    thread_priority: ThreadPriority::None,
                }],
            )
        }
    } else {
        (
            Vec::new(),
            vec![PrimePrefix {
                prefix: "".to_string(),
                cpus: None,
                thread_priority: ThreadPriority::None,
            }],
        )
    };

    // Parse io_priority (optional, defaults to None)
    let io_priority = if rule_parts.len() >= 5 {
        let io_str = rule_parts[4].trim();
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

    // Parse memory_priority (optional, defaults to None)
    let memory_priority = if rule_parts.len() >= 6 {
        let mem_str = rule_parts[5].trim();
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

    // Create ProcessConfig for each member
    for name in members {
        result.configs.insert(
            name.clone(),
            ProcessConfig {
                name: name.clone(),
                priority: priority.clone(),
                affinity_cpus: affinity_cpus.clone(),
                cpu_set_cpus: cpu_set_cpus.clone(),
                prime_threads_cpus: prime_threads_cpus.clone(),
                prime_threads_prefixes: prime_threads_prefixes.clone(),
                io_priority: io_priority.clone(),
                memory_priority: memory_priority.clone(),
            },
        );
    }
    result.process_rules_count += members.len();
}

/// Reads and validates the config file.
/// Supports:
/// - `@CONSTANT = value` - Define a constant
/// - `*alias = cpu_spec` - Define a CPU alias
/// - `[name] { ... },priority,affinity,...` - Process group (name optional)
/// - `name,priority,affinity,cpuset,prime[@prefixes],io,memory` - Single process rule
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

        // Constant: @NAME = value
        if line.starts_with('@') {
            if let Some(eq_pos) = line.find('=') {
                parse_constant(&line[1..eq_pos].trim().to_uppercase(), line[eq_pos + 1..].trim(), line_number, &mut result);
            } else {
                result.errors.push(format!("Line {}: Invalid constant - expected '@NAME = value'", line_number));
            }
            i += 1;
            continue;
        }

        // Alias: *name = cpu_spec
        if line.starts_with('*') {
            if let Some(eq_pos) = line.find('=') {
                parse_alias(
                    &line[1..eq_pos].trim().to_lowercase(),
                    line[eq_pos + 1..].trim(),
                    line_number,
                    &mut cpu_aliases,
                    &mut result,
                );
            } else {
                result.errors.push(format!("Line {}: Invalid alias - expected '*name = cpu_spec'", line_number));
            }
            i += 1;
            continue;
        }

        // Group: [name] { members }:rule  OR  Single: name:rule
        if let Some(brace_start) = line.find('{') {
            let group_name = line[..brace_start].trim();
            let group_label = if group_name.is_empty() {
                format!("anonymous@L{}", line_number)
            } else {
                group_name.to_lowercase()
            };

            // Single-line group: { a: b }:rule
            let (members, rule_suffix, next_i) = if let Some(brace_end) = line.find('}') {
                let mut members = Vec::new();
                collect_members(&line[brace_start + 1..brace_end], &mut members);
                let after = line[brace_end + 1..].trim();
                let suffix = after.strip_prefix(':').map(|s| s.to_string());
                (members, suffix, i + 1)
            } else {
                // Multi-line group
                let first_content = line[brace_start + 1..].trim();
                match collect_group_block(&lines, i + 1, first_content) {
                    Some((members, suffix, next)) => (members, suffix, next),
                    None => {
                        result.errors.push(format!("Line {}: Unclosed group '{}' - missing }}", line_number, group_label));
                        i += 1;
                        continue;
                    }
                }
            };

            i = next_i;

            if members.is_empty() {
                result.warnings.push(format!("Line {}: Group '{}' has no members", line_number, group_label));
                continue;
            }

            result.groups_count += 1;
            result.group_members_count += members.len();

            if let Some(suffix) = rule_suffix {
                let rule_parts: Vec<&str> = suffix.split(':').collect();
                parse_and_insert_rules(&members, &rule_parts, line_number, &cpu_aliases, &mut result);
            } else {
                result
                    .errors
                    .push(format!("Line {}: Group '{}' missing rule - use }}:priority:affinity,...", line_number, group_label));
            }
        } else {
            // Single process line
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() < 3 {
                result
                    .errors
                    .push(format!("Line {}: Too few fields - expected name:priority:affinity,...", line_number));
                i += 1;
                continue;
            }

            let name = parts[0].trim();
            if name.is_empty() {
                result.errors.push(format!("Line {}: Empty process name", line_number));
                i += 1;
                continue;
            }

            parse_and_insert_rules(&[name.to_lowercase()], &parts[1..], line_number, &cpu_aliases, &mut result);
            i += 1;
        }
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
            log!("Error: -in <file> is required for -convert");
            return;
        }
    };
    let out_path = match out_file {
        Some(p) => p,
        None => {
            log!("Error: -out <file> is required for -convert");
            return;
        }
    };

    let content = match read_utf16le_file(&in_path) {
        Ok(c) => c,
        Err(e) => {
            log!("Failed to read {}: {}", in_path, e);
            return;
        }
    };

    let mut output_lines: Vec<String> = Vec::new();

    // Prepend config help with # comment prefix
    for help_line in get_config_help_lines() {
        if help_line.is_empty() {
            output_lines.push("#".to_string());
        } else {
            output_lines.push(format!("# {}", help_line));
        }
    }
    output_lines.push(String::new());
    output_lines.push("# Converted from Process Lasso config".to_string());
    output_lines.push(String::new());

    let mut priorities: HashMap<String, String> = HashMap::new();
    let mut affinities: HashMap<String, String> = HashMap::new();
    let mut named_affinities: Vec<(String, String)> = Vec::new();

    // Process Lasso stores data in these formats:
    // 1. Single line: DefaultPriorities=proc1.exe,priority1,proc2.exe,priority2,...
    // 2. Single line: DefaultAffinitiesEx=proc1.exe,mask,cpuset,proc2.exe,mask,cpuset,...
    // 3. Single line: NamedAffinities=alias1,cpuspec1,alias2,cpuspec2,...
    for line in content.lines() {
        let line = line.trim();

        // Parse NamedAffinities=alias,cpuspec,alias,cpuspec,...
        if line.starts_with("NamedAffinities=") {
            let value = &line["NamedAffinities=".len()..];
            let parts: Vec<&str> = value.split(',').collect();
            // Pairs of (alias_name, cpu_spec)
            let mut i = 0;
            while i + 1 < parts.len() {
                let alias_name = parts[i].trim();
                let cpu_spec = parts[i + 1].trim();
                if !alias_name.is_empty() && !cpu_spec.is_empty() {
                    named_affinities.push((alias_name.to_string(), cpu_spec.to_string()));
                }
                i += 2;
            }
        }

        // Parse DefaultPriorities=name,priority,name,priority,...
        if line.starts_with("DefaultPriorities=") {
            let value = &line["DefaultPriorities=".len()..];
            let parts: Vec<&str> = value.split(',').collect();
            // Pairs of (name, priority)
            let mut i = 0;
            while i + 1 < parts.len() {
                let name = parts[i].trim().to_lowercase();
                let priority = parts[i + 1].trim();
                if !name.is_empty() {
                    priorities.insert(name, priority.to_string());
                }
                i += 2;
            }
        }

        // Parse DefaultAffinitiesEx=name,mask,cpuset,name,mask,cpuset,...
        if line.starts_with("DefaultAffinitiesEx=") {
            let value = &line["DefaultAffinitiesEx=".len()..];
            let parts: Vec<&str> = value.split(',').collect();
            // Triplets of (name, mask, cpuset) - we use cpuset (index 2)
            let mut i = 0;
            while i + 2 < parts.len() {
                let name = parts[i].trim().to_lowercase();
                let _mask = parts[i + 1].trim(); // legacy mask, usually 0
                let cpuset = parts[i + 2].trim(); // the actual CPU range
                if !name.is_empty() && cpuset != "0" && !cpuset.is_empty() {
                    affinities.insert(name, cpuset.to_string());
                }
                i += 3;
            }
        }
    }

    // Build a reverse lookup: cpu_spec -> alias_name (for replacing specs with aliases)
    let mut spec_to_alias: HashMap<String, String> = HashMap::new();
    for (alias_name, cpu_spec) in &named_affinities {
        spec_to_alias.insert(cpu_spec.clone(), format!("*{}", alias_name));
    }

    // Output CPU aliases from NamedAffinities
    if !named_affinities.is_empty() {
        output_lines.push("# CPU Aliases (from Process Lasso NamedAffinities)".to_string());
        for (alias_name, cpu_spec) in &named_affinities {
            output_lines.push(format!("*{} = {}", alias_name, cpu_spec));
        }
        output_lines.push(String::new());
    }

    // Merge priorities and affinities
    let mut all_processes: std::collections::HashSet<String> = priorities.keys().cloned().collect();
    all_processes.extend(affinities.keys().cloned());

    // Sort process names for consistent output
    let mut sorted_processes: Vec<String> = all_processes.into_iter().collect();
    sorted_processes.sort();

    for name in sorted_processes {
        let priority = priorities.get(&name).map(|s| s.as_str()).unwrap_or("none");
        let affinity_raw = affinities.get(&name).map(|s| s.as_str()).unwrap_or("0");
        // Replace CPU spec with alias if it matches
        let affinity = spec_to_alias.get(affinity_raw).map(|s| s.as_str()).unwrap_or(affinity_raw);

        // Convert Process Lasso priority to our format
        // Process Lasso uses text like "high", "above normal", etc. directly
        let priority_str = match priority.to_lowercase().as_str() {
            "idle" => "idle",
            "below normal" => "below normal",
            "normal" => "normal",
            "above normal" => "above normal",
            "high" => "high",
            "realtime" | "real time" => "real time",
            // Also handle numeric values
            "1" => "idle",
            "2" => "below normal",
            "3" => "normal",
            "4" => "above normal",
            "5" => "high",
            "6" => "real time",
            _ => "none",
        };

        output_lines.push(format!("{}:{}:{}:0:0:none:none", name, priority_str, affinity));
    }

    log!(
        "Parsed {} aliases, {} priorities, {} affinities",
        named_affinities.len(),
        priorities.len(),
        affinities.len()
    );

    // Write output file
    let mut out = match File::create(&out_path) {
        Ok(f) => f,
        Err(e) => {
            log!("Failed to create {}: {}", out_path, e);
            return;
        }
    };

    for line in output_lines {
        if writeln!(out, "{}", line).is_err() {
            log!("Failed to write to {}", out_path);
            return;
        }
    }

    log!("Converted {} to {}", in_path, out_path);
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
