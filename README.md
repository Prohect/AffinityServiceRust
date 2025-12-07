# Affinity Service Rust

<!-- languages -->
- 🇺🇸 [English](https://github.com/Prohect/AffinityServiceRust/blob/master/README.md)
- 🇨🇳 [中文 (简体)](https://github.com/Prohect/AffinityServiceRust/blob/master/README.zh-CN.md)

A Windows process management tool written in Rust that automatically applies CPU affinity, priority, I/O priority, and memory priority rules to processes based on a simple configuration file.

## Features

| Feature | Description |
|---------|-------------|
| **Process Priority** | Set priority class (Idle → Real-time) |
| **CPU Affinity** | Restrict processes to specific cores (hard limit, inherited by child processes, ≤64 cores only) |
| **CPU Sets** | Assign preferred cores via Windows CPU sets (soft preference, works on >64 cores) |
| **Prime Core Scheduling** | Assign most active threads to designated cores (soft preference) |
| **I/O Priority** | Control I/O priority (Very Low → High, High requires admin) |
| **Memory Priority** | Control memory page priority (Very Low → Normal) |
| **Timer Resolution** | Adjust Windows timer resolution |

> **Note on >64 core systems:** CPU affinity (hard limit) only works within a single processor group (≤64 cores). For systems with >64 cores, use CPU Sets instead which work across all processor groups as a soft preference.

### Prime Core Scheduling

For multi-threaded applications (e.g., games), this feature identifies CPU-intensive threads and assigns them to designated cores using Windows CPU sets (soft preference, not hard pinning):

- Monitors thread CPU cycle consumption over time
- Filters low-activity threads (entry threshold: 42% of max)
- Protects promoted threads from premature demotion (keep threshold: 69% of max)
- Requires consistent activity (configurable via `@MIN_ACTIVE_STREAK`, default: 2 intervals) before promotion
- Optionally filters threads by start module name using prefix patterns (syntax: `prime_cpus@prefix1;prefix2`, default: empty matches all)
- Logs thread start address with module resolution (e.g., `ntdll.dll+0x3C320`) when running elevated

Useful for games where main/render threads should prefer P-cores while avoiding cores 0/1 (hardware interrupt handlers).

> **Note:** Thread start address resolution requires admin elevation with SeDebugPrivilege. Without elevation, start addresses show as `0x0`.

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

| Option | Description |
|--------|-------------|
| `-help` | Show basic help |
| `-helpall` | Show detailed help with examples |
| `-console` | Output to console instead of log files |
| `-config <file>` | Use custom config file (default: `config.ini`) |
| `-noUAC` | Run without requesting admin privileges |
| `-interval <ms>` | Check interval in milliseconds (default: `5000`) |
| `-resolution <0.0001ms>` | Set timer resolution |
| `-find` | Log unmanaged processes |
| `-convert` | Convert Process Lasso config |
| `-validate` | Validate config file syntax without running |
| `-dryrun` | Show what would be changed without applying |

## Configuration

### Format

```
process_name,priority,affinity,cpu_set,prime_cpus[@prefixes],io_priority,memory_priority
```

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

> **Important:** Plain numbers like `7` mean core 7, not a bitmask. Use `0x7` or `0-2` for cores 0-2.

### Priority Levels

| Type | Levels |
|------|--------|
| Process | `none`, `idle`, `below normal`, `normal`, `above normal`, `high`, `real time` |
| I/O | `none`, `very low`, `low`, `normal`, `high` (admin required) |
| Memory | `none`, `very low`, `low`, `medium`, `below normal`, `normal` |

### Process Groups

Group multiple processes with the same rule using `{ }` syntax. Group name is optional (for documentation only):

```ini
# Named group (single-line)
browsers { chrome.exe, firefox.exe, msedge.exe },normal,*e,0,0,low,below normal

# Named group (multi-line)
sys_utils {
    # Process Lasso
    bitsumsessionagent.exe, processgovernor.exe, processlasso.exe
    # AffinityService
    affinityservicerust.exe, affinityserverc.exe
    # Comments allowed inside groups
},none,*e,0,0,low,none

# Anonymous group (no name needed)
{
    textinputhost.exe, ctfmon.exe, chsime.exe
    dllhost.exe, sihost.exe, ShellHost.exe
},none,*e,0,0,low,none

# Anonymous single-line
{ taskmgr.exe, perfmon.exe },none,*a,0,0,none,none
```

### Example

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

# === RULES ===
# process,priority,affinity,cpuset,prime[@prefixes],io,memory

# Single process rule
cs2.exe,normal,*a,*p,*pN01,normal,normal

# Prime with module filtering - only threads from modules starting with UnityPlayer.dll or GameModule.dll
game.exe,normal,*a,*p,*pN01@UnityPlayer.dll;GameModule.dll,normal,normal

# Named group - browsers on E-cores
browsers { chrome.exe, firefox.exe, msedge.exe },normal,*e,0,0,low,below normal

# Anonymous group - background apps
{
    discord.exe, telegram.exe, slack.exe
},below normal,*e,0,0,low,low

# System (admin required for high I/O)
dwm.exe,high,*p,0,0,high,normal
```

## Tools

### Process Discovery

Use the `-processlogs` mode to discover new processes from logs that aren't yet in your config or blacklist.

**Requirements:**
- Everything search tool with `es.exe` in PATH
- Log files in `logs/` directory (default, can be specified with `-in`)

**Usage:**
```bash
# Use default settings
AffinityServiceRust.exe -processlogs

# Specify log directory and output file
AffinityServiceRust.exe -processlogs -in mylogs -out results.txt
```

This scans `.find.log` files in the `logs/` directory, extracts process names, filters out configured/blacklisted ones, and searches for the rest using `es.exe`. Results are saved to `new_processes_results.txt`, pairing each process with file paths for easy review and addition to config.

Useful for keeping your config up-to-date with new applications.

> **Note:** You can also use the `process_logs.ps1` PowerShell script for similar functionality.

## Debugging

```bash
# Validate config syntax before running
AffinityServiceRust.exe -validate -config config.ini

# Dry run - see what would be changed without applying
AffinityServiceRust.exe -dryrun -noUAC -config config.ini

# Non-admin with console
AffinityServiceRust.exe -console -noUAC -logloop -loop 3 -interval 2000

# Admin (check logs/YYYYMMDD.log after)
AffinityServiceRust.exe -logloop -loop 3 -interval 2000
```

> When running elevated, avoid `-console` as the UAC spawns a new process and the window closes immediately.

See [DEBUG.md](DEBUG.md) for more details.

For AI agent contributors (Zed, Cursor, etc.), see [AGENT.md](AGENT.md) for CLI tools and workflow tips.

## Building

```bash
# Install Rust via rustup (select MSVC build tools)
cargo build --release
```

For rust-analyzer support, also install MSBuild and Windows 11 SDK.

## Contributing

Issues and pull requests are welcome.

For developers using AI agents, see [AGENT.md](AGENT.md) for useful CLI tools and bulk editing workflows.
