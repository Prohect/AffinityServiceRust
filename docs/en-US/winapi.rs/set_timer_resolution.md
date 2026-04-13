# set_timer_resolution function (winapi.rs)

Sets the system-wide timer resolution to a caller-specified value by invoking the undocumented `NtSetTimerResolution` function from `ntdll.dll`. A lower timer resolution value causes the Windows scheduler to tick more frequently, which can reduce scheduling latency for time-sensitive workloads at the cost of slightly higher power consumption and CPU overhead.

## Syntax

```rust
pub fn set_timer_resolution(cli: &CliArgs)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `cli` | `&CliArgs` | Reference to the parsed CLI arguments. The function reads `cli.time_resolution`, which is a `u32` value expressed in 100-nanosecond intervals (e.g., `5000` = 0.5 ms, `10000` = 1.0 ms). A value of `0` is the default and typically means "do not change the timer resolution". |

## Return value

None. The function logs success or failure internally and does not return a result.

## Remarks

### Timer resolution units

The `time_resolution` value uses the Windows kernel's native time unit of 100-nanosecond intervals (also called "hectonanoseconds"). Common values:

| `time_resolution` | Equivalent period | Description |
|-------------------|-------------------|-------------|
| `5000` | 0.5000 ms | Highest resolution supported on most hardware |
| `10000` | 1.0000 ms | Common high-resolution setting |
| `156250` | 15.6250 ms | Default Windows timer resolution |

### NtSetTimerResolution API

The function calls the undocumented `NtSetTimerResolution` from `ntdll.dll` with:

- `desired_resolution` — the requested resolution in 100 ns units (`cli.time_resolution`).
- `set_resolution` — `true` to request the new resolution.
- `p_current_resolution` — pointer to a `u32` that receives the previous (elder) timer resolution.

The return value is an `NTSTATUS`:

| NTSTATUS range | Meaning |
|----------------|---------|
| `>= 0` (non-negative) | Success. The timer resolution was changed (or was already at the requested value). |
| `< 0` (negative) | Failure. The requested resolution may be outside the supported range or the caller lacks permission. |

### Logging

| Outcome | Log message |
|---------|-------------|
| Success | `"Succeed to set timer resolution: {value}ms"` (formatted to 4 decimal places) followed by `"elder timer resolution: {previous_value}"` (raw 100 ns ticks) |
| Failure | `"Failed to set timer resolution: 0x{ntstatus:08X}"` |

### System-wide effect

Timer resolution changes via `NtSetTimerResolution` (and the documented `timeBeginPeriod`) are system-global: the Windows kernel uses the smallest (most frequent) resolution requested by any running process. When the calling process exits, its resolution request is automatically removed, and the system reverts to the next-smallest active request.

### Typical usage

AffinityServiceRust calls `set_timer_resolution` once during startup if the user specifies a `--time_resolution` CLI argument. The default `time_resolution` value of `0` (or no flag) means the function is still called but requests a 0-tick resolution, which `NtSetTimerResolution` rejects (negative NTSTATUS), effectively making the call a no-op.

### Safety

The function body is wrapped in an `unsafe` block because `NtSetTimerResolution` is an FFI call into `ntdll.dll` declared in the module's `extern "system"` block. The only mutable state touched is a stack-local `u32` (`current_resolution`) used as an out-parameter.

### Relationship to process scheduling

A higher-frequency timer tick reduces the minimum sleep granularity and scheduling quantum, which can benefit real-time applications (games, audio engines) that the service manages. However, it also slightly increases system-wide interrupt overhead. Users should choose a value that balances latency requirements against efficiency.

## Requirements

| | |
|---|---|
| **Module** | `winapi` (`src/winapi.rs`) |
| **Visibility** | `pub` |
| **Callers** | [`main`](../main.rs/README.md) (during startup) |
| **Callees** | `NtSetTimerResolution` (ntdll.dll FFI) |
| **API** | `NtSetTimerResolution` — undocumented ntdll function; documented equivalent: [`timeBeginPeriod`](https://learn.microsoft.com/en-us/windows/win32/api/timeapi/nf-timeapi-timebeginperiod) |
| **Privileges** | None required beyond normal process privileges |

## See Also

| Topic | Link |
|-------|------|
| CLI argument parsing (time_resolution flag) | [cli module](../cli.rs/README.md) |
| Service main entry point | [main module](../main.rs/README.md) |
| timeBeginPeriod (documented alternative) | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/timeapi/nf-timeapi-timebeginperiod) |
| Timer resolution deep dive | [Microsoft Learn — Timer Resolution](https://learn.microsoft.com/en-us/windows/win32/sysinfo/acquiring-high-resolution-time-stamps) |