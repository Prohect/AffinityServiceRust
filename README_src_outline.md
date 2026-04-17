# Src Code Structure Outline, auto-generated during cargo build/check, READ this by MULTIPLE calls if it's **too large to fit in one**

## src/apply.rs
- [L32:35]struct ApplyConfigResult {
    pub changes: Vec<String>,
    pub errors: Vec<String>,
}
- [L37]impl ApplyConfigResult
  - [L38:40]fn new() -> Self 
  - [L42:47]fn add_change(&mut self, change: String) 
  - [L49:53]fn add_error(&mut self, error: String) 
  - [L55:57]fn is_empty(&self) -> bool 
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
- [L85:132]fn apply_priority(
    pid: u32,
    config: &ProcessLevelConfig,
    configs: &ConfigResult,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L134:220]fn apply_affinity<'a>(
    pid: u32,
    config: &ProcessLevelConfig,
    configs: &ConfigResult,
    dry_run: bool,
    current_mask: &mut usize,
    process_handle: &ProcessHandle,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L222:307]fn reset_thread_ideal_processors<'a>(
    pid: u32,
    config: &ProcessLevelConfig,
    configs: &ConfigResult,
    dry_run: bool,
    cpus: &[u32],
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L309:421]fn apply_process_default_cpuset<'a>(
    pid: u32,
    config: &ProcessLevelConfig,
    configs: &ConfigResult,
    dry_run: bool,
    process_handle: &ProcessHandle,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L423:510]fn apply_io_priority(
    pid: u32,
    config: &ProcessLevelConfig,
    configs: &ConfigResult,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L512:600]fn apply_memory_priority(
    pid: u32,
    config: &ProcessLevelConfig,
    configs: &ConfigResult,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L602:710]fn prefetch_all_thread_cycles<'a>(
    pid: u32,
    config: &ThreadLevelConfig,
    configs: &ConfigResult,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    prime_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L712:818]fn apply_prime_threads<'a>(
    pid: u32,
    config: &ThreadLevelConfig,
    configs: &ConfigResult,
    dry_run: bool,
    current_mask: &mut usize,
    process: &'a ProcessEntry,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L820:834]fn apply_prime_threads_select(
    pid: u32,
    prime_count: usize,
    tid_with_delta_cycles: &mut [(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
) 
- [L836:989]fn apply_prime_threads_promote(
    pid: u32,
    config: &ThreadLevelConfig,
    configs: &ConfigResult,
    current_mask: &mut usize,
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L991:1093]fn apply_prime_threads_demote<'a>(
    pid: u32,
    config: &ThreadLevelConfig,
    configs: &ConfigResult,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L1095:1398]fn apply_ideal_processors<'a>(
    pid: u32,
    config: &ThreadLevelConfig,
    configs: &ConfigResult,
    dry_run: bool,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    prime_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L1400:1413]fn update_thread_stats(pid: u32, prime_scheduler: &mut PrimeThreadScheduler) 

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
- [L30]impl CliArgs
  - [L31:37]fn new() -> Self 
- [L40:127]fn parse_args(args: &[String], cli: &mut CliArgs) -> Result<()> 
- [L129:155]fn print_help() 
- [L157:207]fn print_cli_help() 
- [L209:318]fn get_config_help_lines() -> Vec<&'static str> 
- [L320:324]fn print_config_help() 
- [L326:331]fn print_help_all() 

## src/collections.rs
- [L4:4]type HashMap<K, V> = FxHashMap<K, V>;
- [L5:5]type HashSet<V> = FxHashSet<V>;
- [L7:7]type List<E> = SmallVec<E>;
- [L36:36]const PIDS: usize = 256; // for very large lists like the process ID list
- [L37:37]const TIDS_FULL: usize = 96; // 96 should be fine for most thread ID lists
- [L38:38]const TIDS_CAPED: usize = 32; // 32 should be fine for most filtered thread ID lists
- [L39:39]const CONSUMER_CPUS_FULL: usize = 32; // for full consumer CPU
- [L40:40]const CONSUMER_CPUS_SINGLE_CLUSTER: usize = 16; // for single-cluster, like a performance core cluster with hyperthreading, cpu alias in rule
- [L41:41]const PENDING: usize = 16; // for pending lists, like newly spawned process_id list
- [L42:42]const LIMITED: usize = 4; // for very small list like the prime_threads_prefixes, probably not that much rule

## src/config.rs
- [L16:18]struct StrIdx(pub u16);
- [L20:22]struct CpuListIdx(pub u16);
- [L24]impl CpuListIdx
  - [L28:30]fn is_empty(self) -> bool 
- [L33:38]struct StringPool {
    strings: List<[String; PIDS]>,
    lookup: HashMap<String, StrIdx>,
}
- [L40]impl Default for StringPool
  - [L41:48]fn default() -> Self 
- [L51]impl StringPool
  - [L52:60]fn intern(&mut self, s: &str) -> StrIdx 
  - [L62:64]fn get(&self, idx: StrIdx) -> &str 
  - [L66:68]fn find(&self, s: &str) -> Option<StrIdx> 
- [L71:75]struct CpuPool {
    lists: List<[List<[u32; CONSUMER_CPUS_FULL]>; PENDING]>,
}
- [L77]impl Default for CpuPool
  - [L78:82]fn default() -> Self 
- [L85]impl CpuPool
  - [L86:98]fn intern(&mut self, list: List<[u32; CONSUMER_CPUS_FULL]>) -> CpuListIdx 
  - [L100:102]fn get(&self, idx: CpuListIdx) -> &[u32] 
- [L106:110]struct PrimePrefix {
    pub prefix: StrIdx,
    pub cpus: Option<CpuListIdx>,
    pub thread_priority: ThreadPriority,
}
- [L113:116]struct IdealProcessorRule {
    pub cpus: CpuListIdx,
    pub prefixes: Vec<StrIdx>,
}
- [L119:127]struct ProcessLevelConfig {
    pub name: StrIdx,
    pub priority: ProcessPriority,
    pub affinity_cpus: CpuListIdx,
    pub cpu_set_cpus: CpuListIdx,
    pub cpu_set_reset_ideal: bool,
    pub io_priority: IOPriority,
    pub memory_priority: MemoryPriority,
}
- [L130:136]struct ThreadLevelConfig {
    pub name: StrIdx,
    pub prime_threads_cpus: CpuListIdx,
    pub prime_threads_prefixes: List<[PrimePrefix; LIMITED]>,
    pub track_top_x_threads: i32,
    pub ideal_processor_rules: Vec<IdealProcessorRule>,
}
- [L139:143]struct ConfigConstants {
    pub min_active_streak: u8,
    pub keep_threshold: f64,
    pub entry_threshold: f64,
}
- [L145]impl Default for ConfigConstants
  - [L146:152]fn default() -> Self 
- [L155:203]fn parse_cpu_spec(s: &str) -> List<[u32; CONSUMER_CPUS_FULL]> 
- [L205:207]fn mask_to_cpu_indices(mask: u64) -> List<[u32; CONSUMER_CPUS_FULL]> 
- [L209:217]fn cpu_indices_to_mask(cpus: &[u32]) -> usize 
- [L219:249]fn format_cpu_indices(cpus: &[u32]) -> String 
- [L252:267]struct ConfigResult {
    pub string_pool: StringPool,
    pub cpu_pool: CpuPool,
    pub process_level_configs: HashMap<u32, HashMap<StrIdx, ProcessLevelConfig>>,
    pub thread_level_configs: HashMap<u32, HashMap<StrIdx, ThreadLevelConfig>>,
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
- [L269]impl ConfigResult
  - [L270:272]fn is_valid(&self) -> bool 
  - [L274:278]fn total_rules(&self) -> usize 
  - [L280:284]fn str(&self, idx: StrIdx) -> &str 
  - [L286:290]fn cpus(&self, idx: CpuListIdx) -> &[u32] 
  - [L292:296]fn has_any_config(&self, name: &str) -> bool 
  - [L298:327]fn print_report(&self) 
- [L330:353]fn resolve_cpu_spec(
    spec: &str,
    field_name: &str,
    line_number: usize,
    cpu_aliases: &HashMap<String, CpuListIdx>,
    cpu_pool: &mut CpuPool,
    errors: &mut Vec<String>,
) -> CpuListIdx 
- [L354:360]fn collect_members(text: &str, members: &mut Vec<String>) 
- [L363:403]fn parse_constant(name: &str, value: &str, line_number: usize, result: &mut ConfigResult) 
- [L405:420]fn parse_alias(name: &str, value: &str, line_number: usize, cpu_aliases: &mut HashMap<String, CpuListIdx>, result: &mut ConfigResult) 
- [L422:485]fn parse_ideal_processor_spec(
    spec: &str,
    line_number: usize,
    cpu_aliases: &HashMap<String, CpuListIdx>,
    string_pool: &mut StringPool,
    errors: &mut Vec<String>,
) -> Vec<IdealProcessorRule> 
- [L487:497]fn collect_group_block(lines: &[String], start_index: usize, first_line_content: &str) -> Option<(Vec<String>, Option<String>, usize)> 
- [L520:869]fn parse_and_insert_rules(
    members: &[String],
    rule_parts: &[&str],
    line_number: usize,
    cpu_aliases: &HashMap<String, CpuListIdx>,
    result: &mut ConfigResult,
) 
- [L871:957]fn read_config<P: AsRef<Path>>(path: P) -> ConfigResult 
- [L1005:1014]fn read_list<P: AsRef<Path>>(path: P) -> Result<Vec<String>> 
- [L1016:1020]fn read_utf16le_file(path: &str) -> Result<String> 
- [L1023:1026]fn parse_mask(s: &str) -> usize 
- [L1028:1191]fn convert(in_file: Option<String>, out_file: Option<String>) 
- [L1193:1405]fn sort_and_group_config(in_file: Option<String>, out_file: Option<String>) 
- [L1407:1429]fn hotreload_blacklist(cli: &CliArgs, blacklist: &mut Vec<String>, last_blacklist_mod_time: &mut Option<std::time::SystemTime>) 
- [L1431:1462]fn hotreload_config(
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
- [L87]impl EtwProcessMonitor
  - [L88:208]fn start() -> Result<(Self, Receiver<EtwProcessEvent>), String> 
  - [L210:241]fn stop(&mut self) 
  - [L243:258]fn stop_existing_session(wide_name: &[u16]) 
- [L261]impl Drop for EtwProcessMonitor
  - [L262:264]fn drop(&mut self) 

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
- [L55:83]fn apply_process_level<'a>(
    pid: u32,
    config: &ProcessLevelConfig,
    configs: &ConfigResult,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    dry_run: bool,
    apply_configs: &mut ApplyConfigResult,
) 
- [L85:129]fn apply_thread_level<'a>(
    pid: u32,
    config: &ThreadLevelConfig,
    configs: &ConfigResult,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    process: &'a ProcessEntry,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    dry_run: bool,
    apply_configs: &mut ApplyConfigResult,
) 
- [L131:167]fn apply_config(
    cli: &CliArgs,
    configs: &ConfigResult,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    process_level_applied: &mut smallvec::SmallVec<[u32; PIDS]>,
    thread_level_applied: &mut smallvec::SmallVec<[u32; PENDING]>,
    grade: &u32,
    pid: &u32,
    name_idx: &StrIdx,
    process_level_config: &ProcessLevelConfig,
    process: &ProcessEntry,
) 
- [L169:183]fn log_apply_results(pid: &u32, name: &str, result: ApplyConfigResult) 
- [L185:268]fn process_logs(configs: &ConfigResult, blacklist: &[String], logs_path: Option<&str>, output_file: Option<&str>) 
- [L270:300]fn process_find(cli: &CliArgs, configs: &ConfigResult, blacklist: &[String]) -> Result<(), windows::core::Error> 
- [L302:632]fn main() -> windows::core::Result<()> 

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
- [L18]impl ProcessPriority
  - [L29:35]fn as_str(&self) -> &'static str 
  - [L37:39]fn as_win_const(&self) -> Option<PROCESS_CREATION_FLAGS> 
  - [L41:48]fn from_str(s: &str) -> Self 
  - [L50:56]fn from_win_const(val: u32) -> &'static str 
- [L60:66]enum IOPriority {
    None,
    VeryLow,
    Low,
    Normal,
    High,
}
- [L68]impl IOPriority
  - [L77:83]fn as_str(&self) -> &'static str 
  - [L85:87]fn as_win_const(&self) -> Option<u32> 
  - [L89:96]fn from_str(s: &str) -> Self 
  - [L98:104]fn from_win_const(val: u32) -> &'static str 
- [L109:109]struct MemoryPriorityInformation(pub u32);
- [L112:119]enum MemoryPriority {
    None,
    VeryLow,
    Low,
    Medium,
    BelowNormal,
    Normal,
}
- [L121]impl MemoryPriority
  - [L131:137]fn as_str(&self) -> &'static str 
  - [L139:141]fn as_win_const(&self) -> Option<MEMORY_PRIORITY> 
  - [L143:150]fn from_str(s: &str) -> Self 
  - [L152:158]fn from_win_const(val: u32) -> &'static str 
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
- [L176]impl ThreadPriority
  - [L191:197]fn as_str(&self) -> &'static str 
  - [L199:201]fn as_win_const(&self) -> Option<i32> 
  - [L203:210]fn from_str(s: &str) -> Self 
  - [L212:218]fn from_win_const(val: i32) -> Self 
  - [L220:235]fn boost_one(&self) -> Self 
  - [L237:239]fn to_thread_priority_struct(self) -> THREAD_PRIORITY 

## src/process.rs
- [L7:7]static SNAPSHOT_BUFFER: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| Mutex::new(vec![0u8; 32]));
- [L9:9]static PID_TO_PROCESS_MAP: Lazy<Mutex<HashMap<u32, ProcessEntry>>> = Lazy::new(|| Mutex::new(HashMap::default()));
- [L6:14]struct ProcessSnapshot<'a> {
    buffer: &'a mut Vec<u8>,
    pub pid_to_process: &'a mut HashMap<u32, ProcessEntry>,
}
- [L16]impl<'a> Drop for ProcessSnapshot<'a>
  - [L17:20]fn drop(&mut self) 
- [L23]impl<'a> ProcessSnapshot<'a>
  - [L24:72]fn take(buffer: &'a mut Vec<u8>, pid_to_process: &'a mut HashMap<u32, ProcessEntry>) -> Result<Self, i32> 
- [L76:80]struct ProcessEntry {
    pub process: SYSTEM_PROCESS_INFORMATION,
    threads_base_ptr: usize,
    name: String,
}
- [L87]impl ProcessEntry
  - [L88:103]fn new(process: SYSTEM_PROCESS_INFORMATION, threads_base_ptr: *const SYSTEM_THREAD_INFORMATION) -> Self 
  - [L105:122]fn get_threads(&self) -> HashMap<u32, SYSTEM_THREAD_INFORMATION> 
  - [L125:127]fn get_name(&self) -> &str 
  - [L131:141]fn get_name_original_case(&self) -> String 
  - [L144:146]fn pid(&self) -> u32 
  - [L149:151]fn thread_count(&self) -> u32 

## src/scheduler.rs
- [L15:18]struct PrimeThreadScheduler {
    pub pid_to_process_stats: HashMap<u32, ProcessStats>,
    pub constants: ConfigConstants,
}
- [L20]impl PrimeThreadScheduler
  - [L21:26]fn new(constants: ConfigConstants) -> Self 
  - [L28:30]fn reset_alive(&mut self) 
  - [L32:34]fn set_alive(&mut self, pid: u32) 
  - [L36:40]fn set_tracking_info(&mut self, pid: u32, track_top_x_threads: i32, process_name: String) 
  - [L43:50]fn get_thread_stats(&mut self, pid: u32, tid: u32) -> &mut ThreadStats 
  - [L52:81]fn update_active_streaks(&mut self, pid: u32, tid_with_delta_cycles: &[(u32, u64)]) 
  - [L83:130]fn select_top_threads_with_hysteresis(
        &mut self,
        pid: u32,
        tid_with_delta_cycles: &mut [(u32, u64, bool)],
        slot_count: usize,
        is_currently_assigned: fn(&ThreadStats) -> bool,
    ) 
  - [L132:182]fn drop_process_by_pid(&mut self, pid: &u32) 
- [L186:193]struct ProcessStats {
    pub alive: bool,
    pub tid_to_thread_stats: HashMap<u32, ThreadStats>,
    pub track_top_x_threads: i32,
    pub process_name: String,
    pub process_id: u32,
}
- [L195]impl ProcessStats
  - [L196:204]fn new(process_id: u32) -> Self 
- [L207]impl Default for ProcessStats
  - [L208:210]fn default() -> Self 
- [L214:224]struct IdealProcessorState {
    pub current_group: u16,
    pub current_number: u8,
    pub previous_group: u16,
    pub previous_number: u8,
    pub is_assigned: bool,
}
- [L226]impl IdealProcessorState
  - [L227:235]fn new() -> Self 
- [L238]impl Default for IdealProcessorState
  - [L239:241]fn default() -> Self 
- [L244:269]struct ThreadStats {
    pub last_total_time: i64,
    pub cached_total_time: i64,
    pub last_cycles: u64,
    pub cached_cycles: u64,
    pub handle: Option<ThreadHandle>,
    pub pinned_cpu_set_ids: List<[u32; CONSUMER_CPUS_FULL]>,
    pub active_streak: u8,
    pub start_address: usize,
    pub original_priority: Option<ThreadPriority>,
    pub last_system_thread_info: Option<SYSTEM_THREAD_INFORMATION>,
    pub ideal_processor: IdealProcessorState,
    pub process_id: u32,
}
- [L271]impl fmt::Debug for ThreadStats
  - [L272:284]fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
- [L287]impl ThreadStats
  - [L288:303]fn new(process_id: u32) -> Self 
- [L306]impl Default for ThreadStats
  - [L307:309]fn default() -> Self 
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
- [L79]impl Drop for ProcessHandle
  - [L80:95]fn drop(&mut self) 
- [L98:197]fn get_process_handle(pid: u32, process_name: &str) -> Option<ProcessHandle> 
- [L199:206]struct ThreadHandle {
    pub r_limited_handle: HANDLE,
    pub r_handle: HANDLE,
    pub w_limited_handle: HANDLE,
    pub w_handle: HANDLE,
}
- [L208]impl Drop for ThreadHandle
  - [L209:228]fn drop(&mut self) 
- [L231:273]fn get_thread_handle(tid: u32, pid: u32, process_name: &str) -> Option<ThreadHandle> 
- [L275:303]fn try_open_thread(pid: u32, tid: u32, process_name: &str, access: THREAD_ACCESS_RIGHTS, internal_op_code: u32) -> HANDLE 
- [L314:314]static CPU_SET_INFORMATION: Lazy<Mutex<Vec<CpuSetData>>> = Lazy::new(|| {
- [L355:357]fn get_cpu_set_information() -> &'static Mutex<Vec<CpuSetData>> 
- [L359:376]fn cpusetids_from_indices(cpu_indices: &[u32]) -> List<[u32; CONSUMER_CPUS_FULL]> 
- [L379:392]fn cpusetids_from_mask(mask: usize) -> List<[u32; CONSUMER_CPUS_FULL]> 
- [L394:410]fn indices_from_cpusetids(cpuids: &[u32]) -> List<[u32; CONSUMER_CPUS_FULL]> 
- [L413:428]fn mask_from_cpusetids(cpuids: &[u32]) -> usize 
- [L430:437]fn filter_indices_by_mask(cpu_indices: &[u32], affinity_mask: usize) -> List<[u32; CONSUMER_CPUS_FULL]> 
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

