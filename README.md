# AffinityServiceRust

<!-- languages -->
- 🇺🇸 [English](https://github.com/Prohect/AffinityServiceRust/blob/master/README.md)
- 🇨🇳 [中文 (简体)](https://github.com/Prohect/AffinityServiceRust/blob/master/README.zh-CN.md)

A high-performance Windows process management service written in Rust that automatically applies CPU affinity, priority, I/O priority, and memory priority rules to processes based on configuration files.

## Features

| Feature | Description |
|---------|-------------|
| **Process Priority** | Set priority class (Idle, Below Normal, Normal, Above Normal, High, Real-time) |
| **CPU Affinity** | Hard pin processes to specific cores (≤64 cores, inherited by children) |
| **CPU Sets** | Soft preference for cores via Windows CPU Sets (works on >64 cores) |
| **Prime Thread Scheduling** | Dynamically assign CPU-intensive threads to performance cores |
| **I/O Priority** | Control I/O priority (Very Low, Low, Normal, High - requires admin) |
| **Memory Priority** | Control memory page priority (Very Low to Normal) |
| **Timer Resolution** | Adjust Windows system timer resolution |
| **Hot Reload** | Automatically reload config when files change |
| **Rule Grades** | Control rule application frequency (every Nth loop) |

> **Note on >64 core systems:** CPU affinity (SetProcessAffinityMask) only works within a single processor group (≤64 cores). For systems with >64 cores, use CPU Sets which work across all processor groups as a soft preference.

### Prime Thread Scheduling

For multi-threaded applications (e.g., games), this feature dynamically identifies the most CPU-intensive threads and assigns them to designated "prime" cores using Windows CPU Sets:

**Algorithm:**
- Monitors thread CPU cycle consumption in real-time
- Applies hysteresis to prevent thrashing:
  - **Entry threshold**: Thread must exceed 42% of max cycles (configurable via `@ENTRY_THRESHOLD`)
  - **Keep threshold**: Once promoted, thread stays prime if above 69% of max cycles (configurable via `@KEEP_THRESHOLD`)
  - **Active streak**: Requires consistent activity for 2+ intervals before promotion (configurable via `@MIN_ACTIVE_STREAK`)
- Filters low-activity threads automatically
- Optional module-based filtering: only promote threads from specific DLLs/modules
- Optional thread tracking: logs top N threads by CPU cycles on process exit
- Logs thread start address with module resolution (e.g., `ntdll.dll+0x3C320`)

**Multi-Segment CPU Assignment:**
- Supports per-module CPU overrides: different modules can run on different core sets
- Syntax: `*alias1@module1.dll;module2.dll*alias2@module3.dll`
- Example: CS2 game threads on P-cores, NVIDIA driver threads on E-cores

**Thread Priority Control:**
- Per-module thread priority: `module.dll!time critical` sets explicit priority
- Auto-boost mode: when priority omitted, automatically boosts by 1 tier (capped at Highest)

**Tracking Mode:**
- `?x*cpus` - Track top x threads and log detailed statistics on process exit
- `??x*cpus` - Monitor-only: track and log threads but don't apply CPU sets
- Logs include: TID, CPU cycles, kernel/user time, context switches, start address with module+offset

> **Note:** Thread start address resolution (module+offset format) requires admin elevation with SeDebugPrivilege. Without elevation, start addresses show as `0x0`. Resolution uses `psapi GetMappedFileName` — no symbol server or internet access needed.

## Quick Start

1. Download or compile `AffinityServiceRust.exe`
2. Download [`config.ini`](https://github.com/Prohect/AffinityServiceRust/blob/master/config.ini) and [`blacklist.ini`](https://github.com/Prohect/AffinityServiceRust/blob/master/blacklist.ini)
3. Edit `config.ini` to match your CPU topology
4. Run the application (admin recommended for full functionality)

```bash
# Basic usage with console output
AffinityServiceRust.exe -config my_config.ini -console

# Show all options
AffinityServiceRust.exe -helpall

# Convert Process Lasso config
AffinityServiceRust.exe -convert -in prolasso.ini -out my_config.ini

# Find unmanaged processes
AffinityServiceRust.exe -find
```

> **Note:** By default, runs silently in background with logs in `logs\YYYYmmDD.log`. Use `-console` for real-time output. Admin privileges enable high I/O priority and system process management.

## Command Line Options

### Basic Options

| Option | Description |
|--------|-------------|
| `-help` | Show basic help |
| `-helpall` | Show detailed help with examples |
| `-console` | Output to console instead of log files |
| `-config <file>` | Use custom config file (default: `config.ini`) |
| `-blacklist <file>` | Blacklist file for `-find` mode |
| `-noUAC` | Run without requesting admin privileges |
| `-interval <ms>` | Check interval in milliseconds (default: `5000`, minimum: `16`) |
| `-resolution <0.0001ms>` | Set timer resolution (e.g., `5210` = 0.5210ms, `0` = don't set) |

### Operating Modes

| Mode | Description |
|------|-------------|
| `-find` | Log unmanaged processes with default affinity |
| `-convert` | Convert Process Lasso config (`-in <file> -out <file>`) |
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

## Configuration

### Format

```
process_name:priority:affinity:cpuset:prime_cpus[@prefixes]:io_priority:memory_priority:ideal[@prefixes]:grade
```

- `ideal[@prefixes]` - Optional ideal-processor assignment specification. Use CPU aliases prefixed with `*` to describe which logical CPUs should be used as "ideal processors" for top threads. Each alias may include an optional module prefix filter after `@` (multiple prefixes separated by `;`). If the `@prefixes` portion is omitted the alias applies to all threads in the process. See "Ideal Processor Assignment" in the Prime Thread Scheduling section for syntax and examples.

### CPU Specification

| Format | Example | Description |
|--------|---------|-------------|
| Range | `0-7` | Cores 0 through 7 |
| Multiple ranges | `0-7;64-71` | For >64 core systems |
| Individual | `0;2;4;6` | Specific cores |
| Single | `7` | Single core (NOT a mask) |
| Hex mask | `0xFF` | Legacy format (≤64 cores) |
| Alias | `*pcore` | Predefined alias |
| No change | `0` | Don't modify |

### Rule Grades

The `grade` field controls how often a rule is applied (default: 1 = every loop):

| Grade | Frequency | Use Case |
|-------|-----------|----------|
| `1` | Every loop | Critical processes (games, real-time apps) |
| `2` | Every 2nd loop | Semi-critical processes |
| `5` | Every 5th loop | Background utilities |
| `10` | Every 10th loop | Rarely changing processes (updaters) |

```ini
# Apply every loop (default)
game.exe:high:*pcore:0:*pcore:normal:normal:0:1

# Apply every 3rd loop (for less critical processes)
background.exe:normal:*ecore:0:0:low:none:0:3

# Apply every 10th loop (minimal monitoring)
updater.exe:normal:0:0:0:normal:none:0:10
```

> **Important:** Plain numbers like `7` mean core 7, not a bitmask. Use `0x7` or `0-2` for cores 0-2.

### Priority Levels

| Type | Levels |
|------|--------|
| Process | `none`, `idle`, `below normal`, `normal`, `above normal`, `high`, `real time` |
| Thread | `none`, `idle`, `lowest`, `below normal`, `normal`, `above normal`, `highest`, `time critical` |
| I/O | `none`, `very low`, `low`, `normal`, `high` (admin required) |
| Memory | `none`, `very low`, `low`, `medium`, `below normal`, `normal` |

### CPU Aliases

Define reusable CPU specifications with `*name = spec`:

```ini
# === ALIASES ===
*a = 0-19           # All cores (8P+12E)
*p = 0-7            # P-cores
*e = 8-19           # E-cores
*pN01 = 2-7         # P-cores except 0-1
```

Aliases support all CPU specification formats including multiple ranges for >64 core systems.

### Process Groups

Group multiple processes with the same rule using `{ }` syntax. Group name is optional (for documentation only):

```ini
# Named group (multi-line)
browsers {
    chrome.exe: firefox.exe: msedge.exe
    # Comments allowed inside groups
}:normal:*e:0:0:low:below normal

# Named group (single-line)
sys_utils { notepad.exe: calc.exe }:none:*e:0:0:low:none

# Anonymous group (no name needed)
{
    textinputhost.exe: ctfmon.exe: chsime.exe
    dllhost.exe: sihost.exe: ShellHost.exe
}:none:*e:0:0:low:none

# Anonymous single-line
{ taskmgr.exe: perfmon.exe }:none:*a:0:0:none:none
```

### Prime Thread Scheduling

The `prime_cpus` field supports multi-segment CPU assignment with per-module filtering and thread priority control.

**Syntax:**
```
[?[?]x]*alias1[@module1[!priority1];module2[!priority2]*alias2@module3[!priority3];module4...]
```

**Parsing rules:**
1. Optional tracking prefix: `?x` (track and apply) or `??x` (track only, no apply)
2. Split by `*` to get segments (each segment = CPU alias + its module list)
3. Within each segment after `@`, split by `;` to get module prefixes
4. Each module prefix can have optional `!priority` suffix

**Components:**

| Component | Description |
|-----------|-------------|
| `prime_cpus` | Base CPU set for prime threads (all modules) |
| `?x*prime_cpus` | Track top x threads, apply rules, log on exit |
| `??x*prime_cpus` | Monitor only: track top x threads, log on exit, don't apply CPU sets |
| `*alias@module1;module2` | Only promote threads from specified module prefixes, use alias CPUs |
| `*alias1@mod1*alias2@mod2` | Multi-segment: mod1 threads on alias1 CPUs, mod2 threads on alias2 CPUs |
| `module!priority` | Set explicit thread priority (idle to time critical) |
| `module` | Use auto-boost (current priority + 1 tier, capped at highest) |

**Examples:**

```ini
# Simple - all prime threads on P-cores except 0-1
game.exe:normal:*a:*p:*pN01:normal:normal:0:1

# Track top 10 threads, apply rules, log on exit
game.exe:normal:*a:*p:?10*pN01:normal:normal:0:1

# Monitor only - track top 20 threads, log on exit, don't apply CPU sets
game.exe:normal:*a:*p:??20*pN01:normal:normal:0:1

# Module filtering - only CS2 and NVIDIA threads
cs2.exe:normal:*a:*p:*pN01@cs2.exe;nvwgf2umx.dll:normal:normal:0:1

# Multi-segment - CS2 on P-cores, NVIDIA on E-cores
cs2.exe:normal:*a:*p:*p@cs2.exe*e@nvwgf2umx.dll:normal:normal:0:1

# Per-module thread priority - CS2 at time critical, NVIDIA at above normal
cs2.exe:normal:*a:*p:*pN01@cs2.exe!time critical;nvwgf2umx.dll!above normal:normal:normal:0:1

# Three segments with different CPUs and priorities
game.exe:normal:*a:*p:*p@engine.dll!time critical*pN01@render.dll!highest*e@background.dll!normal:normal:normal:0:1

# Mixed - some with explicit priority, others auto-boost
game.exe:normal:*a:*p:*pN01@UnityPlayer.dll!time critical;GameModule.dll:normal:normal:0:1

# Track and multi-segment - track top 10, CS2 on P-cores, NVIDIA on E-cores
cs2.exe:normal:*a:*p:?10*p@cs2.exe*e@nvwgf2umx.dll:normal:normal:0:1
```

Ideal Processor Assignment

An optional `ideal` specification can be added (immediately before the rule `grade`) to request static ideal-processor assignments for the busiest threads of a process. This uses CPU aliases (the same `*name` aliases defined under ALIASES) and optional per-rule module filtering.

- Placement: the `ideal` field appears before the final `grade` field in a rule.
- Syntax for `ideal`:
  - `*alias` — Apply the alias CPUs as candidates for ideal processor assignment to all threads.
  - `*alias@prefix1;prefix2` — Apply the alias CPUs only to threads whose start module name begins with one of the prefixes.
  - Multiple rules can be chained: `*alias1@mod1*alias2@mod2`
- Semantics:
  - For each `*alias` rule, the implementation ranks matching threads by total CPU (kernel + user). The top N threads (N = number of CPUs in the alias) are assigned ideal processors in ranking order using the alias's CPU indices.
  - If a thread later falls out of the top N, its previous ideal processor is restored.
  - If the alias has no module filter (no `@...`), it matches all threads in the process.
  - The implementation currently assigns processors within processor group 0 (systems with >64 logical processors / multiple groups may need mapping updates).

**Examples of `ideal` usage:**
```ini
# Use alias *pN01 CPUs as ideal for the top threads of matching modules
game.exe:normal:*a:*p:*pN01@UnityPlayer.dll:normal:normal:*pN01@UnityPlayer.dll:1

# Multi-rule: engine threads -> p cores, render threads -> subset pN01
game.exe:normal:*a:*p:*p@engine.dll*pN01@render.dll:normal:normal:*p@engine.dll*pN01@render.dll:1

# Alias without filter applies to all threads (top N busiest threads get ideal CPUs)
background.exe:normal:*a:*p:*p:normal:normal:*p:5
```

**When a tracked process exits**, detailed statistics are logged for each thread:
- Thread ID and total CPU cycles
- Start address resolved to `module.dll+offset` format
- Kernel time and user time
- Thread priority and state
- Context switches and wait reason

### Scheduler Constants

Configure prime thread scheduling behavior:

```ini
@MIN_ACTIVE_STREAK = 2   # Consecutive active intervals before promotion
@ENTRY_THRESHOLD = 0.42  # Fraction of max cycles to become candidate
@KEEP_THRESHOLD = 0.69   # Fraction of max cycles to stay prime
```

### Complete Example

```ini
# === CONSTANTS ===
@MIN_ACTIVE_STREAK = 2   # Consecutive active intervals before promotion
@ENTRY_THRESHOLD = 0.42  # Fraction of max cycles to become candidate
@KEEP_THRESHOLD = 0.69   # Fraction of max cycles to stay prime

# === ALIASES ===
*a = 0-19           # All cores (8P+12E)
*p = 0-7            # P-cores
*e = 8-19           # E-cores
*pN01 = 2-7         # P-cores except 0-1
*pN0 = 1-7          # P-cores except 0

# === RULES ===
# Format: process:priority:affinity:cpuset:prime[@prefixes]:io:memory:ideal[@prefixes]:grade

# Single process - simple
cs2.exe:normal:*a:*p:*pN01:normal:normal:1

# Prime with module filtering - only specific modules
game.exe:normal:*a:*p:*pN01@UnityPlayer.dll;GameModule.dll:normal:normal:1

# Multi-segment - different cores per module
cs2.exe:normal:*a:*p:*p@cs2.exe*e@nvwgf2umx.dll:normal:normal:1

# Per-module thread priorities
cs2.exe:normal:*a:*p:*pN01@cs2.exe!time critical;nvwgf2umx.dll!above normal:normal:normal:1

# Three segments with different CPUs and priorities
game.exe:normal:*a:*p:*p@engine.dll!time critical*pN01@render.dll!highest*e@background.dll!normal:normal:normal:1

# Track top 10 threads - log on exit
game.exe:normal:*a:*p:?10*pN01@UnityPlayer.dll:normal:normal:1

# Monitor only - track but don't apply
game.exe:normal:*a:*p:??20*pN01:normal:normal:1

# Named group - browsers on E-cores
browsers { chrome.exe: firefox.exe: msedge.exe }:normal:*e:0:0:low:below normal:1

# Anonymous group - background apps
{
    discord.exe: telegram.exe: slack.exe
}:below normal:*e:0:0:low:low:2

# System processes (admin required for high I/O)
dwm.exe:high:*p:0:0:high:normal:1

# Process Lasso (low priority on E-cores)
process_mgmt {
    bitsumsessionagent.exe: processgovernor.exe: processlasso.exe
    affinityservicerust.exe: affinityserverc.exe
}:none:*e:0:0:low:none:1
```

## Tools

### Process Discovery

Use the `-processlogs` mode to discover new processes from logs that aren't yet in your config or blacklist.

**Requirements:**
- Everything search tool with `es.exe` in PATH
- Log files generated by `-find` mode

**Workflow:**
```bash
# 1. Scan for unmanaged processes (run daily or as needed)
AffinityServiceRust.exe -find -console

# 2. Process the logs to find new processes
AffinityServiceRust.exe -processlogs

# 3. With custom config and blacklist
AffinityServiceRust.exe -processlogs -config my_config.ini -blacklist my_blacklist.ini

# 4. Specify log directory and output file
AffinityServiceRust.exe -processlogs -in mylogs -out results.txt
```

This scans `.find.log` files in the `logs/` directory (or specified with `-in`), extracts process names, filters out configured/blacklisted ones, and searches for the rest using `es.exe`. Results are saved to `new_processes_results.txt` (or specified with `-out`), pairing each process with file paths for easy review.

### Config Conversion

Convert Process Lasso configuration files to AffinityServiceRust format:

```bash
AffinityServiceRust.exe -convert -in prolasso.ini -out my_config.ini
```

This converts Process Lasso rules to the AffinityServiceRust config format, allowing easy migration.

### Config Validation

Validate config file syntax before running:

```bash
AffinityServiceRust.exe -validate -config config.ini
```

Checks for:
- Syntax errors
- Undefined CPU aliases
- Invalid priority values
- Malformed process groups

## Debugging

```bash
# Validate config syntax
AffinityServiceRust.exe -validate -config config.ini

# Dry run - see what would be changed without applying
AffinityServiceRust.exe -dryrun -config config.ini

# Non-admin with console (for testing)
AffinityServiceRust.exe -console -noUAC -logloop -loop 3 -interval 2000

# Admin mode (check logs/YYYYMMDD.log after)
AffinityServiceRust.exe -logloop -loop 3 -interval 2000
```

> **Note:** When running elevated with UAC, avoid `-console` as the UAC spawns a new process and the window closes immediately. Check log files instead.

See [DEBUG.md](DEBUG.md) for more details.

For AI agent contributors (Zed, Cursor, etc.), see project_specific_agent.md for CLI tools and workflow tips.

## Building

```bash
# Install Rust via rustup (select MSVC build tools)
cargo build --release
```

Binary will be in `target/release/AffinityServiceRust.exe`.

For rust-analyzer support, also install MSBuild and Windows 11 SDK.

## How It Works

1. **Initialization**
   - Parse command-line arguments
   - Load and validate config file
   - Request admin elevation (unless `-noUAC`)
   - Enable SeDebugPrivilege and SeIncreaseBasePriorityPrivilege
   - Set timer resolution (if specified)

2. **Main Loop** (every interval, default 5000ms)
   - Take snapshot of all running processes via `NtQuerySystemInformation`
   - For each process matching a config rule:
     - Apply process priority
     - Apply CPU affinity (hard limit via SetProcessAffinityMask)
     - Apply CPU sets (soft preference via SetProcessDefaultCpuSets)
     - Apply prime thread scheduling (dynamic thread-to-core assignment)
     - Apply I/O priority (via NtSetInformationProcess)
     - Apply memory priority (via SetProcessInformation)
   - Log all changes
   - Clean up dead process/thread handles
   - Sleep until next interval

3. **Prime Thread Scheduling** (per process, each interval)
   - Select candidate threads (sort by CPU time, filter dead threads)
   - Query CPU cycles for candidates (via QueryThreadCycleTime)
   - Calculate delta cycles since last check
   - Update active streaks (consecutive intervals with high activity)
   - Promote threads exceeding entry threshold with sufficient streak
   - Demote threads falling below keep threshold
   - Apply CPU sets via SetThreadSelectedCpuSets
   - Optionally boost thread priority (auto or explicit)

4. **Hot Reload**
   - Monitor config file modification time
   - On change, reload and validate
   - If valid, apply new config immediately
   - If invalid, keep previous config and log errors

5. **Process Exit Tracking**
   - When tracked process exits, log top N threads by CPU cycles
   - Resolve thread start addresses to `module.dll+offset` format via `psapi GetMappedFileName`
   - Clean up module cache

## Architecture

```
src/
├── main.rs         - Main loop, process snapshot, apply config
├── cli.rs          - Command-line parsing, help messages
├── config.rs       - Config file parsing, CPU spec parsing, aliases, groups
├── scheduler.rs    - Prime thread scheduler (hysteresis, streak tracking)
├── priority.rs     - Priority enums (Process, Thread, I/O, Memory)
├── process.rs      - Process snapshot via NtQuerySystemInformation
├── winapi.rs       - Windows API wrappers (CPU sets, privileges, module resolution)
└── logging.rs      - Logging to console or file
```

## Limitations

- **CPU Affinity** (SetProcessAffinityMask) only works within one processor group (≤64 cores)
  - Use CPU Sets for >64 core systems
- **I/O Priority** "critical" is kernel-only and not available from user mode
- **High I/O Priority** requires admin elevation
- **Thread start address resolution** requires admin with SeDebugPrivilege
  - Without admin, start addresses show as `0x0`
  - Uses `psapi GetMappedFileName` — no symbol server or internet access needed

## Contributing

Issues and pull requests are welcome.

For developers using AI agents, see project_specific_agent.md for useful CLI tools and bulk editing workflows.

## License

See [LICENSE](LICENSE) file.