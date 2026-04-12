# AffinityServiceRust

<!-- languages -->
- 🇺🇸 [English](README.md)
- 🇨🇳 [中文 (简体)](README.zh-CN.md)

A high-performance Windows process management service written in Rust that automatically applies CPU affinity, priority, I/O priority, and memory priority rules to running processes based on configuration files.

## Overview

AffinityServiceRust continuously monitors running processes and applies customized scheduling policies based on rules defined in configuration files. It supports:

- **Process Priority Management**: Set process priority class (Idle to Real-time) — see [Priority Levels](#priority-levels)
- **CPU Affinity**: Hard-pin processes to specific logical processors (legacy ≤64 core systems) — see [`apply_affinity()`](docs/en-US/apply.rs/apply_affinity.md)
- **CPU Sets**: Soft CPU preferences across all processor groups (modern >64 core systems) — see [`apply_process_default_cpuset()`](docs/en-US/apply.rs/apply_process_default_cpuset.md)
- **Prime Thread Scheduling**: Dynamically identify and assign CPU-intensive threads to designated "prime" cores — see [Prime Thread Scheduling](#prime-thread-scheduling) section below
- **Ideal Processor Assignment**: Static thread-to-CPU assignment for top N busiest threads — see [Ideal Processor Assignment](#ideal-processor-assignment) section below
- **I/O Priority Control**: Control disk I/O scheduling priority — see [`apply_io_priority()`](docs/en-US/apply.rs/apply_io_priority.md)
- **Memory Priority Control**: Adjust memory page priority for process working set — see [`apply_memory_priority()`](docs/en-US/apply.rs/apply_memory_priority.md)
- **Hot Reload**: Automatically detect and apply config file changes
- **Rule Grades**: Control application frequency per process rule — see [Rule Grades](#rule-grades)

## Documentation

| Topic | Documentation |
|-------|---------------|
| **Architecture** | [docs/main.md](docs/en-US/main.rs/README.md) - Main loop and entry point |
| **Configuration** | [docs/config.md](docs/en-US/config.rs/README.md) - Config parsing and CPU specifications |
| **Apply Logic** | [docs/apply.md](docs/en-US/apply.rs/README.md) - How settings are applied to processes |
| **Scheduler** | [docs/scheduler.md](docs/en-US/scheduler.rs/README.md) - Prime thread scheduler implementation |
| **CLI Options** | [docs/cli.md](docs/en-US/cli.rs/README.md) - Command-line arguments |
| **Priority Levels** | [docs/priority.md](docs/en-US/priority.rs/README.md) - Priority enum definitions |
| **Windows API** | [docs/winapi.md](docs/en-US/winapi.rs/README.md) - Windows API wrappers |
| **Logging** | [docs/logging.md](docs/en-US/logging.rs/README.md) - Error tracking and logging |

## Quick Start

1. Build or download the release binary
2. Download `config.ini` and `blacklist.ini` to your working directory
3. Edit `config.ini` to match your CPU topology (see [Configuration](#configuration) section)
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
| **CPU Affinity** | Legacy mask-based affinity (≤64 cores, [`SetProcessAffinityMask`](docs/en-US/apply.rs/apply_affinity.md)) |
| **CPU Sets** | Modern soft CPU preferences (unlimited cores, [`SetProcessDefaultCpuSets`](docs/en-US/apply.rs/apply_process_default_cpuset.md)) |
| **Prime Thread Scheduling** | Dynamic thread-to-core assignment using hysteresis-based algorithm |
| **Ideal Processor Assignment** | Hysteresis-based ideal-processor assignment using the same algorithm and constants ([`MIN_ACTIVE_STREAK`](docs/en-US/config.rs/ConfigConstants.md), [`ENTRY_THRESHOLD`](docs/en-US/config.rs/ConfigConstants.md), [`KEEP_THRESHOLD`](docs/en-US/config.rs/ConfigConstants.md)) as Prime Thread Scheduling |
| **I/O Priority** | VeryLow, Low, Normal, High (requires admin for High) |
| **Memory Priority** | VeryLow, Low, Medium, BelowNormal, Normal |
| **Timer Resolution** | Configure system timer resolution for tighter loops |
| **Hot Reload** | Auto-reload config when files change |
| **Rule Grades** | Control how often each rule is applied |

### Prime Thread Scheduling

The prime thread scheduler dynamically identifies the most CPU-intensive threads and assigns them to designated "prime" cores using Windows CPU Sets:

**Algorithm:**
- Monitors thread CPU cycle consumption at configurable intervals via [`prefetch_all_thread_cycles()`](docs/en-US/apply.rs/prefetch_all_thread_cycles.md)
- Applies hysteresis to prevent thrashing:
  - **Entry threshold**: Thread must exceed configured % of max cycles to become a candidate
  - **Keep threshold**: Once promoted, thread stays prime if above configured % of max cycles
  - **Active streak**: Requires consecutive active intervals before promotion
- Filters low-activity threads automatically
- Supports multi-segment CPU assignment: different modules can use different core sets
- Per-module thread priority control (explicit or auto-boost)
- Thread tracking mode: logs detailed statistics when process exits

See [`apply_prime_threads()`](docs/en-US/apply.rs/apply_prime_threads.md) and the [scheduler module](docs/en-US/scheduler.rs/README.md) for implementation details.

**Thread Tracking Output:**
When a tracked process exits, detailed statistics are logged for top N threads:
- Thread ID and total CPU cycles consumed
- Start address resolved to `module.dll+offset` format
- Kernel time and user time
- Thread priority, base priority, context switches
- Thread state and wait reason

### Ideal Processor Assignment

An optional `ideal` specification assigns preferred processors to the most CPU-active threads, using the **same hysteresis-based filter** as prime thread scheduling.

**Algorithm:**
- Per-iteration cycle data is provided by the shared [`prefetch_all_thread_cycles()`](docs/en-US/apply.rs/prefetch_all_thread_cycles.md) pass (one `QueryThreadCycleTime` per thread, capped at the logical-CPU count)
- Applies [`MIN_ACTIVE_STREAK`](docs/en-US/config.rs/ConfigConstants.md), [`ENTRY_THRESHOLD`](docs/en-US/config.rs/ConfigConstants.md), and [`KEEP_THRESHOLD`](docs/en-US/config.rs/ConfigConstants.md) constants — identical to prime thread scheduling:
  - **Pass 1 (keep)**: threads already assigned and still above `KEEP_THRESHOLD` retain their slot with **zero write syscalls**
  - **Pass 2 (promote)**: new candidates above `ENTRY_THRESHOLD` whose streak counter has reached `MIN_ACTIVE_STREAK` receive a slot
- Lazy set: if a newly-selected thread's current ideal processor is already in the free CPU pool, `SetThreadIdealProcessorEx` is skipped — the slot is claimed in-place
- Demotion: threads that fall out of selection have their original ideal processor restored
- Each assignment log line includes `start=module+offset` (e.g. `start=cs2.exe+0xEA60`)
- Multi-rule syntax allows different CPU sets for different module prefixes

See [`apply_ideal_processors()`](docs/en-US/apply.rs/apply_ideal_processors.md) for implementation details.

### Ideal Processor Reset

When a process's CPU affinity is changed, AffinityServiceRust automatically resets per-thread ideal processor assignments to prevent Windows from clamping threads to narrow CPU ranges.

This can also be enabled for CPU Set changes by prefixing the cpuset field with `@`:

```ini
# After setting CPU set to 0-3, redistribute thread ideal processors across CPUs 0-3
game.exe:normal:*a:@0-3:*p:normal:normal:1
```

**How it works:**
- Collects threads' total CPU time and sorts in descending order
- Assigns ideal processors round-robin across the configured CPUs
- Applies a small random shift to avoid clumping
- Runs automatically after affinity changes, or after CPU set changes when `@` prefix is used

See [`reset_thread_ideal_processors()`](docs/en-US/apply.rs/reset_thread_ideal_processors.md) for implementation details.

## Configuration

### Config Format

Process rules follow this format:
```
process_name:priority:affinity:cpuset:prime_cpus[@prefixes]:io_priority:memory_priority:ideal[@prefixes]:grade
```

See [`ProcessConfig`](docs/en-US/config.rs/ProcessConfig.md) struct for the parsed representation.

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

See [`parse_cpu_spec()`](docs/en-US/config.rs/parse_cpu_spec.md) for parsing implementation.

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

See [priority.md](docs/en-US/priority.rs/README.md) for all priority level definitions.

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
These constants govern **both** prime thread scheduling and ideal processor assignment — they share the same hysteresis filter.

See [`ConfigConstants`](docs/en-US/config.rs/ConfigConstants.md) for the struct definition.

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
| `-autogroup` | Auto-group rules with identical settings into named groups (`-in <file> -out <file>`) |
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

See [cli.md](docs/en-US/cli.rs/README.md) for complete CLI documentation.

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

### Config Auto-Grouping

Automatically merge rules with identical settings into named group blocks:
```bash
AffinityServiceRust.exe -autogroup -in config.ini -out config_grouped.ini
```

Rules that share the exact same rule string are collected into a `grp_N { }` block, with members sorted alphabetically. Groups that fit within 128 characters are written on a single line; larger groups wrap across multiple lines with `: `-separated members, each line kept under 128 characters.

**Input:**
```ini
explorer.exe:none:*a:*e:0:none:none:0:4
cmd.exe:none:*a:*e:0:none:none:0:4
notepad.exe:none:*a:*e:0:none:none:0:4
```

**Output:**
```ini
grp_0 { cmd.exe: explorer.exe: notepad.exe }:none:*a:*e:0:none:none:0:4
```

The preamble (`@constants`, `*aliases`, and leading comments) is preserved verbatim. Per-process inline comments between rules are dropped during regrouping.

See [`sort_and_group_config()`](docs/en-US/config.rs/sort_and_group_config.md) for implementation.

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

| File | Description | Documentation |
|------|-------------|---------------|
| `src/main.rs` | Main loop, config application logic | [main.md](docs/en-US/main.rs/README.md) |
| `src/config.rs` | Config file parsing, CPU spec parsing, validation | [config.md](docs/en-US/config.rs/README.md) |
| `src/cli.rs` | Command-line argument parsing | [cli.md](docs/en-US/cli.rs/README.md) |
| `src/priority.rs` | Priority enums (Process, Thread, I/O, Memory) | [priority.md](docs/en-US/priority.rs/README.md) |
| `src/logging.rs` | Logging infrastructure (file and console) | [logging.md](docs/en-US/logging.rs/README.md) |
| `src/process.rs` | Process enumeration and snapshot management | [process.md](docs/en-US/process.rs/README.md) |
| `src/scheduler.rs` | Prime thread scheduler implementation | [scheduler.md](docs/en-US/scheduler.rs/README.md) |
| `src/apply.rs` | Config application to processes | [apply.md](docs/en-US/apply.rs/README.md) |
| `src/winapi.rs` | Windows API wrappers, module resolution, privilege handling | [winapi.md](docs/en-US/winapi.rs/README.md) |
| `config.ini` | Default configuration file | - |
| `blacklist.ini` | Default blacklist for process discovery | - |
| `DEBUG.md` | Debug guide and troubleshooting | - |

## How It Works

1. **Startup**: Parse config file via [`read_config()`](docs/en-US/config.rs/read_config.md), request necessary privileges ([`enable_debug_privilege()`](docs/en-US/winapi.rs/enable_debug_privilege.md), [`enable_inc_base_priority_privilege()`](docs/en-US/winapi.rs/enable_inc_base_priority_privilege.md)), optionally elevate to admin; then terminate any child processes inherited from the launcher (e.g. `conhost.exe` attached by a scheduled task runner) before entering the main loop
2. **Main Loop**: 
   - Enumerate all running processes via [`ProcessSnapshot`](docs/en-US/process.rs/ProcessSnapshot.md)
   - Match each process against configured rules
   - Apply settings via [`apply_config()`](docs/en-US/main.rs/apply_config.md): priority, affinity, CPU sets, prime scheduling, and more
   - Sleep for configured interval
3. **Hot Reload**: Monitor config files for changes, automatically reload and reapply
4. **Prime Thread Scheduler**:
   - Track thread cycle time at each interval
   - Apply hysteresis-based promotion/demotion logic via [`PrimeThreadScheduler`](docs/en-US/scheduler.rs/PrimeThreadScheduler.md)
   - Use Windows CPU Sets for fine-grained thread placement
5. **Shared Cycle Prefetch** (per process, before prime and ideal scheduling):
   - Rank all threads by CPU-time delta from `NtQuerySystemInformation` data (no extra syscall)
   - Keep only the top-N threads (N = logical CPU count) — threads below cannot win any assignment slot
   - Open handles and call `QueryThreadCycleTime` for the top-N only; results cached in [`ThreadStats::cached_cycles`](docs/en-US/scheduler.rs/ThreadStats.md)
6. **Ideal Processor Assignment** (per process, per interval):
   - Apply the same hysteresis filter (streak + keep/entry thresholds) as prime thread scheduling
   - Pass 1: already-assigned threads above `keep_threshold` keep their slot — no write syscall
   - Pass 2: promote threads above `entry_threshold` with satisfied streak; skip `SetThreadIdealProcessorEx` if thread is already on a free-pool CPU (lazy set)
   - Demote threads no longer selected: restore original ideal processor

## Known Behaviours

7. **`[SET_AFFINITY][ACCESS_DENIED]` on own child processes**: When this service spawns a child process (e.g. `conhost.exe` attached by a scheduled task runner or a UAC re-launch) that happens to match one of the configured rules, it will attempt to apply affinity and log a line such as:

   ```
   apply_config: [SET_AFFINITY][ACCESS_DENIED]  6976-conhost.exe
   ```

   to `.find.log`. This is **intentional** — the service applies rules to every matching process name it sees in the snapshot, including its own short-lived children. The child is terminated before the main loop starts (see startup cleanup), so this entry will appear at most once per run and can be safely ignored.

8. **`[OPEN][ACCESS_DENIED]` per-PID deduplication**: When [`apply_config()`](docs/en-US/main.rs/apply_config.md) fails to open a process due to `ACCESS_DENIED`, the error is written to `.find.log` exactly once per unique `(pid, process_name)` pair. After each snapshot, the deduplication map is reconciled: entries whose PID has exited or been reused for a different executable are evicted, so if the same process name later re-appears under a new PID the error fires once more. Multiple concurrent instances of the same executable (e.g. several `svchost.exe` processes with different PIDs) are tracked independently — one denied instance never silences errors for any other PID sharing the same name.

See [`is_new_error()`](docs/en-US/logging.rs/is_new_error.md) for error deduplication implementation.

## Known Limitations

1. **CPU Affinity ≤64 cores**: The legacy SetProcessAffinityMask API only works within a single processor group. For >64 core systems, use CPU Sets instead.

2. **Multi-group/NUMA systems**: This project has not been tested on multi-processor-group or NUMA systems. The `ideal` processor assignment currently assigns processors within processor group 0 only. Systems with >64 logical processors or multiple CPU groups may experience unexpected behavior.

3. **Protected processes**: Processes like `csrss.exe` and `smss.exe` cannot be modified, even with admin privileges.

4. **Console output with elevation**: When using `-console` with UAC elevation, the elevated process spawns in a new window that closes immediately. Use log file output instead.

5. **Thread start address resolution**: Requires admin elevation with SeDebugPrivilege. Without elevation, start addresses show as `0x0`.

6. **Timer resolution**: The system timer resolution affects loop precision. Very low values (<1ms) may impact system stability.

## Contributions

Issues and pull requests are welcome.

AI agent developers refer to [project_specific_agent.md](project_specific_agent.md) for useful CLI tools and batch editing workflows.

## License

License [LICENSE](LICENSE).
