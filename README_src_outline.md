# Src Code Structure Outline, auto-generated during cargo build/check

## src/apply.rs
- [L30:L33]struct ApplyConfigResult {
    pub changes: Vec<String>,
    pub errors: Vec<String>,
}
- [L58:L65]fn get_handles(process_handle: &ProcessHandle) -> (Option<HANDLE>, Option<HANDLE>) 
- [L67:L81]fn log_error_if_new(
    pid: u32,
    tid: u32,
    process_name: &str,
    operation: Operation,
    error_code: u32,
    apply_config_result: &mut ApplyConfigResult,
    format_msg: impl FnOnce() -> String,
) 
- [L83:L129]fn apply_priority(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L131:L208]fn apply_affinity(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    current_mask: &mut usize,
    process_handle: &ProcessHandle,
    process: &mut ProcessEntry,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L210:L313]fn reset_thread_ideal_processors(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    cpus: &[u32],
    process: &mut ProcessEntry,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L315:L418]fn apply_process_default_cpuset(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    process: &mut ProcessEntry,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L420:L506]fn apply_io_priority(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L508:L595]fn apply_memory_priority(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L597:L702]fn prefetch_all_thread_cycles(
    pid: u32,
    config: &ProcessConfig,
    process: &mut ProcessEntry,
    prime_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L704:L800]fn apply_prime_threads(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    current_mask: &mut usize,
    process: &mut ProcessEntry,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L802:L816]fn apply_prime_threads_select(
    pid: u32,
    prime_count: usize,
    tid_with_delta_cycles: &mut [(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
) 
- [L818:L960]fn apply_prime_threads_promote(
    pid: u32,
    config: &ProcessConfig,
    current_mask: &mut usize,
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L962:L1053]fn apply_prime_threads_demote(
    pid: u32,
    config: &ProcessConfig,
    process: &mut ProcessEntry,
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L1055:L1325]fn apply_ideal_processors(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process: &mut ProcessEntry,
    prime_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L1327:L1340]fn update_thread_stats(pid: u32, prime_scheduler: &mut PrimeThreadScheduler) 

## src/cli.rs
- [L5:L28]struct CliArgs {
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
    pub no_etw: bool,
    pub continuous_process_level_apply: bool,
}
- [L40:L127]fn parse_args(args: &[String], cli: &mut CliArgs) -> Result<()> 
- [L129:L155]fn print_help() 
- [L157:L207]fn print_cli_help() 
- [L209:L318]fn get_config_help_lines() -> Vec<&'static str> 
- [L320:L324]fn print_config_help() 
- [L326:L331]fn print_help_all() 

## src/config.rs
- [L17:L21]struct PrimePrefix {
    pub prefix: String,
    pub cpus: Option<Vec<u32>>,
    pub thread_priority: ThreadPriority,
}
- [L24:L27]struct IdealProcessorRule {
    pub cpus: Vec<u32>,
    pub prefixes: Vec<String>,
}
- [L30:L45]struct ProcessConfig {
    pub name: String,
    pub priority: ProcessPriority,
    pub affinity_cpus: Vec<u32>,
    pub cpu_set_cpus: Vec<u32>,
    pub cpu_set_reset_ideal: bool,
    pub prime_threads_cpus: Vec<u32>,
    pub prime_threads_prefixes: Vec<PrimePrefix>,
    pub track_top_x_threads: i32,
    pub io_priority: IOPriority,
    pub memory_priority: MemoryPriority,
    pub ideal_processor_rules: Vec<IdealProcessorRule>,
}
- [L48:L52]struct ConfigConstants {
    pub min_active_streak: u8,
    pub keep_threshold: f64,
    pub entry_threshold: f64,
}
- [L64:L112]fn parse_cpu_spec(s: &str) -> Vec<u32> 
- [L114:L116]fn mask_to_cpu_indices(mask: u64) -> Vec<u32> 
- [L118:L126]fn cpu_indices_to_mask(cpus: &[u32]) -> usize 
- [L128:L158]fn format_cpu_indices(cpus: &[u32]) -> String 
- [L161:L172]struct ConfigResult {
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
- [L215:L235]fn resolve_cpu_spec(
    spec: &str,
    field_name: &str,
    line_number: usize,
    cpu_aliases: &HashMap<String, Vec<u32>>,
    errors: &mut Vec<String>,
) -> Vec<u32> 
- [L237:L243]fn collect_members(text: &str, members: &mut Vec<String>) 
- [L246:L286]fn parse_constant(name: &str, value: &str, line_number: usize, result: &mut ConfigResult) 
- [L288:L302]fn parse_alias(name: &str, value: &str, line_number: usize, cpu_aliases: &mut HashMap<String, Vec<u32>>, result: &mut ConfigResult) 
- [L304:L373]fn parse_ideal_processor_spec(
    spec: &str,
    line_number: usize,
    cpu_aliases: &HashMap<String, Vec<u32>>,
    errors: &mut Vec<String>,
) -> Vec<IdealProcessorRule> 
- [L375:L385]fn collect_group_block(lines: &[String], start_index: usize, first_line_content: &str) -> Option<(Vec<String>, Option<String>, usize)> 
- [L409:L699]fn parse_and_insert_rules(
    members: &[String],
    rule_parts: &[&str],
    line_number: usize,
    cpu_aliases: &HashMap<String, Vec<u32>>,
    result: &mut ConfigResult,
) 
- [L701:L787]fn read_config<P: AsRef<Path>>(path: P) -> ConfigResult 
- [L836:L845]fn read_list<P: AsRef<Path>>(path: P) -> Result<Vec<String>> 
- [L847:L851]fn read_utf16le_file(path: &str) -> Result<String> 
- [L854:L857]fn parse_mask(s: &str) -> usize 
- [L859:L1022]fn convert(in_file: Option<String>, out_file: Option<String>) 
- [L1024:L1236]fn sort_and_group_config(in_file: Option<String>, out_file: Option<String>) 
- [L1238:L1260]fn hotreload_blacklist(cli: &CliArgs, blacklist: &mut Vec<String>, last_blacklist_mod_time: &mut Option<std::time::SystemTime>) 
- [L1262:L1293]fn hotreload_config(
    cli: &CliArgs,
    configs: &mut HashMap<u32, HashMap<String, ProcessConfig>>,
    last_config_mod_time: &mut Option<std::time::SystemTime>,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    process_level_applied: &mut HashSet<u32>,
) 

## src/error_codes.rs
- [L1:L46]fn error_from_code_win32(code: u32) -> String 
- [L47:L70]fn error_from_ntstatus(status: i32) -> String 

## src/event_trace.rs
- [L34:L34]static ETW_SENDER: Lazy<Mutex<Option<Sender<EtwProcessEvent>>>> = Lazy::new(|| Mutex::new(None));
- [L37:L37]static ETW_ACTIVE: AtomicBool = AtomicBool::new(false);
- [L72:L77]struct EtwProcessEvent {
    pub pid: u32,
    pub is_start: bool,
}
- [L79:L85]struct EtwProcessMonitor {
    control_handle: CONTROLTRACE_HANDLE,
    trace_handle: PROCESSTRACE_HANDLE,
    properties_buf: Vec<u8>,
    process_thread: Option<thread::JoinHandle<()>>,
}

## src/logging.rs
- [L11:L11]static FINDS_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));
- [L62:L62]static USE_CONSOLE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));
- [L63:L63]static DUST_BIN_MODE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));
- [L64:L64]static LOCAL_TIME_BUFFER: Lazy<Mutex<DateTime<Local>>> = Lazy::new(|| Mutex::new(Local::now()));
- [L65:L65]static LOG_FILE: Lazy<Mutex<File>> =
- [L67:L67]static FIND_LOG_FILE: Lazy<Mutex<File>> =
- [L69:L69]static FINDS_FAIL_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));
- [L70:L70]static PID_MAP_FAIL_ENTRY_SET: Lazy<Mutex<HashMap<u32, HashMap<ApplyFailEntry, bool>>>> = Lazy::new(|| Mutex::new(HashMap::new()));
- [L74:L95]enum Operation {
    OpenProcess2processQueryLimitedInformation,
    OpenProcess2processSetLimitedInformation,
    OpenProcess2processQueryInformation,
    OpenProcess2processSetInformation,
    OpenThread,
    SetPriorityClass,
    GetProcessAffinityMask,
    SetProcessAffinityMask,
    GetProcessDefaultCpuSets,
    SetProcessDefaultCpuSets,
    QueryThreadCycleTime,
    SetThreadSelectedCpuSets,
    SetThreadPriority,
    NtQueryInformationProcess2ProcessInformationIOPriority,
    NtSetInformationProcess2ProcessInformationIOPriority,
    GetProcessInformation2ProcessMemoryPriority,
    SetProcessInformation2ProcessMemoryPriority,
    SetThreadIdealProcessorEx,
    GetThreadIdealProcessorEx,
    InvalidHandle,
}
- [L97:L102]struct ApplyFailEntry {
    tid: u32,
    process_name: String,
    operation: Operation,
    error_code: u32,
}
- [L104:L147]fn is_new_error(pid: u32, tid: u32, process_name: &str, operation: Operation, error_code: u32) -> bool 
- [L149:L170]fn purge_fail_map(pids_and_names: &[(u32, String)]) 
- [L172:L181]fn get_log_path(suffix: &str) -> PathBuf 
- [L183:L193]fn log_message(args: &str) 
- [L195:L201]fn log_pure_message(args: &str) 
- [L203:L210]fn log_to_find(msg: &str) 
- [L212:L221]fn log_process_find(process_name: &str) 

## src/main.rs
- [L48:L74]fn apply_config_process_level(
    pid: u32,
    config: &ProcessConfig,
    process: &mut ProcessEntry,
    dry_run: bool,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L76:L115]fn apply_config_thread_level(
    pid: u32,
    config: &ProcessConfig,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    process: &mut ProcessEntry,
    dry_run: bool,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L117:L205]fn process_logs(
    configs: &HashMap<u32, HashMap<String, ProcessConfig>>,
    blacklist: &[String],
    logs_path: Option<&str>,
    output_file: Option<&str>,
) 
- [L207:L241]fn process_find(
    cli: &CliArgs,
    configs: &HashMap<u32, HashMap<String, ProcessConfig>>,
    blacklist: &[String],
) -> Result<(), windows::core::Error> 
- [L243:L512]fn main() -> windows::core::Result<()> 

## src/priority.rs
- [L8:L16]enum ProcessPriority {
    None,
    Idle,
    BelowNormal,
    Normal,
    AboveNormal,
    High,
    Realtime,
}
- [L60:L66]enum IOPriority {
    None,
    VeryLow,
    Low,
    Normal,
    High,
}
- [L109:L109]struct MemoryPriorityInformation(pub u32);
- [L112:L119]enum MemoryPriority {
    None,
    VeryLow,
    Low,
    Medium,
    BelowNormal,
    Normal,
}
- [L162:L174]enum ThreadPriority {
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
- [L7:L7]static SNAPSHOT_BUFFER: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| Mutex::new(vec![0u8; 32]));
- [L9:L9]static PID_TO_PROCESS_MAP: Lazy<Mutex<HashMap<u32, ProcessEntry>>> = Lazy::new(|| Mutex::new(HashMap::new()));
- [L6:L14]struct ProcessSnapshot<'a> {
    buffer: &'a mut Vec<u8>,
    pub pid_to_process: &'a mut HashMap<u32, ProcessEntry>,
}
- [L81:L86]struct ProcessEntry {
    pub process: SYSTEM_PROCESS_INFORMATION,
    threads: HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    threads_base_ptr: usize,
    name: String,
}

## src/scheduler.rs
- [L13:L16]struct PrimeThreadScheduler {
    pub pid_to_process_stats: HashMap<u32, ProcessStats>,
    pub constants: ConfigConstants,
}
- [L177:L184]struct ProcessStats {
    pub alive: bool,
    pub tid_to_thread_stats: HashMap<u32, ThreadStats>,
    pub track_top_x_threads: i32,
    pub process_name: String,
    pub process_id: u32,
}
- [L205:L215]struct IdealProcessorState {
    pub current_group: u16,
    pub current_number: u8,
    pub previous_group: u16,
    pub previous_number: u8,
    pub is_assigned: bool,
}
- [L235:L260]struct ThreadStats {
    pub last_total_time: i64,
    pub cached_total_time: i64,
    pub last_cycles: u64,
    pub cached_cycles: u64,
    pub handle: Option<ThreadHandle>,
    pub pinned_cpu_set_ids: Vec<u32>,
    pub active_streak: u8,
    pub start_address: usize,
    pub original_priority: Option<ThreadPriority>,
    pub last_system_thread_info: Option<SYSTEM_THREAD_INFORMATION>,
    pub ideal_processor: IdealProcessorState,
    pub process_id: u32,
}
- [L302:L306]fn format_100ns(time: i64) -> String 
- [L308:L315]fn format_filetime(time: i64) -> String 

## src/winapi.rs
- [L64:L67]struct CpuSetData {
    id: u32,
    logical_processor_index: u8,
}
- [L69:L76]struct ProcessHandle {
    pub r_limited_handle: HANDLE,
    pub r_handle: Option<HANDLE>,
    pub w_limited_handle: HANDLE,
    pub w_handle: Option<HANDLE>,
}
- [L97:L196]fn get_process_handle(pid: u32, process_name: &str) -> Option<ProcessHandle> 
- [L198:L205]struct ThreadHandle {
    pub r_limited_handle: HANDLE,
    pub r_handle: HANDLE,
    pub w_limited_handle: HANDLE,
    pub w_handle: HANDLE,
}
- [L230:L272]fn get_thread_handle(tid: u32, pid: u32, process_name: &str) -> Option<ThreadHandle> 
- [L274:L302]fn try_open_thread(pid: u32, tid: u32, process_name: &str, access: THREAD_ACCESS_RIGHTS, internal_op_code: u32) -> HANDLE 
- [L313:L313]static CPU_SET_INFORMATION: Lazy<Mutex<Vec<CpuSetData>>> = Lazy::new(|| {
- [L354:L356]fn get_cpu_set_information() -> &'static Mutex<Vec<CpuSetData>> 
- [L358:L375]fn cpusetids_from_indices(cpu_indices: &[u32]) -> Vec<u32> 
- [L378:L391]fn cpusetids_from_mask(mask: usize) -> Vec<u32> 
- [L393:L409]fn indices_from_cpusetids(cpuids: &[u32]) -> Vec<u32> 
- [L412:L427]fn mask_from_cpusetids(cpuids: &[u32]) -> usize 
- [L429:L436]fn filter_indices_by_mask(cpu_indices: &[u32], affinity_mask: usize) -> Vec<u32> 
- [L438:L467]fn is_running_as_admin() -> bool 
- [L469:L502]fn request_uac_elevation(console: bool) -> io::Result<()> 
- [L504:L542]fn enable_debug_privilege() 
- [L544:L582]fn enable_inc_base_priority_privilege() 
- [L584:L636]fn is_affinity_unset(pid: u32, process_name: &str) -> bool 
- [L638:L657]fn get_thread_start_address(thread_handle: HANDLE) -> usize 
- [L659:L670]fn set_thread_ideal_processor_ex(thread_handle: HANDLE, group: u16, number: u8) -> Result<PROCESSOR_NUMBER, Error> 
- [L672:L678]fn get_thread_ideal_processor_ex(thread_handle: HANDLE) -> Result<PROCESSOR_NUMBER, Error> 
- [L681:L681]static MODULE_CACHE: Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>> = Lazy::new(|| Mutex::new(HashMap::new()));
- [L683:L709]fn resolve_address_to_module(pid: u32, address: usize) -> String 
- [L711:L714]fn drop_module_cache(pid: u32) 
- [L716:L766]fn terminate_child_processes() 
- [L768:L821]fn enumerate_process_modules(pid: u32) -> Vec<(usize, usize, String)> 
- [L823:L837]fn set_timer_resolution(cli: &CliArgs) 

