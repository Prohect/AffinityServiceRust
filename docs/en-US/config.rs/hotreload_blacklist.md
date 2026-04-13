# hotreload_blacklist function (config.rs)

Checks the blacklist file's last-modified timestamp and reloads it into memory if the file has been modified since the last check. If the file becomes inaccessible (e.g., deleted or locked), the in-memory blacklist is cleared. This function is called once per service loop iteration to support live updates to the blacklist without restarting the service.

## Syntax

```rust
pub fn hotreload_blacklist(
    cli: &CliArgs,
    blacklist: &mut Vec<String>,
    last_blacklist_mod_time: &mut Option<std::time::SystemTime>,
)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `cli` | `&CliArgs` | Reference to the parsed command-line arguments. The `blacklist_file_name` field (`Option<String>`) provides the file path to monitor. If `None`, the function returns immediately without performing any work. |
| `blacklist` | `&mut Vec<String>` | Mutable reference to the live blacklist vector maintained by the service loop. On reload, this vector is replaced with the freshly parsed entries from [read_list](read_list.md). On file-inaccessible, it is cleared. |
| `last_blacklist_mod_time` | `&mut Option<std::time::SystemTime>` | Mutable reference to the cached modification timestamp from the previous check. Updated to `Some(mod_time)` after a successful reload, or reset to `None` when the file becomes inaccessible. Used to detect whether the file has changed since the last poll. |

## Return value

This function does not return a value. All output is communicated through mutations to `blacklist` and `last_blacklist_mod_time`.

## Remarks

### Reload algorithm

1. **No blacklist configured** — If `cli.blacklist_file_name` is `None`, the function returns immediately. No file monitoring occurs.
2. **File metadata check** — `std::fs::metadata` is called on the blacklist file path.
   - **Metadata error (file inaccessible):** If `last_blacklist_mod_time` was previously `Some(...)`, the blacklist is cleared, `last_blacklist_mod_time` is set to `None`, and a log message is emitted: `"Blacklist file '{path}' no longer accessible, clearing blacklist."`. If `last_blacklist_mod_time` was already `None`, no action is taken (avoids repeated log messages on each loop iteration).
   - **Metadata success:** The file's modification time is compared against `*last_blacklist_mod_time`.
3. **Modification time comparison** — If `Some(mod_time) != *last_blacklist_mod_time` (i.e., the file is new or has been modified), a reload is triggered:
   a. `last_blacklist_mod_time` is updated to `Some(mod_time)`.
   b. A log message is emitted: `"Blacklist file '{path}' changed, reloading..."`.
   c. [read_list](read_list.md) is called to parse the file. On error, `unwrap_or_default()` produces an empty vector.
   d. The result replaces the contents of `*blacklist`.
   e. A completion log message is emitted: `"Blacklist reload complete: {N} items loaded."`.
4. **No change** — If the modification time matches the cached value, the function returns without any action.

### First-call behavior

On the first call, `last_blacklist_mod_time` is typically `None` (initialized by the caller). As long as the file exists, `Some(mod_time) != None` evaluates to `true`, triggering the initial load. This means the function handles both initial loading and subsequent reloads through the same code path.

### File deletion and recreation

If the blacklist file is deleted, the next call clears the blacklist and resets `last_blacklist_mod_time` to `None`. If the file is later recreated, the subsequent call detects a new modification time (`Some(mod_time) != None`) and reloads it. This allows users to temporarily disable the blacklist by deleting the file and re-enable it by recreating it.

### Error tolerance

If [read_list](read_list.md) fails (e.g., due to a transient I/O error or encoding issue), `unwrap_or_default()` silently produces an empty blacklist. The modification time is still updated, so the error is not retried until the file is modified again. A subsequent file modification (even a no-op save) will trigger another reload attempt.

### Polling frequency

This function does not manage its own timer. It is called by the service main loop on every iteration (controlled by the `--interval` CLI argument, defaulting to a few seconds). The frequency of file system metadata checks is therefore determined by the service loop interval.

### Logging

All log output uses the `log!` macro, which writes to the service's log file and optionally to the console. The function emits:

- One message when the file becomes inaccessible.
- Two messages on successful reload (start and completion with item count).
- No messages when the file has not changed.

### Thread safety

This function is not thread-safe. It mutates `blacklist` and `last_blacklist_mod_time` through exclusive mutable references. The caller (the service main loop in [main](../main.rs/main.md)) runs single-threaded and holds sole ownership of these values.

### Comparison with hotreload_config

[hotreload_config](hotreload_config.md) performs an analogous hot-reload for the main configuration file but includes additional validation: it only replaces the live configuration if the new parse result has no errors. `hotreload_blacklist` is simpler — it always replaces the blacklist with whatever [read_list](read_list.md) returns, because the blacklist file format has no complex validation requirements.

## Requirements

| | |
|---|---|
| **Module** | `config` (`src/config.rs`) |
| **Visibility** | `pub` |
| **Callers** | [main](../main.rs/main.md) (service loop) |
| **Callees** | [read_list](read_list.md), `std::fs::metadata`, `log!` macro |
| **API** | `std::fs::metadata` for file system polling |
| **Privileges** | Read access to the blacklist file path |

## See Also

| Topic | Link |
|-------|------|
| Blacklist file reader | [read_list](read_list.md) |
| Config file hot-reload (analogous) | [hotreload_config](hotreload_config.md) |
| CLI arguments (blacklist_file_name) | [CliArgs](../cli.rs/CliArgs.md) |
| Service main loop | [main](../main.rs/main.md) |
| Config module overview | [README](README.md) |