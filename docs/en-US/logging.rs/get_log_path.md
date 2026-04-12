# get_log_path function (logging.rs)

Constructs the full file path for a log file by appending the given suffix to the executable's base path and directory.

## Syntax

```rust
pub fn get_log_path(suffix: &str) -> PathBuf
```

## Parameters

`suffix`

A string suffix that determines the type of log file. For example, `".log"` produces the main log file path, and `".find.log"` produces the find log file path. The suffix is appended directly to the executable's file stem.

## Return value

Returns a `PathBuf` representing the absolute path to the log file. The file is placed in the same directory as the running executable, with the executable's name (minus its extension) used as the base name.

For example, if the executable is located at `C:\Tools\AffinityService.exe` and the suffix is `".log"`, the returned path would be `C:\Tools\AffinityService.log`.

## Remarks

This function determines the log file location relative to the running executable rather than using a fixed or configurable directory. This ensures that log files are always co-located with the executable, making them easy to find regardless of the installation directory.

The function uses `std::env::current_exe()` to obtain the path of the running executable, then manipulates the path components to replace the extension with the provided suffix.

This function is called during lazy initialization of the [`LOG_FILE`](LOG_FILE.md) and [`FIND_LOG_FILE`](FIND_LOG_FILE.md) statics. It is not typically called directly by other modules — instead, other modules interact with the log files through [`log_message`](log_message.md), [`log_pure_message`](log_pure_message.md), [`log_to_find`](log_to_find.md), and [`log_process_find`](log_process_find.md).

### Error handling

If `std::env::current_exe()` fails (which is extremely rare on Windows), the function may panic during static initialization. This is acceptable because logging is fundamental to the application's operation and cannot be meaningfully degraded.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/logging.rs |
| **Source lines** | L174–L183 |
| **Called by** | [`LOG_FILE`](LOG_FILE.md) initialization, [`FIND_LOG_FILE`](FIND_LOG_FILE.md) initialization |
| **Dependencies** | `std::env::current_exe`, `std::path::PathBuf` |

## See also

- [LOG_FILE static](LOG_FILE.md)
- [FIND_LOG_FILE static](FIND_LOG_FILE.md)
- [log_message function](log_message.md)
- [logging.rs module overview](README.md)