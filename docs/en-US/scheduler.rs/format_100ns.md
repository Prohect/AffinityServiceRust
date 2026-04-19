# format_100ns function (scheduler.rs)

Formats a time value expressed in 100-nanosecond units into a human-readable `"seconds.milliseconds s"` string. Used in process exit reports to display kernel time, user time, and other timing fields from `SYSTEM_THREAD_INFORMATION`.

## Syntax

```rust
fn format_100ns(time: i64) -> String
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `time` | `i64` | A time duration in 100-nanosecond (100ns) units, as reported by Windows kernel structures such as `SYSTEM_THREAD_INFORMATION.KernelTime` and `SYSTEM_THREAD_INFORMATION.UserTime`. |

## Return value

`String` — A formatted string in the form `"{seconds}.{milliseconds:03} s"`, where milliseconds is zero-padded to three digits.

### Examples

| Input (100ns units) | Output |
|---------------------|--------|
| `0` | `"0.000 s"` |
| `10_000_000` | `"1.000 s"` |
| `156_250_000` | `"15.625 s"` |
| `10_000` | `"0.001 s"` |
| `99_999` | `"0.009 s"` |

## Remarks

- The conversion is performed with integer arithmetic only — no floating-point rounding:
  - Seconds: `time / 10_000_000`
  - Milliseconds: `(time % 10_000_000) / 10_000`
- Sub-millisecond precision (the remaining microsecond and 100ns components) is truncated, not rounded.
- This function is module-private (`fn`, not `pub fn`) and is called exclusively from [PrimeThreadScheduler::drop_process_by_pid](PrimeThreadScheduler.md) when generating thread diagnostic reports on process exit.
- The `" s"` suffix is always appended to distinguish the output from raw numeric values in log output.

### Unit reference

| Windows unit | = SI equivalent |
|---|---|
| 1 tick | 100 nanoseconds |
| 10,000 ticks | 1 millisecond |
| 10,000,000 ticks | 1 second |

## Requirements

| | |
|---|---|
| **Module** | `src/scheduler.rs` |
| **Callers** | [PrimeThreadScheduler::drop_process_by_pid](PrimeThreadScheduler.md) |
| **Dependencies** | None (pure formatting function) |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| FILETIME formatter | [format_filetime](format_filetime.md) |
| Process exit reporting | [PrimeThreadScheduler::drop_process_by_pid](PrimeThreadScheduler.md) |
| Thread statistics container | [ThreadStats](ThreadStats.md) |
| Module overview | [scheduler.rs](README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*