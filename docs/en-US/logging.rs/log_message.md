# log_message function (logging.rs)

Writes a timestamped log message to the main log file and optionally to the console. This is the backend for the `log!` macro, which is the primary logging interface used throughout the codebase.

## Syntax

```rust
pub fn log_message(args: &str)
```

## Parameters

`args`

The message string to log. This is typically produced by `format_args!` via the `log!` macro, but can also be passed directly as a string slice.

## Return value

This function does not return a value.

## Remarks

`log_message` is the central logging function for the application. It performs the following steps:

1. **Dust bin check** — if [`DUST_BIN_MODE`](DUST_BIN_MODE.md) is `true`, the function returns immediately without writing anything. This suppresses all logging during the pre-UAC-elevation phase.
2. **Timestamp formatting** — the function reads the cached timestamp from [`LOCAL_TIME_BUFFER`](LOCAL_TIME_BUFFER.md) and formats it as a human-readable prefix (e.g., `"2024-01-15 14:30:05.123"`). Using the buffered time ensures consistent timestamps across all messages within a single loop iteration.
3. **File write** — the timestamped message is written to the [`LOG_FILE`](LOG_FILE.md) static handle in append mode.
4. **Console echo** — if [`USE_CONSOLE`](USE_CONSOLE.md) is `true`, the same timestamped message is also printed to stdout via `println!`.

### The `log!` macro

The `log!` macro is the idiomatic way to call this function throughout the codebase. It accepts `format!`-style arguments:

```rust
log!("Applied priority {} for pid {}", priority, pid);
```

The macro expands to a `log_message` call with the formatted string, providing a convenient printf-style API without requiring callers to manually format their messages.

### Thread safety

All shared state accessed by this function ([`DUST_BIN_MODE`](DUST_BIN_MODE.md), [`LOCAL_TIME_BUFFER`](LOCAL_TIME_BUFFER.md), [`LOG_FILE`](LOG_FILE.md), [`USE_CONSOLE`](USE_CONSOLE.md)) is guarded by individual `Mutex` locks. Each lock is acquired and released independently, ensuring that log writes are serialized and no interleaving occurs within a single message.

### Comparison with log_pure_message

[`log_pure_message`](log_pure_message.md) is similar but omits the timestamp prefix. Use `log_message` (via `log!`) for normal operational messages, and [`log_pure_message`](log_pure_message.md) for continuation lines or structured output where a timestamp would be distracting.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/logging.rs |
| **Source lines** | L185–L195 |
| **Called by** | `log!` macro (used throughout all modules) |
| **Reads** | [`DUST_BIN_MODE`](DUST_BIN_MODE.md), [`LOCAL_TIME_BUFFER`](LOCAL_TIME_BUFFER.md), [`USE_CONSOLE`](USE_CONSOLE.md) |
| **Writes to** | [`LOG_FILE`](LOG_FILE.md) |

## See also

- [log_pure_message](log_pure_message.md)
- [log_to_find](log_to_find.md)
- [DUST_BIN_MODE static](DUST_BIN_MODE.md)
- [LOCAL_TIME_BUFFER static](LOCAL_TIME_BUFFER.md)
- [LOG_FILE static](LOG_FILE.md)
- [USE_CONSOLE static](USE_CONSOLE.md)
- [logging.rs module overview](README.md)