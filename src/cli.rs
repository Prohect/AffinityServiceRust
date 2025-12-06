//! Command-line argument parsing and help display.
//!
//! Handles parsing of command-line arguments and displays help messages.

use crate::{log, logging::use_console};

/// Parses command-line arguments and sets the corresponding flags and values.
///
/// # Arguments
/// * `args` - The command-line arguments
/// * Various mutable references for output values
///
/// # Returns
/// `Ok(())` on success, or a Windows error on failure.
pub fn parse_args(
    args: &[String],
    interval_ms: &mut u64,
    help_mode: &mut bool,
    help_all_mode: &mut bool,
    convert_mode: &mut bool,
    find_mode: &mut bool,
    validate_mode: &mut bool,
    dry_run: &mut bool,
    config_file_name: &mut String,
    blacklist_file_name: &mut Option<String>,
    in_file_name: &mut Option<String>,
    out_file_name: &mut Option<String>,
    no_uac: &mut bool,
    loop_count: &mut Option<u32>,
    time_resolution: &mut u32,
    log_loop: &mut bool,
    skip_log_before_elevation: &mut bool,
    no_debug_priv: &mut bool,
    no_inc_base_priority: &mut bool,
) -> windows::core::Result<()> {
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-help" | "--help" | "-?" | "/?" | "?" => {
                *help_mode = true;
            }
            "-helpall" | "--helpall" => {
                *help_all_mode = true;
            }
            "-console" => {
                *use_console().lock().unwrap() = true;
            }
            "-noUAC" | "-nouac" => {
                *no_uac = true;
            }
            "-convert" => {
                *convert_mode = true;
            }
            "-find" => {
                *find_mode = true;
            }
            "-validate" => {
                *validate_mode = true;
            }
            "-dryrun" | "-dry-run" | "--dry-run" => {
                *dry_run = true;
            }
            "-interval" if i + 1 < args.len() => {
                *interval_ms = args[i + 1].parse().unwrap_or(5000).max(16);
                i += 1;
            }
            "-loop" if i + 1 < args.len() => {
                *loop_count = Some(args[i + 1].parse().unwrap_or(1).max(1));
                i += 1;
            }
            "-resolution" if i + 1 < args.len() => {
                *time_resolution = args[i + 1].parse().unwrap_or(0).max(0);
                i += 1;
            }
            "-logloop" => {
                *log_loop = true;
            }
            "-config" if i + 1 < args.len() => {
                *config_file_name = args[i + 1].clone();
                i += 1;
            }
            "-blacklist" if i + 1 < args.len() => {
                *blacklist_file_name = Some(args[i + 1].clone());
                i += 1;
            }
            "-in" if i + 1 < args.len() => {
                *in_file_name = Some(args[i + 1].clone());
                i += 1;
            }
            "-out" if i + 1 < args.len() => {
                *out_file_name = Some(args[i + 1].clone());
                i += 1;
            }
            "-skip_log_before_elevation" => {
                *skip_log_before_elevation = true;
            }
            "-noDebugPriv" | "-nodebugpriv" => {
                *no_debug_priv = true;
            }
            "-noIncBasePriority" | "-noincbasepriority" => {
                *no_inc_base_priority = true;
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
    log!("  -noUAC               disable UAC elevation request");
    log!("  -resolution <t>      time resolution 5210 -> 0.5210ms (default: 0, 0 means do not set time resolution)");
    log!("");
    log!("Modes:");
    log!("  -validate            validate config file syntax without running");
    log!("  -dryrun              show what would be changed without applying");
    log!("  -convert             convert Process Lasso config (-in <file> -out <file>)");
    log!("");
    log!("Config Format: process_name,priority,affinity,cpuset,prime,io_priority,memory_priority");
    log!("  Example: notepad.exe,above normal,0-7,0,0,low,normal");
    log!("  Example: game.exe,high,*pcore,0,*pcore,normal,low");
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
        "Config file format: process_name,priority,affinity,cpuset,prime[@regexes],io_priority,memory_priority",
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
        "  Then use: game.exe,high,*pcore,0,*pcore,normal,normal",
        "",
        "----------------------------------------------------------------------------",
        "PROCESS GROUPS",
        "----------------------------------------------------------------------------",
        "  Group multiple processes with the same rule using { } syntax:",
        "",
        "  # Named group (multi-line)",
        "  group_name {",
        "      process1.exe, process2.exe",
        "      # Comments allowed inside",
        "      process3.exe",
        "  },priority,affinity,cpuset,prime_cpus[@regexes],io_priority,memory_priority",
        "",
        "  # Named group (single-line)",
        "  browsers { chrome.exe, firefox.exe },normal,*e,0,0,low,none",
        "",
        "  # Anonymous group (no name)",
        "  { notepad.exe, calc.exe },none,*e,0,0,low,none",
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
        "  - Optionally filters threads by start module name using regex patterns",
        "    Syntax: prime_cpus@regex1;regex2 (default: .* matches all modules)",
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
    log!("  -resolution <t>      time resolution 5210 -> 0.5210ms (default: 0, 0 means do not set time resolution)");
    log!("");
    log!("Operating Modes:");
    log!("  -validate            validate config file for syntax errors and undefined aliases");
    log!("  -dryrun              simulate changes without applying (shows what would happen)");
    log!("  -convert             convert process configs from -in <file>(from process lasso) to -out <file>");
    log!("  -in <file>           input file for -convert");
    log!("  -out <file>          output file for -convert");
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
