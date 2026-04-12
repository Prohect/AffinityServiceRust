# format_100ns function (scheduler.rs)

Formats a 100-nanosecond time value into a human-readable `"seconds.milliseconds s"` string.

## Syntax

```rust
fn format_100ns(time: i64) -> String
```

## Parameters

`time`

A time value expressed in 100-nanosecond units, as used by Windows kernel time fields such as `KernelTime` and `UserTime` in `SYSTEM_THREAD_INFORMATION`. One second equals 10,000,000 of these units.

## Return value

A `String` formatted as `"{seconds}.{milliseconds:03} s"`, where milliseconds is zero-padded to three digits.

**Examples:**

| Input (100ns units) | Output |
| --- | --- |
| `10_000_000` | `"1.000 s"` |
| `15_678_900` | `"1.567 s"` |
| `0` | `"0.000 s"` |
| `123_456_789_0` | `"123.456 s"` |

## Remarks

This function is used internally by [`PrimeThreadScheduler::close_dead_process_handles`](PrimeThreadScheduler.md) to format kernel time and user time values when logging detailed thread statistics on process exit.

The conversion works as follows:

1. **Seconds** are computed via integer division: `time / 10_000_000`.
2. **Milliseconds** are computed from the remainder: `(time % 10_000_000) / 10_000`, discarding sub-millisecond precision.

The function is module-private (`fn`, not `pub fn`) and is not exposed outside `scheduler.rs`.

### Relationship to format_filetime

While `format_100ns` formats *duration* values (elapsed time), the sibling function [`format_filetime`](format_filetime.md) formats *absolute* timestamps (Windows FILETIME epoch). Both accept `i64` values in 100-nanosecond units but interpret them differently.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | `src/scheduler.rs` |
| **Source lines** | L303–L307 |
| **Visibility** | Module-private |
| **Called by** | [`PrimeThreadScheduler::close_dead_process_handles`](PrimeThreadScheduler.md) |

## See also

- [format_filetime](format_filetime.md)
- [PrimeThreadScheduler](PrimeThreadScheduler.md)
- [ThreadStats](ThreadStats.md)
- [scheduler.rs module overview](README.md)