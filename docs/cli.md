# CLI Module Documentation

Command-line argument parsing and help text generation.

## Overview

This module handles:
- CLI argument parsing ([`parse_args()`](#parse_args))
- Help text generation for `-help` and `-helpall` flags
- Config format template for embedding in output files

## Called By

- [`main()`](main.md) - Entry point for CLI parsing
- User via `-help`, `-helpall` flags

## Data Structures

### CliArgs

Parsed command-line arguments.

```rust
pub struct CliArgs {
    pub interval_ms: u64,                    // Check interval (default: 5000)
    pub help_mode: bool,                     // -help flag
    pub help_all_mode: bool,                 // -helpall flag
    pub convert_mode: bool,                  // -convert flag
    pub autogroup_mode: bool,                // -autogroup flag
    pub find_mode: bool,                     // -find flag
    pub validate_mode: bool,                 // -validate flag
    pub process_logs_mode: bool,             // -processlogs flag
    pub dry_run: bool,                       // -dryrun flag
    pub config_file_name: String,            // -config <file>
    pub blacklist_file_name: Option<String>, // -blacklist <file>
    pub in_file_name: Option<String>,        // -in <file>
    pub out_file_name: Option<String>,       // -out <file>
    pub no_uac: bool,                        // -noUAC flag
    pub loop_count: Option<u32>,             // -loop <count>
    pub time_resolution: u32,                // -resolution <t>
    pub log_loop: bool,                      // -logloop flag
    pub skip_log_before_elevation: bool,     // -skip_log_before_elevation
    pub no_debug_priv: bool,                 // -noDebugPriv flag
    pub no_inc_base_priority: bool,          // -noIncBasePriority flag
}
```

## Functions

### parse_args

Parses command-line arguments into [`CliArgs`](#cliargs).

```rust
pub fn parse_args(args: &[String], cli: &mut CliArgs) -> Result<()>
```

**Behavior:**
- Unknown arguments are silently ignored
- Missing values for arguments use defaults
- Validates interval minimum (16ms)

**Called By:** [`main()`](main.md) at startup

### print_help

Prints basic help message.

```rust
pub fn print_help()
```

**Shows:** Common options (help, config, interval, find, console, noUAC, resolution)

### print_cli_help

Prints detailed CLI help with debug options.

```rust
pub fn print_cli_help()
```

**Shows:** All options including operating modes, I/O arguments, and debug/testing options.

### get_config_help_lines

Returns config file template for embedding.

```rust
pub fn get_config_help_lines() -> Vec<&'static str>
```

**Returns:** Template lines describing config format, inserted at top of converted configs.

**Used By:** [`convert()`](config.md#process-lasso-conversion) when generating output

### print_config_help

Prints config format help.

```rust
pub fn print_config_help()
```

**Called By:** [`print_help_all()`](#print_help_all)

### print_help_all

Prints complete help (CLI + config format).

```rust
pub fn print_help_all()
```

**Called By:** `-helpall` flag handling in [`main()`](main.md)

## CLI Arguments Summary

### Categories

| Category | Flags |
|----------|-------|
| **Help** | `-help`, `-helpall` |
| **Output** | `-console` |
| **Config** | `-config <file>`, `-blacklist <file>` |
| **Timing** | `-interval <ms>`, `-resolution <t>` |
| **Modes** | `-convert`, `-autogroup`, `-find`, `-validate`, `-processlogs`, `-dryrun` |
| **I/O** | `-in <file/dir>`, `-out <file>` |
| **Debug** | `-loop <n>`, `-logloop`, `-noDebugPriv`, `-noIncBasePriority`, `-skip_log_before_elevation` |
| **Privileges** | `-noUAC` |

### Common Use Cases

**Basic run:**
```bash
AffinityServiceRust.exe -config myapp.ini
```

**Debug with console output:**
```bash
AffinityServiceRust.exe -console -noUAC -logloop -loop 3 -interval 2000 -config test.ini
```

**Validate config:**
```bash
AffinityServiceRust.exe -validate -config test.ini
```

**Dry run (preview changes):**
```bash
AffinityServiceRust.exe -dryrun -noUAC -config test.ini
```

**Convert Process Lasso config:**
```bash
AffinityServiceRust.exe -convert -in prolasso.ini -out myconfig.ini
```

## Config Format Template

The `get_config_help_lines()` function provides a minimal template describing the config format. For full config documentation, see [config.md](config.md).

**Template includes:**
- Field descriptions (process, priority, affinity, cpuset, prime, io, memory, ideal, grade)
- CPU alias examples
- Group syntax

## Dependencies

- `crate::log` - Output via logging macro
- `crate::logging` - Console output flag
- `windows::core::Result` - Error handling

## Notes

- When running with UAC elevation, `-console` output goes to a new session that can't be shown in the current session. Use log files instead.
- The `-skip_log_before_elevation` flag is automatically added during UAC elevation to prevent duplicate logging.
