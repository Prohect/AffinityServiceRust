# format_filetime function (scheduler.rs)

Converts a Windows `FILETIME` value (represented as a 64-bit integer counting 100-nanosecond intervals since January 1, 1601 UTC) into a human-readable local date-time string formatted as `YYYY-MM-DD HH:MM:SS.mmm`. This function is used when logging detailed thread information on process exit.

## Syntax

```AffinityServiceRust/src/scheduler.rs#L279-L286
fn format_filetime(time: i64) -> String
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `time` | `i64` | A 64-bit signed integer representing a Windows `FILETIME` value — the number of 100-nanosecond intervals since January 1, 1601 00:00:00 UTC. Typically sourced from `SYSTEM_THREAD_INFORMATION::CreateTime.QuadPart()`. |

## Return value

Returns a `String` containing the local date-time representation formatted as `YYYY-MM-DD HH:MM:SS.mmm` (e.g. `"2025-01-15 14:32:07.456"`). If the timestamp cannot be converted to a valid `DateTime` (e.g. the input is negative or represents a date outside the representable range), the raw `i64` value is returned as its decimal string representation.

## Remarks

### Conversion algorithm

1. **FILETIME to Unix epoch** — The function first converts from the Windows epoch (1601-01-01) to the Unix epoch (1970-01-01) by dividing by 10,000,000 to get whole seconds and then subtracting the constant `11,644,473,600` (the number of seconds between the two epochs).
2. **Sub-second precision** — The nanosecond component is extracted from the remainder of `time % 10_000_000`, multiplied by 100 to convert from 100-nanosecond units to nanoseconds.
3. **Local time conversion** — The resulting UTC timestamp is converted to local time using `chrono::Local` timezone and formatted with the pattern `"%Y-%m-%d %H:%M:%S%.3f"`, which includes millisecond precision.

### Edge cases

- If `DateTime::from_timestamp` returns `None` (invalid or out-of-range timestamp), the function falls back to returning `time.to_string()`, the raw 100-nanosecond tick count as a decimal string.
- A `time` value of `0` corresponds to the Windows epoch (1601-01-01 00:00:00 UTC), which converts to a negative Unix timestamp and may or may not be representable depending on the platform.
- Thread create times very close to system boot may carry unusual values depending on the Windows kernel version.

### Usage context

This function is called inside `PrimeThreadScheduler::drop_process_by_pid` to format the `CreateTime` field of `SYSTEM_THREAD_INFORMATION` when generating the top-N thread diagnostic report on process exit.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `scheduler.rs` |
| Callers | [`PrimeThreadScheduler::drop_process_by_pid`](PrimeThreadScheduler.md) |
| Callees | `chrono::DateTime::from_timestamp`, `chrono::DateTime::with_timezone`, `chrono::DateTime::format` |
| Dependencies | `chrono` crate (`DateTime`, `Local`) |
| Privileges | None |

## See Also

| Reference | Link |
|-----------|------|
| format_100ns | [format_100ns](format_100ns.md) |
| PrimeThreadScheduler | [PrimeThreadScheduler](PrimeThreadScheduler.md) |
| ThreadStats | [ThreadStats](ThreadStats.md) |
| scheduler module overview | [README](README.md) |

---
*Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
