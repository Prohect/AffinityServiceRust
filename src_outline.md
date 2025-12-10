# AffinityServiceRust Src Code Structure Outline

## cli.rs
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
- [L43:L127]fn parse_args(args: &[String], cli: &mut CliArgs) -> windows::core::Result<()>
- [L129:L158]fn print_help()
- [L160:L267]fn get_config_help_lines() -> Vec<&'static str>
- [L269:L274]fn print_config_help()
- [L276:L321]fn print_cli_help()
- [L323:L329]fn print_help_all()

## config.rs
- [L30:L37]struct PrimePrefix {
    pub prefix: String,
    pub cpus: Option<Vec<u32>>,
}
- [L39:L56]struct ProcessConfig {
    pub name: String,
    pub priority: ProcessPriority,
    pub affinity_cpus: Vec<u32>,
    pub cpu_set_cpus: Vec<u32>,
    pub prime_threads_cpus: Vec<u32>,
    pub prime_threads_prefixes: Vec<PrimePrefix>,
    pub io_priority: IOPriority,
    pub memory_priority: MemoryPriority,
}
- [L58:L70]struct ConfigConstants {
    pub min_active_streak: u8,
    pub keep_threshold: f64,
    pub entry_threshold: f64,
}
- [L82:L140]fn parse_cpu_spec(s: &str) -> Vec<u32>
- [L142:L144]fn mask_to_cpu_indices(mask: u64) -> Vec<u32>
- [L146:L156]fn cpu_indices_to_mask(cpus: &[u32]) -> usize
- [L158:L191]fn format_cpu_indices(cpus: &[u32]) -> String
- [L193:L211]struct ConfigResult {
    pub configs: HashMap<String, ProcessConfig>,
    pub constants: ConfigConstants,
    pub constants_count: usize,
    pub aliases_count: usize,
    pub groups_count: usize,
    pub group_members_count: usize,
    pub process_rules_count: usize,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}
- [L241:L253]fn resolve_cpu_spec(spec: &str, field_name: &str, line_number: usize, cpu_aliases: &HashMap<String, Vec<u32>>, errors: &mut Vec<String>) -> Vec<u32>
- [L255:L262]fn collect_members(text: &str, members: &mut Vec<String>)
- [L265:L301]fn parse_constant(name: &str, value: &str, line_number: usize, result: &mut ConfigResult)
- [L303:L317]fn parse_alias(name: &str, value: &str, line_number: usize, cpu_aliases: &mut HashMap<String, Vec<u32>>, result: &mut ConfigResult)
- [L319:L343]fn collect_group_block(lines: &[String], start_index: usize, first_line_content: &str) -> Option<(Vec<String>, Option<String>, usize)>
- [L355:L506]fn parse_and_insert_rules(members: &[String], rule_parts: &[&str], line_number: usize, cpu_aliases: &HashMap<String, Vec<u32>>, result: &mut ConfigResult)
- [L508:L595]fn read_config<P: AsRef<Path>>(path: P) -> ConfigResult
- [L641:L651]fn read_list<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>>
- [L653:L658]fn read_utf16le_file(path: &str) -> io::Result<String>
- [L660:L666]fn parse_mask(s: &str) -> usize
- [L668:L844]fn convert(in_file: Option<String>, out_file: Option<String>)

## logging.rs
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

## main.rs
- [L45:L48]struct ApplyConfigResult {
    changes: Vec<String>,
    errors: Vec<String>,
}
- [L69:L94]fn apply_priority(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult)
- [L96:L131]fn apply_affinity(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, current_mask: &mut usize, apply_config_result: &mut ApplyConfigResult)
- [L133:L190]fn apply_process_default_cpuset(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult)
- [L192:L227]fn apply_prime_threads(
    pid: u32,
    config: &ProcessConfig,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    mut processes: Option<&mut ProcessSnapshot>,
    dry_run: bool,
    current_mask: &mut usize,
    apply_config_result: &mut ApplyConfigResult,
)
- [L229:L244]fn apply_prime_threads_select_candidates(process: &mut process::ProcessEntry, candidate_tids: &mut [u32], prime_core_scheduler: &mut PrimeThreadScheduler, pid: u32)
- [L246:L316]fn apply_prime_threads_query_cycles(
    candidate_tids: &[u32],
    tid_with_delta_cycles: &mut [(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    pid: u32,
    process_name: &str,
    apply_config_result: &mut ApplyConfigResult,
)
- [L318:L358]fn apply_prime_threads_update_streaks(tid_with_delta_cycles: &mut [(u32, u64, bool)], prime_core_scheduler: &mut PrimeThreadScheduler, pid: u32, prime_count: usize)
- [L360:L446]fn apply_prime_threads_promote(
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    pid: u32,
    config: &ProcessConfig,
    current_mask: &mut usize,
    apply_config_result: &mut ApplyConfigResult,
)
- [L448:L494]fn apply_prime_threads_demote(
    process: &mut process::ProcessEntry,
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    pid: u32,
    config: &ProcessConfig,
    apply_config_result: &mut ApplyConfigResult,
)
- [L496:L554]fn apply_io_priority(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult)
- [L556:L614]fn apply_memory_priority(pid: u32, config: &ProcessConfig, dry_run: bool, h_prc: HANDLE, apply_config_result: &mut ApplyConfigResult)
- [L616:L659]fn apply_config(
    pid: u32,
    config: &ProcessConfig,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    processes: Option<&mut ProcessSnapshot>,
    dry_run: bool,
) -> ApplyConfigResult
- [L661:L731]fn process_logs(configs: &HashMap<String, ProcessConfig>, blacklist: &Vec<String>, logs_path: Option<&str>, output_file: Option<&str>)
- [L733:L915]fn main() -> windows::core::Result<()>

## priority.rs
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
    Unknown(i32),
}

## process.rs
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

## scheduler.rs
- [L10:L29]struct PrimeThreadScheduler {
    pid_to_process_stats: HashMap<u32, ProcessStats>,
    pub constants: ConfigConstants,
}
- [L80:L85]struct ProcessStats {
    pub alive: bool,
    pub tid_to_thread_stats: HashMap<u32, ThreadStats>,
}
- [L102:L118]struct ThreadStats {
    pub last_total_time: i64,
    pub last_cycles: u64,
    pub handle: Option<HANDLE>,
    pub cpu_set_ids: Vec<u32>,
    pub active_streak: u8,
    pub start_address: usize,
    pub original_priority: Option<ThreadPriority>,
}

## winapi.rs
- [L54:L59]struct CpuSetData {
    id: u32,
    logical_processor_index: u8,
}
- [L75:L75]static CPU_SET_INFORMATION: Lazy<Mutex<Vec<CpuSetData>>> = Lazy::new(|| {
- [L121:L124]fn get_cpu_set_information() -> &'static Mutex<Vec<CpuSetData>>
- [L126:L149]fn cpusetids_from_indices(cpu_indices: &[u32]) -> Vec<u32>
- [L151:L175]fn cpusetids_from_mask(mask: usize) -> Vec<u32>
- [L177:L197]fn indices_from_cpusetids(cpuids: &[u32]) -> Vec<u32>
- [L199:L223]fn mask_from_cpusetids(cpuids: &[u32]) -> usize
- [L225:L241]fn filter_indices_by_mask(cpu_indices: &[u32], affinity_mask: usize) -> Vec<u32>
- [L243:L277]fn is_running_as_admin() -> bool
- [L279:L303]fn request_uac_elevation() -> io::Result<()>
- [L305:L349]fn enable_debug_privilege()
- [L351:L395]fn enable_inc_base_priority_privilege()
- [L397:L443]fn is_affinity_unset(pid: u32, process_name: &str) -> bool
- [L445:L463]fn get_thread_start_address(thread_handle: HANDLE) -> usize
- [L467:L467]static MODULE_CACHE: Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>> = Lazy::new(|| Mutex::new(HashMap::new()));
- [L465:L516]fn resolve_address_to_module(pid: u32, address: usize) -> String
- [L518:L521]fn clear_module_cache(pid: u32)
- [L523:L585]fn enumerate_process_modules(pid: u32) -> Vec<(usize, usize, String)>

