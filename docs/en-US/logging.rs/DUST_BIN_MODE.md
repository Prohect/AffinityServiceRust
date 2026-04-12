# DUST_BIN_MODE static (logging.rs)

Controls whether log output is suppressed. When enabled, all messages written via [`log_message`](log_message.md) and [`log_pure_message`](log_pure_message.md) are silently discarded instead of being written to the log file or console.

## Syntax

```rust
static DUST_BIN_MODE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));
```

## Members

The static holds a `bool` behind a `Mutex`:

- `false` (default) — logging operates normally; messages are written to the log file and optionally to the console.
- `true` — logging is suppressed; all output is discarded.

## Remarks

The name "dust bin mode" reflects the idea that log messages are being thrown away — sent to the trash bin. This mode exists to handle a specific timing issue during UAC elevation.

### Purpose

When the application starts without administrator privileges and determines that UAC elevation is required, it will relaunch itself as an elevated process via [`request_uac_elevation`](../winapi.rs/request_uac_elevation.md). Any log output produced by the non-elevated instance before the relaunch is written to a log file that will be immediately abandoned once the elevated instance starts writing to its own log file. To avoid confusing partial log files, `DUST_BIN_MODE` is set to `true` before the elevation attempt, suppressing all output from the short-lived non-elevated instance.

### Lifecycle

1. The application starts with `DUST_BIN_MODE` set to `false` (normal logging).
2. If [`main`](../main.rs/main.md) determines that UAC elevation is needed and the `skip_log_before_elevation` flag in [`CliArgs`](../cli.rs/CliArgs.md) is set, `DUST_BIN_MODE` is set to `true`.
3. All subsequent log calls in the non-elevated instance produce no output.
4. The elevated instance starts fresh with `DUST_BIN_MODE` at its default `false` value.

### Thread safety

Access is synchronized through a `Mutex`. The lock is acquired briefly by each logging function to check the current mode, then released immediately.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/logging.rs |
| **Source line** | L63 |
| **Read by** | [`log_message`](log_message.md), [`log_pure_message`](log_pure_message.md) |
| **Set by** | [`main`](../main.rs/main.md) |

## See also

- [USE_CONSOLE static](USE_CONSOLE.md)
- [log_message function](log_message.md)
- [CliArgs](../cli.rs/CliArgs.md) (`skip_log_before_elevation` flag)
- [request_uac_elevation](../winapi.rs/request_uac_elevation.md)
- [logging.rs module overview](README.md)