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

Before running, make sure u have a config file for it, u may use -convert command on this program to transform one from processlasso's config or download the example config from this project and edit it youself since different cpu has different conditions.
To run the service,simply double click it(it requires no args and could run with its builtin default values) if you already have config for this program, or make a .bat or open a command prompt and execute `AffinityServiceRust.exe` with desired arguments.
However, it a good idea to set a schduled task for this program to run automatically since windows will drops its chance to get cpu if you or its console dont send anything to it, 
under that condition u may find in log file OMG this program sleeps half an hour during which the newly created processes are not managed by it.

#### Example Commands

  * **Run with a custom configuration file:**
    ```bash
    AffinityServiceRust.exe -config my_custom_config.txt -console
    ```
    `the config is "config.ini" by default, u dont have to set it`
    `the program logs stuff to a log file by default, unless u run it with "-console"`
  * **Convert a Process Lasso configuration file:**
    ```bash
    AffinityServiceRust.exe -convert -in prolasso.ini -out my_new_config.ini
    ```
  * **Find and log processes with default affinity:**
    ```bash
    AffinityServiceRust.exe -find -blacklist no_change.txt -interval 16000
    ```
  * **Print help information:**
    ```bash
    AffinityServiceRust.exe -help
    ```
    `"-?", "?", "--help". All of them do same thing as above`

## Configuration

### `config.ini`

The service reads from `config.ini` by default. Each line in the file represents a process configuration and should be formatted as follows:

```
process_name,priority,affinity_mask
```

  * `process_name`: The name of the process executable (e.g., `game.exe`).
  * `priority`: The desired process priority. Possible values are: `none`, `idle`, `below normal`, `normal`, `above normal`, `high`, `real time`.`none` means the program wont take care of it
  * `affinity_mask`: A hexadecimal value representing the CPU affinity mask (e.g., `0xFFFE`). Any value equals `0` means the program wont take care of it. 

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
