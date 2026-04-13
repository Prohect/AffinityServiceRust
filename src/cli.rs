use crate::{get_use_console, log};
use windows::core::Result;

#[derive(Debug, Default)]
pub struct CliArgs {
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
}

impl CliArgs {
    pub fn new() -> Self {
        Self {
            interval_ms: 5000,
            config_file_name: "config.ini".to_string(),
            ..Default::default()
        }
    }
}

pub fn parse_args(args: &[String], cli: &mut CliArgs) -> Result<()> {
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-help" | "--help" | "-?" | "/?" | "?" => {
                cli.help_mode = true;
            }
            "-helpall" | "--helpall" => {
                cli.help_all_mode = true;
            }
            "-console" => {
                *get_use_console!() = true;
            }
            "-noUAC" | "-nouac" => {
                cli.no_uac = true;
            }
            "-convert" => {
                cli.convert_mode = true;
            }
            "-autogroup" => {
                cli.autogroup_mode = true;
            }
            "-find" => {
                cli.find_mode = true;
            }
            "-validate" => {
                cli.validate_mode = true;
                *get_use_console!() = true;
            }
            "-processlogs" => {
                cli.process_logs_mode = true;
            }
            "-dryrun" | "-dry-run" | "--dry-run" => {
                cli.dry_run = true;
            }
            "-interval" if i + 1 < args.len() => {
                cli.interval_ms = args[i + 1].parse().unwrap_or(5000).max(16);
                i += 1;
            }
            "-loop" if i + 1 < args.len() => {
                cli.loop_count = Some(args[i + 1].parse().unwrap_or(1).max(1));
                i += 1;
            }
            "-resolution" if i + 1 < args.len() => {
                cli.time_resolution = args[i + 1].parse().unwrap_or(0);
                i += 1;
            }
            "-logloop" => {
                cli.log_loop = true;
            }
            "-config" if i + 1 < args.len() => {
                cli.config_file_name = args[i + 1].clone();
                i += 1;
            }
            "-blacklist" if i + 1 < args.len() => {
                cli.blacklist_file_name = Some(args[i + 1].clone());
                i += 1;
            }
            "-in" if i + 1 < args.len() => {
                cli.in_file_name = Some(args[i + 1].clone());
                i += 1;
            }
            "-out" if i + 1 < args.len() => {
                cli.out_file_name = Some(args[i + 1].clone());
                i += 1;
            }

            "-skip_log_before_elevation" => {
                cli.skip_log_before_elevation = true;
            }
            "-noDebugPriv" | "-nodebugpriv" => {
                cli.no_debug_priv = true;
            }
            "-noIncBasePriority" | "-noincbasepriority" => {
                cli.no_inc_base_priority = true;
            }
            "-no_etw" | "-noetw" => {
                cli.no_etw = true;
            }
            _ => {}
        }
        i += 1;
    }
    Ok(())
}

pub fn print_help() {
    *get_use_console!() = true;
    log!(
        r#"
    A Windows service to manage process priority, CPU affinity, IO priority, and memory priority.
    usage: AffinityServiceRust.exe [args]

    Common Options:
      -help | --help       show this help message
      -helpall             detailed options and debugging features.
      -console             output to console instead of log file
      -config <file>       config file to use (default: config.ini)
      -find                find processes with default affinity (-blacklist <file>)
      -interval <ms>       check interval in milliseconds (default: 5000)

      -noUAC               disable UAC elevation request
      -resolution <t>      time resolution 5210 -> 0.5210ms (default: 0, 0 means do not set time resolution)

    Modes:
      -validate            validate config file syntax without running
      -processlogs         process logs (from -find mode) to find new processes and search paths (-config <file> -blacklist <file> -in <logs dir> -out <file>)
      -dryrun              show what would be changed without applying
      -convert             convert Process Lasso config (-in <file> -out <file>)
      -autogroup           auto-group rules with identical settings (-in <file> -out <file>)
    "#
    );
}

pub fn print_cli_help() {
    log!(
        r#"
        A Windows service to manage process priority, CPU affinity, IO priority, and memory priority.
        usage: AffinityServiceRust.exe [args]

        === COMMAND LINE OPTIONS ===

        Basic Arguments:
          -help | --help       print basic help message
          -? | /? | ?          print basic help message
          -helpall | --helpall print this detailed help with debug options
          -console             use console as output instead of log file
          -noUAC | -nouac      disable UAC elevation request
          -config <file>       the config file u wanna use (config.ini by default)
          -find                find those whose affinity is same as system default which is all possible cores windows could use
          -blacklist <file>    the blacklist for -find
          -interval <ms>       set interval for checking again (5000 by default, minimal 16)
          -resolution <t>      time resolution 5210 -> 0.5210ms (default: 0, 0 means do not set time resolution)

        Operating Modes:
          -validate            validate config file for syntax errors and undefined aliases then exit
          -processlogs         process logs (from -find mode) to find new processes and search paths with everything (-config <file> -blacklist <file> -in <logs dir> -out <file>)
          -dryrun              simulate changes without applying (shows what would happen)
          -convert             convert process configs from -in <file>(from process lasso) to -out <file>
          -autogroup           auto-group rules with identical settings into named group blocks (-in <file> -out <file>)
          -in <file>           input file for -convert / logs directory for -processlogs (default: logs)
          -out <file>          output file for -convert / results file for -processlogs (default: new_processes_results.txt)

        Debug & Testing Options:
          -loop <count>        number of loops to run (default: infinite) - for testing
          -logloop             log a message at the start of each loop for testing
          -noDebugPriv         not request SeDebugPrivilege
          -noIncBasePriority   not request SeIncreaseBasePriorityPrivilege
          -no_etw | -noetw     not request ETW tracing

        === DEBUGGING ===

        Quick debug command (non-admin):
          AffinityServiceRust.exe -console -noUAC -logloop -loop 3 -interval 2000 -config test.ini

        Admin debug (check log file after, do NOT use -console):
          AffinityServiceRust.exe -logloop -loop 3 -interval 2000 -config test.ini
          Then check: logs/YYYYMMDD.log

        Note: When running with UAC elevation, -console output goes to a new session
        that cant be shown in currerent session. Use log files instead.
        "#
    );
}

/// Returns configuration file help template for embedding in converted configs.
///
/// For comprehensive documentation, see docs/cli.md and docs/config.md
pub fn get_config_help_lines() -> Vec<&'static str> {
    vec![
        r#"
        ## ============================================================================
        ## AffinityServiceRust Configuration File
        ## ============================================================================
        ##
        ## This config file defines CPU affinity: priority: and scheduling rules for
        ## Windows processes. Customize the aliases below to match YOUR CPU topology.
        ##
        ## Logs are stored in logs/ directory by default for -processlogs mode.
        ##
        ## ----------------------------------------------------------------------------
        ## TERMINOLOGY
        ## ----------------------------------------------------------------------------
        ##   P-core  = Performance core (Intel hybrid CPUs)
        ##   E-core  = Efficiency core (Intel hybrid CPUs)
        ##   p       = P-core with HyperThreading OFF (1 thread per core)
        ##   pp      = P-core with HyperThreading ON (2 threads per core)
        ##   e       = E-core (always 1 thread per core)
        ##
        ## Example: Intel i7-14700KF with HT off = 8p + 12e = 20 logical processors
        ##
        ## ----------------------------------------------------------------------------
        ## CONFIG FORMAT
        ## ----------------------------------------------------------------------------
        ##   process_name:priority:affinity:cpuset:prime_cpus[@startModuleName1;startModuleName2]:io_priority:memory_priority:grade
        ##
        ##   Field descriptions:
        ##     process_name     - Executable name (e.g.: game.exe)
        ##     priority         - Process priority class
        ##     affinity         - Hard CPU affinity mask (inherited by child processes)
        ##     cpuset           - Soft CPU preference via Windows CPU Sets
        ##     prime_cpus       - CPUs for prime thread scheduling (CPU-intensive threads). Optionally @prefix1;prefix2 to match start module names (default empty)
        ##     io_priority      - I/O priority level
        ##     memory_priority  - Memory page priority
        ##     ideal_processor  - Ideal CPU assignment based on thread start module. Format: *cpu_spec[@prefix1;prefix2] (default: 0)
        ##     grade            - Rule application frequency (default: 1). Rule runs every Nth loop
        ##
        ## ----------------------------------------------------------------------------
        ## CPU SPECIFICATION FORMATS
        ## ----------------------------------------------------------------------------
        ##   0           - Don't modify (keep current setting)
        ##   0-7         - CPU range: cores 0 through 7 (RECOMMENDED)
        ##   0;4;8       - Individual CPUs: cores 0: 4: and 8
        ##   0-7;64-71   - Multiple ranges: for >64 core systems
        ##   7           - Single CPU: core 7 only (NOT a bitmask!)
        ##   0xFF        - Hex bitmask: legacy format: ≤64 cores only
        ##   *alias      - Use predefined alias (e.g.: *pcore: *ecore)
        ##
        ##   NOTE: "7" means core 7: NOT a bitmask for cores 0-2.
        ##         Use "0x7" or "0-2" if you want cores 0: 1: and 2.
        ##
        ## ----------------------------------------------------------------------------
        ## PRIORITY LEVELS
        ## ----------------------------------------------------------------------------
        ##   priority:        none: idle: below normal: normal: above normal: high: real time
        ##   io_priority:     none: very low: low: normal: high (high requires admin)
        ##   memory_priority: none: very low: low: medium: below normal: normal
        ##
        ##   Use "none" to skip setting that attribute (keep Windows default).
        ##
        ## ----------------------------------------------------------------------------
        ## IDEAL PROCESSOR SYNTAX
        ## ----------------------------------------------------------------------------
        ##   Specifies preferred CPU for threads based on their start module.
        ##   The scheduler assigns ideal CPUs to top N threads by total CPU time
        ##   (where N = number of CPUs specified). Falls back to previous ideal CPU
        ##   when thread drops out of top N.
        ##
        ##   Format: *alias[@prefix1;prefix2;...]
        ##
        ##   Components:
        ##     *            - Required prefix marker for each rule
        ##     alias        - CPU alias name (e.g., pN01, e, 4567 - must be defined in ALIAS section)
        ##     @prefix      - Optional module prefix filter (e.g., engine.dll;render.dll)
        ##
        ##   Multi-segment (different CPUs for different modules):
        ##     *p@engine.dll*e@helper.dll  - Alias *p for engine, *e for helper
        ##
        ##   Examples:
        ##     *pN01@cs2.exe;nvwgf2umx.dll  - Use alias *pN01 for CS2/nvidia threads
        ##     *pN01                        - Use alias *pN01 for all threads
        ##     *p@worker*e@background      - Alias *p for worker, *e for background
        ##
        ## ----------------------------------------------------------------------------
        ## PROCESS GROUPS
        ## ----------------------------------------------------------------------------
        ##   Group multiple processes with the same rule using { } syntax.
        ##   Group name is optional (for documentation/debugging only):
        ##
        ##   # Named group (multi-line)
        ##   group_name {
        ##       process1.exe: process2.exe
        ##       # Comments allowed inside
        ##       process3.exe
        ##   }:priority:affinity:cpuset:prime_cpus[@prefixes]:io_priority:memory_priority:ideal_processor:grade
        ##
        ##   # Named group (single-line)
        ##   browsers { chrome.exe: firefox.exe }:normal:*e:0:0:low:none:0:1
        ##
        ##   # Anonymous group (no name)
        ##   { notepad.exe: calc.exe }:none:*e:0:0:low:none:0:1
        ##
        ## ============================================================================"#,
    ]
}

pub fn print_config_help() {
    for line in get_config_help_lines() {
        log!("{}", line);
    }
}

pub fn print_help_all() {
    *get_use_console!() = true;
    print_cli_help();
    log!("");
    print_config_help();
}
