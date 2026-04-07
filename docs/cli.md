# CLI Module Documentation

Command-line argument parsing and help text generation.

## Overview

This module handles:
- CLI argument parsing (`parse_args`)
- Help text generation for `-help` and `-helpall` flags
- Config format documentation output

## Called By

- `main()` in [main.rs](main.md) - Entry point for CLI parsing
- User via `-help`, `-helpall` flags

## Configuration

### Config Format Template

The `get_config_help_lines()` function returns a comprehensive template for `config.ini` files. This template is embedded in converted configs and displayed via `-helpall`.

**Template Location:** `src/cli.rs:199-305`

### Field Descriptions

| Field | Description | Example |
|-------|-------------|---------|
| `process_name` | Executable name (case-insensitive) | `game.exe` |
| `priority` | Process priority class | `normal`, `high`, `real time` |
| `affinity` | Hard CPU affinity mask | `0-7`, `*p`, `0xFF` |
| `cpuset` | Soft CPU preference via Windows CPU Sets | `*e`, `0-19` |
| `prime_cpus` | CPUs for prime thread scheduling with optional module prefixes | `*pN01`, `?10*p` |
| `io_priority` | I/O priority level | `low`, `normal`, `high` |
| `memory_priority` | Memory page priority | `below normal`, `normal` |
| `ideal_processor` | Ideal CPU assignment based on thread start module | `*p@engine.dll` |
| `grade` | Rule application frequency (runs every Nth loop) | `1`, `5`, `10` |

### CPU Specification Formats

| Format | Syntax | Example | Description |
|--------|--------|---------|-------------|
| No change | `0` or empty | `0` | Keep current setting |
| Single CPU | `N` | `7` | Core 7 only (NOT a bitmask!) |
| Range | `start-end` | `0-7` | Cores 0 through 7 |
| Multiple ranges | `range1;range2` | `0-7;64-71` | For >64 core systems |
| Individual | `n1;n2;n3` | `0;4;8` | Specific cores |
| Hex mask | `0xNN` | `0xFF` | Legacy format (≤64 cores) |
| Alias | `*alias` | `*pcore` | Reference to predefined alias |

**Important:** Plain numbers mean core indices, not bitmasks. Use `0-7` for cores 0-7, NOT `7`.

### Priority Levels

**Process Priority:** `none`, `idle`, `below normal`, `normal`, `above normal`, `high`, `real time`

**Thread Priority:** `none`, `idle`, `lowest`, `below normal`, `normal`, `above normal`, `highest`, `time critical`

**I/O Priority:** `none`, `very low`, `low`, `normal`, `high` (admin required for high)

**Memory Priority:** `none`, `very low`, `low`, `medium`, `below normal`, `normal`

### Ideal Processor Syntax

Format: `*alias[@prefix1;prefix2]`

Components:
- `*` - Required prefix marker for each rule segment
- `alias` - CPU alias name (must be defined in ALIAS section)
- `@prefix` - Optional module prefix filter (e.g., `engine.dll;render.dll`)

Multi-segment syntax: `*p@engine.dll*e@helper.dll`

### Process Groups

Group multiple processes with the same rule using `{ }` syntax:

```ini
# Named group (multi-line)
group_name {
    process1.exe: process2.exe
    # Comments allowed inside
    process3.exe
}:priority:affinity:cpuset:prime_cpus[@prefixes]:io_priority:memory_priority:ideal_processor:grade

# Named group (single-line)
browsers { chrome.exe: firefox.exe }:normal:*e:0:0:low:none:0:1

# Anonymous group (no name)
{ notepad.exe: calc.exe }:none:*e:0:0:low:none:0:1
```

## Examples

### Basic Help

```bash
AffinityServiceRust.exe -help
```

Shows common options: `-help`, `-helpall`, `-console`, `-config`, `-find`, `-interval`, `-noUAC`, `-resolution`.

### Detailed Help

```bash
AffinityServiceRust.exe -helpall
```

Shows all options including debug flags: `-validate`, `-processlogs`, `-dryrun`, `-loop`, `-logloop`, `-noDebugPriv`, `-noIncBasePriority`.

### Debug Command Examples

Quick debug (non-admin):
```bash
AffinityServiceRust.exe -console -noUAC -logloop -loop 3 -interval 2000 -config test.ini
```

Admin debug (check log file after):
```bash
AffinityServiceRust.exe -logloop -loop 3 -interval 2000 -config test.ini
# Then check: logs/YYYYMMDD.log
```

## CLI Arguments

### Basic Arguments

| Argument | Description | Default |
|----------|-------------|---------|
| `-help` `\|--help` `-?` `/?` `?` | Print basic help | - |
| `-helpall` `\|--helpall` | Print detailed help with debug options | - |
| `-console` | Use console output instead of log file | Log to file |
| `-noUAC` `-nouac` | Disable UAC elevation request | Request elevation |
| `-config <file>` | Config file to use | `config.ini` |
| `-find` | Find processes with default affinity | - |
| `-blacklist <file>` | Blacklist file for `-find` | - |
| `-interval <ms>` | Check interval in milliseconds | `5000` (min: 16) |
| `-resolution <t>` | Timer resolution (1 tick = 0.0001ms) | `0` (don't set) |

### Operating Modes

| Mode | Description |
|------|-------------|
| `-validate` | Validate config file syntax without running |
| `-processlogs` | Process logs from `-find` mode to find new processes |
| `-dryrun` `-dry-run` `--dry-run` | Simulate changes without applying |
| `-convert` | Convert Process Lasso config (requires `-in` and `-out`) |
| `-autogroup` | Auto-group rules with identical settings |

### I/O Arguments

| Argument | Description | Used With |
|----------|-------------|-----------|
| `-in <file>` | Input file for `-convert` | `-convert`, `-autogroup` |
| `-in <dir>` | Logs directory for `-processlogs` | `-processlogs` |
| `-out <file>` | Output file for `-convert` | `-convert`, `-autogroup` |
| `-out <file>` | Results file for `-processlogs` | `-processlogs` |

### Debug & Testing Options

| Option | Description | Default |
|--------|-------------|---------|
| `-loop <count>` | Number of loops to run | infinite |
| `-logloop` | Log message at start of each loop | disabled |
| `-noDebugPriv` `-nodebugpriv` | Don't request SeDebugPrivilege | request |
| `-noIncBasePriority` `-noincbasepriority` | Don't request SeIncreaseBasePriorityPrivilege | request |
| `-skip_log_before_elevation` | Skip logging before UAC elevation | disabled |

## Notes

- When running with UAC elevation, `-console` output goes to a new session that can't be shown in the current session. Use log files instead.
- The `-skip_log_before_elevation` flag is automatically added during UAC elevation to prevent duplicate logging.
