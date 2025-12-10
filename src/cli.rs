//! Command-line argument parsing and help display.
//!
//! Handles parsing of command-line arguments and displays help messages.

use crate::{log, logging::use_console};

/// Configuration struct for CLI arguments.
/// Defaults are set here; `parse_args` will override them based on user input.
#[derive(Debug, Default)]
pub struct CliArgs {
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

impl CliArgs {
    /// Creates a new `CliArgs` with sensible defaults.
    pub fn new() -> Self {
        Self {
            interval_ms: 5000,                          // Default from the original code
            config_file_name: "config.ini".to_string(), // Default
            ..Default::default()
        }
    }
}

/// Parses command-line arguments and sets the corresponding flags and values.
///
/// # Arguments
/// * `args` - The command-line arguments
/// * `cli` - Mutable reference to the CLI config struct
///
/// # Returns
/// `Ok(())` on success, or a Windows error on failure.
pub fn parse_args(args: &[String], cli: &mut CliArgs) -> windows::core::Result<()> {
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
                *use_console().lock().unwrap() = true;
            }
            "-noUAC" | "-nouac" => {
                cli.no_uac = true;
            }
            "-convert" => {
                cli.convert_mode = true;
            }
            "-find" => {
                cli.find_mode = true;
            }
            "-validate" => {
                cli.validate_mode = true;
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
                cli.time_resolution = args[i + 1].parse().unwrap_or(0).max(0);
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

/// Prints the basic help message.
pub fn print_help() {
    *use_console().lock().unwrap() = true;
    log!("usage: AffinityServiceRust.exe [args]");
    log!("");
    log!("A Windows service to manage process priority, CPU affinity, IO priority, and memory priority.");
    log!("");
    log!("Common Options:");
    log!("  -help | --help       show this help message");
    log!("  -console             output to console instead of log file");
    log!("  -config <file>       config file to use (default: config.ini)");
    log!("  -find                find processes with default affinity (-blacklist <file>)");
    log!("  -interval <ms>       check interval in milliseconds (default: 5000)");
    log!("");
    log!("  -noUAC               disable UAC elevation request");
    log!("  -resolution <t>      time resolution 5210 -> 0.5210ms (default: 0, 0 means do not set time resolution)");
    log!("");
    log!("Modes:");
    log!("  -validate            validate config file syntax without running");
    log!("  -processlogs         process logs (from -find mode) to find new processes and search paths (-config <file> -blacklist <file> -in <logs dir> -out <file>)");
    log!("  -dryrun              show what would be changed without applying");
    log!("  -convert             convert Process Lasso config (-in <file> -out <file>)");
    log!("");
    log!("Config Format: process_name:priority:affinity:cpuset:prime:io_priority:memory_priority");
    log!("  Example: notepad.exe:above normal:0-7:0:0:low:normal");
    log!("  Example: game.exe:high:*pcore:0:*pcore@module.dll:normal:low");
    log!("  Example: cs2.exe:high:*pcore:0:*pcore@cs2.exe*p;nvidia.dll*e:normal:low");
    log!("");
    log!("Use -helpall for detailed options and debugging features.");
}

/// Returns configuration help lines (for embedding in converted files).
pub fn get_config_help_lines() -> Vec<&'static str> {
    vec![
        "============================================================================",
        "AffinityServiceRust Configuration Format",
        "============================================================================",
        "",
        "Config file format: process_name:priority:affinity:cpuset:prime[@prefixes]:io_priority:memory_priority",
        "",
        "  prime[@prefixes] supports per-module CPU overrides:",
        "    prime_cpus@module1*alias1;module2*alias2",
        "",
        "----------------------------------------------------------------------------",
        "PRIORITY OPTIONS",
        "----------------------------------------------------------------------------",
        "  priority:        none, idle, below normal, normal, above normal, high, real time",
        "  io_priority:     none, very low, low, normal, high (high requires admin)",
        "  memory_priority: none, very low, low, medium, below normal, normal",
        "",
        "  Use 'none' to skip setting that attribute (keep Windows default).",
        "",
        "----------------------------------------------------------------------------",
        "CPU SPECIFICATION FORMATS",
        "----------------------------------------------------------------------------",
        "  0              - no change",
        "  0xFF           - hex mask (legacy, ≤64 cores)",
        "  0-7            - CPU range",
        "  0-7;64-71      - multiple ranges (for >64 cores)",
        "  0;4;8;64       - individual CPUs",
        "  *alias_name    - use predefined alias",
        "",
        "  NOTE: \"7\" means core 7, NOT a bitmask for cores 0-2.",
        "        Use \"0x7\" or \"0-2\" if you want cores 0, 1, and 2.",
        "",
        "----------------------------------------------------------------------------",
        "CPU ALIASES (supports >64 cores)",
        "----------------------------------------------------------------------------",
        "  Define reusable CPU specs with: *alias_name = spec",
        "  Examples:",
        "    *pcore = 0-7;64-71      # P-cores in both CPU groups",
        "    *ecore = 8-15;72-79     # E-cores in both CPU groups",
        "    *allcores = 0-127       # All 128 cores",
        "    *legacy = 0xFF          # Old hex mask still works",
        "  Then use: game.exe:high:*pcore:0:*pcore:normal:normal",
        "",
        "----------------------------------------------------------------------------",
        "PROCESS GROUPS",
        "----------------------------------------------------------------------------",
        "  Group multiple processes with the same rule using { } syntax:",
        "",
        "  # Named group (multi-line)",
        "  group_name {",
        "      process1.exe: process2.exe",
        "      # Comments allowed inside",
        "      process3.exe",
        "  }:priority:affinity:cpuset:prime_cpus[@prefixes]:io_priority:memory_priority",
        "",
        "  # Named group (single-line)",
        "  browsers { chrome.exe: firefox.exe }:normal:*e:0:0:low:none",
        "",
        "  # Prime with per-module CPU assignment",
        "  cs2.exe:high:*pcore:0:*pcore@cs2.exe*p;nvidia.dll*e:high:none",
        "",
        "  # Anonymous group (no name)",
        "  { notepad.exe: calc.exe }:none:*e:0:0:low:none",
        "",
        "----------------------------------------------------------------------------",
        "SCHEDULER CONSTANTS (for Prime Thread Scheduling)",
        "----------------------------------------------------------------------------",
        "  @MIN_ACTIVE_STREAK = 2      # consecutive active intervals before promotion",
        "  @KEEP_THRESHOLD    = 0.69   # keep prime when above this * highest cycles",
        "  @ENTRY_THRESHOLD   = 0.42   # become prime when above this * highest cycles",
        "",
        "----------------------------------------------------------------------------",
        "PRIME THREAD SCHEDULING",
        "----------------------------------------------------------------------------",
        "  For processes with prime_cpus configured, the scheduler:",
        "  - Monitors thread CPU cycles and promotes active threads to prime cores",
        "  - Optionally filters threads by start module name using prefix patterns",
        "    Syntax: prime_cpus@prefix1;prefix2 (default: empty matches all modules)",
        "  - Logs promoted/demoted threads with start address module resolution",
        "    e.g., \"11184-1234-game.exe -> (promoted, [2-7], cycles=123456, start=ntdll.dll+0x3C320)\"",
        "",
        "  NOTE: Thread start address resolution (module+offset) requires:",
        "    - Admin elevation (UAC)",
        "    - SeDebugPrivilege enabled",
        "  Without elevation, start addresses show as 0x0.",
        "",
        "----------------------------------------------------------------------------",
        "MULTI-CPU GROUP SUPPORT",
        "----------------------------------------------------------------------------",
        "  - Systems with >64 logical processors use multiple CPU groups",
        "  - Use range syntax (0-7;64-71) instead of hex masks for >64 cores",
        "  - CPU Set APIs are used automatically for full multi-group support",
        "  - Legacy hex masks (0xFF) still work for ≤64 core systems",
        "",
        "----------------------------------------------------------------------------",
        "LIMITATIONS & NOTES",
        "----------------------------------------------------------------------------",
        "  - Admin privileges needed for managing system processes",
        "  - SetProcessAffinityMask only works within one CPU group (≤64 cores)",
        "  - For >64 cores, use CPU Set features (cpuset column) instead of affinity",
        "  - IO priority 'high' requires admin elevation",
        "  - IO priority 'critical' is kernel-only and not available from user mode",
        "",
        "============================================================================",
    ]
}

/// Prints configuration format help to console.
pub fn print_config_help() {
    for line in get_config_help_lines() {
        log!("{}", line);
    }
}

/// Prints CLI help (command line arguments).
pub fn print_cli_help() {
    log!("usage: AffinityServiceRust.exe [args]");
    log!("");
    log!("=== COMMAND LINE OPTIONS ===");
    log!("");
    log!("Basic Arguments:");
    log!("  -help | --help       print basic help message");
    log!("  -helpall | --helpall print this detailed help with debug options");
    log!("  -? | /? | ?          print basic help message");
    log!("  -console             use console as output instead of log file");
    log!("  -noUAC | -nouac      disable UAC elevation request");
    log!("  -config <file>       the config file u wanna use (config.ini by default)");
    log!("  -find                find those whose affinity is same as system default which is all possible cores windows could use");
    log!("  -blacklist <file>    the blacklist for -find");
    log!("  -interval <ms>       set interval for checking again (5000 by default, minimal 16)");
    log!("");
    log!("  -resolution <t>      time resolution 5210 -> 0.5210ms (default: 0, 0 means do not set time resolution)");
    log!("");
    log!("Operating Modes:");
    log!("  -validate            validate config file for syntax errors and undefined aliases");
    log!("  -processlogs         process logs (from -find mode) to find new processes and search paths (-config <file> -blacklist <file> -in <logs dir> -out <file>)");
    log!("  -dryrun              simulate changes without applying (shows what would happen)");
    log!("  -convert             convert process configs from -in <file>(from process lasso) to -out <file>");
    log!("  -in <file>           input file for -convert / logs directory for -processlogs (default: logs)");
    log!("  -out <file>          output file for -convert / results file for -processlogs (default: new_processes_results.txt)");
    log!("");
    log!("Debug & Testing Options:");
    log!("  -loop <count>        number of loops to run (default: infinite) - for testing");
    log!("  -logloop             log a message at the start of each loop for testing");
    log!("  -noDebugPriv         not request SeDebugPrivilege");
    log!("  -noIncBasePriority   not request SeIncreaseBasePriorityPrivilege");
    log!("");
    log!("=== DEBUGGING ===");
    log!("");
    log!("Quick debug command (non-admin):");
    log!("  AffinityServiceRust.exe -console -noUAC -logloop -loop 3 -interval 2000 -config test.ini");
    log!("");
    log!("Admin debug (check log file after, do NOT use -console):");
    log!("  AffinityServiceRust.exe -logloop -loop 3 -interval 2000 -config test.ini");
    log!("  Then check: logs/YYYYMMDD.log");
    log!("");
    log!("Note: When running with UAC elevation, -console output goes to a new window");
    log!("that closes immediately. Use log files instead for admin testing.");
    log!("");
}

/// Prints the detailed help message with all options (CLI + Config).
pub fn print_help_all() {
    *use_console().lock().unwrap() = true;
    print_cli_help();
    log!("");
    print_config_help();
}
