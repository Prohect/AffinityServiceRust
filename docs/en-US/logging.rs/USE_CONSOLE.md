# USE_CONSOLE static (logging.rs)

Controls whether log output is also written to the console (stdout) in addition to the log file. When enabled, all messages written by [`log_message`](log_message.md) are echoed to the console for interactive monitoring.

## Syntax

```rust
static USE_CONSOLE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));
```

## Members

The static holds a `bool` behind a `Mutex`, initialized to `false`:

- `true` — log messages are written to both the log file and the console.
- `false` — log messages are written to the log file only.

## Remarks

`USE_CONSOLE` defaults to `false` and is set to `true` by [`main`](../main.rs/main.md) when the application is running in an interactive console session. This allows the user to see real-time log output during normal interactive operation, while suppressing console output when running as a background service or in scenarios where stdout is not available.

The flag is checked by [`log_message`](log_message.md) on every log call. When enabled, the message is printed to stdout via `println!` after being written to the log file. The console output uses the same format as the file output, including the timestamp prefix.

### Thread safety

Access to the flag is synchronized through a `Mutex`. Since the value is typically set once during startup and then only read during operation, contention is minimal. The lock is acquired briefly by [`log_message`](log_message.md) for each log call.

### Interaction with DUST_BIN_MODE

When [`DUST_BIN_MODE`](DUST_BIN_MODE.md) is active, logging is suppressed entirely — both file and console output are skipped. `USE_CONSOLE` only has an effect when `DUST_BIN_MODE` is `false`.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/logging.rs |
| **Source line** | L62 |
| **Set by** | [`main`](../main.rs/main.md) |
| **Read by** | [`log_message`](log_message.md) |

## See also

- [DUST_BIN_MODE static](DUST_BIN_MODE.md)
- [log_message function](log_message.md)
- [logging.rs module overview](README.md)