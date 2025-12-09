# AffinityServiceRust Src Code Structure Outline

## cli.rs
- fn parse_args(
    args: &[String],
    interval_ms: &mut u64,
    help_mode: &mut bool,
    help_all_mode: &mut bool,
    convert_mode: &mut bool,
    find_mode: &mut bool,
    validate_mode: &mut bool,
    process_logs_mode: &mut bool,
    dry_run: &mut bool,
    config_file_name: &mut String,
    blacklist_file_name: &mut Option<String>,
    in_file_name: &mut Option<String>,
    out_file_name: &mut Option<String>,
    no_uac: &mut bool,
    loop_count: &mut Option<u32>,
    time_resolution: &mut u32,
    log_loop: &mut bool,
    skip_log_before_elevation: &mut bool,
    no_debug_priv: &mut bool,
    no_inc_base_priority: &mut bool,
) -> windows::core::Result<()> {
- fn print_help()
- fn get_config_help_lines() -> Vec<&'static str>
- fn print_config_help()
- fn print_cli_help()
- fn print_help_all()

## config.rs
- struct PrimePrefix {
    pub prefix: String,
    pub cpus: Option<Vec<u32>>,
}
- struct ProcessConfig {
    pub name: String,
    pub priority: ProcessPriority,
    pub affinity_cpus: Vec<u32>,
    pub cpu_set_cpus: Vec<u32>,
    pub prime_threads_cpus: Vec<u32>,
    pub prime_threads_prefixes: Vec<PrimePrefix>,
    pub io_priority: IOPriority,
    pub memory_priority: MemoryPriority,
}
- struct ConfigConstants {
    pub min_active_streak: u8,
    pub keep_threshold: f64,
    pub entry_threshold: f64,
}
- fn parse_cpu_spec(s: &str) -> Vec<u32>
- fn mask_to_cpu_indices(mask: u64) -> Vec<u32>
- fn cpu_indices_to_mask(cpus: &[u32]) -> usize
- fn format_cpu_indices(cpus: &[u32]) -> String
- struct ConfigResult {
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
- fn resolve_cpu_spec(spec: &str, field_name: &str, line_number: usize, cpu_aliases: &HashMap<String, Vec<u32>>, errors: &mut Vec<String>) -> Vec<u32>
- fn collect_members(text: &str, members: &mut Vec<String>)
- fn parse_constant(name: &str, value: &str, line_number: usize, result: &mut ConfigResult)
- fn parse_alias(name: &str, value: &str, line_number: usize, cpu_aliases: &mut HashMap<String, Vec<u32>>, result: &mut ConfigResult)
- fn collect_group_block(lines: &[String], start_index: usize, first_line_content: &str) -> Option<(Vec<String>, Option<String>, usize)>
- fn parse_and_insert_rules(members: &[String], rule_parts: &[&str], line_number: usize, cpu_aliases: &HashMap<String, Vec<u32>>, result: &mut ConfigResult)
- fn read_config<P: AsRef<Path>>(path: P) -> ConfigResult
- fn read_list<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>>
- fn read_utf16le_file(path: &str) -> io::Result<String>
- fn parse_mask(s: &str) -> usize
- fn convert(in_file: Option<String>, out_file: Option<String>)

## logging.rs
- static LOCALTIME_BUFFER: Lazy<Mutex<DateTime<Local>>> = Lazy::new(|| Mutex::new(Local::now()));
- static FINDS_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));
- static FAIL_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));
- static USE_CONSOLE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));
- static LOG_FILE: Lazy<Mutex<File>> = Lazy::new(|| Mutex::new(OpenOptions::new().append(true).create(true).open(get_log_path("")).unwrap()));
- static FIND_LOG_FILE: Lazy<Mutex<File>> = Lazy::new(|| Mutex::new(OpenOptions::new().append(true).create(true).open(get_log_path(".find")).unwrap()));
- fn use_console() -> &'static Mutex<bool>
- fn logger() -> &'static Mutex<File>
- fn find_logger() -> &'static Mutex<File>
- fn get_log_path(suffix: &str) -> PathBuf
- fn log_message(args: &str)
- fn log_pure_message(args: &str)
- fn log_to_find(msg: &str)
- fn log_process_find(process_name: &str)
- fn error_from_code(code: u32) -> String

## main.rs
- struct ApplyConfigResult {
    changes: Vec<String>,
    errors: Vec<String>,
}
- fn apply_priority(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult)
- fn apply_affinity(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, current_mask: &mut usize, apply_config_result: &mut ApplyConfigResult)
- fn apply_process_default_cpuset(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult)
- fn apply_prime_threads(
    pid: u32,
    config: &ProcessConfig,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    mut processes: Option<&mut ProcessSnapshot>,
    dry_run: bool,
    current_mask: &mut usize,
    apply_config_result: &mut ApplyConfigResult,
)
- fn apply_io_priority(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult)
- fn apply_memory_priority(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult)
- fn apply_config(
    pid: u32,
    config: &ProcessConfig,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    processes: Option<&mut ProcessSnapshot>,
    dry_run: bool,
) -> ApplyConfigResult
- fn process_logs(configs: &HashMap<String, ProcessConfig>, blacklist: &Vec<String>, logs_path: Option<&str>, output_file: Option<&str>)
- fn main() -> windows::core::Result<()>

## priority.rs
- enum ProcessPriority {
    None,
    Idle,
    BelowNormal,
    Normal,
    AboveNormal,
    High,
    Realtime,
}
- enum IOPriority {
    None,
    VeryLow,
    Low,
    Normal,
}
- struct MemoryPriorityInformation(pub u32);
- enum MemoryPriority {
    None,
    VeryLow,
    Low,
    Medium,
    BelowNormal,
    Normal,
}
- enum ThreadPriority {
    Unknown(i32),
}

## process.rs
- struct ProcessSnapshot {
    buffer: Vec<u8>,
    pub pid_to_process: HashMap<u32, ProcessEntry>,
}
- struct ProcessEntry {
    pub process: SYSTEM_PROCESS_INFORMATION,
    threads: HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    threads_base_ptr: usize,
    name: String,
}

## scheduler.rs
- struct PrimeThreadScheduler {
    pid_to_process_stats: HashMap<u32, ProcessStats>,
    pub constants: ConfigConstants,
}
- struct ProcessStats {
    pub alive: bool,
    pub tid_to_thread_stats: HashMap<u32, ThreadStats>,
}
- struct ThreadStats {
    pub last_total_time: i64,
    pub last_cycles: u64,
    pub handle: Option<HANDLE>,
    pub cpu_set_ids: Vec<u32>,
    pub active_streak: u8,
    pub start_address: usize,
    pub original_priority: Option<ThreadPriority>,
}

## winapi.rs
- struct CpuSetData {
    id: u32,
    logical_processor_index: u8,
}
- static CPU_SET_INFORMATION: Lazy<Mutex<Vec<CpuSetData>>> = Lazy::new(|| {
- fn get_cpu_set_information() -> &'static Mutex<Vec<CpuSetData>>
- fn cpusetids_from_indices(cpu_indices: &[u32]) -> Vec<u32>
- fn cpusetids_from_mask(mask: usize) -> Vec<u32>
- fn indices_from_cpusetids(cpuids: &[u32]) -> Vec<u32>
- fn mask_from_cpusetids(cpuids: &[u32]) -> usize
- fn filter_indices_by_mask(cpu_indices: &[u32], affinity_mask: usize) -> Vec<u32>
- fn is_running_as_admin() -> bool
- fn request_uac_elevation() -> io::Result<()>
- fn enable_debug_privilege()
- fn enable_inc_base_priority_privilege()
- fn is_affinity_unset(pid: u32, process_name: &str) -> bool
- fn get_thread_start_address(thread_handle: HANDLE) -> usize
- static MODULE_CACHE: Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>> = Lazy::new(|| Mutex::new(HashMap::new()));
- fn resolve_address_to_module(pid: u32, address: usize) -> String
- fn clear_module_cache(pid: u32)
- fn enumerate_process_modules(pid: u32) -> Vec<(usize, usize, String)>

