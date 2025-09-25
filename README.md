
----

# Affinity Service Rust

A Windows service written in Rust that automatically manages **process priority**, **CPU affinity**, and **IO priority** for specific applications. It reads from a simple configuration file and applies settings to processes as they run.

## Features

  * **Process Priority Management:** Automatically sets priority class (Idle, Normal, High, etc.)
  * **CPU Affinity Management:** Assigns processes to specific CPU cores using affinity masks
  * **IO Priority Management:** Controls disk I/O priority (Very Low, Low, Normal)
  * **Simple Configuration:** Easy-to-edit INI file with process settings
  * **Process Lasso Compatibility:** Convert existing Process Lasso configurations
  * **Flexible Operation:** Run with or without admin privileges, console or background mode

## Quick Start

1. **Download or compile** the `AffinityServiceRust.exe`
2. **Create or edit** `config.ini` with your process settings (see Configuration section)
3. **Run the application** - double-click the exe or use command line with options

### Basic Usage

```bash
# Run with default settings (uses config.ini)
AffinityServiceRust.exe

# Run with console output (see real-time logs)
AffinityServiceRust.exe -console

# Run without UAC prompts (may limited by privileges if not run as admin)
AffinityServiceRust.exe -console -noUAC

# Use custom config file
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

**Convert Process Lasso config:**
```bash
AffinityServiceRust.exe -convert -in prolasso.ini -out my_config.ini
```

**Find unmanaged processes:**
```bash
AffinityServiceRust.exe -find -console
```
This helps discover which processes could benefit from custom settings.

### Common Options

| Option | Description |
|--------|-------------|
| `-help` | Show basic help message |
| `-helpall` | Show detailed help with all options and examples |
| `-console` | Output to console instead of log file |
| `-config <file>` | Use custom config file (default: config.ini) |
| `-noUAC` | Run without requesting admin privileges |
| `-interval <ms>` | Check interval in milliseconds (default: 5000) |

Use `-helpall` to see all available options including conversion and debugging features.

---

## Configuration

### Configuration File

Create a `config.ini` file with process settings. Format: `process_name,priority,affinity_mask,io_priority`

**Example config.ini:**
```ini
# Gaming - high priority, specific cores, normal IO
game.exe,high,0x0F,normal

# Background apps - low priority, efficiency cores, low IO  
discord.exe,below normal,0xF000,low
steam.exe,below normal,0xF000,very low

# Work apps - normal priority, all cores except first
chrome.exe,normal,0xFFFE,low
code.exe,above normal,0xFFFE,normal

# No changes - just monitor
system_process.exe,none,0,none
```

**Settings Explained:**

| Field | Options | Description |
|-------|---------|-------------|
| **Priority** | `none`, `idle`, `below normal`, `normal`, `above normal`, `high`, `real time` | Process priority class |
| **Affinity** | `0` (no change), `0xFF` (cores 0-7), `0xF000` (cores 12-15) | CPU cores as hex mask |
| **IO Priority** | `none`, `very low`, `low`, `normal` | Disk I/O priority level |

**Tips:**
- Use `none` to skip changing that setting
- Affinity `0xFF` = cores 0-7, `0xF000` = cores 12-15 (efficiency cores on Intel)
- `very low` IO priority for background tasks to reduce system impact
- Run `AffinityServiceRust.exe -helpall` for detailed configuration help

---

## Tips & Notes

- **Admin Privileges:** Recommended for managing system processes, but `-noUAC` option available for limited scenarios
- **Performance Impact:** Minimal CPU usage, only scans processes every 5 seconds by default
- **Logging:** Creates timestamped log files, use `-console` to see real-time output
- **Process Lasso Users:** Use `-convert` to import your existing configurations

## Contributing

Feel free to open an issue or submit a pull request if you find a bug or have an idea for an improvement.

---
