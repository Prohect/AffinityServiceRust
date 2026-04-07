# Src Code Structure Outline, auto-generated during cargo build/check

## src/apply.rs
- [L29:L32]struct ApplyConfigResult {
    pub changes: Vec<String>,
    pub errors: Vec<String>,
}
- [L53:L97]fn apply_priority(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L99:L161]fn apply_affinity(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    current_mask: &mut usize,
    apply_config_result: &mut ApplyConfigResult,
    processes: &mut ProcessSnapshot,
) 
- [L163:L241]fn apply_process_default_cpuset(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L243:L351]fn prefetch_all_thread_cycles(
    pid: u32,
    config: &ProcessConfig,
    processes: &mut ProcessSnapshot,
    prime_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L353:L366]fn update_thread_stats(pid: u32, prime_scheduler: &mut PrimeThreadScheduler) 
- [L368:L470]fn apply_prime_threads(
    pid: u32,
    config: &ProcessConfig,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    processes: &mut ProcessSnapshot,
    dry_run: bool,
    current_mask: &mut usize,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L472:L486]fn apply_prime_threads_select(
    tid_with_delta_cycles: &mut [(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    pid: u32,
    prime_count: usize,
) 
- [L488:L594]fn apply_prime_threads_promote(
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    pid: u32,
    config: &ProcessConfig,
    current_mask: &mut usize,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L596:L657]fn apply_prime_threads_demote(
    process: &mut crate::process::ProcessEntry,
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    pid: u32,
    config: &ProcessConfig,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L659:L745]fn apply_io_priority(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L747:L830]fn apply_memory_priority(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L832:L929]fn reset_thread_ideal_processors(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    apply_config_result: &mut ApplyConfigResult,
    processes: &mut ProcessSnapshot,
) 
- [L931:L1135]fn apply_ideal_processors(
    pid: u32,
    config: &ProcessConfig,
    processes: &mut ProcessSnapshot,
    prime_scheduler: &mut PrimeThreadScheduler,
    dry_run: bool,
    apply_config_result: &mut ApplyConfigResult,
) 

## src/cli.rs
- [L4:L25]struct CliArgs {
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
- [L37:L118]fn parse_args(args: &[String], cli: &mut CliArgs) -> windows::core::Result<()> 
- [L120:L146]fn print_help() 
- [L148:L196]fn print_cli_help() 
- [L198:L304]fn get_config_help_lines() -> Vec<&'static str> 
- [L306:L310]fn print_config_help() 
- [L312:L317]fn print_help_all() 

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
- [L36:L47]struct ProcessConfig {
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
- [L50:L54]struct ConfigConstants {
    pub min_active_streak: u8,
    pub keep_threshold: f64,
    pub entry_threshold: f64,
}
- [L66:L114]fn parse_cpu_spec(s: &str) -> Vec<u32> 
- [L116:L118]fn mask_to_cpu_indices(mask: u64) -> Vec<u32> 
- [L120:L128]fn cpu_indices_to_mask(cpus: &[u32]) -> usize 
- [L130:L160]fn format_cpu_indices(cpus: &[u32]) -> String 
- [L163:L174]struct ConfigResult {
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
- [L217:L237]fn resolve_cpu_spec(
    spec: &str,
    field_name: &str,
    line_number: usize,
    cpu_aliases: &HashMap<String, Vec<u32>>,
    errors: &mut Vec<String>,
) -> Vec<u32> 
- [L239:L245]fn collect_members(text: &str, members: &mut Vec<String>) 
- [L248:L288]fn parse_constant(name: &str, value: &str, line_number: usize, result: &mut ConfigResult) 
- [L290:L304]fn parse_alias(name: &str, value: &str, line_number: usize, cpu_aliases: &mut HashMap<String, Vec<u32>>, result: &mut ConfigResult) 
- [L306:L375]fn parse_ideal_processor_spec(
    spec: &str,
    line_number: usize,
    cpu_aliases: &HashMap<String, Vec<u32>>,
    errors: &mut Vec<String>,
) -> Vec<IdealProcessorRule> 
- [L377:L387]fn collect_group_block(lines: &[String], start_index: usize, first_line_content: &str) -> Option<(Vec<String>, Option<String>, usize)> 
- [L411:L689]fn parse_and_insert_rules(
    members: &[String],
    rule_parts: &[&str],
    line_number: usize,
    cpu_aliases: &HashMap<String, Vec<u32>>,
    result: &mut ConfigResult,
) 
- [L691:L777]fn read_config<P: AsRef<Path>>(path: P) -> ConfigResult 
- [L826:L835]fn read_list<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<String>> 
- [L837:L841]fn read_utf16le_file(path: &str) -> std::io::Result<String> 
- [L844:L847]fn parse_mask(s: &str) -> usize 
- [L849:L1012]fn convert(in_file: Option<String>, out_file: Option<String>) 
- [L1014:L1226]fn sort_and_group_config(in_file: Option<String>, out_file: Option<String>) 

## src/error_codes.rs
- [L1:L44]fn error_from_code_win32(code: u32) -> String 
- [L45:L68]fn error_from_ntstatus(status: i32) -> String 

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
- [L47:L104]fn apply_config(
    pid: u32,
    config: &ProcessConfig,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    processes: &mut ProcessSnapshot,
    dry_run: bool,
) -> ApplyConfigResult 
- [L106:L194]fn process_logs(
    configs: &HashMap<u32, HashMap<String, ProcessConfig>>,
    blacklist: &[String],
    logs_path: Option<&str>,
    output_file: Option<&str>,
) 
- [L196:L458]fn main() -> windows::core::Result<()> 

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
- [L13:L16]struct PrimeThreadScheduler {
    pub pid_to_process_stats: HashMap<u32, ProcessStats>,
    pub constants: ConfigConstants,
}
- [L186:L193]struct ProcessStats {
    pub alive: bool,
    pub tid_to_thread_stats: HashMap<u32, ThreadStats>,
    pub track_top_x_threads: i32,
    pub process_name: String,
    pub process_id: u32,
}
- [L214:L224]struct IdealProcessorState {
    pub current_group: u16,
    pub current_number: u8,
    pub previous_group: u16,
    pub previous_number: u8,
    pub is_assigned: bool,
}
- [L244:L266]struct ThreadStats {
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
- [L309:L313]fn format_100ns(time: i64) -> String 
- [L315:L322]fn format_filetime(time: i64) -> String 

## src/winapi.rs
- [L58:L61]struct CpuSetData {
    id: u32,
    logical_processor_index: u8,
}
- [L63:L70]struct ProcessHandle {
    pub r_limited_handle: HANDLE,
    pub r_handle: Option<HANDLE>,
    pub w_limited_handle: HANDLE,
    pub w_handle: Option<HANDLE>,
}
- [L91:L191]fn get_process_handle(pid: u32, process_name: &str) -> Option<ProcessHandle> 
- [L202:L202]static CPU_SET_INFORMATION: Lazy<Mutex<Vec<CpuSetData>>> = Lazy::new(|| {
- [L243:L245]fn get_cpu_set_information() -> &'static Mutex<Vec<CpuSetData>> 
- [L247:L264]fn cpusetids_from_indices(cpu_indices: &[u32]) -> Vec<u32> 
- [L267:L280]fn cpusetids_from_mask(mask: usize) -> Vec<u32> 
- [L282:L298]fn indices_from_cpusetids(cpuids: &[u32]) -> Vec<u32> 
- [L301:L316]fn mask_from_cpusetids(cpuids: &[u32]) -> usize 
- [L318:L328]fn filter_indices_by_mask(cpu_indices: &[u32], affinity_mask: usize) -> Vec<u32> 
- [L330:L359]fn is_running_as_admin() -> bool 
- [L361:L394]fn request_uac_elevation(console: bool) -> io::Result<()> 
- [L396:L434]fn enable_debug_privilege() 
- [L436:L474]fn enable_inc_base_priority_privilege() 
- [L476:L528]fn is_affinity_unset(pid: u32, process_name: &str) -> bool 
- [L530:L549]fn get_thread_start_address(thread_handle: HANDLE) -> usize 
- [L551:L562]fn set_thread_ideal_processor_ex(thread_handle: HANDLE, group: u16, number: u8) -> Result<PROCESSOR_NUMBER, windows::core::Error> 
- [L564:L570]fn get_thread_ideal_processor_ex(thread_handle: HANDLE) -> Result<PROCESSOR_NUMBER, windows::core::Error> 
- [L573:L573]static MODULE_CACHE: Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>> = Lazy::new(|| Mutex::new(HashMap::new()));
- [L575:L601]fn resolve_address_to_module(pid: u32, address: usize) -> String 
- [L603:L606]fn drop_module_cache(pid: u32) 
- [L608:L658]fn terminate_child_processes() 
- [L660:L713]fn enumerate_process_modules(pid: u32) -> Vec<(usize, usize, String)> 

