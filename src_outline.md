# Src Code Structure Outline

## src/cli.rs
- [L7:L30]struct CliArgs {
    pub interval_ms: u64,
    pub help_mode: bool,
    pub help_all_mode: bool,
    pub convert_mode: bool,
    pub find_mode: bool,
    pub validate_mode: bool,
    pub process_logs_mode: bool,
    pub dry_run: bool,
    pub config_file_name: String,
    pub blacklist_file_name: Option<String>,
    pub in_file_name: Option<String>,
    pub out_file_name: Option<String>,
    pub no_uac: bool,
    pub loop_count: Option<u32>,
    pub time_resolution: u32,
    pub log_loop: bool,
    pub skip_log_before_elevation: bool,
    pub no_debug_priv: bool,
    pub no_inc_base_priority: bool,
}
- [L43:L127]fn parse_args(args: &[String], cli: &mut CliArgs) -> windows::core::Result<()> 
- [L129:L158]fn print_help() 
- [L160:L267]fn get_config_help_lines() -> Vec<&'static str> 
- [L269:L274]fn print_config_help() 
- [L276:L321]fn print_cli_help() 
- [L323:L329]fn print_help_all() 

## src/config.rs
- [L30:L39]struct PrimePrefix {
    pub prefix: String,
    pub cpus: Option<Vec<u32>>,
    pub thread_priority: ThreadPriority,
}
- [L41:L61]struct ProcessConfig {
    pub name: String,
    pub priority: ProcessPriority,
    pub affinity_cpus: Vec<u32>,
    pub cpu_set_cpus: Vec<u32>,
    pub prime_threads_cpus: Vec<u32>,
    pub prime_threads_prefixes: Vec<PrimePrefix>,
    pub prime_threads_monitor: bool,
    pub prime_threads_monitor_only: bool,
    pub prime_threads_top_x: Option<usize>,
    pub io_priority: IOPriority,
    pub memory_priority: MemoryPriority,
}
- [L63:L75]struct ConfigConstants {
    pub min_active_streak: u8,
    pub keep_threshold: f64,
    pub entry_threshold: f64,
}
- [L87:L145]fn parse_cpu_spec(s: &str) -> Vec<u32> 
- [L147:L149]fn mask_to_cpu_indices(mask: u64) -> Vec<u32> 
- [L151:L161]fn cpu_indices_to_mask(cpus: &[u32]) -> usize 
- [L163:L196]fn format_cpu_indices(cpus: &[u32]) -> String 
- [L198:L216]struct ConfigResult {
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
- [L246:L258]fn resolve_cpu_spec(spec: &str, field_name: &str, line_number: usize, cpu_aliases: &HashMap<String, Vec<u32>>, errors: &mut Vec<String>) -> Vec<u32> 
- [L260:L267]fn collect_members(text: &str, members: &mut Vec<String>) 
- [L270:L306]fn parse_constant(name: &str, value: &str, line_number: usize, result: &mut ConfigResult) 
- [L308:L322]fn parse_alias(name: &str, value: &str, line_number: usize, cpu_aliases: &mut HashMap<String, Vec<u32>>, result: &mut ConfigResult) 
- [L324:L348]fn collect_group_block(lines: &[String], start_index: usize, first_line_content: &str) -> Option<(Vec<String>, Option<String>, usize)> 
- [L360:L605]fn parse_and_insert_rules(members: &[String], rule_parts: &[&str], line_number: usize, cpu_aliases: &HashMap<String, Vec<u32>>, result: &mut ConfigResult) 
- [L607:L694]fn read_config<P: AsRef<Path>>(path: P) -> ConfigResult 
- [L740:L750]fn read_list<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> 
- [L752:L757]fn read_utf16le_file(path: &str) -> io::Result<String> 
- [L759:L765]fn parse_mask(s: &str) -> usize 
- [L767:L943]fn convert(in_file: Option<String>, out_file: Option<String>) 

## src/logging.rs
- [L16:L16]static LOCALTIME_BUFFER: Lazy<Mutex<DateTime<Local>>> = Lazy::new(|| Mutex::new(Local::now()));
- [L19:L19]static FINDS_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));
- [L22:L22]static FAIL_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));
- [L25:L25]static USE_CONSOLE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));
- [L28:L28]static LOG_FILE: Lazy<Mutex<File>> = Lazy::new(|| Mutex::new(OpenOptions::new().append(true).create(true).open(get_log_path("")).unwrap()));
- [L31:L31]static FIND_LOG_FILE: Lazy<Mutex<File>> = Lazy::new(|| Mutex::new(OpenOptions::new().append(true).create(true).open(get_log_path(".find")).unwrap()));
- [L15:L36]fn use_console() -> &'static Mutex<bool> 
- [L38:L41]fn logger() -> &'static Mutex<File> 
- [L43:L46]fn find_logger() -> &'static Mutex<File> 
- [L48:L58]fn get_log_path(suffix: &str) -> PathBuf 
- [L68:L76]fn log_message(args: &str) 
- [L78:L85]fn log_pure_message(args: &str) 
- [L87:L95]fn log_to_find(msg: &str) 
- [L97:L102]fn log_process_find(process_name: &str) 
- [L104:L125]fn error_from_code(code: u32) -> String 

## src/main.rs
- [L45:L48]struct ApplyConfigResult {
    changes: Vec<String>,
    errors: Vec<String>,
}
- [L69:L94]fn apply_priority(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult) 
- [L96:L131]fn apply_affinity(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, current_mask: &mut usize, apply_config_result: &mut ApplyConfigResult) 
- [L133:L190]fn apply_process_default_cpuset(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult) 
- [L192:L227]fn apply_prime_threads(
    pid: u32,
    config: &ProcessConfig,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    mut processes: Option<&mut ProcessSnapshot>,
    dry_run: bool,
    current_mask: &mut usize,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L229:L245]fn apply_prime_threads_select_candidates(process: &mut process::ProcessEntry, candidate_tids: &mut [u32], prime_core_scheduler: &mut PrimeThreadScheduler, pid: u32) 
- [L247:L321]fn apply_prime_threads_query_cycles(
    candidate_tids: &[u32],
    tid_with_delta_cycles: &mut [(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    pid: u32,
    process_name: &str,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L323:L363]fn apply_prime_threads_update_streaks(tid_with_delta_cycles: &mut [(u32, u64, bool)], prime_core_scheduler: &mut PrimeThreadScheduler, pid: u32, prime_count: usize) 
- [L365:L469]fn apply_prime_threads_promote(
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    pid: u32,
    config: &ProcessConfig,
    current_mask: &mut usize,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L471:L517]fn apply_prime_threads_demote(
    process: &mut process::ProcessEntry,
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    pid: u32,
    config: &ProcessConfig,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L519:L577]fn apply_io_priority(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult) 
- [L579:L637]fn apply_memory_priority(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult) 
- [L639:L682]fn apply_config(
    pid: u32,
    config: &ProcessConfig,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    processes: Option<&mut ProcessSnapshot>,
    dry_run: bool,
) -> ApplyConfigResult 
- [L684:L754]fn process_logs(configs: &HashMap<String, ProcessConfig>, blacklist: &Vec<String>, logs_path: Option<&str>, output_file: Option<&str>) 
- [L756:L985]fn main() -> windows::core::Result<()> 

## src/priority.rs
- [L17:L27]enum ProcessPriority {
    None,
    Idle,
    BelowNormal,
    Normal,
    AboveNormal,
    High,
    Realtime,
}
- [L67:L75]enum IOPriority {
    None,
    VeryLow,
    Low,
    Normal,
    High,
}
- [L108:L111]struct MemoryPriorityInformation(pub u32);
- [L113:L122]enum MemoryPriority {
    None,
    VeryLow,
    Low,
    Medium,
    BelowNormal,
    Normal,
}
- [L156:L183]enum ThreadPriority {
    None,
    ErrorReturn,
    ModeBackgroundBegin,
    ModeBackgroundEnd,
    Idle,
    Lowest,
    BelowNormal,
    Normal,
    AboveNormal,
    Highest,
    TimeCritical,
}

## src/process.rs
- [L9:L23]struct ProcessSnapshot {
    buffer: Vec<u8>,
    pub pid_to_process: HashMap<u32, ProcessEntry>,
}
- [L89:L103]struct ProcessEntry {
    pub process: SYSTEM_PROCESS_INFORMATION,
    threads: HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    threads_base_ptr: usize,
    name: String,
}

## src/scheduler.rs
- [L16:L35]struct PrimeThreadScheduler {
    pid_to_process_stats: HashMap<u32, ProcessStats>,
    pub constants: ConfigConstants,
}
- [L182:L188]struct ThreadHistory {
    pub tid: u32,
    pub info: SYSTEM_THREAD_INFORMATION,
    pub total_cycles: u64,
    pub module_name: String,
}
- [L190:L201]struct ProcessStats {
    pub alive: bool,
    pub tid_to_thread_stats: HashMap<u32, ThreadStats>,
    pub top_threads_history: Vec<ThreadHistory>,
    pub total_threads_tracked: usize,
    pub process_name: String,
}
- [L221:L245]struct ThreadStats {
    pub last_total_time: i64,
    pub last_cycles: u64,
    pub handle: Option<HANDLE>,
    pub cpu_set_ids: Vec<u32>,
    pub active_streak: u8,
    pub start_address: usize,
    pub original_priority: Option<ThreadPriority>,
    pub total_cycles: u64,
    pub last_thread_info: Option<SYSTEM_THREAD_INFORMATION>,
    pub module_name: String,
    pub seen: bool,
}

## src/winapi.rs
- [L54:L59]struct CpuSetData {
    id: u32,
    logical_processor_index: u8,
}
- [L75:L75]static CPU_SET_INFORMATION: Lazy<Mutex<Vec<CpuSetData>>> = Lazy::new(|| {
- [L121:L124]fn get_cpu_set_information() -> &'static Mutex<Vec<CpuSetData>> 
- [L126:L149]fn cpusetids_from_indices(cpu_indices: &[u32]) -> Vec<u32> 
- [L151:L175]fn cpusetids_from_mask(mask: usize) -> Vec<u32> 
- [L177:L197]fn indices_from_cpusetids(cpuids: &[u32]) -> Vec<u32> 
- [L199:L223]fn mask_from_cpusetids(cpuids: &[u32]) -> usize 
- [L225:L241]fn filter_indices_by_mask(cpu_indices: &[u32], affinity_mask: usize) -> Vec<u32> 
- [L243:L277]fn is_running_as_admin() -> bool 
- [L279:L303]fn request_uac_elevation() -> io::Result<()> 
- [L305:L349]fn enable_debug_privilege() 
- [L351:L395]fn enable_inc_base_priority_privilege() 
- [L397:L443]fn is_affinity_unset(pid: u32, process_name: &str) -> bool 
- [L445:L463]fn get_thread_start_address(thread_handle: HANDLE) -> usize 
- [L467:L467]static MODULE_CACHE: Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>> = Lazy::new(|| Mutex::new(HashMap::new()));
- [L465:L516]fn resolve_address_to_module(pid: u32, address: usize) -> String 
- [L518:L521]fn clear_module_cache(pid: u32) 
- [L523:L585]fn enumerate_process_modules(pid: u32) -> Vec<(usize, usize, String)> 

