# Src Code Structure Outline, auto-generated during cargo build/check, READ this by MULTIPLE calls if it's **too large to fit in one**

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
