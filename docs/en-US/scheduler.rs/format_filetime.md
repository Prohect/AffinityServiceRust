# format_filetime function (scheduler.rs)

Converts a Windows FILETIME value (100-nanosecond intervals since January 1, 1601) into a human-readable local date-time string.

## Syntax

```rust
fn format_filetime(time: i64) -> String
```

## Parameters

`time`

A 64-bit signed integer representing a Windows FILETIME value. This is measured in 100-nanosecond intervals since January 1, 1601 (UTC), matching the format used by `SYSTEM_THREAD_INFORMATION.CreateTime`.

## Return value

A `String` containing the formatted local date-time. On success, the format is `"YYYY-MM-DD HH:MM:SS.mmm"` (e.g. `"2024-07-15 14:32:01.456"`). If the timestamp cannot be converted to a valid `DateTime`, the raw `i64` value is returned as a string.

## Remarks

This function performs the standard Windows FILETIME-to-Unix epoch conversion by dividing the 100-nanosecond count by 10,000,000 to get seconds, then subtracting the Windows-Unix epoch offset of 11,644,473,600 seconds (the number of seconds between January 1, 1601 and January 1, 1970).

The sub-second component is preserved by extracting the fractional 100-nanosecond ticks (`time % 10_000_000`) and converting them to nanoseconds by multiplying by 100.

The resulting Unix timestamp is converted to a `chrono::DateTime` via `DateTime::from_timestamp`, then shifted to the local timezone using `with_timezone(&Local)` for display.

This function is primarily used within [`close_dead_process_handles`](PrimeThreadScheduler.md#methods) to format the `CreateTime` field of `SYSTEM_THREAD_INFORMATION` when logging thread statistics on process exit.

### Conversion formula

```
unix_seconds = (filetime / 10_000_000) - 11_644_473_600
nanoseconds  = (filetime % 10_000_000) * 100
```

### Relationship to format_100ns

While [`format_100ns`](format_100ns.md) formats a duration (elapsed time), `format_filetime` formats an absolute point in time (a timestamp). Both accept the same raw unit (100-nanosecond ticks) but interpret it differently:

| Function | Input meaning | Output format |
| --- | --- | --- |
| [`format_100ns`](format_100ns.md) | Duration in 100ns ticks | `"seconds.milliseconds s"` |
| `format_filetime` | FILETIME absolute timestamp | `"YYYY-MM-DD HH:MM:SS.mmm"` |

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/scheduler.rs |
| **Source lines** | L309–L316 |
| **Visibility** | Crate-private (`fn`, not `pub fn`) |
| **Called by** | [`PrimeThreadScheduler::close_dead_process_handles`](PrimeThreadScheduler.md) |
| **Dependencies** | `chrono::DateTime`, `chrono::Local` |

## See also

- [format_100ns](format_100ns.md)
- [PrimeThreadScheduler](PrimeThreadScheduler.md)
- [ThreadStats](ThreadStats.md)
- [scheduler.rs module overview](README.md)