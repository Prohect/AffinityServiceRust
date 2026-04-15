# format_100ns function (scheduler.rs)

Converts a Windows 100-nanosecond time interval value into a human-readable string formatted as `seconds.milliseconds s`. This is a module-private utility used to format kernel time and user time values from `SYSTEM_THREAD_INFORMATION` when logging thread diagnostics on process exit.

## Syntax

```AffinityServiceRust/src/scheduler.rs#L265-267
fn format_100ns(time: i64) -> String
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `time` | `i64` | A time duration expressed in 100-nanosecond units, as returned by Windows kernel time fields such as `SYSTEM_THREAD_INFORMATION.KernelTime` and `SYSTEM_THREAD_INFORMATION.UserTime`. |

## Return value

Returns a `String` in the format `"<seconds>.<milliseconds> s"`, where milliseconds is zero-padded to three digits.

**Examples:**

| Input (100ns units) | Output |
|---------------------|--------|
| `0` | `"0.000 s"` |
| `10_000_000` | `"1.000 s"` |
| `156_250_000` | `"15.625 s"` |
| `10_000` | `"0.001 s"` |

## Remarks

- The conversion divides the input by 10,000,000 to obtain whole seconds, then extracts the fractional millisecond portion by taking the remainder modulo 10,000,000 and dividing by 10,000.
- Sub-millisecond precision (the remaining microsecond and 100-nanosecond components) is truncated, not rounded.
- Negative input values are technically valid from a division standpoint but produce implementation-defined formatting (Rust's `%` operator preserves the sign of the dividend). In practice, Windows kernel time values are always non-negative.
- This function is **not** `pub` and is only accessible within the `scheduler` module.
- This function performs no Win32 API calls and has no side effects.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `scheduler.rs` |
| Visibility | Private (crate-internal, not exported) |
| Callers | [`PrimeThreadScheduler::drop_process_by_pid`](PrimeThreadScheduler.md) |
| Callees | `std::format!` |
| API | None |
| Privileges | None |

## See Also

| Reference | Link |
|-----------|------|
| format_filetime | [format_filetime](format_filetime.md) |
| PrimeThreadScheduler | [PrimeThreadScheduler](PrimeThreadScheduler.md) |
| ThreadStats | [ThreadStats](ThreadStats.md) |
| scheduler module overview | [README](README.md) |

---
> Commit SHA: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
