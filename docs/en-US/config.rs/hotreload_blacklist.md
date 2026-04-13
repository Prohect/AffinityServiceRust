# hotreload_blacklist function (config.rs)

Checks if the blacklist file has been modified since the last check and reloads it if so.

## Syntax

```rust
pub fn hotreload_blacklist(
    cli: &CliArgs,
    blacklist: &mut Vec<String>,
    last_blacklist_mod_time: &mut Option<std::time::SystemTime>,
)
```

## Parameters

`cli`

Reference to [`CliArgs`](../cli.rs/CliArgs.md) containing the blacklist file path.

`blacklist`

Mutable reference to the current blacklist vector. Updated in-place when the file changes.

`last_blacklist_mod_time`

Tracks the last known modification time. Updated when the file is reloaded. Set to `None` if the file becomes inaccessible, which also clears the blacklist.

## Remarks

Called each loop iteration from [`main`](../main.rs/main.md) after sleeping. Handles three cases:

1. **File inaccessible** — If the file cannot be read and was previously accessible, clears the blacklist and resets the modification time.
2. **File modified** — If the modification time has changed, reloads the blacklist via [`read_list`](read_list.md).
3. **No change** — Does nothing.

This function was previously inline in `main()` and has been extracted for clarity.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/config.rs |
| **Called by** | [`main`](../main.rs/main.md) |

## See also

- [hotreload_config](hotreload_config.md)
- [read_list](read_list.md)
- [config.rs module overview](README.md)