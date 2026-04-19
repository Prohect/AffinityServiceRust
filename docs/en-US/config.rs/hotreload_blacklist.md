# hotreload_blacklist function (config.rs)

Checks whether the blacklist file has been modified on disk and reloads it if the modification timestamp has changed. This function is called every polling iteration to support live updates to the process exclusion list without restarting the service.

## Syntax

```rust
pub fn hotreload_blacklist(
    cli: &CliArgs,
    blacklist: &mut Vec<String>,
    last_blacklist_mod_time: &mut Option<std::time::SystemTime>,
)
```

## Parameters

`cli: &CliArgs`

Reference to the parsed command-line arguments. The `blacklist_file_name` field (`Option<String>`) specifies the path to the blacklist file. If `None`, the function returns immediately without action.

`blacklist: &mut Vec<String>`

**\[in, out\]** The current in-memory blacklist of lowercase process names. On successful reload, this vector is replaced with the newly parsed contents. If the blacklist file becomes inaccessible (deleted, moved, or permissions changed), the vector is cleared.

`last_blacklist_mod_time: &mut Option<std::time::SystemTime>`

**\[in, out\]** Tracks the last-known modification timestamp of the blacklist file. Used to detect changes:

- `None` indicates no previous successful read (initial state or file became inaccessible).
- `Some(time)` holds the `modified()` timestamp from the most recent successful reload.

The function compares the file's current modification time against this stored value. If they differ, a reload is triggered and this value is updated. If the file becomes inaccessible, this is reset to `None`.

## Return value

This function does not return a value. Side effects are communicated through the `blacklist` and `last_blacklist_mod_time` out-parameters.

## Remarks

### Reload logic

The function follows this decision tree:

1. **No blacklist file configured:** If `cli.blacklist_file_name` is `None`, the function returns immediately.
2. **File inaccessible:** If `std::fs::metadata()` fails for the blacklist path:
   - If a previous modification time was recorded (`last_blacklist_mod_time.is_some()`), the blacklist is cleared and the timestamp is reset to `None`.
   - If no previous timestamp existed, no action is taken (avoids repeated logging on startup when no file exists).
3. **File unchanged:** If the file's `modified()` timestamp matches `*last_blacklist_mod_time`, the function returns without reloading.
4. **File changed:** If the timestamps differ, the file is reloaded via [read_bleack_list](read_bleack_list.md). On success, `*blacklist` is replaced and `*last_blacklist_mod_time` is updated. On failure, the previous blacklist and timestamp are retained.

### File disappearance handling

When a previously accessible blacklist file becomes inaccessible (e.g., deleted by the user), the function proactively clears the in-memory blacklist. This ensures that processes previously excluded by the blacklist are no longer blocked. A log message is emitted: `"Blacklist file '{path}' no longer accessible, clearing blacklist."`.

### Logging

- On reload: `"Blacklist file '{path}' changed, reloading..."`
- After reload: `"Blacklist reload complete: {N} items loaded."`
- On file disappearance: `"Blacklist file '{path}' no longer accessible, clearing blacklist."`

### Polling frequency

This function is called once per main service polling iteration. The overhead is minimal — a single `metadata()` syscall per iteration when the file hasn't changed. No file I/O occurs unless the modification timestamp differs.

### Thread safety

This function is not thread-safe. It is designed to be called from the single-threaded main polling loop. The mutable references to `blacklist` and `last_blacklist_mod_time` enforce exclusive access at compile time.

## Requirements

| | |
|---|---|
| **Module** | [`src/config.rs`](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/config.rs) |
| **Callers** | Main polling loop in `main.rs` |
| **Callees** | [read_bleack_list](read_bleack_list.md), `std::fs::metadata` |
| **Dependencies** | [CliArgs](../cli.rs/CliArgs.md) |
| **Privileges** | File system read access to the blacklist file |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [config.rs](README.md) |
| Blacklist file reader | [read_bleack_list](read_bleack_list.md) |
| Config hot-reload counterpart | [hotreload_config](hotreload_config.md) |
| CLI arguments struct | [CliArgs](../cli.rs/CliArgs.md) |
| Main service loop | [main.rs](../main.rs/README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*