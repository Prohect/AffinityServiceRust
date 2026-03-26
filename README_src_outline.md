# Src Code Structure Outline, auto-generated during cargo build/check

## src/apply.rs
- [L23:L26]struct ApplyConfigResult {
    pub changes: Vec<String>,
    pub errors: Vec<String>,
}
- [L47:L72]fn apply_priority(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult) 
- [L74:L116]fn apply_affinity(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    h_prc: HANDLE,
    current_mask: &mut usize,
    apply_config_result: &mut ApplyConfigResult,
    processes: &mut Option<&mut ProcessSnapshot>,
) 
- [L118:L175]fn apply_process_default_cpuset(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult) 
- [L177:L261]fn apply_prime_threads(
    pid: u32,
    config: &ProcessConfig,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    processes: &mut Option<&mut ProcessSnapshot>,
    dry_run: bool,
    current_mask: &mut usize,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L263:L283]fn apply_prime_threads_select_candidates(
    process: &mut crate::process::ProcessEntry,
    candidate_tids: &mut [u32],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    pid: u32,
) 
- [L285:L352]fn apply_prime_threads_query_cycles(
    candidate_tids: &[u32],
    tid_with_delta_cycles: &mut [(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    pid: u32,
    process_name: &str,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L354:L394]fn apply_prime_threads_update_streaks(tid_with_delta_cycles: &mut [(u32, u64, bool)], prime_core_scheduler: &mut PrimeThreadScheduler, pid: u32, prime_count: usize) 
- [L396:L497]fn apply_prime_threads_promote(
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    pid: u32,
    config: &ProcessConfig,
    current_mask: &mut usize,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L499:L546]fn apply_prime_threads_demote(
    process: &mut crate::process::ProcessEntry,
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    pid: u32,
    config: &ProcessConfig,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L548:L606]fn apply_io_priority(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult) 
- [L608:L665]fn apply_memory_priority(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult) 
- [L667:L777]fn reset_thread_ideal_processors(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    _h_prc: HANDLE,
    apply_config_result: &mut ApplyConfigResult,
    processes: &mut Option<&mut ProcessSnapshot>,
) 
- [L779:L1007]fn apply_ideal_processors(
    pid: u32,
    config: &ProcessConfig,
    processes: &mut Option<&mut ProcessSnapshot>,
    prime_scheduler: &mut PrimeThreadScheduler,
    dry_run: bool,
    apply_config_result: &mut ApplyConfigResult,
) 

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
- [L43:L129]fn parse_args(args: &[String], cli: &mut CliArgs) -> windows::core::Result<()> 
- [L131:L157]fn print_help() 
- [L159:L207]fn print_cli_help() 
- [L209:L316]fn get_config_help_lines() -> Vec<&'static str> 
- [L318:L323]fn print_config_help() 
- [L325:L331]fn print_help_all() 

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

## src/logging.rs
- [L16:L16]static LOCALTIME_BUFFER: Lazy<Mutex<DateTime<Local>>> = Lazy::new(|| Mutex::new(Local::now()));
- [L19:L19]static FINDS_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));
- [L22:L22]static FAIL_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));
- [L25:L25]static USE_CONSOLE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));
- [L28:L28]static DUST_BIN_MODE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));
- [L31:L31]static LOG_FILE: Lazy<Mutex<File>> = Lazy::new(|| Mutex::new(OpenOptions::new().append(true).create(true).open(get_log_path("")).unwrap()));
- [L34:L34]static FIND_LOG_FILE: Lazy<Mutex<File>> = Lazy::new(|| Mutex::new(OpenOptions::new().append(true).create(true).open(get_log_path(".find")).unwrap()));
- [L15:L39]fn use_console() -> &'static Mutex<bool> 
- [L41:L44]fn logger() -> &'static Mutex<File> 
- [L46:L49]fn find_logger() -> &'static Mutex<File> 
- [L51:L61]fn get_log_path(suffix: &str) -> PathBuf 
- [L71:L82]fn log_message(args: &str) 
- [L84:L91]fn log_pure_message(args: &str) 
- [L93:L101]fn log_to_find(msg: &str) 
- [L103:L108]fn log_process_find(process_name: &str) 
- [L110:L131]fn error_from_code(code: u32) -> String 

## src/main.rs
- [L42:L94]fn apply_config(
    pid: u32,
    config: &ProcessConfig,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    mut processes: Option<&mut ProcessSnapshot>,
    dry_run: bool,
) -> ApplyConfigResult 
- [L96:L168]fn process_logs(configs: &HashMap<u32, HashMap<String, ProcessConfig>>, blacklist: &[String], logs_path: Option<&str>, output_file: Option<&str>) 
- [L170:L417]fn main() -> windows::core::Result<()> 

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
- [L15:L34]struct PrimeThreadScheduler {
    pub pid_to_process_stats: HashMap<u32, ProcessStats>,
    pub constants: ConfigConstants,
}
- [L125:L132]struct ProcessStats {
    pub alive: bool,
    pub tid_to_thread_stats: HashMap<u32, ThreadStats>,
    pub track_top_x_threads: i32,
    pub process_name: String,
}
- [L151:L164]struct IdealProcessorState {
    pub current_group: u16,
    pub current_number: u8,
    pub previous_group: u16,
    pub previous_number: u8,
    pub is_assigned: bool,
}
- [L184:L203]struct ThreadStats {
    pub last_total_time: i64,
    pub last_cycles: u64,
    pub handle: Option<HANDLE>,
    pub cpu_set_ids: Vec<u32>,
    pub active_streak: u8,
    pub start_address: usize,
    pub original_priority: Option<ThreadPriority>,
    pub last_system_thread_info: Option<ntapi::ntexapi::SYSTEM_THREAD_INFORMATION>,
    pub ideal_processor: IdealProcessorState,
}
- [L226:L230]fn format_100ns(time: i64) -> String 
- [L232:L239]fn format_filetime(time: i64) -> String 

## src/winapi.rs
- [L58:L63]struct CpuSetData {
    id: u32,
    logical_processor_index: u8,
}
- [L79:L79]static CPU_SET_INFORMATION: Lazy<Mutex<Vec<CpuSetData>>> = Lazy::new(|| {
- [L125:L128]fn get_cpu_set_information() -> &'static Mutex<Vec<CpuSetData>> 
- [L130:L153]fn cpusetids_from_indices(cpu_indices: &[u32]) -> Vec<u32> 
- [L155:L179]fn cpusetids_from_mask(mask: usize) -> Vec<u32> 
- [L181:L201]fn indices_from_cpusetids(cpuids: &[u32]) -> Vec<u32> 
- [L203:L227]fn mask_from_cpusetids(cpuids: &[u32]) -> usize 
- [L229:L245]fn filter_indices_by_mask(cpu_indices: &[u32], affinity_mask: usize) -> Vec<u32> 
- [L247:L281]fn is_running_as_admin() -> bool 
- [L283:L312]fn request_uac_elevation(console: bool) -> io::Result<()> 
- [L314:L358]fn enable_debug_privilege() 
- [L360:L404]fn enable_inc_base_priority_privilege() 
- [L406:L452]fn is_affinity_unset(pid: u32, process_name: &str) -> bool 
- [L454:L472]fn get_thread_start_address(thread_handle: HANDLE) -> usize 
- [L474:L496]fn set_thread_ideal_processor_ex(thread_handle: HANDLE, group: u16, number: u8) -> Result<PROCESSOR_NUMBER, windows::core::Error> 
- [L498:L512]fn get_thread_ideal_processor_ex(thread_handle: HANDLE) -> Result<PROCESSOR_NUMBER, windows::core::Error> 
- [L517:L517]static MODULE_CACHE: Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>> = Lazy::new(|| Mutex::new(HashMap::new()));
- [L514:L556]fn resolve_address_to_module(pid: u32, address: usize) -> String 
- [L558:L561]fn clear_module_cache(pid: u32) 
- [L563:L623]fn enumerate_process_modules(pid: u32) -> Vec<(usize, usize, String)> 

