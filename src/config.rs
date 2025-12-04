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
    /// CPU indices for legacy process-wide affinity. Empty = don't change.
    pub affinity_cpus: Vec<u32>,
    /// CPU indices for process default CPU sets (Windows 10+). Empty = don't change.
    pub cpu_set_cpus: Vec<u32>,
    /// CPU indices for Prime Thread Scheduler. Active threads get pinned to these cores.
    pub prime_cpus: Vec<u32>,
    pub io_priority: IOPriority,
    pub memory_priority: MemoryPriority,
}

/// Tuning parameters for Prime Thread Scheduler's hysteresis behavior.
///
/// The two thresholds create a "sticky" promotion system:
/// - `entry_threshold` (lower): new threads must exceed this to become prime candidates
/// - `keep_threshold` (higher): existing prime threads stay prime if they exceed this
///
/// This gap prevents rapid promote/demote oscillation when thread activity fluctuates.
/// Set via config.ini: `@KEEP_THRESHOLD=0.69`, `@ENTRY_THRESHOLD=0.42`
#[derive(Debug, Clone, Copy)]
pub struct ConfigConstants {
    /// Reserved for future use.
    pub hysteresis_ratio: f64,
    /// Thread keeps prime status if cycles >= max_cycles * keep_threshold (default 0.69).
    pub keep_threshold: f64,
    /// Thread can become prime if cycles >= max_cycles * entry_threshold (default 0.42).
    pub entry_threshold: f64,
}

impl Default for ConfigConstants {
    fn default() -> Self {
        Self {
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
/// - Decimal mask (≤64 cores): `255`
/// - Single CPU index: `5`
/// - Range: `0-7`
/// - Multiple ranges with semicolons: `0-7;64-71`
/// - Mixed: `0-3;8;9;64-67`
///
/// # Examples
/// ```
/// parse_cpu_spec("0xFF")       // -> [0, 1, 2, 3, 4, 5, 6, 7]
/// parse_cpu_spec("0-7;64-71")  // -> [0, 1, 2, 3, 4, 5, 6, 7, 64, 65, 66, 67, 68, 69, 70, 71]
/// parse_cpu_spec("0;4;8")      // -> [0, 4, 8]
/// ```
pub fn parse_cpu_spec(s: &str) -> Vec<u32> {
    let s = s.trim();

    if s.is_empty() || s == "0" {
        return Vec::new();
    }

    // Check for hex format: 0x... or 0X...
    if s.starts_with("0x") || s.starts_with("0X") {
        if let Ok(mask) = u64::from_str_radix(&s[2..], 16) {
            return mask_to_cpu_indices(mask);
        }
        return Vec::new();
    }

    // Check if it's a plain decimal number (could be a mask for backward compat)
    // Only treat as mask if it doesn't contain semicolons or dashes
    if !s.contains(';') && !s.contains('-') {
        if let Ok(value) = s.parse::<u64>() {
            // If it's a small number (< 256), could be either a mask or a single CPU index
            // For backward compatibility, treat numbers >= 2 as masks if they could represent multiple CPUs
            // Single CPU indices should use the range format: "5" as mask vs "5;5" or just document it
            // Actually, for full backward compat, treat any plain number as a mask
            return mask_to_cpu_indices(value);
        }
    }

    // Parse range/index format: "0-7;64-71;128"
    let mut cpus = Vec::new();
    for part in s.split(';') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        if let Some((start, end)) = part.split_once('-') {
            if let (Ok(start), Ok(end)) = (start.trim().parse::<u32>(), end.trim().parse::<u32>()) {
                cpus.extend(start..=end);
            }
        } else if let Ok(cpu) = part.parse::<u32>() {
            cpus.push(cpu);
        }
    }
    cpus.sort();
    cpus.dedup();
    cpus
}

/// Converts a bitmask to a vector of CPU indices.
fn mask_to_cpu_indices(mask: u64) -> Vec<u32> {
    (0..64).filter(|i| (mask >> i) & 1 != 0).collect()
}

/// Converts CPU indices to a bitmask (for ≤64 CPUs, used for legacy API).
/// Returns 0 if any CPU index >= 64 (cannot be represented in a single mask).
pub fn cpu_indices_to_mask(cpus: &[u32]) -> usize {
    if cpus.is_empty() {
        return 0;
    }
    // Check if all CPUs fit in a single 64-bit mask
    if cpus.iter().any(|&cpu| cpu >= 64) {
        // For now, return mask of only the CPUs that fit
        // The caller should use CPU Set APIs for full support
        cpus.iter().filter(|&&cpu| cpu < 64).fold(0usize, |mask, &cpu| mask | (1 << cpu))
    } else {
        cpus.iter().fold(0usize, |mask, &cpu| mask | (1 << cpu))
    }
}

/// Formats CPU indices as a human-readable string.
/// Used for logging.
pub fn format_cpu_indices(cpus: &[u32]) -> String {
    if cpus.is_empty() {
        return "none".to_string();
    }
    if cpus.len() <= 8 {
        // Short list: just show the indices
        cpus.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(",")
    } else {
        // Long list: show as ranges
        let mut result = String::new();
        let mut i = 0;
        while i < cpus.len() {
            let start = cpus[i];
            let mut end = start;
            while i + 1 < cpus.len() && cpus[i + 1] == end + 1 {
                i += 1;
                end = cpus[i];
            }
            if !result.is_empty() {
                result.push(';');
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
}

/// Reads process configuration from an INI-style file.
///
/// # Format
/// - `# comment` - Lines starting with `#` are ignored
/// - `@CONSTANT=value` - Set scheduler constants (KEEP_THRESHOLD, ENTRY_THRESHOLD)
/// - `*alias=spec` - Define a reusable CPU spec alias (e.g., `*pcore=0-7;64-71`)
/// - `name,priority,affinity,cpuset,prime,io,memory` - Process config (only first 3 required)
pub fn read_config<P: AsRef<Path>>(path: P) -> io::Result<(HashMap<String, ProcessConfig>, ConfigConstants)> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut configs = HashMap::new();
    let mut cpu_aliases: HashMap<String, Vec<u32>> = HashMap::new();
    let mut constants = ConfigConstants::default();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        } else if line.starts_with('@') {
            // define constant: @NAME=VALUE
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
            continue;
        } else if line.starts_with('*') {
            // define CPU alias: *NAME=SPEC
            if let Some(eq_pos) = line.find('=') {
                let alias_name = line[1..eq_pos].trim().to_lowercase();
                let alias_value = line[eq_pos + 1..].trim();
                let cpus = parse_cpu_spec(alias_value);
                cpu_aliases.insert(alias_name, cpus);
            }
            continue;
        }

        // process configuration line
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() >= 3 {
            let name = parts[0].to_lowercase();
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

            configs.insert(
                name.clone(),
                ProcessConfig {
                    name,
                    priority,
                    affinity_cpus,
                    cpu_set_cpus,
                    prime_cpus,
                    io_priority,
                    memory_priority,
                },
            );
        }
    }
    Ok((configs, constants))
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
pub fn convert(in_file_name: Option<String>, out_file_name: Option<String>) {
    if let Some(ref in_file) = in_file_name {
        if let Some(ref out_file) = out_file_name {
            match read_utf16le_file(in_file) {
                Ok(in_content) => {
                    let mut configs: Vec<ProcessConfig> = Vec::new();
                    for line in in_content.lines() {
                        let line = line.trim();
                        if line.is_empty() {
                            continue;
                        }
                        if let Some(rest) = line.strip_prefix("DefaultPriorities=") {
                            let fields = split_trim_nonempty(rest);
                            for chunk in fields.chunks(2) {
                                if chunk.len() == 2 {
                                    let name = chunk[0].to_lowercase();
                                    let priority = ProcessPriority::from_str(chunk[1]);
                                    match configs.iter_mut().find(|c| c.name == name) {
                                        Some(cfg) => cfg.priority = priority,
                                        None => configs.push(ProcessConfig {
                                            name,
                                            priority,
                                            affinity_cpus: Vec::new(),
                                            cpu_set_cpus: Vec::new(),
                                            prime_cpus: Vec::new(),
                                            io_priority: IOPriority::None,
                                            memory_priority: MemoryPriority::None,
                                        }),
                                    }
                                } else {
                                    log_message(&format!("Invalid priority configuration line: {}", line));
                                }
                            }
                        } else if let Some(rest) = line.strip_prefix("DefaultAffinitiesEx=") {
                            let fields = split_trim_nonempty(rest);
                            for chunk in fields.chunks(3) {
                                if chunk.len() == 3 {
                                    let name = chunk[0].to_lowercase();
                                    let cpus = parse_cpu_spec(chunk[2]);
                                    match configs.iter_mut().find(|c| c.name == name) {
                                        Some(cfg) => cfg.affinity_cpus = cpus,
                                        None => configs.push(ProcessConfig {
                                            name,
                                            priority: ProcessPriority::None,
                                            affinity_cpus: cpus,
                                            cpu_set_cpus: Vec::new(),
                                            prime_cpus: Vec::new(),
                                            io_priority: IOPriority::None,
                                            memory_priority: MemoryPriority::None,
                                        }),
                                    }
                                } else {
                                    log_message(&format!("Invalid affinity configuration line: {}", line));
                                }
                            }
                        }
                    }
                    match File::create(out_file) {
                        Ok(mut output) => {
                            for cfg in &configs {
                                let affinity_str = format_cpu_indices(&cfg.affinity_cpus);
                                let cpuset_str = format_cpu_indices(&cfg.cpu_set_cpus);
                                let prime_str = format_cpu_indices(&cfg.prime_cpus);
                                let _ = writeln!(
                                    output,
                                    "{},{},{},{},{},{}",
                                    cfg.name,
                                    cfg.priority.as_str(),
                                    if cfg.affinity_cpus.is_empty() { "0".to_string() } else { affinity_str },
                                    if cfg.cpu_set_cpus.is_empty() { "0".to_string() } else { cpuset_str },
                                    if cfg.prime_cpus.is_empty() { "0".to_string() } else { prime_str },
                                    cfg.io_priority.as_str()
                                );
                            }
                            log_message(&format!("convert done, {} process configs have been output", configs.len()));
                        }
                        Err(_) => {
                            log_message(&format!("cannot create output file: {}", out_file));
                        }
                    }
                }
                Err(_) => {
                    log_message(&format!("cannot read from file: {}", in_file));
                }
            }
        } else {
            log_message("no output file (-out <file>)!");
        }
    } else {
        log_message("no input file (-in <file>)!");
    };
}

#[inline]
fn split_trim_nonempty(s: &str) -> Vec<&str> {
    s.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cpu_spec_hex() {
        assert_eq!(parse_cpu_spec("0xFF"), vec![0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(parse_cpu_spec("0x0F"), vec![0, 1, 2, 3]);
        assert_eq!(parse_cpu_spec("0x1"), vec![0]);
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
        assert_eq!(cpu_indices_to_mask(&[]), 0);
    }

    #[test]
    fn test_format_cpu_indices() {
        assert_eq!(format_cpu_indices(&[0, 1, 2, 3]), "0,1,2,3");
        assert_eq!(format_cpu_indices(&[0, 1, 2, 3, 4, 5, 6, 7, 8]), "0-8");
        assert_eq!(format_cpu_indices(&[0, 1, 2, 5, 6, 7, 10, 11, 12]), "0-2;5-7;10-12");
        assert_eq!(format_cpu_indices(&[]), "none");
    }
}
