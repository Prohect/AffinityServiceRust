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
- [L43:L129]fn parse_args(args: &[String], cli: &mut CliArgs) -> windows::core::Result<()> 
- [L131:L160]fn print_help() 
- [L162:L291]fn get_config_help_lines() -> Vec<&'static str> 
- [L293:L298]fn print_config_help() 
- [L300:L345]fn print_cli_help() 
- [L347:L353]fn print_help_all() 

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
- [L357:L443]fn parse_ideal_processor_spec(
    spec: &str,
    line_number: usize,
    cpu_aliases: &HashMap<String, Vec<u32>>,
    errors: &mut Vec<String>,
) -> Vec<IdealProcessorRule> 
- [L445:L469]fn collect_group_block(lines: &[String], start_index: usize, first_line_content: &str) -> Option<(Vec<String>, Option<String>, usize)> 
- [L481:L779]fn parse_and_insert_rules(members: &[String], rule_parts: &[&str], line_number: usize, cpu_aliases: &HashMap<String, Vec<u32>>, result: &mut ConfigResult) 
- [L781:L868]fn read_config<P: AsRef<Path>>(path: P) -> ConfigResult 
- [L914:L924]fn read_list<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> 
- [L926:L931]fn read_utf16le_file(path: &str) -> io::Result<String> 
- [L933:L939]fn parse_mask(s: &str) -> usize 
- [L941:L1117]fn convert(in_file: Option<String>, out_file: Option<String>) 

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
- [L192:L276]fn apply_prime_threads(
    pid: u32,
    config: &ProcessConfig,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    processes: &mut Option<&mut ProcessSnapshot>,
    dry_run: bool,
    current_mask: &mut usize,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L278:L293]fn apply_prime_threads_select_candidates(process: &mut process::ProcessEntry, candidate_tids: &mut [u32], prime_core_scheduler: &mut PrimeThreadScheduler, pid: u32) 
- [L295:L365]fn apply_prime_threads_query_cycles(
    candidate_tids: &[u32],
    tid_with_delta_cycles: &mut [(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    pid: u32,
    process_name: &str,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L367:L407]fn apply_prime_threads_update_streaks(tid_with_delta_cycles: &mut [(u32, u64, bool)], prime_core_scheduler: &mut PrimeThreadScheduler, pid: u32, prime_count: usize) 
- [L409:L508]fn apply_prime_threads_promote(
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    pid: u32,
    config: &ProcessConfig,
    current_mask: &mut usize,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L510:L556]fn apply_prime_threads_demote(
    process: &mut process::ProcessEntry,
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    pid: u32,
    config: &ProcessConfig,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L558:L616]fn apply_io_priority(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult) 
- [L618:L676]fn apply_memory_priority(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult) 
- [L678:L902]fn apply_ideal_processors(
    pid: u32,
    config: &ProcessConfig,
    processes: &mut Option<&mut ProcessSnapshot>,
    prime_scheduler: &mut PrimeThreadScheduler,
    dry_run: bool,
    apply_config_result: &mut ApplyConfigResult,
) 
- [L904:L956]fn apply_config(
    pid: u32,
    config: &ProcessConfig,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    mut processes: Option<&mut ProcessSnapshot>,
    dry_run: bool,
) -> ApplyConfigResult 
- [L958:L1030]fn process_logs(configs: &HashMap<u32, HashMap<String, ProcessConfig>>, blacklist: &Vec<String>, logs_path: Option<&str>, output_file: Option<&str>) 
- [L1032:L1275]fn main() -> windows::core::Result<()> 

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
- [L130:L137]struct ProcessStats {
    pub alive: bool,
    pub tid_to_thread_stats: HashMap<u32, ThreadStats>,
    pub track_top_x_threads: i32,
    pub process_name: String,
}
- [L156:L169]struct IdealProcessorState {
    pub current_group: u16,
    pub current_number: u8,
    pub previous_group: u16,
    pub previous_number: u8,
    pub is_assigned: bool,
}
- [L189:L208]struct ThreadStats {
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
- [L231:L235]fn format_100ns(time: i64) -> String 
- [L237:L244]fn format_filetime(time: i64) -> String 

## src/winapi.rs
- [L55:L60]struct CpuSetData {
    id: u32,
    logical_processor_index: u8,
}
- [L76:L76]static CPU_SET_INFORMATION: Lazy<Mutex<Vec<CpuSetData>>> = Lazy::new(|| {
- [L122:L125]fn get_cpu_set_information() -> &'static Mutex<Vec<CpuSetData>> 
- [L127:L150]fn cpusetids_from_indices(cpu_indices: &[u32]) -> Vec<u32> 
- [L152:L176]fn cpusetids_from_mask(mask: usize) -> Vec<u32> 
- [L178:L198]fn indices_from_cpusetids(cpuids: &[u32]) -> Vec<u32> 
- [L200:L224]fn mask_from_cpusetids(cpuids: &[u32]) -> usize 
- [L226:L242]fn filter_indices_by_mask(cpu_indices: &[u32], affinity_mask: usize) -> Vec<u32> 
- [L244:L278]fn is_running_as_admin() -> bool 
- [L280:L304]fn request_uac_elevation() -> io::Result<()> 
- [L306:L350]fn enable_debug_privilege() 
- [L352:L396]fn enable_inc_base_priority_privilege() 
- [L398:L444]fn is_affinity_unset(pid: u32, process_name: &str) -> bool 
- [L446:L464]fn get_thread_start_address(thread_handle: HANDLE) -> usize 
- [L466:L492]fn set_thread_ideal_processor_ex(
    thread_handle: HANDLE,
    group: u16,
    number: u8,
) -> Result<PROCESSOR_NUMBER, windows::core::Error> 
- [L494:L510]fn get_thread_ideal_processor_ex(
    thread_handle: HANDLE,
) -> Result<PROCESSOR_NUMBER, windows::core::Error> 
- [L514:L514]static MODULE_CACHE: Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>> = Lazy::new(|| Mutex::new(HashMap::new()));
- [L512:L553]fn resolve_address_to_module(pid: u32, address: usize) -> String 
- [L555:L558]fn clear_module_cache(pid: u32) 
- [L560:L622]fn enumerate_process_modules(pid: u32) -> Vec<(usize, usize, String)> 

