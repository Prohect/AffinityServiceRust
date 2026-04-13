# format_100ns function (scheduler.rs)

Formats a 100-nanosecond interval value as a human-readable seconds string with millisecond precision.

## Syntax

```rust
fn format_100ns(time: i64) -> String
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `time` | `i64` | A time duration expressed in 100-nanosecond intervals (the native unit used by Windows kernel time fields such as `KernelTime` and `UserTime` in `SYSTEM_THREAD_INFORMATION`). |

## Return value

Returns a `String` in the format `"{seconds}.{milliseconds} s"`, where milliseconds is zero-padded to three digits.

**Examples of output:**

| Input (100 ns ticks) | Output |
|----------------------|--------|
| `0` | `"0.000 s"` |
| `10_000_000` | `"1.000 s"` |
| `156_250_000` | `"15.625 s"` |
| `45_678` | `"0.004 s"` |

## Remarks

This is a module-private helper function used when logging thread diagnostics in [`PrimeThreadScheduler::drop_process_by_pid`](PrimeThreadScheduler.md#drop_process_by_pid). It converts raw kernel/user time values from `SYSTEM_THREAD_INFORMATION` into a readable format for the process-exit thread report.

Windows represents kernel and user times as 100-nanosecond intervals (10 million ticks = 1 second). The function performs integer division to extract seconds and truncated milliseconds:

- **Seconds:** `time / 10_000_000`
- **Milliseconds:** `(time % 10_000_000) / 10_000`

The millisecond value is truncated, not rounded. Sub-millisecond precision (the remaining microseconds and 100-nanosecond ticks) is discarded.

Negative values for `time` are technically valid at the arithmetic level but are not expected in normal use, as kernel and user times are always non-negative.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `scheduler.rs` |
| **Visibility** | Module-private (`fn`, no `pub`) |
| **Callers** | [`PrimeThreadScheduler::drop_process_by_pid`](PrimeThreadScheduler.md#drop_process_by_pid) |
| **Dependencies** | None (pure computation) |

## See Also

| Topic | Link |
|-------|------|
| format_filetime | [format_filetime](format_filetime.md) |
| PrimeThreadScheduler::drop_process_by_pid | [PrimeThreadScheduler](PrimeThreadScheduler.md#drop_process_by_pid) |
| ThreadStats | [ThreadStats](ThreadStats.md) |
| SYSTEM_THREAD_INFORMATION | [Microsoft Learn — SYSTEM_THREAD_INFORMATION](https://learn.microsoft.com/en-us/windows/win32/api/winternl/nf-winternl-ntquerysysteminformation#system_thread_information) |