# AffinityServiceRust

<!-- languages -->
- 🇺🇸 [English](https://github.com/Prohect/AffinityServiceRust/blob/master/README.md)
- 🇨🇳 [中文 (简体)](https://github.com/Prohect/AffinityServiceRust/blob/master/README.zh-CN.md)

A high-performance Windows process management service written in Rust that automatically applies CPU affinity, priority, I/O priority, and memory priority rules to running processes based on configuration files.

## Overview

AffinityServiceRust continuously monitors running processes and applies customized scheduling policies based on rules defined in configuration files. It supports:

- **Process Priority Management**: Set process priority class (Idle to Real-time)
- **CPU Affinity**: Hard-pin processes to specific logical processors (legacy ≤64 core systems)
- **CPU Sets**: Soft CPU preferences across all processor groups (modern >64 core systems)
- **Prime Thread Scheduling**: Dynamically identify and assign CPU-intensive threads to designated "prime" cores
- **Ideal Processor Assignment**: Static thread-to-CPU assignment for top N busiest threads
- **I/O Priority Control**: Control disk I/O scheduling priority
- **Memory Priority Control**: Adjust memory page priority for process working set
- **Hot Reload**: Automatically detect and apply config file changes
- **Rule Grades**: Control application frequency per process rule

## Quick Start

1. Build or download the release binary
2. Download `config.ini` and `blacklist.ini` to your working directory
3. Edit `config.ini` to match your CPU topology (see Configuration section)
4. Run with appropriate privileges:

```bash
# Basic usage with console output
AffinityServiceRust.exe -config my_config.ini -console

# Run with admin elevation (recommended for full functionality)
powershell -Command "Start-Process -FilePath './AffinityServiceRust.exe' -Verb RunAs -Wait"

# Show all available options
AffinityServiceRust.exe -helpall
```

## Features

| Feature | Description |
|---------|-------------|
| **Process Priority** | Set priority class: Idle, BelowNormal, Normal, AboveNormal, High, Realtime |
| **CPU Affinity** | Legacy mask-based affinity (≤64 cores, SetProcessAffinityMask) |
| **CPU Sets** | Modern soft CPU preferences (unlimited cores, SetProcessDefaultCpuSets) |
| **Prime Thread Scheduling** | Dynamic thread-to-core assignment using hysteresis-based algorithm |
| **Ideal Processor Assignment** | Static ideal-processor for top N threads by total CPU time |
| **I/O Priority** | VeryLow, Low, Normal, High (requires admin for High) |
| **Memory Priority** | VeryLow, Low, Medium, BelowNormal, Normal |
| **Timer Resolution** | Configure system timer resolution for tighter loops |
| **Hot Reload** | Auto-reload config when files change |
| **Rule Grades** | Control how often each rule is applied |

### Prime Thread Scheduling

The prime thread scheduler dynamically identifies the most CPU-intensive threads and assigns them to designated "prime" cores using Windows CPU Sets:

**Algorithm:**
- Monitors thread CPU cycle consumption at configurable intervals
- Applies hysteresis to prevent thrashing:
  - **Entry threshold**: Thread must exceed configured % of max cycles to become a candidate
  - **Keep threshold**: Once promoted, thread stays prime if above configured % of max cycles
  - **Active streak**: Requires consecutive active intervals before promotion
- Filters low-activity threads automatically
- Supports multi-segment CPU assignment: different modules can use different core sets
- Per-module thread priority control (explicit or auto-boost)
- Thread tracking mode: logs detailed statistics when process exits

**Thread Tracking Output:**
When a tracked process exits, detailed statistics are logged for top N threads:
- Thread ID and total CPU cycles consumed
- Start address resolved to `module.dll+offset` format
- Kernel time and user time
- Thread priority, base priority, context switches
- Thread state and wait reason

### Ideal Processor Assignment

An optional `ideal` specification can be added to rules to request static ideal-processor assignments for the busiest threads. This uses CPU aliases and optional per-rule module filtering:

- Threads are ranked by total CPU time (kernel + user)
- Top N threads (where N = number of CPUs in the alias) receive ideal processor assignment
- Multi-rule syntax allows different CPU sets for different modules
- Threads that fall out of top N have their ideal processor restored

Note about affinity changes and ideal processor resetting:
- When a process's CPU affinity is changed by the service, AffinityServiceRust will proactively reset per-thread ideal processor assignments for that process. This prevents the Windows kernel from clamping many threads toward a narrow CPU range after an affinity change.
- The reset logic:
  - Collects threads' total CPU time and per-thread cycle counts,
  - Sorts threads primarily by total CPU time (descending) and secondarily by cycle count,
  - Assigns ideal processors round-robin across the configured affinity CPUs with a small random shift to avoid clumping.
- This behavior runs automatically when affinity is applied and does not require additional configuration.

## Configuration

### Config Format

Process rules follow this format:
```
process_name:priority:affinity:cpuset:prime_cpus[@prefixes]:io_priority:memory_priority:ideal[@prefixes]:grade
```

### CPU Specification Formats

| Format | Example | Description |
|--------|---------|-------------|
| Range | `0-7` | Cores 0 through 7 |
| Multiple ranges | `0-7;64-71` | For systems with >64 logical processors |
| Individual | `0;2;4;6` | Specific cores |
| Single | `5` | Single core (NOT a bitmask) |
| Hex mask | `0xFF` | Legacy format (≤64 cores only) |
| Alias | `*pcore` | Reference to predefined CPU alias |

**Important:** Plain numbers mean core indices, not bitmasks. Use `0-7` for cores 0-7, NOT `7`.

### Rule Grades

The `grade` field (default: 1) controls how often a rule is applied:

| Grade | Frequency | Use Case |
|-------|-----------|----------|
| `1` | Every loop | Critical processes (games, real-time apps) |
| `2` | Every 2nd loop | Semi-critical processes |
| `5` | Every 5th loop | Background utilities |
| `10` | Every 10th loop | Rarely changing processes |

### Priority Levels

**Process Priority:** `none`, `idle`, `below normal`, `normal`, `above normal`, `high`, `real time`

**Thread Priority:** `none`, `idle`, `lowest`, `below normal`, `normal`, `above normal`, `highest`, `time critical`

**I/O Priority:** `none`, `very low`, `low`, `normal`, `high` (admin required for high)

**Memory Priority:** `none`, `very low`, `low`, `medium`, `below normal`, `normal`

### CPU Aliases

Define reusable CPU specifications under the `ALIASES` section:

```ini
# === ALIASES ===
*a = 0-19           # All cores (8P+12E example)
*p = 0-7            # P-cores (performance cores)
*e = 8-19           # E-cores (efficiency cores)
*pN01 = 2-7         # P-cores except 0-1
```

Aliases support all CPU specification formats, including multiple ranges for >64 core systems.

### Process Groups

Group multiple processes with the same rule using `{ }` syntax:

```ini
# Named group (multi-line)
browsers { chrome.exe: firefox.exe: msedge.exe }:normal:*e:0:0:low:below normal

# Anonymous group (no name needed)
{
    ctfmon.exe: chsime.exe
    sihost.exe: ShellHost.exe
}:none:*e:0:0:low:none
```

### Prime Thread Scheduling Syntax

The `prime_cpus` field supports advanced features:

```
[?[?]x]*alias[@module1[!priority];module2[!priority]*alias2@module3...]
```

**Components:**
- `?x*cpus` - Track top x threads, apply rules, log on exit
- `??x*cpus` - Monitor only: track and log on exit, don't apply CPU sets
- `*alias@module1;module2` - Only affect threads from specified modules
- `*alias1@mod1*alias2@mod2` - Multi-segment: different CPUs per module
- `module!priority` - Set explicit thread priority (idle to time critical)
- `module` - Auto-boost (current priority + 1 tier, capped at highest)

**Examples:**

```ini
# Track top 10 threads on P-cores (except 0-1)
game.exe:normal:*a:*p:?10*pN01:normal:normal:1

# Multi-segment: CS2 on P-cores, NVIDIA on E-cores
cs2.exe:normal:*a:*p:*p@cs2.exe*e@nvwgf2umx.dll:normal:normal:1

# Per-module thread priorities
cs2.exe:normal:*a:*p:*pN01@cs2.exe!time critical;nvwgf2umx.dll!above normal:normal:normal:1

# Monitor only (no apply): track top 20 threads
game.exe:normal:*a:*p:??20*pN01:normal:normal:1
```

### Ideal Processor Assignment Syntax

Add `ideal` specification before the `grade` field:

```ini
# Simple: top N threads get ideal CPUs from alias
background.exe:normal:*a:*p:*p:normal:normal:*p:5

# Module-filtered: only threads from UnityPlayer.dll
game.exe:normal:*a:*p:*pN01@UnityPlayer.dll:normal:normal:*pN01@UnityPlayer.dll:1

# Multi-rule: engine threads -> p cores, render threads -> subset
game.exe:normal:*a:*p:*p@engine.dll*pN01@render.dll:normal:normal:*p@engine.dll*pN01@render.dll:1
```

### Scheduler Constants

Configure prime thread scheduler behavior:

```ini
@MIN_ACTIVE_STREAK = 2    # Consecutive intervals before promotion (default: 2)
@ENTRY_THRESHOLD = 0.42   # Fraction of max cycles to become candidate (default: 0.42)
@KEEP_THRESHOLD = 0.69    # Fraction of max cycles to stay prime (default: 0.69)
```

### Complete Example

```ini
# ============================================================================
# AffinityServiceRust Configuration File
# ============================================================================

# === CONSTANTS ===
@MIN_ACTIVE_STREAK = 2
@ENTRY_THRESHOLD = 0.42
@KEEP_THRESHOLD = 0.69

# === ALIASES ===
*a = 0-19           # All cores
*p = 0-7            # P-cores
*e = 8-19           # E-cores
*pN01 = 2-7         # P-cores except 0-1

# === RULES ===
# Format: name:priority:affinity:cpuset:prime:io:memory:ideal:grade

# Simple rule
cs2.exe:normal:*a:*p:*pN01:normal:normal:1

# Prime with module filtering
game.exe:normal:*a:*p:*pN01@UnityPlayer.dll;GameModule.dll:normal:normal:1

# Multi-segment prime scheduling
cs2.exe:normal:*a:*p:*p@cs2.exe*e@nvwgf2umx.dll:normal:normal:1

# Per-module thread priorities
engine.exe:normal:*a:*p:*p@engine.dll!time critical*pN01@render.dll!highest:normal:normal:1

# Thread tracking
game.exe:normal:*a:*p:?10*pN01:normal:normal:1

# Ideal processor assignment
background.exe:normal:*a:*p:*p:normal:normal:*p:5

# Named group
browsers { chrome.exe: firefox.exe: msedge.exe }:normal:*e:0:0:low:below normal:1
```

## Command Line Options

### Basic Options

| Option | Description | Default |
|--------|-------------|---------|
| `-help` | Show basic help | - |
| `-helpall` | Show detailed help with examples | - |
| `-console` | Output to console instead of log files | Log to file |
| `-config <file>` | Custom config file | `config.ini` |
| `-blacklist <file>` | Blacklist file for `-find` mode | - |
| `-noUAC` | Run without requesting admin privileges | Request elevation |
| `-interval <ms>` | Check interval in milliseconds (min: 16) | `5000` |
| `-resolution <ticks>` | Timer resolution (1 tick = 0.0001ms), `0` = don't set | - |

### Operating Modes

| Mode | Description |
|------|-------------|
| `-convert` | Convert Process Lasso config (`-in <file> -out <file>`) |
| `-find` | Log unmanaged processes with default affinity |
| `-validate` | Validate config file syntax without running |
| `-processlogs` | Process logs to find new processes and search paths |
| `-dryrun` | Show what would be changed without applying |

### Debug Options

| Option | Description |
|--------|-------------|
| `-loop <count>` | Number of loops to run (default: infinite) |
| `-logloop` | Log message at start of each loop |
| `-noDebugPriv` | Don't request SeDebugPrivilege |
| `-noIncBasePriority` | Don't request SeIncreaseBasePriorityPrivilege |

## Tools

### Config Validation

Validate your configuration before running:
```bash
AffinityServiceRust.exe -validate -config my_config.ini
```

### Dry Run Mode

Preview changes without applying them:
```bash
AffinityServiceRust.exe -dryrun -noUAC -config test.ini
```

### Process Discovery

Find processes not covered by your config:
```bash
AffinityServiceRust.exe -find -blacklist blacklist.ini
```

### Config Conversion

Convert Process Lasso config format:
```bash
AffinityServiceRust.exe -convert -in prolasso.ini -out my_config.ini
```

## Privileges and Capabilities

### What You Need to Know

| Target Process | No Admin | Admin | Notes |
|----------------|----------|-------|-------|
| Same-user processes | ✅ Full | ✅ Full | Works without elevation |
| Elevated processes | ❌ | ✅ Full | Needs admin |
| SYSTEM processes | ❌ | ✅ Full | Needs admin |
| Protected processes | ❌ | ❌ | Even admin cannot modify |

| Rule | No Admin | Admin | Notes |
|------|----------|-------|-------|
| Process Priority | ✅ | ✅ | All levels work |
| CPU Affinity | ✅ | ✅ | ≤64 cores only |
| CPU Sets | ✅ | ✅ | Works on >64 cores |
| Prime Scheduling | ✅ | ✅ | Thread-level CPU sets |
| I/O Priority - High | ❌ | ✅ | Requires admin (SeIncreaseBasePriorityPrivilege) |
| Memory Priority | ✅ | ✅ | All levels work |

**Recommendation:** Run with admin privileges for full functionality, especially for I/O priority `high` and managing SYSTEM processes.

## Building

### Requirements

- Rust toolchain (edition 2021 or 2024)
- Windows SDK
- Visual Studio Build Tools (for MSVC) or MinGW (for GNU toolchain)

### Build Commands

```bash
# Release build
cargo build --release

# Run tests
cargo test

# Validate config
cargo build --release && ./target/release/AffinityServiceRust.exe -validate
```

### Output

The release binary will be at:
```
target/release/AffinityServiceRust.exe
```

## Project Structure

| File | Description |
|------|-------------|
| `src/main.rs` | Main loop, config application logic |
| `src/config.rs` | Config file parsing, CPU spec parsing, validation |
| `src/cli.rs` | Command-line argument parsing |
| `src/priority.rs` | Priority enums (Process, Thread, I/O, Memory) |
| `src/logging.rs` | Logging infrastructure (file and console) |
| `src/process.rs` | Process enumeration and snapshot management |
| `src/scheduler.rs` | Prime thread scheduler implementation |
| `src/winapi.rs` | Windows API wrappers, module resolution, privilege handling |
| `config.ini` | Default configuration file |
| `blacklist.ini` | Default blacklist for process discovery |
| `DEBUG.md` | Debug guide and troubleshooting |

## How It Works

1. **Startup**: Parse config file, request necessary privileges (SeDebugPrivilege, SeIncreaseBasePriorityPrivilege), optionally elevate to admin
2. **Main Loop**: 
   - Enumerate all running processes
   - Match each process against configured rules
   - Apply priority, affinity, CPU sets, prime scheduling, and other settings
   - Sleep for configured interval
3. **Hot Reload**: Monitor config files for changes, automatically reload and reapply
4. **Prime Thread Scheduler**:
   - Track thread cycle time at each interval
   - Apply hysteresis-based promotion/demotion logic
   - Use Windows CPU Sets for fine-grained thread placement
5. **Ideal Processor Assignment**:
   - Rank threads by total CPU time
   - Assign ideal processors to top N threads
   - Restore ideal processors for threads that fall out of ranking

## Known Limitations

1. **CPU Affinity ≤64 cores**: The legacy SetProcessAffinityMask API only works within a single processor group. For >64 core systems, use CPU Sets instead.

2. **Multi-group/NUMA systems**: This project has not been tested on multi-processor-group or NUMA systems. The `ideal` processor assignment currently assigns processors within processor group 0 only. Systems with >64 logical processors or multiple CPU groups may experience unexpected behavior.

3. **Protected processes**: Processes like `csrss.exe` and `smss.exe` cannot be modified, even with admin privileges.

4. **Console output with elevation**: When using `-console` with UAC elevation, the elevated process spawns in a new window that closes immediately. Use log file output instead.

5. **Thread start address resolution**: Requires admin elevation with SeDebugPrivilege. Without elevation, start addresses show as `0x0`.

6. **Timer resolution**: The system timer resolution affects loop precision. Very low values (<1ms) may impact system stability.

## Contributions

Issues and pull requests are welcome.

AI agent developers refer to project_specific_agent.md for useful CLI tools and batch editing workflows.

## License

License [LICENSE](LICENSE).
