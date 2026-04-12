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
        "## ============================================================================",
        "## AffinityServiceRust Configuration File",
        "## ============================================================================",
        "##",
        "## Full documentation: docs/cli.md and docs/config.md",
        "##",
        "## Format: process:priority:affinity:cpuset:prime:io:memory:ideal:grade",
        "##   process   - Executable name (e.g., game.exe)",
        "##   priority  - Process priority: none|idle|below normal|normal|above normal|high|real time",
        "##   affinity  - Hard CPU affinity: 0-7|0;4;8|0xFF|*alias",
        "##   cpuset    - Soft CPU preference: *p|*e|*alias",
        "##   prime     - Prime thread CPUs: ?10*pN01|*p@module.dll",
        "##   io        - I/O priority: none|very low|low|normal|high",
        "##   memory    - Memory priority: none|very low|low|medium|below normal|normal",
        "##   ideal     - Ideal processor: *alias[@prefix] or 0",
        "##   grade     - Application frequency: 1=every loop, 5=every 5th loop",
        "##",
        "## CPU Aliases (define in ALIASES section):",
        "##   *a = 0-19     # All cores",
        "##   *p = 0-7      # P-cores",
        "##   *e = 8-19     # E-cores",
        "##",
        "## Groups: { proc1: proc2 }:priority:affinity...",
        "## ============================================================================",
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
