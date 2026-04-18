# hotreload_blacklist function (config.rs)

Watches the blacklist file for modifications by comparing its filesystem modification timestamp against a cached value, and reloads the file contents when a change is detected. This function is called on each polling iteration to support live updates to the blacklist without restarting the service.

## Syntax

```AffinityServiceRust/src/config.rs#L1279-1301
pub fn hotreload_blacklist(
    cli: &CliArgs,
    blacklist: &mut Vec<String>,
    last_blacklist_mod_time: &mut Option<std::time::SystemTime>,
)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `cli` | `&CliArgs` | A reference to the parsed command-line arguments. The function reads `cli.blacklist_file_name` to determine the path to the blacklist file. If `blacklist_file_name` is `None`, the function returns immediately without performing any work. |
| `blacklist` | `&mut Vec<String>` | A mutable reference to the in-memory blacklist vector. When the blacklist file is reloaded, this vector is replaced with the new contents. When the blacklist file becomes inaccessible, this vector is cleared. |
| `last_blacklist_mod_time` | `&mut Option<std::time::SystemTime>` | A mutable reference to the cached modification timestamp of the blacklist file from the last successful check. Used to detect changes. Set to `None` initially and when the file becomes inaccessible; set to `Some(mod_time)` after a successful reload. |

## Return value

This function does not return a value. All results are communicated through mutations to `blacklist` and `last_blacklist_mod_time`.

## Remarks

### Change detection algorithm

1. If `cli.blacklist_file_name` is `None`, the function returns immediately — no blacklist file is configured.
2. The function calls `std::fs::metadata` on the blacklist file path:
   - **Metadata call fails** (file deleted, permissions changed, etc.): If `last_blacklist_mod_time` was previously `Some` (meaning the file was loaded at least once), the blacklist is cleared, `last_blacklist_mod_time` is reset to `None`, and a log message is emitted: `"Blacklist file '{path}' no longer accessible, clearing blacklist."`. If it was already `None`, no action is taken (avoids repeated log spam).
   - **Metadata call succeeds**: The file's modification time is compared against `last_blacklist_mod_time`. If the timestamps differ (or if `last_blacklist_mod_time` is `None`, indicating first load), the file is reloaded.

3. On reload, the function calls [`read_bleack_list`](read_bleack_list.md) to parse the file contents into a vector of lowercase, non-empty, non-comment strings. If `read_bleack_list` returns an error, an empty vector is used as a fallback via `unwrap_or_default()`.
4. The cached timestamp is updated to the file's current modification time.
5. Log messages are emitted:
   - `"Blacklist file '{path}' changed, reloading..."` — before the reload.
   - `"Blacklist reload complete: {n} items loaded."` — after the reload.

### File disappearance handling

When a previously accessible blacklist file is deleted or becomes inaccessible:

- The in-memory blacklist is **cleared** (all entries removed).
- `last_blacklist_mod_time` is set to `None`.
- A single log message is emitted. Subsequent polling iterations will not log again as long as the file remains inaccessible (because `last_blacklist_mod_time` is already `None`).

If the file reappears later, the next metadata check will succeed, detect a timestamp mismatch (since the cached time is `None`), and trigger a fresh reload.

### First-load behavior

On the first call with `last_blacklist_mod_time` set to `None` and a valid blacklist file present, the function detects a timestamp mismatch (`Some(mod_time) != None`) and loads the file. This handles both the initial startup load and the recovery-after-disappearance case uniformly.

### Polling frequency

This function is designed to be called once per polling loop iteration. The cost of each call is a single `std::fs::metadata` syscall when the file exists and has not changed, making it lightweight for frequent invocation.

### Thread safety

This function is not thread-safe and must be called from a single thread (the main polling loop). The mutable references to `blacklist` and `last_blacklist_mod_time` enforce this at the language level through Rust's borrow checker.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `config.rs` |
| Visibility | `pub` |
| Callers | `main.rs` (main polling loop) |
| Callees | [`read_bleack_list`](read_bleack_list.md), `std::fs::metadata`, `log!` macro |
| Dependencies | [`CliArgs`](../cli.rs/CliArgs.md) (for `blacklist_file_name`), `std::time::SystemTime` |
| I/O | Filesystem metadata query + optional file read via [`read_bleack_list`](read_bleack_list.md) |
| Privileges | Filesystem read access to the blacklist file path |

## See Also

| Resource | Link |
|----------|------|
| hotreload_config | [hotreload_config](hotreload_config.md) |
| read_bleack_list | [read_bleack_list](read_bleack_list.md) |
| CliArgs | [CliArgs](../cli.rs/CliArgs.md) |
| read_config | [read_config](read_config.md) |
| config module overview | [README](README.md) |

---
*Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*