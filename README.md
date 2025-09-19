-----

# Affinity Service Rust

This is a Windows service written in Rust that allows you to manage the **process priority** and **CPU affinity** for specific applications. It can be configured to automatically set these values for processes as they run, and it also includes a utility for converting configuration files from other applications like Process Lasso.

## Features

  * **Process Priority Management:** Automatically sets the priority class for specified processes (e.g., Idle, High, Realtime).
  * **CPU Affinity Management:** Assigns a specific set of CPU cores to a process using an affinity mask.
  * **Configuration:** Reads settings from a simple, comma-separated configuration file.
  * **Conversion Utility:** Converts configuration files from a different format (specifically, the one used by Process Lasso) into a format this service can use.
  * **Process Monitoring:** Can identify and log processes that are running with the default system CPU affinity.

## Getting Started

### Prerequisites

  * **Windows:** This service is built using the Windows API and is designed to run on Windows.
  * **Rust:** You need to have Rust and Cargo or a IDE with them installed if u r a devloper who wants to edit and compile it by yourself.

### Running

To run the service, open a command prompt and execute the `AffinityServiceRust.exe` file with the desired arguments.

#### Example Commands

  * **Run with a custom configuration file:**
    ```bash
    AffinityServiceRust.exe -config my_custom_config.txt
    ```
  * **Convert a Process Lasso configuration file:**
    ```bash
    AffinityServiceRust.exe -convert -in "ProcessLasso.config" -out "my_new_config.ini"
    ```
  * **Find and log processes with default affinity:**
    ```bash
    AffinityServiceRust.exe -find -blacklist no_change.txt
    ```
  * **Print help information:**
    ```bash
    AffinityServiceRust.exe -help
    ```

## Configuration

### `config.ini`

The service reads from a `config.ini` file by default. Each line in the file represents a process configuration and should be formatted as follows:

```
process_name,priority,affinity_mask
```

  * `process_name`: The name of the process executable (e.g., `game.exe`).
  * `priority`: The desired process priority. Possible values are: `none`, `idle`, `below normal`, `normal`, `above normal`, `high`, `real time`.
  * `affinity_mask`: A hexadecimal value representing the CPU affinity mask (e.g., `0xAA`). A value of `0` means no change.

**Example `config.ini`:**

```ini
# This is an example configuration file
discord.exe,below normal,0
game.exe,high,0x0A
video_editor.exe,high,0xAA
```

### `blacklist.txt` (for `-find` mode)

This is a simple text file with a list of process names, one per line, that you want to exclude from the `-find` mode's logging.

**Example `blacklist.txt`:**

```txt
# Do not log these processes
explorer.exe
```

## Contributing

Feel free to open an issue or submit a pull request if you find a bug or have an idea for an improvement.
