# Src Code Structure Outline, auto-generated during cargo build/check

## src/apply.rs
- [L25:L28]struct ApplyConfigResult {
    pub changes: Vec<String>,
    pub errors: Vec<String>,
}
- [L48:L73]fn apply_priority(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult) 
- [L75:L117]fn apply_affinity(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    h_prc: HANDLE,
    current_mask: &mut usize,
    apply_config_result: &mut ApplyConfigResult,
    processes: &mut ProcessSnapshot,
) 
- [L119:L175]fn apply_process_default_cpuset(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult) 
- [L177:L275]fn prefetch_all_thread_cycles(
    pid: u32,
    process_name: &str,
    processes: &mut ProcessSnapshot,
    prime_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L277:L290]fn update_thread_stats(pid: u32, prime_scheduler: &mut PrimeThreadScheduler) 
- [L292:L376]fn apply_prime_threads(
    pid: u32,
    config: &ProcessConfig,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    processes: &mut ProcessSnapshot,
    dry_run: bool,
    current_mask: &mut usize,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L378:L385]fn apply_prime_threads_select(tid_with_delta_cycles: &mut [(u32, u64, bool)], prime_core_scheduler: &mut PrimeThreadScheduler, pid: u32, prime_count: usize) 
- [L387:L489]fn apply_prime_threads_promote(
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    pid: u32,
    config: &ProcessConfig,
    current_mask: &mut usize,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L491:L548]fn apply_prime_threads_demote(
    process: &mut crate::process::ProcessEntry,
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    pid: u32,
    config: &ProcessConfig,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L550:L608]fn apply_io_priority(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult) 
- [L610:L667]fn apply_memory_priority(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult) 
- [L669:L750]fn reset_thread_ideal_processors(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    _h_prc: HANDLE,
    apply_config_result: &mut ApplyConfigResult,
    processes: &mut ProcessSnapshot,
) 
- [L752:L936]fn apply_ideal_processors(
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
- [L213:L224]fn resolve_cpu_spec(spec: &str, field_name: &str, line_number: usize, cpu_aliases: &HashMap<String, Vec<u32>>, errors: &mut Vec<String>) -> Vec<u32> 
- [L226:L232]fn collect_members(text: &str, members: &mut Vec<String>) 
- [L235:L270]fn parse_constant(name: &str, value: &str, line_number: usize, result: &mut ConfigResult) 
- [L272:L285]fn parse_alias(name: &str, value: &str, line_number: usize, cpu_aliases: &mut HashMap<String, Vec<u32>>, result: &mut ConfigResult) 
- [L287:L341]fn parse_ideal_processor_spec(spec: &str, line_number: usize, cpu_aliases: &HashMap<String, Vec<u32>>, errors: &mut Vec<String>) -> Vec<IdealProcessorRule> 
- [L343:L353]fn collect_group_block(lines: &[String], start_index: usize, first_line_content: &str) -> Option<(Vec<String>, Option<String>, usize)> 
- [L377:L639]fn parse_and_insert_rules(members: &[String], rule_parts: &[&str], line_number: usize, cpu_aliases: &HashMap<String, Vec<u32>>, result: &mut ConfigResult) 
- [L641:L716]fn read_config<P: AsRef<Path>>(path: P) -> ConfigResult 
- [L761:L770]fn read_list<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> 
- [L772:L776]fn read_utf16le_file(path: &str) -> io::Result<String> 
- [L779:L782]fn parse_mask(s: &str) -> usize 
- [L784:L947]fn convert(in_file: Option<String>, out_file: Option<String>) 
- [L949:L1157]fn sort_and_group_config(in_file: Option<String>, out_file: Option<String>) 

## src/logging.rs
- [L11:L11]static LOCALTIME_BUFFER: Lazy<Mutex<DateTime<Local>>> = Lazy::new(|| Mutex::new(Local::now()));
- [L13:L13]static FINDS_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));
- [L15:L15]static FAIL_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));
- [L17:L20]struct ApplyFailEntry {
    proc_name: String,
    alive: bool,
}
- [L22:L22]static APPLY_FAIL_MAP: Lazy<Mutex<HashMap<u32, ApplyFailEntry>>> = Lazy::new(|| Mutex::new(HashMap::new()));
- [L24:L44]fn apply_fail_insert_if_new(pid: u32, proc_name: &str) -> bool 
- [L46:L63]fn purge_apply_fail_map(pids_and_names: &[(u32, String)]) 
- [L65:L65]static USE_CONSOLE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));
- [L67:L67]static DUST_BIN_MODE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));
- [L69:L69]static LOG_FILE: Lazy<Mutex<File>> = Lazy::new(|| Mutex::new(OpenOptions::new().append(true).create(true).open(get_log_path("")).unwrap()));
- [L71:L71]static FIND_LOG_FILE: Lazy<Mutex<File>> = Lazy::new(|| Mutex::new(OpenOptions::new().append(true).create(true).open(get_log_path(".find")).unwrap()));
- [L73:L75]fn use_console() -> &'static Mutex<bool> 
- [L77:L79]fn logger() -> &'static Mutex<File> 
- [L81:L83]fn find_logger() -> &'static Mutex<File> 
- [L85:L94]fn get_log_path(suffix: &str) -> PathBuf 
- [L103:L113]fn log_message(args: &str) 
- [L115:L121]fn log_pure_message(args: &str) 
- [L123:L130]fn log_to_find(msg: &str) 
- [L132:L140]fn log_process_find(process_name: &str) 
- [L142:L162]fn error_from_code(code: u32) -> String 

## src/main.rs
- [L46:L100]fn apply_config(pid: u32, config: &ProcessConfig, prime_core_scheduler: &mut PrimeThreadScheduler, processes: &mut ProcessSnapshot, dry_run: bool) -> ApplyConfigResult 
- [L102:L172]fn process_logs(configs: &HashMap<u32, HashMap<String, ProcessConfig>>, blacklist: &[String], logs_path: Option<&str>, output_file: Option<&str>) 
- [L174:L425]fn main() -> windows::core::Result<()> 

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
- [L52:L58]enum IOPriority {
    None,
    VeryLow,
    Low,
    Normal,
    High,
}
- [L93:L93]struct MemoryPriorityInformation(pub u32);
- [L96:L103]enum MemoryPriority {
    None,
    VeryLow,
    Low,
    Medium,
    BelowNormal,
    Normal,
}
- [L138:L150]enum ThreadPriority {
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
- [L66:L71]struct ProcessEntry {
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
- [L183:L190]struct ProcessStats {
    pub alive: bool,
    pub tid_to_thread_stats: HashMap<u32, ThreadStats>,
    pub track_top_x_threads: i32,
    pub process_name: String,
    pub process_id: u32,
}
- [L211:L221]struct IdealProcessorState {
    pub current_group: u16,
    pub current_number: u8,
    pub previous_group: u16,
    pub previous_number: u8,
    pub is_assigned: bool,
}
- [L241:L263]struct ThreadStats {
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
- [L306:L310]fn format_100ns(time: i64) -> String 
- [L312:L319]fn format_filetime(time: i64) -> String 

## src/winapi.rs
- [L49:L52]struct CpuSetData {
    id: u32,
    logical_processor_index: u8,
}
- [L63:L63]static CPU_SET_INFORMATION: Lazy<Mutex<Vec<CpuSetData>>> = Lazy::new(|| {
- [L104:L106]fn get_cpu_set_information() -> &'static Mutex<Vec<CpuSetData>> 
- [L108:L125]fn cpusetids_from_indices(cpu_indices: &[u32]) -> Vec<u32> 
- [L128:L141]fn cpusetids_from_mask(mask: usize) -> Vec<u32> 
- [L143:L159]fn indices_from_cpusetids(cpuids: &[u32]) -> Vec<u32> 
- [L162:L177]fn mask_from_cpusetids(cpuids: &[u32]) -> usize 
- [L179:L189]fn filter_indices_by_mask(cpu_indices: &[u32], affinity_mask: usize) -> Vec<u32> 
- [L191:L220]fn is_running_as_admin() -> bool 
- [L222:L255]fn request_uac_elevation(console: bool) -> io::Result<()> 
- [L257:L295]fn enable_debug_privilege() 
- [L297:L335]fn enable_inc_base_priority_privilege() 
- [L337:L379]fn is_affinity_unset(pid: u32, process_name: &str) -> bool 
- [L381:L400]fn get_thread_start_address(thread_handle: HANDLE) -> usize 
- [L402:L413]fn set_thread_ideal_processor_ex(thread_handle: HANDLE, group: u16, number: u8) -> Result<PROCESSOR_NUMBER, windows::core::Error> 
- [L415:L421]fn get_thread_ideal_processor_ex(thread_handle: HANDLE) -> Result<PROCESSOR_NUMBER, windows::core::Error> 
- [L424:L424]static MODULE_CACHE: Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>> = Lazy::new(|| Mutex::new(HashMap::new()));
- [L426:L452]fn resolve_address_to_module(pid: u32, address: usize) -> String 
- [L454:L457]fn drop_module_cache(pid: u32) 
- [L459:L505]fn terminate_child_processes() 
- [L507:L509]fn clear_module_cache(pid: u32) 
- [L511:L564]fn enumerate_process_modules(pid: u32) -> Vec<(usize, usize, String)>
