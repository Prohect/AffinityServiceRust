# log! macro (logging.rs)

Convenience macro that formats its arguments using `format!` syntax and delegates to [log_message](log_message.md) for timestamped output. This macro is the primary logging entry point used throughout AffinityServiceRust; it produces `[HH:MM:SS]`-prefixed log lines directed to either the console or the main log file, depending on the [USE_CONSOLE](USE_CONSOLE.md) flag. Output is suppressed when [DUST_BIN_MODE](DUST_BIN_MODE.md) is active.

## Syntax

```logging.rs
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        $crate::logging::log_message(format!($($arg)*).as_str())
    };
}
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `$($arg:tt)*` | Token tree (variadic) | Any token sequence accepted by `std::format!`. Typically a format string literal followed by zero or more expressions. |

## Remarks

- The macro is annotated with `#[macro_export]`, which places it at the crate root. Callers import it as `crate::log!` or simply `log!` within the crate.
- Internally, the macro calls `format!($($arg)*)` to produce a heap-allocated `String`, then passes a `&str` reference to [log_message](log_message.md). This means the formatted string is allocated on every invocation regardless of whether logging is suppressed by [DUST_BIN_MODE](DUST_BIN_MODE.md). The suppression check occurs inside `log_message`, not in the macro expansion.
- The macro name shadows the `log` crate's `log!` macro. AffinityServiceRust does not use the `log` crate, so there is no conflict.
- Because the macro delegates entirely to [log_message](log_message.md), its output behavior is identical: a `[HH:MM:SS]` timestamp prefix is prepended using the cached time from [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md), and the destination (console or file) is determined by [USE_CONSOLE](USE_CONSOLE.md).

### Example expansion

A call like:

```/dev/null/example.rs#L1-1
log!("Applied {} rules to PID {}", count, pid);
```

expands to:

```/dev/null/example.rs#L1-1
crate::logging::log_message(format!("Applied {} rules to PID {}", count, pid).as_str());
```

Which produces a log line such as:

```/dev/null/example.log#L1-1
[14:32:07]Applied 3 rules to PID 1234
```

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `logging` (exported at crate root via `#[macro_export]`) |
| Delegates to | [log_message](log_message.md) |
| Respects | [DUST_BIN_MODE](DUST_BIN_MODE.md), [USE_CONSOLE](USE_CONSOLE.md), [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md), [LOG_FILE](LOG_FILE.md) |
| Std dependencies | `std::format!` |

## See Also

| Topic | Link |
|-------|------|
| Underlying log function | [log_message](log_message.md) |
| Raw logging without timestamp | [log_pure_message](log_pure_message.md) |
| Find-mode logging | [log_to_find](log_to_find.md) |
| Log suppression flag | [DUST_BIN_MODE](DUST_BIN_MODE.md) |
| Console output flag | [USE_CONSOLE](USE_CONSOLE.md) |
| logging module overview | [logging module](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd