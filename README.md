
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
2. **Get configuration files (or Create one on your own)** from the [GitHub repository](https://github.com/Prohect/AffinityServiceRust/blob/master/config.ini):
   - Use the pre-configured `config.ini` as a starting point (covers 200+ common processes)
   - Use the included `blacklist.ini` for process discovery mode
   - Edit these files to match your CPU configuration and needs
3. **Run the application** - double-click the exe or use command line with options

   **Note:** By default, the application runs silently in the background and logs its activity to a file (`logs\YYYYmmDD.log`). Use the `-console` flag to see real-time output.

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

**Getting Started:** Download the pre-configured `config.ini` from the [GitHub repository](https://github.com/Prohect/AffinityServiceRust/blob/master/config.ini) - it includes settings for 200+ common applications and serves as an excellent starting point. Or create your own configuration file all manually.

**Format:** `process_name,priority,affinity_mask,io_priority`

**New: Affinity Aliases** - Define reusable affinity masks for easier configuration management:

```ini
# Define aliases first (lines starting with *)
*pcore = 0xFF          # Performance cores 0-7
*ecore = 0xFFF00       # Efficiency cores 8-19
*allcores = 0xFFFFF    # All available cores

# Use aliases in configurations
game.exe,high,*pcore,normal
discord.exe,below normal,*ecore,low
```

**Example config.ini:**
```ini
# === AFFINITY ALIASES === (Define once, use everywhere)
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
| **Priority** | `none`, `idle`, `below normal`, `normal`, `above normal`, `high`, `real time` | Process priority class |
| **Affinity** | `0` (no change), `0xFF`, `*alias_name` | CPU cores as hex mask or alias |
| **IO Priority** | `none`, `very low`, `low`, `normal` | Disk I/O priority level |

**Affinity Options:**
- **Direct values**: `0xFF` (cores 0-7), `0xF000` (cores 12-15), `255` (decimal)
- **Aliases**: `*pcore`, `*ecore`, `*allcores` (define with `*name = 0xFF`)
- **Zero**: `0` means no change to current affinity

**Tips:**
- **Best Practice:** Use aliases for cleaner, maintainable configs
- **Quick Setup:** Download `config.ini` from the [repository](https://github.com/Prohect/AffinityServiceRust/blob/master/config.ini) and adjust aliases for your CPU
- **CPU Migration:** Change aliases once to update all processes when upgrading CPU
- Use `none` to skip changing that setting
- `very low` IO priority for background tasks to reduce system impact
- For process discovery, also download `blacklist.ini` to exclude system processes
- Run `AffinityServiceRust.exe -helpall` for detailed configuration help and alias examples

### Using Repository Configuration Files

** Pre-configured Files Available:**
The [GitHub repository](https://github.com/Prohect/AffinityServiceRust/blob/master/config.ini) includes ready-to-use configuration files:

- **`config.ini`** - Pre-configured settings for 200+ common applications including:
  - Games (Steam, Epic, individual game executables)
  - Development tools (VS Code, IDEs, compilers)
  - Creative apps (Adobe suite, video editors)
  - System utilities (browsers, Discord, etc.)
  - Background processes with optimized priority/affinity

- **`blacklist.ini`** - Excludes system processes from `-find` mode discovery

**Quick Setup Steps:**
1. Download `config.ini` and `blacklist.ini` from the repository
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
- **Performance Impact:** Minimal CPU usage, only scans processes every 5 seconds by default
- **Logging:** Creates timestamped log files, use `-console` to see real-time output
- **Process Lasso Users:** Use `-convert` to import your existing configurations

## Contributing

Feel free to open an issue or submit a pull request if you find a bug or have an idea for an improvement.

---
