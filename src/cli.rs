//! Command-line argument parsing and help display.
//!
//! Handles parsing of command-line arguments and displays help messages.

use crate::logging::use_console;

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
    config_file_name: &mut String,
    blacklist_file_name: &mut Option<String>,
    in_file_name: &mut Option<String>,
    out_file_name: &mut Option<String>,
    no_uac: &mut bool,
    loop_count: &mut Option<u32>,
    time_resolution: &mut u32,
    log_loop: &mut bool,
    skip_log_before_elevation: &mut bool,
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
            _ => {}
        }
        i += 1;
    }
    Ok(())
}

/// Prints the basic help message.
pub fn print_help() {
    println!("usage: AffinityServiceRust.exe [args]");
    println!();
    println!("A Windows service to manage process priority, CPU affinity, and IO priority.");
    println!();
    println!("Common Options:");
    println!("  -help | --help       show this help message");
    println!("  -console             output to console instead of log file");
    println!("  -config <file>       config file to use (default: config.ini)");
    println!("  -find                find processes with default affinity (-blacklist <file>)");
    println!("  -interval <ms>       check interval in milliseconds (default: 5000)");
    println!("  -noUAC               disable UAC elevation request");
    println!("  -resolution <t>      time resolution 5210 -> 0.5210ms (default: 0, 0 means do not set time resolution)");
    println!();
    println!("Modes:");
    println!("  -convert             convert Process Lasso config (-in <file> -out <file>)");
    println!();
    println!("Config Format: process_name,priority,affinity,cpuset,prime,io_priority,memory_priority");
    println!("  Example: notepad.exe,above normal,0-7,0,0,low,normal");
    println!("  Example: game.exe,high,*pcore,0,*pcore,normal,normal");
    println!();
    println!("Use -helpall for detailed options and debugging features.");
}

/// Prints the detailed help message with all options.
pub fn print_help_all() {
    println!("usage: AffinityServiceRust.exe [args]");
    println!();
    println!("=== DETAILED HELP & DEBUG OPTIONS ===");
    println!();
    println!("Basic Arguments:");
    println!("  -help | --help       print basic help message");
    println!("  -helpall | --helpall print this detailed help with debug options");
    println!("  -? | /? | ?          print basic help message");
    println!("  -console             use console as output instead of log file");
    println!("  -noUAC | -nouac      disable UAC elevation request");
    println!("  -config <file>       the config file u wanna use (config.ini by default)");
    println!("  -find                find those whose affinity is same as system default which is all possible cores windows could use");
    println!("  -blacklist <file>    the blacklist for -find");
    println!("  -interval <ms>       set interval for checking again (5000 by default, minimal 16)");
    println!();
    println!("Operating Modes:");
    println!("  -convert             convert process configs from -in <file>(from process lasso) to -out <file>");
    println!("  -in <file>           input file for -convert");
    println!("  -out <file>          output file for -convert");
    println!();
    println!("Debug & Testing Options:");
    println!("  -loop <count>        number of loops to run (default: infinite) - for testing");
    println!("  -resolution <t>      time resolution 5210 -> 0.5210ms (default: 0, 0 means do not set time resolution)");
    println!("  -logloop             log a message at the start of each loop for testing");
    println!();
    println!("=== CONFIGURATION FORMAT ===");
    println!();
    println!("Config file format: process_name,priority,affinity,cpuset,prime,io_priority,memory_priority");
    println!();
    println!("Priority Options:");
    println!("  none, idle, below normal, normal, above normal, high, real time");
    println!("  'none' means the program won't change it");
    println!();
    println!("CPU Specification (affinity, cpuset, prime):");
    println!("  0              - no change");
    println!("  0xFF           - hex mask (legacy, ≤64 cores)");
    println!("  0-7            - CPU range");
    println!("  0-7;64-71      - multiple ranges (for >64 cores)");
    println!("  0;4;8;64       - individual CPUs");
    println!("  *alias_name    - use predefined alias");
    println!();
    println!("CPU Aliases (supports >64 cores):");
    println!("  Define reusable CPU specs with: *alias_name = spec");
    println!("  Examples:");
    println!("    *pcore = 0-7;64-71      # P-cores in both CPU groups");
    println!("    *ecore = 8-15;72-79     # E-cores in both CPU groups");
    println!("    *allcores = 0-127       # All 128 cores");
    println!("    *legacy = 0xFF          # Old hex mask still works");
    println!("  Then use: game.exe,high,*pcore,0,*pcore,normal,normal");
    println!();
    println!("IO Priority Options:");
    println!("  none, very low, low, normal");
    println!("  'none' means the program won't change it");
    println!("  Note: high/critical removed due to privilege requirements");
    println!();
    println!("Example Configuration:");
    println!("  # Scheduler constants");
    println!("  @KEEP_THRESHOLD=0.69");
    println!("  @ENTRY_THRESHOLD=0.42");
    println!();
    println!("  # Define CPU aliases (works with >64 cores)");
    println!("  *pcore = 0-7;64-71");
    println!("  *ecore = 8-15;72-79");
    println!();
    println!("  # Process configs: name,priority,affinity,cpuset,prime,io,memory");
    println!("  notepad.exe,above normal,*ecore,0,0,low,normal");
    println!("  game.exe,high,0,0,*pcore,normal,normal");
    println!("  background.exe,idle,*ecore,0,0,very low,low");
    println!();
    println!("=== MULTI-CPU GROUP SUPPORT ===");
    println!();
    println!("- Systems with >64 logical processors use multiple CPU groups");
    println!("- Use range syntax (0-7;64-71) instead of hex masks for >64 cores");
    println!("- CPU Set APIs are used automatically for full multi-group support");
    println!("- Legacy hex masks (0xFF) still work for ≤64 core systems");
    println!();
    println!("=== LIMITATIONS & NOTES ===");
    println!();
    println!("- Admin privileges needed for managing system processes");
    println!("- SetProcessAffinityMask only works within one CPU group (≤64 cores)");
    println!("- For >64 cores, use CPU Set features (cpuset column) instead of affinity");
    println!();
}
