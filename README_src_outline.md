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
- [L5:L26]struct CliArgs {
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
- [L38:L119]fn parse_args(args: &[String], cli: &mut CliArgs) -> Result<()> 
- [L121:L147]fn print_help() 
- [L149:L197]fn print_cli_help() 
- [L199:L229]fn get_config_help_lines() -> Vec<&'static str> 
- [L231:L235]fn print_config_help() 
- [L237:L242]fn print_help_all() 

## src/config.rs
- [L16:L20]struct PrimePrefix {
    pub prefix: String,
    pub cpus: Option<Vec<u32>>,
    pub thread_priority: ThreadPriority,
}
- [L23:L26]struct IdealProcessorRule {
    pub cpus: Vec<u32>,
    pub prefixes: Vec<String>,
}
- [L29:L44]struct ProcessConfig {
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
- [L47:L51]struct ConfigConstants {
    pub min_active_streak: u8,
    pub keep_threshold: f64,
    pub entry_threshold: f64,
}
- [L63:L111]fn parse_cpu_spec(s: &str) -> Vec<u32> 
- [L113:L115]fn mask_to_cpu_indices(mask: u64) -> Vec<u32> 
- [L117:L125]fn cpu_indices_to_mask(cpus: &[u32]) -> usize 
- [L127:L157]fn format_cpu_indices(cpus: &[u32]) -> String 
- [L160:L171]struct ConfigResult {
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
- [L214:L234]fn resolve_cpu_spec(
    spec: &str,
    field_name: &str,
    line_number: usize,
    cpu_aliases: &HashMap<String, Vec<u32>>,
    errors: &mut Vec<String>,
) -> Vec<u32> 
- [L236:L242]fn collect_members(text: &str, members: &mut Vec<String>) 
- [L245:L285]fn parse_constant(name: &str, value: &str, line_number: usize, result: &mut ConfigResult) 
- [L287:L301]fn parse_alias(name: &str, value: &str, line_number: usize, cpu_aliases: &mut HashMap<String, Vec<u32>>, result: &mut ConfigResult) 
- [L303:L372]fn parse_ideal_processor_spec(
    spec: &str,
    line_number: usize,
    cpu_aliases: &HashMap<String, Vec<u32>>,
    errors: &mut Vec<String>,
) -> Vec<IdealProcessorRule> 
- [L374:L384]fn collect_group_block(lines: &[String], start_index: usize, first_line_content: &str) -> Option<(Vec<String>, Option<String>, usize)> 
- [L408:L698]fn parse_and_insert_rules(
    members: &[String],
    rule_parts: &[&str],
    line_number: usize,
    cpu_aliases: &HashMap<String, Vec<u32>>,
    result: &mut ConfigResult,
) 
- [L700:L786]fn read_config<P: AsRef<Path>>(path: P) -> ConfigResult 
- [L835:L844]fn read_list<P: AsRef<Path>>(path: P) -> Result<Vec<String>> 
- [L846:L850]fn read_utf16le_file(path: &str) -> Result<String> 
- [L853:L856]fn parse_mask(s: &str) -> usize 
- [L858:L1021]fn convert(in_file: Option<String>, out_file: Option<String>) 
- [L1023:L1235]fn sort_and_group_config(in_file: Option<String>, out_file: Option<String>) 

## src/error_codes.rs
- [L1:L46]fn error_from_code_win32(code: u32) -> String 
- [L47:L70]fn error_from_ntstatus(status: i32) -> String 

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
- [L97:L103]struct ApplyFailEntry {
    pid: u32,
    tid: u32,
    process_name: String,
    operation: Operation,
    error_code: u32,
}
- [L105:L149]fn is_new_error(pid: u32, tid: u32, process_name: &str, operation: Operation, error_code: u32) -> bool 
- [L151:L172]fn purge_fail_map(pids_and_names: &[(u32, String)]) 
- [L174:L183]fn get_log_path(suffix: &str) -> PathBuf 
- [L185:L195]fn log_message(args: &str) 
- [L197:L203]fn log_pure_message(args: &str) 
- [L205:L212]fn log_to_find(msg: &str) 
- [L214:L223]fn log_process_find(process_name: &str) 

## src/main.rs
- [L46:L104]fn apply_config(
    pid: u32,
    config: &ProcessConfig,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    process: &mut ProcessEntry,
    dry_run: bool,
) -> ApplyConfigResult 
- [L106:L194]fn process_logs(
    configs: &HashMap<u32, HashMap<String, ProcessConfig>>,
    blacklist: &[String],
    logs_path: Option<&str>,
    output_file: Option<&str>,
) 
- [L196:L456]fn main() -> windows::core::Result<()> 

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
- [L4:L7]struct ProcessSnapshot<'a> {
    buffer: &'a mut Vec<u8>,
    pub pid_to_process: HashMap<u32, ProcessEntry>,
}
- [L74:L79]struct ProcessEntry {
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
- [L178:L185]struct ProcessStats {
    pub alive: bool,
    pub tid_to_thread_stats: HashMap<u32, ThreadStats>,
    pub track_top_x_threads: i32,
    pub process_name: String,
    pub process_id: u32,
}
- [L206:L216]struct IdealProcessorState {
    pub current_group: u16,
    pub current_number: u8,
    pub previous_group: u16,
    pub previous_number: u8,
    pub is_assigned: bool,
}
- [L236:L261]struct ThreadStats {
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
- [L303:L307]fn format_100ns(time: i64) -> String 
- [L309:L316]fn format_filetime(time: i64) -> String 

## src/winapi.rs
- [L63:L66]struct CpuSetData {
    id: u32,
    logical_processor_index: u8,
}
- [L68:L75]struct ProcessHandle {
    pub r_limited_handle: HANDLE,
    pub r_handle: Option<HANDLE>,
    pub w_limited_handle: HANDLE,
    pub w_handle: Option<HANDLE>,
}
- [L96:L195]fn get_process_handle(pid: u32, process_name: &str) -> Option<ProcessHandle> 
- [L197:L204]struct ThreadHandle {
    pub r_limited_handle: HANDLE,
    pub r_handle: HANDLE,
    pub w_limited_handle: HANDLE,
    pub w_handle: HANDLE,
}
- [L229:L271]fn get_thread_handle(tid: u32, pid: u32, process_name: &str) -> Option<ThreadHandle> 
- [L273:L301]fn try_open_thread(pid: u32, tid: u32, process_name: &str, access: THREAD_ACCESS_RIGHTS, internal_op_code: u32) -> HANDLE 
- [L312:L312]static CPU_SET_INFORMATION: Lazy<Mutex<Vec<CpuSetData>>> = Lazy::new(|| {
- [L353:L355]fn get_cpu_set_information() -> &'static Mutex<Vec<CpuSetData>> 
- [L357:L374]fn cpusetids_from_indices(cpu_indices: &[u32]) -> Vec<u32> 
- [L377:L390]fn cpusetids_from_mask(mask: usize) -> Vec<u32> 
- [L392:L408]fn indices_from_cpusetids(cpuids: &[u32]) -> Vec<u32> 
- [L411:L426]fn mask_from_cpusetids(cpuids: &[u32]) -> usize 
- [L428:L435]fn filter_indices_by_mask(cpu_indices: &[u32], affinity_mask: usize) -> Vec<u32> 
- [L437:L466]fn is_running_as_admin() -> bool 
- [L468:L501]fn request_uac_elevation(console: bool) -> io::Result<()> 
- [L503:L541]fn enable_debug_privilege() 
- [L543:L581]fn enable_inc_base_priority_privilege() 
- [L583:L635]fn is_affinity_unset(pid: u32, process_name: &str) -> bool 
- [L637:L656]fn get_thread_start_address(thread_handle: HANDLE) -> usize 
- [L658:L669]fn set_thread_ideal_processor_ex(thread_handle: HANDLE, group: u16, number: u8) -> Result<PROCESSOR_NUMBER, Error> 
- [L671:L677]fn get_thread_ideal_processor_ex(thread_handle: HANDLE) -> Result<PROCESSOR_NUMBER, Error> 
- [L680:L680]static MODULE_CACHE: Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>> = Lazy::new(|| Mutex::new(HashMap::new()));
- [L682:L708]fn resolve_address_to_module(pid: u32, address: usize) -> String 
- [L710:L713]fn drop_module_cache(pid: u32) 
- [L715:L765]fn terminate_child_processes() 
- [L767:L820]fn enumerate_process_modules(pid: u32) -> Vec<(usize, usize, String)> 

