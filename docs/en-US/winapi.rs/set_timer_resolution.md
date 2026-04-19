# set_timer_resolution function (winapi.rs)

Sets the Windows global timer resolution to a user-specified interval via the undocumented `NtSetTimerResolution` NT API. This allows the service to request a higher-frequency system timer tick, which can reduce scheduling latency for time-sensitive workloads.

## Syntax

```rust
pub fn set_timer_resolution(cli: &CliArgs)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `cli` | `&CliArgs` | Reference to the parsed command-line arguments. The `time_resolution` field specifies the desired timer resolution in 100-nanosecond units. A value of `0` disables the feature (no-op). |

## Return value

This function does not return a value. Success or failure is communicated via log output.

## Remarks

### Resolution units

The `cli.time_resolution` value is expressed in 100-nanosecond (100ns) units, matching the native unit used by `NtSetTimerResolution`. Common values:

| Desired resolution | Value (100ns units) | Equivalent |
|--------------------|---------------------|------------|
| 0.5 ms | `5000` | 0.5000 ms |
| 1.0 ms | `10000` | 1.0000 ms |
| 15.6 ms (default) | `156250` | 15.6250 ms |

### Behavior

1. If `cli.time_resolution == 0`, the function returns immediately without calling any API.
2. Otherwise, `NtSetTimerResolution` is called with `set_resolution = true` to request the specified interval.
3. The API returns the *previous* ("elder") timer resolution in its `p_current_resolution` out-parameter.
4. On success (`NTSTATUS >= 0`), both the requested and previous resolutions are logged in milliseconds (value ÷ 10,000).
5. On failure (`NTSTATUS < 0`), the NTSTATUS code is logged in hexadecimal format.

### System-wide effect

`NtSetTimerResolution` affects the global Windows timer resolution, not just the calling process. The system uses the *smallest* (most precise) resolution requested by any process. When the requesting process exits, its resolution request is automatically released and the system may revert to a coarser interval.

### Log output examples

**Success:**
```
Succeed to set timer resolution: 0.5000ms
elder timer resolution: 156250
```

**Failure:**
```
Failed to set timer resolution: 0xC0000022
```

### NtSetTimerResolution signature

```c
NTSTATUS NtSetTimerResolution(
    ULONG   DesiredResolution,  // in 100ns units
    BOOLEAN SetResolution,      // TRUE to set, FALSE to reset
    PULONG  CurrentResolution   // receives the previous resolution
);
```

This is an undocumented NT API imported directly from `ntdll.dll` via the `#[link(name = "ntdll")]` FFI block in winapi.rs.

### Relationship to multimedia timers

This achieves the same effect as the documented `timeBeginPeriod`/`timeEndPeriod` APIs from `winmm.dll`, but uses the lower-level NT interface directly. The NT API provides finer granularity (100ns units vs. 1ms units for `timeBeginPeriod`).

## Requirements

| | |
|---|---|
| **Module** | `src/winapi.rs` |
| **Callers** | Main startup logic in `src/main.rs` |
| **Callees** | `NtSetTimerResolution` (ntdll.dll FFI) |
| **NT API** | `NtSetTimerResolution` (undocumented, linked via `ntdll.dll`) |
| **Privileges** | None required; any process may request a higher timer resolution |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [winapi.rs](README.md) |
| CLI argument parsing | [cli.rs](../cli.rs/README.md) |
| NT FFI declarations | [winapi.rs](README.md) (External FFI section) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*