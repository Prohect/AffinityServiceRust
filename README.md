----

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
  * **Rust:** You need to have Rust and Cargo or an IDE with them installed if you are a developer who wants to edit and compile it yourself.

### Running

Before running, make sure you have a config file for it. You may use the `-convert` command on this program to transform one from Process Lasso's config or download the example config from this project and **edit it yourself** since different CPUs have different conditions.  
To run the service, simply double-click it (it requires no args and can run with its built-in default values) if you already have a config for this program, or make a `.bat` or open a command prompt and execute `AffinityServiceRust.exe` with desired arguments.  
However, it is a good idea to set a scheduled task for this program to run automatically since Windows will drop its chance to get CPU if you or its console don't send anything to it. Under that condition, you may find in the log file that this program sleeps for half an hour during which newly created processes are not managed by it.

#### Example Commands

  * **Run with a custom configuration file:**
    ```bash
    AffinityServiceRust.exe -config my_custom_config.txt -console
    ```
    `The config is "config.ini" by default, you don't have to set it.`  
    `The program logs to a log file by default, unless you run it with "-console".`

  * **Convert a Process Lasso configuration file:**
    ```bash
    AffinityServiceRust.exe -convert -in prolasso.ini -out my_new_config.ini
    ```

  * **Find and log processes with default affinity:**
    ```bash
    AffinityServiceRust.exe -find -blacklist no_change.txt -interval 16000
    ```
    This command scans all running processes and logs those that are **not listed in your config file** and are currently using the **default system affinity mask** (i.e., all cores).  
    It's useful for discovering processes that could benefit from custom affinity or priority settings.

    **How to use this to improve your config:**
    1. Run the above command and let it log for a while.
    2. Open the log file and look for entries like:
       ```
       [12:36:53]find: [default_affinity] 11932-git.exe
       ```
    3. If you see a process you want to manage, add a line to `config.ini`:
       ```
       git.exe,below normal,0x0F
       ```
       This example sets `git.exe` to below normal priority and restricts it to cores 0–3.
    4. Repeat until your config covers all relevant processes.
    5. rerun this programme

    **Note:** The `-blacklist` file is used to exclude known system processes or anything you don't want to manage. Each line should be a process name like:
    ```txt
    explorer.exe
    ```

  * **Print help information:**
    ```bash
    AffinityServiceRust.exe -help
    ```
    `"-?", "?", "--help" all do the same thing as above.`

## Configuration

### `config.ini`

The service reads from `config.ini` by default. Each line in the file represents a process configuration and should be formatted as follows:

```
process_name,priority,affinity_mask
```

  * `process_name`: The name of the process executable (e.g., `game.exe`).
  * `priority`: The desired process priority. Possible values are: `none`, `idle`, `below normal`, `normal`, `above normal`, `high`, `real time`. `none` means the program won't take care of it.
  * `affinity_mask`: A hexadecimal value representing the CPU affinity mask (e.g., `0xFFFE`). Any value equal to `0` means the program won't take care of it.

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

----
