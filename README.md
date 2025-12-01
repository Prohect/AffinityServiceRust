
----

# Affinity Service Rust

A simple app for windows written in Rust that automatically manages **process priority**, **CPU affinity**, **CPU sets**, and **IO priority** for specific process. It reads from a simple configuration file and applies settings by scan process list with certain rate.

## Features

  * **Process Priority Management:** Automatically sets priority class (Idle,Below Normal, Normal, Above Normal, High, Realtime)
  * **CPU Affinity Management:** Assigns processes to limit(hard) CPU cores using affinity masks
  * **CPU Set Management:** Assigns windows to prefer(soft) CPU sets for a specific process
  * **IO Priority Management:** Controls I/O priority (Very Low, Low, Normal)
  * **Timer resolution Management:** Assigns timer resolution for windows
  * **Simple Configuration:** Easy-to-edit INI file with process settings
  * **Find unmanaged processes:** Discover which processes could benefit from custom settings. And it could tell u what's been run on your system.
  * **Process Lasso Compatibility:** Convert existing Process Lasso configurations to Affinity Service Rust format
  * **Flexible Operation:** Run with or without admin privileges, console or background mode
  
  Difference between CPU affinity and CPU set: CPU affinity is a hard limit on which cores a process can run on, if a process creates a son process then the son process will inherit the affinity, while CPU set is a new concept from Windows 10 version 1709 (Fall Creators Update), it allows a process to run on a subset of available cores, but it does not guarantee that the process will run on those cores. CPU set is a soft preference, meaning that the operating system may still schedule the process on other cores if necessary.And CPU set will not be inherited by son process.

## Quick Start

1. **Download or compile** the `AffinityServiceRust.exe`
2. **Get configuration files (or Create one on your own)** from this repository:
   - Use the pre-configured `config.ini` as a starting point (covers 200+ common processes)
   - Use the included `blacklist.ini` for process discovery mode
   - Edit these files to match your CPU configuration and needs
3. **Run the application** - use command line(recommand) with args or double-click the exe to run with default args

   **Note:** By default, the application runs silently in the background and logs its activity to files (`logs\YYYYmmDD.log`,`logs\YYYYmmDD.find.log`). Use the `-console` arg to see real-time output.

### Basic Usage

```bash

# Use custom config file (may limited by privileges if not run as admin with arg '-noUAC')
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
# any process with default affinity and not in config nor blacklist will be found and output to logs\YYYYmmDD.find.log
AffinityServiceRust.exe -find
```
This helps discover which processes could benefit from custom settings. And it could tell u what's been run on your system.

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

**Format:** `process_name,priority,affinity_mask,cpu_set_mask,io_priority`

**Example config.ini:**
```ini
# === AFFINITY ALIASES === (Define once, use everywhere, only need to change definitions of aliases when cpu type changes)
*pcore = 0xFF          # Performance cores 0-7
*ecore = 0xFFF00       # Efficiency cores 8-19
*pcore_no0 = 0xFE      # P-cores except core 0
*allcores = 0xFFFFF    # All available cores

# === PROCESS CONFIGURATIONS ===
# Gaming - high priority, performance cores
game.exe,high,*pcore,normal
steam.exe,below normal,*pcore_no0,low

# Background apps - efficiency cores, low priority
discord.exe,below normal,*ecore,low
chrome.exe,normal,*ecore,low

# Work applications - mixed configurations
code.exe,above normal,*ecore,normal
notepad.exe,normal,*pcore,none

# Traditional hex values still work
system_process.exe,none,0xFF,none
```

**Settings Explained:**

| Field | Options | Description |
|-------|---------|-------------|
| **Priority** | `none`(no change), `idle`, `below normal`, `normal`, `above normal`, `high`, `real time` | Process priority class |
| **Affinity** | `0` (no change), `0xFF`, `*alias_name` | CPU cores as hex mask or alias |
| **CPU set** | `0` (no change), `0xFF`, `*alias_name` | CPU cores as hex mask or alias |
| **IO Priority** | `none`, `very low`, `low`, `normal` | I/O priority level |

**Affinity Options:**
- **Direct values**: `0xFF` (cores 0-7), `0xF000` (cores 12-15), `255` (decimal)
- **Aliases**: `*pcore`, `*ecore`, `*allcores` (define with `*name = 0xFF`)
- **Zero**: `0` means no change to current affinity

**Tips:**
- **Best Practice:** Use aliases for cleaner, maintainable configs
- **Quick Setup:** Download the pre-configured [`config.ini`](https://github.com/Prohect/AffinityServiceRust/blob/master/config.ini) and pre-blacklist [`blacklist.ini`](https://github.com/Prohect/AffinityServiceRust/blob/master/blacklist.ini) from this  repository - it includes settings for 200+ common applications and serves as an excellent starting point. Or create your own configuration file manually.`blacklist.ini` contains some system processes that -find should exclude.
- **CPU Migration:** Change aliases once to update all settings when upgrading CPU
- Use `none` to skip changing that setting
- Run `AffinityServiceRust.exe -helpall` for detailed configuration help and alias examples

### Using Repository Configuration Files

**Quick Setup Steps:**
1. Download [`config.ini`](https://github.com/Prohect/AffinityServiceRust/blob/master/config.ini) and [`blacklist.ini`](https://github.com/Prohect/AffinityServiceRust/blob/master/blacklist.ini) from the repository
2. **Edit the affinity aliases** in `config.ini` for your CPU:
   ```ini
   # Intel 8P+12E (like 14700KF)
   *pcore = 0xFF          # Cores 0-7
   *ecore = 0xFFF00       # Cores 8-19

   # Intel 6P+8E
   *pcore = 0x3F          # Cores 0-5
   *ecore = 0x3FC0        # Cores 6-13

   # AMD or custom - adjust based on your CPU
   ```
3. Place files in the same folder as `AffinityServiceRust.exe`
4. Run and enjoy optimized system performance!

**Benefits:**
- **Instant optimization** for hundreds of common applications
- **Tested configurations** that work well for most systems
- **Easy customization** - just edit the alias definitions for your CPU
- **Maintainable configs** - change CPU setup once, applies everywhere
- **Community maintained** - configurations improve over time

---

## Tips & Notes

- **Admin Privileges:** Recommended for managing system processes, but `-noUAC` option available for limited scenarios
- **Performance Impact:** Minimal CPU usage and memory usage, only scans processes every 5 seconds by default. Rate configurable
- **Logging:** Creates timestamped log files, or use `-console` to see real-time output
- **Process Lasso Users:** Use `-convert` to import your existing configurations

## Contributing

Feel free to open an issue or submit a pull request if you find a bug or have an idea for an improvement.

---
