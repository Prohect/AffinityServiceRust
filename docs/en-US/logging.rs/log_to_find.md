# log_to_find function (logging.rs)

Writes a timestamped message to the `.find` log file or to stdout, depending on the current console mode setting. This function is the dedicated logging channel for `-find` mode output and diagnostic messages related to process discovery and affinity inspection.

## Syntax

```rust
pub fn log_to_find(msg: &str)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `msg` | `&str` | The message string to write. A timestamp prefix in `[HH:MM:SS]` format is automatically prepended. |

## Return value

This function does not return a value.

## Remarks

The function operates as follows:

1. Acquires the `LOCAL_TIME_BUFFER` mutex (via `get_local_time!()`) and formats the current time as `HH:MM:SS`.
2. Checks the `USE_CONSOLE` flag (via `get_use_console!()`):
   - If `true`, writes the timestamped message to **stdout** using `writeln!`.
   - If `false`, writes the timestamped message to the **find log file** (`FIND_LOG_FILE`) using `writeln!` via the `get_logger_find!()` macro.
3. Write errors are silently ignored (`let _ = writeln!(...)`).

### Differences from related logging functions

| Function | Log destination | Timestamp | Dust-bin mode check |
|----------|----------------|-----------|---------------------|
| [`log_message`](log_message.md) | Main log file (`LOG_FILE`) or stdout | Yes (`[HH:MM:SS]`) | Yes — suppresses output when enabled |
| [`log_pure_message`](log_pure_message.md) | Main log file (`LOG_FILE`) or stdout | No | No |
| **`log_to_find`** | Find log file (`FIND_LOG_FILE`) or stdout | Yes (`[HH:MM:SS]`) | **No** — always writes regardless of dust-bin mode |

Unlike [`log_message`](log_message.md), this function does **not** check the `DUST_BIN_MODE` flag. Find-mode diagnostics are always written, even when the main log output is suppressed in dust-bin mode. This ensures that process discovery results and access-denied diagnostics are never silently discarded.

### Output format

```text
[14:32:07]find chrome.exe
[14:32:07]is_affinity_unset: [OPEN][ACCESS_DENIED]  1234-svchost.exe
```

### Log file

The find log file is a date-stamped file with a `.find` suffix, created under the `logs/` directory. Its path is determined by [`get_log_path`](get_log_path.md) with `".find"` as the suffix argument (e.g., `logs/20250101.find.log`). The file handle is stored in the [`FIND_LOG_FILE`](statics.md#find_log_file) static and opened in append mode on first access.

### Thread safety

The function acquires up to two mutex locks per call (`LOCAL_TIME_BUFFER` and either `USE_CONSOLE` + stdout or `FIND_LOG_FILE`). Lock acquisition order is consistent across all logging functions, preventing deadlocks.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `logging.rs` |
| **Callers** | [`is_affinity_unset`](../winapi.rs/is_affinity_unset.md), [`get_process_handle`](../winapi.rs/get_process_handle.md), [`get_thread_handle`](../winapi.rs/get_thread_handle.md), [`log_process_find`](log_process_find.md), `CPU_SET_INFORMATION` initializer, and other find-mode diagnostics in `winapi.rs` and `apply.rs` |
| **Callees** | `get_local_time!()`, `get_use_console!()`, `get_logger_find!()`, `std::io::Write::write_fmt` (via `writeln!`) |
| **Statics** | [`LOCAL_TIME_BUFFER`](statics.md#local_time_buffer), [`USE_CONSOLE`](statics.md#use_console), [`FIND_LOG_FILE`](statics.md#find_log_file) |
| **Platform** | Windows (logging infrastructure), but the function itself contains no platform-specific code |

## See Also

| Topic | Link |
|-------|------|
| log_message function | [log_message](log_message.md) |
| log_pure_message function | [log_pure_message](log_pure_message.md) |
| log_process_find function | [log_process_find](log_process_find.md) |
| get_log_path function | [get_log_path](get_log_path.md) |
| logging statics | [statics](statics.md) |
| logging module overview | [README](README.md) |

---
*Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
