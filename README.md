# Affinity Service Rust
<!-- languages -->
- 🇺🇸 [en us](https://github.com/Prohect/AffinityServiceRust/blob/master/README.md)
- 🇨🇳 [中文 (简体)](https://github.com/Prohect/AffinityServiceRust/blob/master/README.zh-CN.md)

A simple app for Windows written in Rust that automatically manages process priority, CPU affinity, Windows CPU sets, thread-level CPU scheduling, and I/O priority for specific processes.  It reads from a simple configuration file and applies rules continuously.

## Features

- Process Priority Management: Automatically sets priority class (Idle, Below Normal, Normal, Above Normal, High, Real-time)
- CPU Affinity Management: Restricts processes to specific CPU cores using affinity masks (hard limit)
- CPU Set Management: Assigns processes to preferred Windows CPU sets (soft preference)
- Thread-level Prime Core Scheduling: Dynamically identifies and promotes the most active threads to designated high-performance cores
- I/O Priority Management: Controls I/O priority (Very Low, Low, Normal)
- Timer Resolution Management: Adjusts Windows timer resolution
- Simple Configuration: Easy-to-edit INI file with process rules
- Find Unmanaged Processes: Discover which processes could benefit from custom settings and log what has run on your system
- Process Lasso Compatibility: Convert existing Process Lasso configurations to Affinity Service Rust format
- Flexible Operation: Run with or without admin privileges; supports console or background mode

Note on affinity vs.  CPU sets: CPU affinity is a hard limit on which cores a process may run on (child processes inherit affinity), while Windows CPU sets are a scheduler preference that indicates preferred cores but does not strictly enforce them. 

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
   - Use the pre-configured [`config.ini`](https://github.com/Prohect/AffinityServiceRust/blob/master/config.ini) as a starting point (covers 200+ common processes)
   - Use the included [`blacklist.ini`](https://github.com/Prohect/AffinityServiceRust/blob/master/blacklist.ini) for process discovery mode
   - Edit these files to match your CPU layout and preferences
3.  Run the application — command-line usage is recommended.  You can double-click the . exe to run with default options.

Note: By default the application runs silently in the background and logs activity to `logs\YYYYmmDD. log` and `logs\YYYYmmDD. find. log`. Use the `-console` argument to see real-time output. 

> Recommended: run as Administrator to allow changing system/global settings.  The `-noUAC` argument can be used to avoid requesting elevated privileges when necessary.

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
# Any process with default affinity and not listed in config or blacklist will be logged to logs\YYYYmmDD.find.log
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
| `-resolution <0. 0001ms>` | Timer resolution to set (default: don't change) |

Use `-helpall` to see all available options including conversion and debugging features.

---

## Configuration

### Configuration File Format

Format: `process_name,priority,affinity_mask,cpu_set_mask,prime_cpu_mask,io_priority`

- process_name: executable name (e.g., `chrome.exe`)
- priority: one of `none`, `idle`, `below normal`, `normal`, `above normal`, `high`, `real time`
- affinity_mask: CPU affinity mask as hex (e.g., `0xFF`) or alias (e.g., `*pcore`), or `0` to leave unchanged
- cpu_set_mask: CPU set mask (same format as affinity), or `0` to leave unchanged
- prime_cpu_mask: CPU mask for thread-level prime core scheduling, or `0` to disable
- io_priority: one of `none`, `very low`, `low`, `normal`

### Scheduling Constants

You can tune the thread scheduling behavior with these constants in config:

```ini
@ENTRY_THRESHOLD = 0.42    # Minimum cycles ratio to be considered for promotion
@KEEP_THRESHOLD = 0.69     # Minimum cycles ratio to protect from demotion
```

Example `config.ini`:
```ini
# === SCHEDULING CONSTANTS ===
@ENTRY_THRESHOLD = 0. 42
@KEEP_THRESHOLD = 0.69

# === AFFINITY ALIASES ===
# Define aliases once and reuse; change aliases if CPU topology changes
*a = 0xFFFFF        # All cores
*p = 0xFF           # Performance cores 0-7
*e = 0xFFF00        # Efficiency cores 8-19
*pN0 = 0xFE         # P-cores except core 0
*pN01 = 0xFC        # P-cores except cores 0-1

# === PROCESS CONFIGURATIONS ===
# Columns: process_name,priority,affinity_mask,cpu_set_mask,prime_cpu_mask,io_priority

# Gaming - use prime core scheduling to pin main/render threads to P-cores (avoiding core 0/1)
cs2.exe,normal,*a,*p,*pN01,normal
game.exe,high,*a,*p,*pN01,normal

# Background apps - efficiency cores, low I/O priority
discord.exe,below normal,*e,0,0,low
chrome.exe,normal,*e,0,0,low

# Work applications
code.exe,above normal,*a,*e,0,normal
```

### Settings Explained

| Field | Options | Description |
|-------|---------|-------------|
| Priority | `none`, `idle`, `below normal`, `normal`, `above normal`, `high`, `real time` | Process priority class |
| Affinity | `0`, `0xFF`, `*alias_name` | CPU cores as hex mask, decimal, or alias (`0` = no change) |
| CPU set | `0`, `0xFF`, `*alias_name` | Windows CPU set mask for process (same format) |
| Prime CPU | `0`, `0xFF`, `*alias_name` | CPU mask for thread-level scheduling (`0` = disabled) |
| I/O Priority | `none`, `very low`, `low`, `normal` | I/O priority level |

Affinity options:
- Direct values: `0xFF` (cores 0-7), `0xF000` (cores 12-15), also supports decimal like `255`
- Aliases: define with `*name = 0xFF` and refer as `*name`
- `0`: do not change current value

Tips:
- Best practice: use aliases for cleaner, maintainable configs
- Quick setup: download the pre-configured [`config.ini`](https://github.com/Prohect/AffinityServiceRust/blob/master/config.ini) and [`blacklist.ini`](https://github. com/Prohect/AffinityServiceRust/blob/master/blacklist.ini) from the repository
- When upgrading CPU, change the alias definitions once to update all rules
- Use `none` or `0` to skip changing a particular setting
- For games, consider using prime_cpu_mask to avoid cores 0/1 which handle interrupts
- Run `AffinityServiceRust.exe -helpall` for detailed configuration help and alias examples

### Using Repository Configuration Files

Quick setup:
1.  Download [`config.ini`](https://github.com/Prohect/AffinityServiceRust/blob/master/config.ini) and [`blacklist.ini`](https://github.com/Prohect/AffinityServiceRust/blob/master/blacklist.ini) from this repository
2. Edit affinity aliases in `config.ini` to match your CPU topology:
```ini
# Intel 8P+12E (e.g., 14700KF)
*p = 0xFF          # cores 0-7
*e = 0xFFF00       # cores 8-19

# Intel 6P+8E
*p = 0x3F          # cores 0-5
*e = 0x3FC0        # cores 6-13
```
3. Place files in the same folder as `AffinityServiceRust.exe`
4. Run the application

Benefits:
- Instant optimization for many common applications
- Tested configurations for broad compatibility
- Easy customization by editing aliases
- Maintainable configs — update CPU topology in one place
- Community-maintained rules that can improve over time

---

## Tips & Notes

- Admin privileges are recommended to manage system processes; `-noUAC` is available for limited scenarios
- Performance impact: minimal CPU and memory usage; default scan interval is 5 seconds (configurable)
- Logging: generates timestamped logs in the `logs` folder; use `-console` for real-time output
- Process Lasso users: use `-convert` to import existing settings
- For games with thread pools, prime core scheduling can help stabilize frame times by keeping critical threads on fast cores

## Compile

- You can use rustup to install Rust and cargo
- During installation, it asks to install Visual Studio build tools
- By default, only one individual component is selected: MSVC
- That's enough for cargo to build the application
- But if you need rust-analyzer, you will need the following components:
    - MSBuild
    - Windows 11 SDK
- Run `cargo build --release` to compile the application

## Contributing

If you find a bug or have an idea for improvement, feel free to open an issue or submit a pull request. 

---
