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

#[derive(Debug, Clone)]
pub struct PrimePrefix {
    pub prefix: String,
    pub cpus: Option<Vec<u32>>,
    pub thread_priority: ThreadPriority,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct IdealProcessorPrefix {
    pub prefix: String,
    pub cpus: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct IdealProcessorRule {
    pub cpus: Vec<u32>,
    pub prefixes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ProcessConfig {
    pub name: String,
    pub priority: ProcessPriority,
    pub affinity_cpus: Vec<u32>,
    pub cpu_set_cpus: Vec<u32>,
    pub prime_threads_cpus: Vec<u32>,
    pub prime_threads_prefixes: Vec<PrimePrefix>,
    pub track_top_x_threads: i32,
    pub io_priority: IOPriority,
    pub memory_priority: MemoryPriority,
    pub ideal_processor_rules: Vec<IdealProcessorRule>,
}

#[derive(Debug, Clone)]
pub struct ConfigConstants {
    pub min_active_streak: u8,
    pub keep_threshold: f64,
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

pub fn parse_cpu_spec(s: &str) -> Vec<u32> {
    let s = s.trim();
    if s.is_empty() || s == "0" {
        return Vec::new();
    }

    if s.starts_with("0x") || s.starts_with("0X") {
        if let Ok(mask) = u64::from_str_radix(&s[2..], 16) {
            return mask_to_cpu_indices(mask);
        }
        return Vec::new();
    }

    let mut cpus = Vec::new();
    for part in s.split(';') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        if let Some(dash_pos) = part.find('-') {
            let start: u32 = part[..dash_pos].trim().parse().unwrap_or(0);
            let end: u32 = part[dash_pos + 1..].trim().parse().unwrap_or(start);
            for cpu in start..=end {
                if !cpus.contains(&cpu) {
                    cpus.push(cpu);
                }
            }
        } else if let Ok(cpu) = part.parse::<u32>() {
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

pub fn cpu_indices_to_mask(cpus: &[u32]) -> usize {
    let mut mask: usize = 0;
    for &cpu in cpus {
        if cpu < 64 {
            mask |= 1usize << cpu;
        }
    }
    mask
}

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

#[derive(Debug, Default)]
pub struct ConfigResult {
    pub configs: HashMap<u32, HashMap<String, ProcessConfig>>,
    pub constants: ConfigConstants,
    pub constants_count: usize,
    pub aliases_count: usize,
    pub groups_count: usize,
    pub group_members_count: usize,
    pub process_rules_count: usize,
    pub redundant_rules_count: usize,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ConfigResult {
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn total_rules(&self) -> usize {
        self.configs.values().map(|grade_configs| grade_configs.len()).sum()
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
        if self.redundant_rules_count > 0 || !self.is_valid() {
            for warning in &self.warnings {
                log_to_find(&format!("⚠ {}", warning));
            }
        }
    }
}

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

fn collect_members(text: &str, members: &mut Vec<String>) {
    for item in text.split(':') {
        let item = item.trim().to_lowercase();
        if !item.is_empty() && !item.starts_with('#') {
            members.push(item);
        }
    }
}

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

fn parse_ideal_processor_spec(spec: &str, line_number: usize, cpu_aliases: &HashMap<String, Vec<u32>>, errors: &mut Vec<String>) -> Vec<IdealProcessorRule> {
    let spec = spec.trim();
    if spec.is_empty() || spec == "0" {
        return Vec::new();
    }

    if !spec.starts_with('*') {
        errors.push(format!("Line {}: Ideal processor spec must start with '*', got '{}'", line_number, spec));
        return Vec::new();
    }

    let mut rules = Vec::new();

    for segment in spec.split('*').skip(1) {
        if segment.is_empty() {
            continue;
        }

        let (alias_part, prefixes_str) = match segment.find('@') {
            Some(at_pos) => (&segment[..at_pos], &segment[at_pos + 1..]),
            None => (segment, ""),
        };

        let alias = alias_part.trim().to_lowercase();
        if alias.is_empty() {
            errors.push(format!("Line {}: Empty alias in ideal processor rule '*{}'", line_number, segment));
            continue;
        }

        let cpus = if let Some(alias_cpus) = cpu_aliases.get(&alias) {
            alias_cpus.clone()
        } else {
            errors.push(format!("Line {}: Unknown CPU alias '*{}' in ideal processor specification", line_number, alias));
            Vec::new()
        };

        if cpus.is_empty() {
            continue;
        }

        let prefixes: Vec<String> = prefixes_str.split(';').map(|p| p.trim().to_lowercase()).filter(|p| !p.is_empty()).collect();

        rules.push(IdealProcessorRule { cpus, prefixes });
    }

    rules
}

fn collect_group_block(lines: &[String], start_index: usize, first_line_content: &str) -> Option<(Vec<String>, Option<String>, usize)> {
    let mut members = Vec::new();
    let mut i = start_index;

    if !first_line_content.is_empty() && !first_line_content.starts_with('#') {
        collect_members(first_line_content, &mut members);
    }

    while i < lines.len() {
        let block_line = lines[i].trim();

        if let Some(pos) = block_line.find('}') {
            let before = block_line[..pos].trim();
            if !before.is_empty() && !before.starts_with('#') {
                collect_members(before, &mut members);
            }
            let after = block_line[pos + 1..].trim();
            let suffix = after.strip_prefix(':').map(|s| s.to_string());
            return Some((members, suffix, i + 1));
        }

        if !block_line.is_empty() && !block_line.starts_with('#') {
            collect_members(block_line, &mut members);
        }
        i += 1;
    }

    None
}

fn parse_and_insert_rules(members: &[String], rule_parts: &[&str], line_number: usize, cpu_aliases: &HashMap<String, Vec<u32>>, result: &mut ConfigResult) {
    if rule_parts.len() < 2 {
        result.errors.push(format!(
            "Line {}: Too few fields ({}) - expected at least 2 (priority,affinity)",
            line_number,
            rule_parts.len()
        ));
        return;
    }

    let priority_str = rule_parts[0].trim();
    let priority = ProcessPriority::from_str(priority_str);
    if priority == ProcessPriority::None && !priority_str.eq_ignore_ascii_case("none") {
        result
            .warnings
            .push(format!("Line {}: Unknown priority '{}' - will be treated as 'none'", line_number, priority_str));
    }

    let affinity_cpus = resolve_cpu_spec(rule_parts[1], "affinity", line_number, cpu_aliases, &mut result.errors);

    let cpu_set_cpus = if rule_parts.len() >= 3 {
        resolve_cpu_spec(rule_parts[2], "cpuset", line_number, cpu_aliases, &mut result.errors)
    } else {
        Vec::new()
    };

    let (prime_threads_cpus, prime_threads_prefixes, track_top_x_threads) = if rule_parts.len() >= 4 {
        let mut prime_spec = rule_parts[3].trim();
        let mut track_top_x_threads = 0;
        if prime_spec == "0" {
            (Vec::<u32>::new(), Vec::new(), 0)
        } else {
            if prime_spec.starts_with("??") {
                let rest = &prime_spec[2..];
                let end_idx = rest.find(|c: char| !c.is_ascii_digit()).unwrap_or(rest.len());
                if let Ok(val) = rest[..end_idx].parse::<i32>() {
                    track_top_x_threads = -val;
                    prime_spec = &rest[end_idx..];
                    if prime_spec.starts_with('x') || prime_spec.starts_with('X') {
                        prime_spec = &prime_spec[1..];
                    }
                }
            } else if prime_spec.starts_with('?') {
                let rest = &prime_spec[1..];
                let end_idx = rest.find(|c: char| !c.is_ascii_digit()).unwrap_or(rest.len());
                if let Ok(val) = rest[..end_idx].parse::<i32>() {
                    track_top_x_threads = val;
                    prime_spec = &rest[end_idx..];
                    if prime_spec.starts_with('x') || prime_spec.starts_with('X') {
                        prime_spec = &prime_spec[1..];
                    }
                }
            }

            if prime_spec.find('@').is_some() {
                let mut all_prefixes: Vec<PrimePrefix> = Vec::new();
                let mut base_cpus: Vec<u32> = Vec::new();

                let mut segments: Vec<&str> = Vec::new();

                if prime_spec.starts_with('*') {
                    segments.push("");
                }

                for (idx, part) in prime_spec.split('*').enumerate() {
                    if idx == 0 && !prime_spec.starts_with('*') {
                        segments.push(part);
                    } else if !part.is_empty() {
                        segments.push(part);
                    }
                }

                for segment in segments {
                    if segment.is_empty() {
                        continue;
                    }

                    if let Some(at_pos) = segment.find('@') {
                        let alias = segment[..at_pos].trim();

                        let remaining = &segment[at_pos + 1..];

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

                        for cpu in &segment_cpus {
                            if !base_cpus.contains(cpu) {
                                base_cpus.push(*cpu);
                            }
                        }

                        for prefix_str in remaining.split(';') {
                            let prefix_str = prefix_str.trim();
                            if prefix_str.is_empty() {
                                continue;
                            }

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

                (base_cpus, all_prefixes, track_top_x_threads)
            } else {
                let cpus = resolve_cpu_spec(prime_spec, "prime_cpus", line_number, cpu_aliases, &mut result.errors);
                (
                    cpus,
                    vec![PrimePrefix {
                        prefix: "".to_string(),
                        cpus: None,
                        thread_priority: ThreadPriority::None,
                    }],
                    track_top_x_threads,
                )
            }
        }
    } else {
        (
            Vec::new(),
            vec![PrimePrefix {
                prefix: "".to_string(),
                cpus: None,
                thread_priority: ThreadPriority::None,
            }],
            0,
        )
    };

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

    let (ideal_processor_rules, grade) = if rule_parts.len() >= 7 {
        let field6 = rule_parts[6].trim();

        if field6.starts_with('*') || field6 == "0" {
            let ideal = parse_ideal_processor_spec(field6, line_number, cpu_aliases, &mut result.errors);
            let g = if rule_parts.len() >= 8 {
                let grade_str = rule_parts[7].trim();
                match grade_str.parse::<u32>() {
                    Ok(val) if val >= 1 => val,
                    Ok(0) => {
                        result.warnings.push(format!("Line {}: Grade cannot be 0, using 1 instead", line_number));
                        1
                    }
                    _ => {
                        result.warnings.push(format!("Line {}: Invalid grade '{}', using 1", line_number, grade_str));
                        1
                    }
                }
            } else {
                1
            };
            (ideal, g)
        } else if let Ok(g) = field6.parse::<u32>() {
            if g == 0 {
                result.warnings.push(format!("Line {}: Grade cannot be 0, using 1 instead", line_number));
                (Vec::new(), 1)
            } else {
                (Vec::new(), g)
            }
        } else {
            let ideal = parse_ideal_processor_spec(field6, line_number, cpu_aliases, &mut result.errors);
            (ideal, 1)
        }
    } else {
        (Vec::new(), 1)
    };

    for name in members {
        if result.configs.values().any(|f| f.contains_key(name)) {
            result.redundant_rules_count += 1;
            result.warnings.push(format!(
                "Line {}: Redundant rule - '{}' already defined (previous definition will be overwritten)",
                line_number, name
            ));
        }

        result.configs.entry(grade).or_default().insert(
            name.clone(),
            ProcessConfig {
                name: name.clone(),
                priority,
                affinity_cpus: affinity_cpus.clone(),
                cpu_set_cpus: cpu_set_cpus.clone(),
                prime_threads_cpus: prime_threads_cpus.clone(),
                prime_threads_prefixes: prime_threads_prefixes.clone(),
                track_top_x_threads,
                io_priority,
                memory_priority,
                ideal_processor_rules: ideal_processor_rules.clone(),
            },
        );
    }
    result.process_rules_count += members.len();
}

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
    let lines: Vec<String> = reader.lines().map_while(Result::ok).collect();
    let mut i = 0;

    while i < lines.len() {
        let line_number = i + 1;
        let line = lines[i].trim();

        if line.is_empty() || line.starts_with('#') {
            i += 1;
            continue;
        }

        if line.starts_with('@') {
            if let Some(eq_pos) = line.find('=') {
                parse_constant(&line[1..eq_pos].trim().to_uppercase(), line[eq_pos + 1..].trim(), line_number, &mut result);
            } else {
                result.errors.push(format!("Line {}: Invalid constant - expected '@NAME = value'", line_number));
            }
            i += 1;
            continue;
        }

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

        if let Some(brace_start) = line.find('{') {
            let group_name = line[..brace_start].trim();
            let group_label = if group_name.is_empty() {
                format!("anonymous@L{}", line_number)
            } else {
                group_name.to_lowercase()
            };

            let (members, rule_suffix, next_i) = if let Some(brace_end) = line.find('}') {
                let mut members = Vec::new();
                collect_members(&line[brace_start + 1..brace_end], &mut members);
                let after = line[brace_end + 1..].trim();
                let suffix = after.strip_prefix(':').map(|s| s.to_string());
                (members, suffix, i + 1)
            } else {
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

pub fn read_list<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    Ok(reader
        .lines()
        .map_while(Result::ok)
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty() && !s.starts_with('#'))
        .collect())
}

pub fn read_utf16le_file(path: &str) -> io::Result<String> {
    let bytes = fs::read(path)?;
    let utf16: Vec<u16> = bytes.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
    Ok(String::from_utf16_lossy(&utf16))
}

#[allow(dead_code)]
pub fn parse_mask(s: &str) -> usize {
    let cpus = parse_cpu_spec(s);
    cpu_indices_to_mask(&cpus)
}

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

    for help_line in get_config_help_lines() {
        output_lines.push(help_line.to_string());
    }
    output_lines.push(String::new());
    output_lines.push("# Converted from Process Lasso config".to_string());
    output_lines.push(String::new());

    let mut priorities: HashMap<String, String> = HashMap::new();
    let mut affinities: HashMap<String, String> = HashMap::new();
    let mut named_affinities: Vec<(String, String)> = Vec::new();

    for line in content.lines() {
        let line = line.trim();

        if line.starts_with("NamedAffinities=") {
            let value = line.strip_prefix("NamedAffinities=").unwrap();
            let parts: Vec<&str> = value.split(',').collect();

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

        if line.starts_with("DefaultPriorities=") {
            let value = line.strip_prefix("DefaultPriorities=").unwrap();
            let parts: Vec<&str> = value.split(',').collect();

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

        if line.starts_with("DefaultAffinitiesEx=") {
            let value = line.strip_prefix("DefaultAffinitiesEx=").unwrap();
            let parts: Vec<&str> = value.split(',').collect();

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

    let mut spec_to_alias: HashMap<String, String> = HashMap::new();
    for (alias_name, cpu_spec) in &named_affinities {
        spec_to_alias.insert(cpu_spec.clone(), format!("*{}", alias_name));
    }

    if !named_affinities.is_empty() {
        output_lines.push("# CPU Aliases (from Process Lasso NamedAffinities)".to_string());
        for (alias_name, cpu_spec) in &named_affinities {
            output_lines.push(format!("*{} = {}", alias_name, cpu_spec));
        }
        output_lines.push(String::new());
    }

    let mut all_processes: std::collections::HashSet<String> = priorities.keys().cloned().collect();
    all_processes.extend(affinities.keys().cloned());

    let mut sorted_processes: Vec<String> = all_processes.into_iter().collect();
    sorted_processes.sort();

    for name in sorted_processes {
        let priority = priorities.get(&name).map(|s| s.as_str()).unwrap_or("none");
        let affinity_raw = affinities.get(&name).map(|s| s.as_str()).unwrap_or("0");

        let affinity = spec_to_alias.get(affinity_raw).map(|s| s.as_str()).unwrap_or(affinity_raw);

        let priority_str = match priority.to_lowercase().as_str() {
            "idle" => "idle",
            "below normal" => "below normal",
            "normal" => "normal",
            "above normal" => "above normal",
            "high" => "high",
            "realtime" | "real time" => "real time",

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

pub fn sort_and_group_config(in_file: Option<String>, out_file: Option<String>) {
    let in_path = match in_file {
        Some(p) => p,
        None => {
            log!("Error: -in <file> is required for -autogroup");
            return;
        }
    };
    let out_path = match out_file {
        Some(p) => p,
        None => {
            log!("Error: -out <file> is required for -autogroup");
            return;
        }
    };

    let content = match fs::read_to_string(&in_path) {
        Ok(c) => c,
        Err(e) => {
            log!("Failed to read {}: {}", in_path, e);
            return;
        }
    };

    let lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();

    let mut preamble_lines: Vec<String> = Vec::new();
    let mut in_rules_section = false;

    let mut rule_order: Vec<String> = Vec::new();

    let mut rule_to_members: HashMap<String, Vec<String>> = HashMap::new();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();

        if line.is_empty() {
            if !in_rules_section {
                preamble_lines.push(lines[i].clone());
            }
            i += 1;
            continue;
        }

        if line.starts_with('#') {
            if !in_rules_section {
                preamble_lines.push(lines[i].clone());
            }

            i += 1;
            continue;
        }

        if line.starts_with('@') || line.starts_with('*') {
            preamble_lines.push(lines[i].clone());
            i += 1;
            continue;
        }

        in_rules_section = true;

        if line.contains('{') {
            let brace_start = line.find('{').unwrap();
            let (members, rule_suffix, next_i) = if let Some(brace_end) = line.find('}') {
                let mut members = Vec::new();
                collect_members(&line[brace_start + 1..brace_end], &mut members);
                let after = line[brace_end + 1..].trim();
                let suffix = after.strip_prefix(':').map(|s| s.to_string());
                (members, suffix, i + 1)
            } else {
                let first_content = line[brace_start + 1..].trim();
                match collect_group_block(&lines, i + 1, first_content) {
                    Some((members, suffix, next)) => (members, suffix, next),
                    None => {
                        i += 1;
                        continue;
                    }
                }
            };

            i = next_i;

            if let Some(rule) = rule_suffix {
                let rule = rule.trim().to_string();
                if !rule.is_empty() {
                    if !rule_to_members.contains_key(&rule) {
                        rule_order.push(rule.clone());
                        rule_to_members.insert(rule.clone(), Vec::new());
                    }
                    rule_to_members.get_mut(&rule).unwrap().extend(members);
                }
            }
        } else {
            if let Some(colon_pos) = line.find(':') {
                let name = line[..colon_pos].trim().to_lowercase();
                let rule = line[colon_pos + 1..].trim().to_string();
                if !name.is_empty() && !rule.is_empty() {
                    if !rule_to_members.contains_key(&rule) {
                        rule_order.push(rule.clone());
                        rule_to_members.insert(rule.clone(), Vec::new());
                    }
                    rule_to_members.get_mut(&rule).unwrap().push(name);
                }
            }
            i += 1;
        }
    }

    let mut output_lines: Vec<String> = Vec::new();

    let preamble_end = preamble_lines.iter().rposition(|l| !l.trim().is_empty()).map(|p| p + 1).unwrap_or(0);
    for line in &preamble_lines[..preamble_end] {
        output_lines.push(line.clone());
    }
    output_lines.push(String::new());

    let mut group_idx: usize = 0;
    let mut single_count: usize = 0;
    let mut group_count: usize = 0;
    let mut grouped_member_count: usize = 0;

    for rule_string in &rule_order {
        let members = match rule_to_members.get_mut(rule_string) {
            Some(m) => m,
            None => continue,
        };

        members.sort();
        members.dedup();

        if members.is_empty() {
            continue;
        }

        if members.len() == 1 {
            output_lines.push(format!("{}:{}", members[0], rule_string));
            single_count += 1;
        } else {
            let group_name = format!("grp_{}", group_idx);
            group_idx += 1;
            group_count += 1;
            grouped_member_count += members.len();

            let members_inline = members.join(": ");
            let single_line = format!("{} {{ {} }}:{}", group_name, members_inline, rule_string);
            if single_line.len() < 128 {
                output_lines.push(single_line);
            } else {
                output_lines.push(format!("{} {{", group_name));
                const INDENT: &str = "    ";
                let mut cur = String::from(INDENT);
                let mut first = true;
                for member in members.iter() {
                    if first {
                        cur.push_str(member);
                        first = false;
                    } else {
                        let candidate = format!("{}: {}", cur, member);
                        if candidate.len() < 128 {
                            cur = candidate;
                        } else {
                            output_lines.push(cur);
                            cur = format!("{}{}", INDENT, member);
                        }
                    }
                }
                if !first {
                    output_lines.push(cur);
                }
                output_lines.push(format!("}}:{}", rule_string));
            }
        }

        output_lines.push(String::new());
    }

    while output_lines.last().map(|l: &String| l.trim().is_empty()).unwrap_or(false) {
        output_lines.pop();
    }

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

    log!(
        "Auto-grouped: {} total process rules → {} individual + {} processes merged into {} groups",
        single_count + grouped_member_count,
        single_count,
        grouped_member_count,
        group_count
    );
    log!("Written to {}", out_path);
}
