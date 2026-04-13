# format_filetime function (scheduler.rs)

Formats a Windows `FILETIME` value (represented as an `i64` of 100-nanosecond intervals since January 1, 1601) as a local date-time string with millisecond precision.

## Syntax

```rust
fn format_filetime(time: i64) -> String
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `time` | `i64` | A 64-bit signed integer representing a Windows `FILETIME` value — the number of 100-nanosecond intervals since January 1, 1601 (UTC). |

## Return value

Returns a `String` containing the formatted local date-time. The format is `YYYY-MM-DD HH:MM:SS.mmm`, where `mmm` is milliseconds.

If the timestamp cannot be converted to a valid `DateTime` (for example, if it represents a date outside the representable range), the raw `i64` value is returned as a string instead.

## Remarks

This function converts from the Windows `FILETIME` epoch (January 1, 1601 UTC) to the Unix epoch (January 1, 1970 UTC) by subtracting the well-known constant **11,644,473,600 seconds**. The conversion proceeds as follows:

1. Divide `time` by 10,000,000 to get whole seconds since the Windows epoch.
2. Subtract 11,644,473,600 to rebase to the Unix epoch.
3. Extract the sub-second remainder as nanoseconds: `(time % 10_000_000) * 100`.
4. Construct a UTC `DateTime` via `DateTime::from_timestamp`, then convert to the local time zone.

The output is formatted with `chrono`'s `%Y-%m-%d %H:%M:%S%.3f` format specifier, which always produces exactly three fractional digits (milliseconds).

### Visibility

This function is module-private (`fn`, not `pub fn`). It is called internally by [`PrimeThreadScheduler::drop_process_by_pid`](PrimeThreadScheduler.md) to format the `CreateTime` field from `SYSTEM_THREAD_INFORMATION` in thread diagnostic reports.

### Edge cases

| Scenario | Behavior |
|----------|----------|
| `time` is `0` | Converts to `1601-01-01` minus epoch offset, which is a negative Unix timestamp. Falls back to printing `"0"` if out of range. |
| `time` is negative | Represents a date before the Windows epoch. The Unix conversion will underflow further; falls back to printing the raw value. |
| Local time zone unavailable | `chrono` uses UTC as fallback on platforms where local time zone detection fails. |

### Example output

For a `FILETIME` value representing July 4, 2025 at 14:30:00.123 local time:

```text
2025-07-04 14:30:00.123
```

## Requirements

| &nbsp; | &nbsp; |
|--------|--------|
| **Module** | `scheduler.rs` |
| **Crate dependency** | `chrono` (for `DateTime`, `Local`) |
| **Callers** | [`PrimeThreadScheduler::drop_process_by_pid`](PrimeThreadScheduler.md) |
| **Callees** | `chrono::DateTime::from_timestamp`, `DateTime::with_timezone`, `DateTime::format` |
| **Platform** | Windows (values originate from Windows `FILETIME` structures) |

## See Also

| Link | Description |
|------|-------------|
| [`format_100ns`](format_100ns.md) | Formats 100-nanosecond intervals as a seconds.milliseconds duration string. |
| [`PrimeThreadScheduler::drop_process_by_pid`](PrimeThreadScheduler.md) | The method that calls `format_filetime` when logging thread diagnostics on process exit. |
| [`ThreadStats`](ThreadStats.md) | Per-thread statistics struct whose `last_system_thread_info` contains the `FILETIME` values formatted by this function. |