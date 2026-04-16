# Src Code Structure Outline, auto-generated during cargo build/check

Read this by multiple calls if it's too large to fit in one

## src/apply.rs
- [L32:35]struct ApplyConfigResult {
    pub changes: Vec<String>,
    pub errors: Vec<String>,
}
- [L60:67]fn get_handles(process_handle: &ProcessHandle) -> (Option<HANDLE>, Option<HANDLE>) 
- [L69:83]fn log_error_if_new(
    pid: u32,
    tid: u32,
    process_name: &str,
    operation: Operation,
    error_code: u32,
    apply_config_result: &mut ApplyConfigResult,
    format_msg: impl FnOnce() -> String,
) 
- [L85:131]fn apply_priority(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L133:209]fn apply_affinity<'a>(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    current_mask: &mut usize,
    process_handle: &ProcessHandle,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L211:295]fn reset_thread_ideal_processors<'a>(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    cpus: &[u32],
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L297:400]fn apply_process_default_cpuset<'a>(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L402:488]fn apply_io_priority(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L490:577]fn apply_memory_priority(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L579:686]fn prefetch_all_thread_cycles<'a>(
    pid: u32,
    config: &ThreadLevelConfig,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    prime_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L688:788]fn apply_prime_threads<'a>(
    pid: u32,
    config: &ThreadLevelConfig,
    dry_run: bool,
    current_mask: &mut usize,
    process: &'a ProcessEntry,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L790:804]fn apply_prime_threads_select(
    pid: u32,
    prime_count: usize,
    tid_with_delta_cycles: &mut [(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
) 
- [L806:948]fn apply_prime_threads_promote(
    pid: u32,
    config: &ThreadLevelConfig,
    current_mask: &mut usize,
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L950:1041]fn apply_prime_threads_demote<'a>(
    pid: u32,
    config: &ThreadLevelConfig,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L1043:1311]fn apply_ideal_processors<'a>(
    pid: u32,
    config: &ThreadLevelConfig,
    dry_run: bool,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    prime_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L1313:1326]fn update_thread_stats(pid: u32, prime_scheduler: &mut PrimeThreadScheduler) 

## src/cli.rs
- [L5:28]struct CliArgs {
    pub interval_ms: u32,
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
- [L40:127]fn parse_args(args: &[String], cli: &mut CliArgs) -> Result<()> 
- [L129:155]fn print_help() 
- [L157:207]fn print_cli_help() 
- [L209:318]fn get_config_help_lines() -> Vec<&'static str> 
- [L320:324]fn print_config_help() 
- [L326:331]fn print_help_all() 

## src/collections.rs
- [L4:4]type HashMap<K, V> = FxHashMap<K, V>;
- [L5:5]type HashSet<V> = FxHashSet<V>;
- [L6:6]type List<E> = SmallVec<E>;
- [L9:9]const PIDS: usize = 512;
- [L10:10]const TIDS_FULL: usize = 128;
- [L11:11]const TIDS_CAPED: usize = 64;
- [L12:12]const CONSUMER_CPUS: usize = 32;
- [L13:13]const PENDING: usize = 16;

## src/config.rs
- [L17:19]struct PrimePrefix {
    pub prefix: String,
    pub cpus: Option<List<[u32; CONSUMER_CPUS]>>,
- [L24:25]struct IdealProcessorRule {
    pub cpus: List<[u32; CONSUMER_CPUS]>,
- [L30:33]struct ProcessLevelConfig {
    pub name: String,
    pub priority: ProcessPriority,
    pub affinity_cpus: List<[u32; CONSUMER_CPUS]>,
- [L40:42]struct ThreadLevelConfig {
    pub name: String,
    pub prime_threads_cpus: List<[u32; CONSUMER_CPUS]>,
- [L49:53]struct ConfigConstants {
    pub min_active_streak: u8,
    pub keep_threshold: f64,
    pub entry_threshold: f64,
}
- [L65:113]fn parse_cpu_spec(s: &str) -> List<[u32; CONSUMER_CPUS]> 
- [L115:117]fn mask_to_cpu_indices(mask: u64) -> List<[u32; CONSUMER_CPUS]> 
- [L119:127]fn cpu_indices_to_mask(cpus: &[u32]) -> usize 
- [L129:159]fn format_cpu_indices(cpus: &[u32]) -> String 
- [L162:175]struct ConfigResult {
    pub process_level_configs: HashMap<u32, HashMap<String, ProcessLevelConfig>>,
    pub thread_level_configs: HashMap<u32, HashMap<String, ThreadLevelConfig>>,
    pub constants: ConfigConstants,
    pub constants_count: usize,
    pub aliases_count: usize,
    pub groups_count: usize,
    pub group_members_count: usize,
    pub process_rules_count: usize,
    pub redundant_rules_count: usize,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub thread_level_configs_count: usize,
}
- [L220:240]fn resolve_cpu_spec(
    spec: &str,
    field_name: &str,
    line_number: usize,
    cpu_aliases: &HashMap<String, List<[u32; CONSUMER_CPUS]>>,
    errors: &mut Vec<String>,
) -> List<[u32; CONSUMER_CPUS]> 
- [L242:248]fn collect_members(text: &str, members: &mut Vec<String>) 
- [L251:291]fn parse_constant(name: &str, value: &str, line_number: usize, result: &mut ConfigResult) 
- [L293:313]fn parse_alias(
    name: &str,
    value: &str,
    line_number: usize,
    cpu_aliases: &mut HashMap<String, List<[u32; CONSUMER_CPUS]>>,
    result: &mut ConfigResult,
) 
- [L315:384]fn parse_ideal_processor_spec(
    spec: &str,
    line_number: usize,
    cpu_aliases: &HashMap<String, List<[u32; CONSUMER_CPUS]>>,
    errors: &mut Vec<String>,
) -> Vec<IdealProcessorRule> 
- [L386:396]fn collect_group_block(lines: &[String], start_index: usize, first_line_content: &str) -> Option<(Vec<String>, Option<String>, usize)> 
- [L420:741]fn parse_and_insert_rules(
    members: &[String],
    rule_parts: &[&str],
    line_number: usize,
    cpu_aliases: &HashMap<String, List<[u32; CONSUMER_CPUS]>>,
    result: &mut ConfigResult,
) 
- [L743:829]fn read_config<P: AsRef<Path>>(path: P) -> ConfigResult 
- [L877:886]fn read_list<P: AsRef<Path>>(path: P) -> Result<Vec<String>> 
- [L888:892]fn read_utf16le_file(path: &str) -> Result<String> 
- [L895:898]fn parse_mask(s: &str) -> usize 
- [L900:1063]fn convert(in_file: Option<String>, out_file: Option<String>) 
- [L1065:1277]fn sort_and_group_config(in_file: Option<String>, out_file: Option<String>) 
- [L1279:1301]fn hotreload_blacklist(cli: &CliArgs, blacklist: &mut Vec<String>, last_blacklist_mod_time: &mut Option<std::time::SystemTime>) 
- [L1303:1334]fn hotreload_config(
    cli: &CliArgs,
    configs: &mut ConfigResult,
    last_config_mod_time: &mut Option<std::time::SystemTime>,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    process_level_applied: &mut List<[u32; PIDS]>,
) 

## src/error_codes.rs
- [L1:46]fn error_from_code_win32(code: u32) -> String 
- [L47:70]fn error_from_ntstatus(status: i32) -> String 

## src/event_trace.rs
- [L34:34]static ETW_SENDER: Lazy<Mutex<Option<Sender<EtwProcessEvent>>>> = Lazy::new(|| Mutex::new(None));
- [L37:37]static ETW_ACTIVE: AtomicBool = AtomicBool::new(false);
- [L72:77]struct EtwProcessEvent {
    pub pid: u32,
    pub is_start: bool,
}
- [L79:85]struct EtwProcessMonitor {
    control_handle: CONTROLTRACE_HANDLE,
    trace_handle: PROCESSTRACE_HANDLE,
    properties_buf: Vec<u8>,
    process_thread: Option<thread::JoinHandle<()>>,
}

## src/logging.rs
- [L11:11]static FINDS_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::default()));
- [L62:62]static USE_CONSOLE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));
- [L63:63]static DUST_BIN_MODE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));
- [L64:64]static LOCAL_TIME_BUFFER: Lazy<Mutex<DateTime<Local>>> = Lazy::new(|| Mutex::new(Local::now()));
- [L65:65]static LOG_FILE: Lazy<Mutex<File>> =
- [L67:67]static FIND_LOG_FILE: Lazy<Mutex<File>> =
- [L69:69]static FINDS_FAIL_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::default()));
- [L70:70]static PID_MAP_FAIL_ENTRY_SET: Lazy<Mutex<HashMap<u32, HashMap<ApplyFailEntry, bool>>>> = Lazy::new(|| Mutex::new(HashMap::default()));
- [L74:95]enum Operation {
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
- [L97:102]struct ApplyFailEntry {
    tid: u32,
    process_name: String,
    operation: Operation,
    error_code: u32,
}
- [L104:147]fn is_new_error(pid: u32, tid: u32, process_name: &str, operation: Operation, error_code: u32) -> bool 
- [L149:170]fn purge_fail_map(pids_and_names: &[(u32, &str)]) 
- [L172:181]fn get_log_path(suffix: &str) -> PathBuf 
- [L183:193]fn log_message(args: &str) 
- [L195:201]fn log_pure_message(args: &str) 
- [L203:210]fn log_to_find(msg: &str) 
- [L212:221]fn log_process_find(process_name: &str) 

## src/main.rs
- [L56:74]fn apply_process_level<'a>(
    pid: u32,
    config: &ProcessLevelConfig,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    dry_run: bool,
    apply_configs: &mut ApplyConfigResult,
) 
- [L76:117]fn apply_thread_level<'a>(
    pid: u32,
    config: &ThreadLevelConfig,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    process: &'a ProcessEntry,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    dry_run: bool,
    apply_configs: &mut ApplyConfigResult,
) 
- [L119:154]fn apply_config(
    cli: &CliArgs,
    configs: &ConfigResult,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    process_level_applied: &mut smallvec::SmallVec<[u32; PIDS]>,
    thread_level_applied: &mut smallvec::SmallVec<[u32; PENDING]>,
    grade: &u32,
    pid: &u32,
    name: &&str,
    process_level_config: &ProcessLevelConfig,
    process: &ProcessEntry,
) 
- [L156:170]fn log_apply_results(pid: &u32, name: &String, result: ApplyConfigResult) 
- [L172:264]fn process_logs(configs: &ConfigResult, blacklist: &[String], logs_path: Option<&str>, output_file: Option<&str>) 
- [L266:303]fn process_find(cli: &CliArgs, configs: &ConfigResult, blacklist: &[String]) -> Result<(), windows::core::Error> 
- [L305:627]fn main() -> windows::core::Result<()> 

## src/priority.rs
- [L8:16]enum ProcessPriority {
    None,
    Idle,
    BelowNormal,
    Normal,
    AboveNormal,
    High,
    Realtime,
}
- [L60:66]enum IOPriority {
    None,
    VeryLow,
    Low,
    Normal,
    High,
}
- [L109:109]struct MemoryPriorityInformation(pub u32);
- [L112:119]enum MemoryPriority {
    None,
    VeryLow,
    Low,
    Medium,
    BelowNormal,
    Normal,
}
- [L162:174]enum ThreadPriority {
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
- [L7:7]static SNAPSHOT_BUFFER: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| Mutex::new(vec![0u8; 32]));
- [L9:9]static PID_TO_PROCESS_MAP: Lazy<Mutex<HashMap<u32, ProcessEntry>>> = Lazy::new(|| Mutex::new(HashMap::default()));
- [L6:14]struct ProcessSnapshot<'a> {
    buffer: &'a mut Vec<u8>,
    pub pid_to_process: &'a mut HashMap<u32, ProcessEntry>,
}
- [L76:80]struct ProcessEntry {
    pub process: SYSTEM_PROCESS_INFORMATION,
    threads_base_ptr: usize,
    name: String,
}

## src/scheduler.rs
- [L15:18]struct PrimeThreadScheduler {
    pub pid_to_process_stats: HashMap<u32, ProcessStats>,
    pub constants: ConfigConstants,
}
- [L186:193]struct ProcessStats {
    pub alive: bool,
    pub tid_to_thread_stats: HashMap<u32, ThreadStats>,
    pub track_top_x_threads: i32,
    pub process_name: String,
    pub process_id: u32,
}
- [L214:224]struct IdealProcessorState {
    pub current_group: u16,
    pub current_number: u8,
    pub previous_group: u16,
    pub previous_number: u8,
    pub is_assigned: bool,
}
- [L244:258]struct ThreadStats {
    pub last_total_time: i64,
    pub cached_total_time: i64,
    pub last_cycles: u64,
    pub cached_cycles: u64,
    pub handle: Option<ThreadHandle>,
    pub pinned_cpu_set_ids: List<[u32; CONSUMER_CPUS]>,
- [L311:315]fn format_100ns(time: i64) -> String 
- [L317:324]fn format_filetime(time: i64) -> String 

## src/winapi.rs
- [L65:68]struct CpuSetData {
    id: u32,
    logical_processor_index: u8,
}
- [L70:77]struct ProcessHandle {
    pub r_limited_handle: HANDLE,
    pub r_handle: Option<HANDLE>,
    pub w_limited_handle: HANDLE,
    pub w_handle: Option<HANDLE>,
}
- [L98:197]fn get_process_handle(pid: u32, process_name: &str) -> Option<ProcessHandle> 
- [L199:206]struct ThreadHandle {
    pub r_limited_handle: HANDLE,
    pub r_handle: HANDLE,
    pub w_limited_handle: HANDLE,
    pub w_handle: HANDLE,
}
- [L231:273]fn get_thread_handle(tid: u32, pid: u32, process_name: &str) -> Option<ThreadHandle> 
- [L275:303]fn try_open_thread(pid: u32, tid: u32, process_name: &str, access: THREAD_ACCESS_RIGHTS, internal_op_code: u32) -> HANDLE 
- [L314:314]static CPU_SET_INFORMATION: Lazy<Mutex<Vec<CpuSetData>>> = Lazy::new(|| {
- [L355:357]fn get_cpu_set_information() -> &'static Mutex<Vec<CpuSetData>> 
- [L359:376]fn cpusetids_from_indices(cpu_indices: &[u32]) -> List<[u32; CONSUMER_CPUS]> 
- [L379:392]fn cpusetids_from_mask(mask: usize) -> List<[u32; CONSUMER_CPUS]> 
- [L394:410]fn indices_from_cpusetids(cpuids: &[u32]) -> List<[u32; CONSUMER_CPUS]> 
- [L413:428]fn mask_from_cpusetids(cpuids: &[u32]) -> usize 
- [L430:437]fn filter_indices_by_mask(cpu_indices: &[u32], affinity_mask: usize) -> List<[u32; CONSUMER_CPUS]> 
- [L439:468]fn is_running_as_admin() -> bool 
- [L470:503]fn request_uac_elevation(console: bool) -> io::Result<()> 
- [L505:543]fn enable_debug_privilege() 
- [L545:583]fn enable_inc_base_priority_privilege() 
- [L585:637]fn is_affinity_unset(pid: u32, process_name: &str) -> bool 
- [L639:658]fn get_thread_start_address(thread_handle: HANDLE) -> usize 
- [L660:671]fn set_thread_ideal_processor_ex(thread_handle: HANDLE, group: u16, number: u8) -> Result<PROCESSOR_NUMBER, Error> 
- [L673:679]fn get_thread_ideal_processor_ex(thread_handle: HANDLE) -> Result<PROCESSOR_NUMBER, Error> 
- [L682:682]static MODULE_CACHE: Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>> = Lazy::new(|| Mutex::new(HashMap::default()));
- [L684:710]fn resolve_address_to_module(pid: u32, address: usize) -> String 
- [L712:715]fn drop_module_cache(pid: u32) 
- [L717:767]fn terminate_child_processes() 
- [L769:822]fn enumerate_process_modules(pid: u32) -> Vec<(usize, usize, String)> 
- [L824:838]fn set_timer_resolution(cli: &CliArgs) 

