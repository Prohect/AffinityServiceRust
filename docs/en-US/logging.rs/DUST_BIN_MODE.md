# DUST_BIN_MODE static (logging.rs)

Global flag that suppresses all logging output when set to `true`. This mode is activated before UAC elevation to prevent the unprivileged (pre-elevation) process from writing to log files that it may not own or have permission to create. Once the elevated process takes over, `DUST_BIN_MODE` is set back to `false` and normal logging resumes.

## Syntax

```logging.rs
pub static DUST_BIN_MODE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));
```

## Members

| Component | Type | Description |
|-----------|------|-------------|
| Outer | `Lazy<…>` | Deferred initialization via `once_cell::sync::Lazy`. Created on first access. |
| Inner | `Mutex<bool>` | Thread-safe interior mutability. `false` = logging enabled (default), `true` = all log output suppressed. |

## Remarks

- When `DUST_BIN_MODE` is `true`, the [log_message](log_message.md) function returns immediately without writing anything to the console or log file. This is a hard suppression — no buffering or deferred output occurs; messages are silently discarded.
- The flag is accessed through the [get_dust_bin_mod!](get_dust_bin_mod.md) macro, which acquires the mutex lock and returns a `MutexGuard<bool>`. Callers can dereference the guard to read the current value, or dereference mutably to change it.
- The name "dust bin" is metaphorical — log messages are effectively thrown away, as if into a waste bin.
- [log_pure_message](log_pure_message.md) and [log_to_find](log_to_find.md) do **not** check `DUST_BIN_MODE`. Only [log_message](log_message.md) (and by extension the [log!](log.md) macro) respects this flag.

### Typical lifecycle

1. **Service startup (unprivileged):** If the process detects it needs UAC elevation, it sets `DUST_BIN_MODE` to `true` via `*get_dust_bin_mod!() = true` before performing any logging. This is controlled by the `--skip-log-before-elevation` CLI flag.
2. **UAC re-launch:** The process calls `request_uac_elevation` and exits. The elevated child process starts fresh with `DUST_BIN_MODE` defaulting to `false`.
3. **Normal operation:** `DUST_BIN_MODE` remains `false` for the entire service lifetime, and all log output proceeds normally.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `logging` |
| Crate dependencies | `once_cell` (`Lazy`), `std::sync::Mutex` |
| Writers | [main](../main.rs/README.md) (sets to `true` before elevation) |
| Readers | [log_message](log_message.md) |
| Accessor macro | [get_dust_bin_mod!](get_dust_bin_mod.md) |

## See Also

| Topic | Link |
|-------|------|
| Console vs. file output flag | [USE_CONSOLE](USE_CONSOLE.md) |
| Timestamped log output function | [log_message](log_message.md) |
| Convenience logging macro | [log!](log.md) |
| UAC elevation request | [request_uac_elevation](../winapi.rs/request_uac_elevation.md) |
| CLI arguments controlling logging behavior | [cli module](../cli.rs/README.md) |
| logging module overview | [logging module](README.md) |