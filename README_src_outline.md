# Src Code Structure Outline, auto-generated during cargo build/check

## src/apply.rs
- [L25:L28]struct ApplyConfigResult {
    pub changes: Vec<String>,
    pub errors: Vec<String>,
}
- [L49:L74]fn apply_priority(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult) 
- [L76:L118]fn apply_affinity(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    h_prc: HANDLE,
    current_mask: &mut usize,
    apply_config_result: &mut ApplyConfigResult,
    processes: &mut ProcessSnapshot,
) 
- [L120:L177]fn apply_process_default_cpuset(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult) 
- [L179:L266]fn prefetch_all_thread_cycles(
    pid: u32,
    process_name: &str,
    processes: &mut ProcessSnapshot,
    prime_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L268:L368]fn apply_prime_threads(
    pid: u32,
    config: &ProcessConfig,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    processes: &mut ProcessSnapshot,
    dry_run: bool,
    current_mask: &mut usize,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L370:L372]fn apply_prime_threads_select(tid_with_delta_cycles: &mut [(u32, u64, bool)], prime_core_scheduler: &mut PrimeThreadScheduler, pid: u32, prime_count: usize) 
- [L374:L387]fn update_thread_stats(pid: u32, prime_scheduler: &mut PrimeThreadScheduler) 
- [L389:L487]fn apply_prime_threads_promote(
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    pid: u32,
    config: &ProcessConfig,
    current_mask: &mut usize,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L489:L555]fn apply_prime_threads_demote(
    process: &mut crate::process::ProcessEntry,
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    pid: u32,
    config: &ProcessConfig,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L557:L615]fn apply_io_priority(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult) 
- [L617:L674]fn apply_memory_priority(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult) 
- [L676:L755]fn reset_thread_ideal_processors(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    _h_prc: HANDLE,
    apply_config_result: &mut ApplyConfigResult,
    processes: &mut ProcessSnapshot,
) 
- [L757:L959]fn apply_ideal_processors(
    pid: u32,
    config: &ProcessConfig,
    processes: &mut ProcessSnapshot,
    prime_scheduler: &mut PrimeThreadScheduler,
    dry_run: bool,
    apply_config_result: &mut ApplyConfigResult,
) 

## src/cli.rs
- [L7:L31]struct CliArgs {
    pub interval_ms: u64,
    pub help_mode: bool,
    pub help_all_mode: bool,
    pub convert_mode: bool,
    pub autogroup_mode: bool,
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
- [L44:L133]fn parse_args(args: &[String], cli: &mut CliArgs) -> windows::core::Result<()> 
- [L135:L162]fn print_help() 
- [L164:L213]fn print_cli_help() 
- [L215:L322]fn get_config_help_lines() -> Vec<&'static str> 
- [L324:L329]fn print_config_help() 
- [L331:L337]fn print_help_all() 

## src/config.rs
- [L30:L39]struct PrimePrefix {
    pub prefix: String,
    pub cpus: Option<Vec<u32>>,
    pub thread_priority: ThreadPriority,
}
- [L41:L50]struct IdealProcessorPrefix {
    pub prefix: String,
    pub cpus: Vec<u32>,
}
- [L52:L60]struct IdealProcessorRule {
    pub cpus: Vec<u32>,
    pub prefixes: Vec<String>,
}
- [L62:L81]struct ProcessConfig {
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
- [L83:L95]struct ConfigConstants {
    pub min_active_streak: u8,
    pub keep_threshold: f64,
    pub entry_threshold: f64,
}
- [L107:L165]fn parse_cpu_spec(s: &str) -> Vec<u32> 
- [L167:L169]fn mask_to_cpu_indices(mask: u64) -> Vec<u32> 
- [L171:L181]fn cpu_indices_to_mask(cpus: &[u32]) -> usize 
- [L183:L216]fn format_cpu_indices(cpus: &[u32]) -> String 
- [L218:L239]struct ConfigResult {
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
- [L279:L291]fn resolve_cpu_spec(spec: &str, field_name: &str, line_number: usize, cpu_aliases: &HashMap<String, Vec<u32>>, errors: &mut Vec<String>) -> Vec<u32> 
- [L293:L300]fn collect_members(text: &str, members: &mut Vec<String>) 
- [L303:L339]fn parse_constant(name: &str, value: &str, line_number: usize, result: &mut ConfigResult) 
- [L341:L355]fn parse_alias(name: &str, value: &str, line_number: usize, cpu_aliases: &mut HashMap<String, Vec<u32>>, result: &mut ConfigResult) 
- [L357:L425]fn parse_ideal_processor_spec(spec: &str, line_number: usize, cpu_aliases: &HashMap<String, Vec<u32>>, errors: &mut Vec<String>) -> Vec<IdealProcessorRule> 
- [L427:L451]fn collect_group_block(lines: &[String], start_index: usize, first_line_content: &str) -> Option<(Vec<String>, Option<String>, usize)> 
- [L463:L756]fn parse_and_insert_rules(members: &[String], rule_parts: &[&str], line_number: usize, cpu_aliases: &HashMap<String, Vec<u32>>, result: &mut ConfigResult) 
- [L758:L845]fn read_config<P: AsRef<Path>>(path: P) -> ConfigResult 
- [L891:L901]fn read_list<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> 
- [L903:L908]fn read_utf16le_file(path: &str) -> io::Result<String> 
- [L910:L916]fn parse_mask(s: &str) -> usize 
- [L918:L1090]fn convert(in_file: Option<String>, out_file: Option<String>) 
- [L1092:L1329]fn sort_and_group_config(in_file: Option<String>, out_file: Option<String>) 

## src/logging.rs
- [L16:L16]static LOCALTIME_BUFFER: Lazy<Mutex<DateTime<Local>>> = Lazy::new(|| Mutex::new(Local::now()));
- [L19:L19]static FINDS_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));
- [L22:L22]static FAIL_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));
- [L15:L30]struct ApplyFailEntry {
    proc_name: String,
    alive: bool,
}
- [L36:L36]static APPLY_FAIL_MAP: Lazy<Mutex<HashMap<u32, ApplyFailEntry>>> = Lazy::new(|| Mutex::new(HashMap::new()));
- [L32:L57]fn apply_fail_insert_if_new(pid: u32, proc_name: &str) -> bool 
- [L59:L81]fn purge_apply_fail_map(pids_and_names: &[(u32, String)]) 
- [L84:L84]static USE_CONSOLE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));
- [L87:L87]static DUST_BIN_MODE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));
- [L90:L90]static LOG_FILE: Lazy<Mutex<File>> = Lazy::new(|| Mutex::new(OpenOptions::new().append(true).create(true).open(get_log_path("")).unwrap()));
- [L93:L93]static FIND_LOG_FILE: Lazy<Mutex<File>> = Lazy::new(|| Mutex::new(OpenOptions::new().append(true).create(true).open(get_log_path(".find")).unwrap()));
- [L83:L98]fn use_console() -> &'static Mutex<bool> 
- [L100:L103]fn logger() -> &'static Mutex<File> 
- [L105:L108]fn find_logger() -> &'static Mutex<File> 
- [L110:L120]fn get_log_path(suffix: &str) -> PathBuf 
- [L130:L141]fn log_message(args: &str) 
- [L143:L150]fn log_pure_message(args: &str) 
- [L152:L160]fn log_to_find(msg: &str) 
- [L162:L167]fn log_process_find(process_name: &str) 
- [L169:L190]fn error_from_code(code: u32) -> String 

## src/main.rs
- [L46:L87]fn apply_config(pid: u32, config: &ProcessConfig, prime_core_scheduler: &mut PrimeThreadScheduler, processes: &mut ProcessSnapshot, dry_run: bool) -> ApplyConfigResult 
- [L89:L161]fn process_logs(configs: &HashMap<u32, HashMap<String, ProcessConfig>>, blacklist: &[String], logs_path: Option<&str>, output_file: Option<&str>) 
- [L163:L419]fn main() -> windows::core::Result<()> 

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
    pub pid_to_process_stats: HashMap<u32, ProcessStats>,
    pub constants: ConfigConstants,
}
- [L212:L219]struct ProcessStats {
    pub alive: bool,
    pub tid_to_thread_stats: HashMap<u32, ThreadStats>,
    pub track_top_x_threads: i32,
    pub process_name: String,
}
- [L238:L251]struct IdealProcessorState {
    pub current_group: u16,
    pub current_number: u8,
    pub previous_group: u16,
    pub previous_number: u8,
    pub is_assigned: bool,
}
- [L271:L297]struct ThreadStats {
    pub last_total_time: i64,
    pub cached_total_time: i64,
    pub last_cycles: u64,
    pub cached_cycles: u64,
    pub handle: Option<HANDLE>,
    pub cpu_set_ids: Vec<u32>,
    pub active_streak: u8,
    pub start_address: usize,
    pub original_priority: Option<ThreadPriority>,
    pub last_system_thread_info: Option<ntapi::ntexapi::SYSTEM_THREAD_INFORMATION>,
    pub ideal_processor: IdealProcessorState,
}
- [L322:L326]fn format_100ns(time: i64) -> String 
- [L328:L335]fn format_filetime(time: i64) -> String 

## src/winapi.rs
- [L59:L64]struct CpuSetData {
    id: u32,
    logical_processor_index: u8,
}
- [L80:L80]static CPU_SET_INFORMATION: Lazy<Mutex<Vec<CpuSetData>>> = Lazy::new(|| {
- [L126:L129]fn get_cpu_set_information() -> &'static Mutex<Vec<CpuSetData>> 
- [L131:L154]fn cpusetids_from_indices(cpu_indices: &[u32]) -> Vec<u32> 
- [L156:L180]fn cpusetids_from_mask(mask: usize) -> Vec<u32> 
- [L182:L202]fn indices_from_cpusetids(cpuids: &[u32]) -> Vec<u32> 
- [L204:L228]fn mask_from_cpusetids(cpuids: &[u32]) -> usize 
- [L230:L246]fn filter_indices_by_mask(cpu_indices: &[u32], affinity_mask: usize) -> Vec<u32> 
- [L248:L282]fn is_running_as_admin() -> bool 
- [L284:L313]fn request_uac_elevation(console: bool) -> io::Result<()> 
- [L315:L359]fn enable_debug_privilege() 
- [L361:L405]fn enable_inc_base_priority_privilege() 
- [L407:L453]fn is_affinity_unset(pid: u32, process_name: &str) -> bool 
- [L455:L473]fn get_thread_start_address(thread_handle: HANDLE) -> usize 
- [L475:L497]fn set_thread_ideal_processor_ex(thread_handle: HANDLE, group: u16, number: u8) -> Result<PROCESSOR_NUMBER, windows::core::Error> 
- [L499:L513]fn get_thread_ideal_processor_ex(thread_handle: HANDLE) -> Result<PROCESSOR_NUMBER, windows::core::Error> 
- [L518:L518]static MODULE_CACHE: Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>> = Lazy::new(|| Mutex::new(HashMap::new()));
- [L515:L557]fn resolve_address_to_module(pid: u32, address: usize) -> String 
- [L559:L610]fn terminate_child_processes() 
- [L612:L615]fn clear_module_cache(pid: u32) 
- [L617:L677]fn enumerate_process_modules(pid: u32) -> Vec<(usize, usize, String)> 

