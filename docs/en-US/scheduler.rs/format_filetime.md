# format_filetime function (scheduler.rs)

Converts a Windows FILETIME value (100-nanosecond intervals since January 1, 1601 UTC) to a human-readable local date-time string. Used in diagnostic reports when logging thread creation times and other kernel timestamps.

## Syntax

```rust
fn format_filetime(time: i64) -> String
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `time` | `i64` | A Windows FILETIME value expressed as 100-nanosecond intervals since the Windows epoch (January 1, 1601 00:00:00 UTC). This is the same unit used by `SYSTEM_THREAD_INFORMATION.CreateTime` and other kernel time fields stored as `LARGE_INTEGER`. |

## Return value

`String` — A formatted local date-time string in the pattern `"YYYY-MM-DD HH:MM:SS.mmm"` (e.g., `"2025-01-15 14:32:07.123"`). If the timestamp cannot be converted to a valid `DateTime`, the raw `i64` value is returned as its decimal string representation.

## Remarks

### Conversion algorithm

1. **FILETIME to Unix epoch:** The input is divided by `10,000,000` to convert from 100ns ticks to whole seconds, then the Windows-to-Unix epoch offset (`11,644,473,600` seconds) is subtracted. This bridges the gap between the Windows epoch (1601-01-01) and the Unix epoch (1970-01-01).
2. **Sub-second precision:** The fractional part `(time % 10_000_000) * 100` is passed as the nanosecond component to `DateTime::from_timestamp`.
3. **Timezone conversion:** The resulting UTC `DateTime` is converted to the local timezone via `dt.with_timezone(&Local)`.
4. **Formatting:** The `chrono` format string `"%Y-%m-%d %H:%M:%S%.3f"` produces millisecond-precision output.

### Fallback behavior

If `DateTime::from_timestamp` returns `None` (e.g., the input value is negative or represents a date outside the representable range), the function falls back to returning `time.to_string()` — the raw 100ns tick count as a plain decimal number.

### Example conversions

| Input (100ns ticks) | Meaning | Output (example, UTC+8) |
|---------------------|---------|-------------------------|
| `133500000000000000` | ~2024-01-15 | `"2024-01-15 08:00:00.000"` |
| `0` | Windows epoch (1601-01-01) | `"1601-01-01 08:00:00.000"` or local equivalent |
| `-1` | Invalid | `"-1"` |

### Relationship to format_100ns

While [format_100ns](format_100ns.md) formats *durations* (elapsed 100ns intervals) as `"seconds.milliseconds s"`, `format_filetime` formats *absolute timestamps* (100ns intervals since 1601) as calendar date-time strings. Both operate on `i64` values in the same 100ns unit but serve different semantic purposes.

## Requirements

| | |
|---|---|
| **Module** | `src/scheduler.rs` |
| **Callers** | [PrimeThreadScheduler::drop_process_by_pid](PrimeThreadScheduler.md) (formats `CreateTime` in top-threads report) |
| **Dependencies** | `chrono::DateTime`, `chrono::Local` |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [scheduler.rs](README.md) |
| Duration formatter | [format_100ns](format_100ns.md) |
| Scheduler drop logic | [PrimeThreadScheduler](PrimeThreadScheduler.md) |
| Thread statistics | [ThreadStats](ThreadStats.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*