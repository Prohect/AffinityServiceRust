# Src Code Structure Outline, auto-generated during cargo build/check

## src/apply.rs
- [L31:L34]struct ApplyConfigResult {
    pub changes: Vec<String>,
    pub errors: Vec<String>,
}
- [L59:L66]fn get_handles(process_handle: &ProcessHandle) -> (Option<HANDLE>, Option<HANDLE>) 
- [L68:L81]fn log_error_if_new(
    pid: u32,
    process_name: &str,
    operation: Operation,
    error_code: u32,
    apply_config_result: &mut ApplyConfigResult,
    format_msg: impl FnOnce() -> String,
) 
- [L83:L128]fn apply_priority(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L130:L205]fn apply_affinity(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    current_mask: &mut usize,
    apply_config_result: &mut ApplyConfigResult,
    processes: &mut ProcessSnapshot,
) 
- [L207:L313]fn reset_thread_ideal_processors(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    cpus: &[u32],
    apply_config_result: &mut ApplyConfigResult,
    processes: &mut ProcessSnapshot,
) 
- [L315:L411]fn apply_process_default_cpuset(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L413:L497]fn apply_io_priority(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L499:L584]fn apply_memory_priority(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L586:L701]fn prefetch_all_thread_cycles(
    pid: u32,
    config: &ProcessConfig,
    processes: &mut ProcessSnapshot,
    prime_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L703:L805]fn apply_prime_threads(
    pid: u32,
    config: &ProcessConfig,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    processes: &mut ProcessSnapshot,
    dry_run: bool,
    current_mask: &mut usize,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L807:L821]fn apply_prime_threads_select(
    tid_with_delta_cycles: &mut [(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    pid: u32,
    prime_count: usize,
) 
- [L823:L943]fn apply_prime_threads_promote(
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    pid: u32,
    config: &ProcessConfig,
    current_mask: &mut usize,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L945:L1020]fn apply_prime_threads_demote(
    process: &mut ProcessEntry,
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    pid: u32,
    config: &ProcessConfig,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L1022:L1247]fn apply_ideal_processors(
    pid: u32,
    config: &ProcessConfig,
    processes: &mut ProcessSnapshot,
    prime_scheduler: &mut PrimeThreadScheduler,
    dry_run: bool,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L1249:L1262]fn update_thread_stats(pid: u32, prime_scheduler: &mut PrimeThreadScheduler) 

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
- [L24:L27]struct IdealProcessorPrefix {
    pub prefix: String,
    pub cpus: Vec<u32>,
}
- [L30:L33]struct IdealProcessorRule {
    pub cpus: Vec<u32>,
    pub prefixes: Vec<String>,
}
- [L36:L51]struct ProcessConfig {
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
- [L54:L58]struct ConfigConstants {
    pub min_active_streak: u8,
    pub keep_threshold: f64,
    pub entry_threshold: f64,
}
- [L70:L118]fn parse_cpu_spec(s: &str) -> Vec<u32> 
- [L120:L122]fn mask_to_cpu_indices(mask: u64) -> Vec<u32> 
- [L124:L132]fn cpu_indices_to_mask(cpus: &[u32]) -> usize 
- [L134:L164]fn format_cpu_indices(cpus: &[u32]) -> String 
- [L167:L178]struct ConfigResult {
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
- [L221:L241]fn resolve_cpu_spec(
    spec: &str,
    field_name: &str,
    line_number: usize,
    cpu_aliases: &HashMap<String, Vec<u32>>,
    errors: &mut Vec<String>,
) -> Vec<u32> 
- [L243:L249]fn collect_members(text: &str, members: &mut Vec<String>) 
- [L252:L292]fn parse_constant(name: &str, value: &str, line_number: usize, result: &mut ConfigResult) 
- [L294:L308]fn parse_alias(name: &str, value: &str, line_number: usize, cpu_aliases: &mut HashMap<String, Vec<u32>>, result: &mut ConfigResult) 
- [L310:L379]fn parse_ideal_processor_spec(
    spec: &str,
    line_number: usize,
    cpu_aliases: &HashMap<String, Vec<u32>>,
    errors: &mut Vec<String>,
) -> Vec<IdealProcessorRule> 
- [L381:L391]fn collect_group_block(lines: &[String], start_index: usize, first_line_content: &str) -> Option<(Vec<String>, Option<String>, usize)> 
- [L415:L705]fn parse_and_insert_rules(
    members: &[String],
    rule_parts: &[&str],
    line_number: usize,
    cpu_aliases: &HashMap<String, Vec<u32>>,
    result: &mut ConfigResult,
) 
- [L707:L793]fn read_config<P: AsRef<Path>>(path: P) -> ConfigResult 
- [L842:L851]fn read_list<P: AsRef<Path>>(path: P) -> Result<Vec<String>> 
- [L853:L857]fn read_utf16le_file(path: &str) -> Result<String> 
- [L860:L863]fn parse_mask(s: &str) -> usize 
- [L865:L1028]fn convert(in_file: Option<String>, out_file: Option<String>) 
- [L1030:L1242]fn sort_and_group_config(in_file: Option<String>, out_file: Option<String>) 

## src/error_codes.rs
- [L1:L46]fn error_from_code_win32(code: u32) -> String 
- [L47:L70]fn error_from_ntstatus(status: i32) -> String 

## src/logging.rs
- [L11:L11]static LOCALTIME_BUFFER: Lazy<Mutex<DateTime<Local>>> = Lazy::new(|| Mutex::new(Local::now()));
- [L12:L12]static FINDS_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));
- [L13:L13]static FINDS_FAIL_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));
- [L14:L14]static PID_MAP_FAIL_ENTRY_SET: Lazy<Mutex<HashMap<u32, HashMap<ApplyFailEntry, bool>>>> = Lazy::new(|| Mutex::new(HashMap::new()));
- [L17:L38]enum Operation {
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
- [L40:L45]struct ApplyFailEntry {
    pid: u32,
    process_name: String,
    operation: Operation,
    error_code: u32,
}
- [L47:L90]fn is_new_error(pid: u32, process_name: &str, operation: Operation, error_code: u32) -> bool 
- [L92:L112]fn purge_fail_map(pids_and_names: &[(u32, String)]) 
- [L114:L114]static USE_CONSOLE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));
- [L116:L116]static DUST_BIN_MODE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));
- [L118:L118]static LOG_FILE: Lazy<Mutex<File>> = Lazy::new(|| Mutex::new(OpenOptions::new().append(true).create(true).open(get_log_path("")).unwrap()));
- [L120:L120]static FIND_LOG_FILE: Lazy<Mutex<File>> =
- [L123:L125]fn use_console() -> &'static Mutex<bool> 
- [L127:L129]fn logger() -> &'static Mutex<File> 
- [L131:L133]fn find_logger() -> &'static Mutex<File> 
- [L135:L144]fn get_log_path(suffix: &str) -> PathBuf 
- [L153:L163]fn log_message(args: &str) 
- [L165:L171]fn log_pure_message(args: &str) 
- [L173:L180]fn log_to_find(msg: &str) 
- [L182:L190]fn log_process_find(process_name: &str) 

## src/main.rs
- [L49:L110]fn apply_config(
    pid: u32,
    config: &ProcessConfig,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    processes: &mut ProcessSnapshot,
    dry_run: bool,
) -> ApplyConfigResult 
- [L112:L200]fn process_logs(
    configs: &HashMap<u32, HashMap<String, ProcessConfig>>,
    blacklist: &[String],
    logs_path: Option<&str>,
    output_file: Option<&str>,
) 
- [L202:L458]fn main() -> windows::core::Result<()> 

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
- [L4:L7]struct ProcessSnapshot {
    buffer: Vec<u8>,
    pub pid_to_process: HashMap<u32, ProcessEntry>,
}
- [L71:L76]struct ProcessEntry {
    pub process: SYSTEM_PROCESS_INFORMATION,
    threads: HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    threads_base_ptr: usize,
    name: String,
}

## src/scheduler.rs
- [L14:L17]struct PrimeThreadScheduler {
    pub pid_to_process_stats: HashMap<u32, ProcessStats>,
    pub constants: ConfigConstants,
}
- [L185:L192]struct ProcessStats {
    pub alive: bool,
    pub tid_to_thread_stats: HashMap<u32, ThreadStats>,
    pub track_top_x_threads: i32,
    pub process_name: String,
    pub process_id: u32,
}
- [L213:L223]struct IdealProcessorState {
    pub current_group: u16,
    pub current_number: u8,
    pub previous_group: u16,
    pub previous_number: u8,
    pub is_assigned: bool,
}
- [L243:L265]struct ThreadStats {
    pub last_total_time: i64,
    pub cached_total_time: i64,
    pub last_cycles: u64,
    pub cached_cycles: u64,
    pub handle: Option<HANDLE>,
    pub pinned_cpu_set_ids: Vec<u32>,
    pub active_streak: u8,
    pub start_address: usize,
    pub original_priority: Option<ThreadPriority>,
    pub last_system_thread_info: Option<SYSTEM_THREAD_INFORMATION>,
    pub ideal_processor: IdealProcessorState,
    pub process_id: u32,
}
- [L308:L312]fn format_100ns(time: i64) -> String 
- [L314:L321]fn format_filetime(time: i64) -> String 

## src/winapi.rs
- [L61:L64]struct CpuSetData {
    id: u32,
    logical_processor_index: u8,
}
- [L66:L73]struct ProcessHandle {
    pub r_limited_handle: HANDLE,
    pub r_handle: Option<HANDLE>,
    pub w_limited_handle: HANDLE,
    pub w_handle: Option<HANDLE>,
}
- [L94:L205]fn get_process_handle(pid: u32, process_name: &str) -> Option<ProcessHandle> 
- [L216:L216]static CPU_SET_INFORMATION: Lazy<Mutex<Vec<CpuSetData>>> = Lazy::new(|| {
- [L257:L259]fn get_cpu_set_information() -> &'static Mutex<Vec<CpuSetData>> 
- [L261:L278]fn cpusetids_from_indices(cpu_indices: &[u32]) -> Vec<u32> 
- [L281:L294]fn cpusetids_from_mask(mask: usize) -> Vec<u32> 
- [L296:L312]fn indices_from_cpusetids(cpuids: &[u32]) -> Vec<u32> 
- [L315:L330]fn mask_from_cpusetids(cpuids: &[u32]) -> usize 
- [L332:L339]fn filter_indices_by_mask(cpu_indices: &[u32], affinity_mask: usize) -> Vec<u32> 
- [L341:L370]fn is_running_as_admin() -> bool 
- [L372:L405]fn request_uac_elevation(console: bool) -> io::Result<()> 
- [L407:L445]fn enable_debug_privilege() 
- [L447:L485]fn enable_inc_base_priority_privilege() 
- [L487:L539]fn is_affinity_unset(pid: u32, process_name: &str) -> bool 
- [L541:L560]fn get_thread_start_address(thread_handle: HANDLE) -> usize 
- [L562:L573]fn set_thread_ideal_processor_ex(thread_handle: HANDLE, group: u16, number: u8) -> Result<PROCESSOR_NUMBER, Error> 
- [L575:L581]fn get_thread_ideal_processor_ex(thread_handle: HANDLE) -> Result<PROCESSOR_NUMBER, Error> 
- [L584:L584]static MODULE_CACHE: Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>> = Lazy::new(|| Mutex::new(HashMap::new()));
- [L586:L612]fn resolve_address_to_module(pid: u32, address: usize) -> String 
- [L614:L617]fn drop_module_cache(pid: u32) 
- [L619:L669]fn terminate_child_processes() 
- [L671:L724]fn enumerate_process_modules(pid: u32) -> Vec<(usize, usize, String)> 

