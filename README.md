# Affinity Service Rust
<!-- languages -->
- 🇺🇸 [en us](https://github.com/Prohect/AffinityServiceRust/blob/master/README.md)
- 🇨🇳 [中文 (简体)](https://github.com/Prohect/AffinityServiceRust/blob/master/README.zh-CN.md)

A simple app for Windows written in Rust that automatically manages process priority, CPU affinity, Windows CPU sets, thread-level CPU scheduling, I/O priority, and memory priority for specific processes. It reads from a simple configuration file and applies rules continuously.

## Features

- Process Priority Management: Automatically sets priority class (Idle, Below Normal, Normal, Above Normal, High, Real-time)
- CPU Affinity Management: Restricts processes to specific CPU cores using affinity masks (hard limit)
- CPU Set Management: Assigns processes to preferred Windows CPU sets (soft preference)
- Thread-level Prime Core Scheduling: Dynamically identifies and promotes the most active threads to designated high-performance cores
- I/O Priority Management: Controls I/O priority (Very Low, Low, Normal, High with admin)
- Memory Priority Management: Controls memory priority (Very Low, Low, Medium, Below Normal, Normal)
- Timer Resolution Management: Adjusts Windows timer resolution
- Multi-CPU Group Support: Works with systems having >64 logical processors
- Simple Configuration: Easy-to-edit INI file with process rules and CPU range syntax
- Find Unmanaged Processes: Discover which processes could benefit from custom settings
- Process Lasso Compatibility: Convert existing Process Lasso configurations
- Flexible Operation: Run with or without admin privileges; supports console or background mode

Note on affinity vs. CPU sets: CPU affinity is a hard limit on which cores a process may run on (child processes inherit affinity), while Windows CPU sets are a scheduler preference that indicates preferred cores but does not strictly enforce them.

### Thread-level Prime Core Scheduling

For applications with many threads (e.g., games using thread pools), the prime core scheduling feature identifies the most CPU-intensive threads and pins them to designated cores.

How it works:
1. Monitors thread CPU cycle consumption over time
2. Uses entry threshold (default 42% of max) to filter out low-activity threads
3. Uses keep threshold (default 69% of max) to protect already-promoted threads from demotion
4. Requires threads to be consistently active (2+ consecutive intervals) before promotion
5. Reduces unnecessary promote/demote operations to minimize system call overhead

This is useful for:
- Games that use thread pools where the main thread and render thread should have priority access to P-cores
- Avoiding CPU cores 0/1 which often handle hardware interrupts
- Reducing L2 cache thrashing and context switches for critical threads

## Quick Start

1. Download or compile `AffinityServiceRust.exe`
2. Get configuration files (or create one) from this repository:
   - Use the pre-configured [`config.ini`](https://github.com/Prohect/AffinityServiceRust/blob/master/config.ini) as a starting point
   - Use the included [`blacklist.ini`](https://github.com/Prohect/AffinityServiceRust/blob/master/blacklist.ini) for process discovery mode
   - Edit these files to match your CPU layout and preferences
3. Run the application — command-line usage is recommended. You can double-click the .exe to run with default options.

Note: By default the application runs silently in the background and logs activity to `logs\YYYYmmDD.log` and `logs\YYYYmmDD.find.log`. Use the `-console` argument to see real-time output.

> Recommended: run as Administrator to allow changing system/global settings and to use high I/O priority. The `-noUAC` argument can be used to avoid requesting elevated privileges when necessary.

### Basic Usage

```bash
# Use custom config file (may be limited if not run as admin; see '-noUAC')
AffinityServiceRust.exe -config my_config.ini -console
```

### Getting Help

```bash
# Show basic help
AffinityServiceRust.exe -help

# Show detailed help with all options and examples
AffinityServiceRust.exe -helpall
```

### Advanced Usage

Convert Process Lasso config:
```bash
AffinityServiceRust.exe -convert -in prolasso.ini -out my_config.ini
```

Find unmanaged processes:
```bash
# Any process with default affinity and not listed in config or blacklist will be logged
AffinityServiceRust.exe -find
```

## Common Options

| Option | Description |
|--------|-------------|
| `-help` | Show basic help message |
| `-helpall` | Show detailed help with all options and examples |
| `-console` | Output to console instead of log files |
| `-config <file>` | Use custom config file (default: `config.ini`) |
| `-noUAC` | Run without requesting admin privileges |
| `-interval <ms>` | Check interval in milliseconds (default: `5000`) |
| `-loop <count>` | Number of loops to run (default: infinite) - for testing |
| `-logloop` | Log a message at the start of each loop - for testing |
| `-resolution <0.0001ms>` | Timer resolution to set (default: don't change) |

Use `-helpall` to see all available options including conversion and debugging features.

---

## Configuration

### Configuration File Format

Format: `process_name,priority,affinity,cpu_set,prime_cpus,io_priority,memory_priority`

- **process_name**: executable name (e.g., `chrome.exe`)
- **priority**: `none`, `idle`, `below normal`, `normal`, `above normal`, `high`, `real time`
- **affinity**: CPU specification (see below), or `0` to leave unchanged
- **cpu_set**: CPU specification for Windows CPU sets, or `0` to leave unchanged
- **prime_cpus**: CPU specification for thread-level prime core scheduling, or `0` to disable
- **io_priority**: `none`, `very low`, `low`, `normal`, `high` (high requires admin)
- **memory_priority**: `none`, `very low`, `low`, `medium`, `below normal`, `normal`

### CPU Specification Formats

The new CPU specification format supports systems with >64 logical processors:

| Format | Example | Description |
|--------|---------|-------------|
| Hex mask | `0xFF` | Legacy format, cores 0-7 (≤64 cores only) |
| Range | `0-7` | Cores 0 through 7 |
| Multiple ranges | `0-7;64-71` | Cores 0-7 and 64-71 (for >64 core systems) |
| Individual | `0;2;4;6` | Specific cores 0, 2, 4, 6 |
| Alias | `*pcore` | Use predefined alias |
| No change | `0` | Don't modify this setting |

### Defining CPU Aliases

Define aliases once and reuse them throughout your config:

```ini
*pcore = 0-7           # Performance cores
*ecore = 8-19          # Efficiency cores
*all = 0-19            # All cores
*pN0 = 1-7             # P-cores except core 0
*pN01 = 2-7            # P-cores except cores 0-1

# For >64 core systems
*pcore = 0-7;64-71     # P-cores across both CPU groups
*ecore = 8-15;72-79    # E-cores across both CPU groups
```

### Scheduling Constants

Tune the thread scheduling behavior with these constants:

```ini
@HYSTERESIS_RATIO = 1.259  # Hysteresis for thread promotion decisions
@ENTRY_THRESHOLD = 0.42    # Minimum cycles ratio to be considered for promotion
@KEEP_THRESHOLD = 0.69     # Minimum cycles ratio to protect from demotion
```

### Priority Levels

#### I/O Priority

| Level | Value | Status |
|-------|-------|--------|
| `very low` | 0 | ✅ Works |
| `low` | 1 | ✅ Works |
| `normal` | 2 | ✅ Works |
| `high` | 3 | ✅ Requires admin elevation |

Note: `critical` I/O priority is reserved for kernel use and not available from user mode.

#### Memory Priority

| Level | Description |
|-------|-------------|
| `very low` | Pages most likely to be paged out |
| `low` | Low priority for memory retention |
| `medium` | Medium priority |
| `below normal` | Below normal priority |
| `normal` | Default memory priority |

Lower memory priority means pages are more likely to be paged out under memory pressure.

### Example Configuration

```ini
# === SCHEDULING CONSTANTS ===
@ENTRY_THRESHOLD = 0.42
@KEEP_THRESHOLD = 0.69

# === AFFINITY ALIASES ===
*a = 0-19           # All cores (Intel 8P+12E)
*p = 0-7            # Performance cores
*e = 8-19           # Efficiency cores
*pN0 = 1-7          # P-cores except core 0
*pN01 = 2-7         # P-cores except cores 0-1

# === PROCESS CONFIGURATIONS ===
# Format: process_name,priority,affinity,cpuset,prime,io_priority,memory_priority

# Gaming - use prime core scheduling to pin main/render threads to P-cores
cs2.exe,normal,*a,*p,*pN01,normal,normal
game.exe,high,*a,*p,*pN01,normal,normal

# Background apps - efficiency cores, low priority
discord.exe,below normal,*e,0,0,low,low
chrome.exe,normal,*e,0,0,low,below normal

# Work applications
code.exe,above normal,*a,*e,0,normal,normal

# System - high I/O priority (requires admin)
dwm.exe,high,*p,0,0,high,normal
```

### Settings Summary

| Field | Options | Description |
|-------|---------|-------------|
| Priority | `none`, `idle`, `below normal`, `normal`, `above normal`, `high`, `real time` | Process priority class |
| Affinity | `0`, `0xFF`, `0-7`, `*alias` | CPU cores (hex, range, or alias) |
| CPU Set | `0`, `0xFF`, `0-7`, `*alias` | Windows CPU set preference |
| Prime CPU | `0`, `0xFF`, `0-7`, `*alias` | CPUs for thread-level scheduling |
| I/O Priority | `none`, `very low`, `low`, `normal`, `high` | I/O priority (`high` needs admin) |
| Memory Priority | `none`, `very low`, `low`, `medium`, `below normal`, `normal` | Memory page priority |

---

## Debugging

### Quick Debug Commands

**Non-admin (with console output):**
```bash
AffinityServiceRust.exe -console -noUAC -logloop -loop 3 -interval 2000 -config test.ini
```

**Admin elevation (check log file after):**
```bash
AffinityServiceRust.exe -logloop -loop 3 -interval 2000 -config test.ini
# Then check: logs/YYYYMMDD.log
```

> **Important:** When running with admin elevation, do NOT use `-console`. The UAC elevation spawns a new process via PowerShell, and the console window closes immediately after execution. Without `-console`, output goes to log files which persist after the process exits.

See [DEBUG.md](DEBUG.md) for detailed debugging information.

---

## Tips & Notes

- Admin privileges are recommended to manage system processes and use high I/O priority
- Performance impact: minimal CPU and memory usage; default scan interval is 5 seconds
- Logging: generates timestamped logs in the `logs` folder; use `-console` for real-time output
- Process Lasso users: use `-convert` to import existing settings
- For games with thread pools, prime core scheduling can help stabilize frame times
- Use range syntax (`0-7;64-71`) instead of hex masks for systems with >64 cores
- Memory priority affects how aggressively Windows pages out process memory under pressure

## Compile

- Use rustup to install Rust and cargo
- During installation, it asks to install Visual Studio build tools
- By default, only one individual component is selected: MSVC
- That's enough for cargo to build the application
- But if you need rust-analyzer, you will need:
    - MSBuild
    - Windows 11 SDK
- Run `cargo build --release` to compile the application

## Contributing

If you find a bug or have an idea for improvement, feel free to open an issue or submit a pull request.

---