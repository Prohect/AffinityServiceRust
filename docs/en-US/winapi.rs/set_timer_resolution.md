# set_timer_resolution function (winapi.rs)

Sets the system timer resolution to a caller-specified value via the NT-native `NtSetTimerResolution` API. This allows AffinityServiceRust to increase the precision of system-wide timing (e.g., `Sleep`, waitable timers) by requesting a smaller timer interval than the default ~15.6 ms.

## Syntax

```rust
pub fn set_timer_resolution(cli: &CliArgs)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `cli` | `&CliArgs` | Reference to the parsed command-line arguments. The `cli.time_resolution` field (`u32`) specifies the desired timer resolution in 100-nanosecond units (e.g., `5000` = 0.5 ms). |

## Return value

This function does not return a value. Success or failure is communicated through log messages written via the `log!` macro.

## Remarks

### Zero-resolution guard

If `cli.time_resolution` is `0`, the function returns immediately without calling `NtSetTimerResolution`.

### Mechanism

The function calls `NtSetTimerResolution(desired_resolution, true, &mut current_resolution)` where:

- `desired_resolution` is `cli.time_resolution` (in 100-ns units).
- The second parameter (`set_resolution: true`) requests that the resolution be **set** rather than queried.
- `current_resolution` receives the timer resolution that was in effect **before** the change (the "elder" resolution).

### NTSTATUS handling

| Condition | Behavior |
|-----------|----------|
| `NTSTATUS < 0` (failure) | Logs `"Failed to set timer resolution: 0x{NTSTATUS}"`. |
| `NTSTATUS >= 0` (success) | Logs the requested resolution in milliseconds (4 decimal places) and the previous ("elder") resolution. |

The resolution value is converted to milliseconds for display by dividing by `10000.0` (since the unit is 100 ns).

### Resolution value examples

| `time_resolution` value | Equivalent interval |
|-------------------------|---------------------|
| `156250` | 15.6250 ms (Windows default) |
| `10000` | 1.0000 ms |
| `5000` | 0.5000 ms (500 µs) |

### Important side effects

- **System-wide impact.** `NtSetTimerResolution` affects the global timer resolution for the entire operating system, not just the calling process. While the resolution is raised, all processes benefit from higher-precision timing, but power consumption may increase.
- **Sticky until reverted.** The elevated resolution remains in effect as long as the calling process is running and has not called `NtSetTimerResolution` with `set_resolution: false`. When the process exits, Windows automatically reverts the resolution to the next-highest resolution requested by any remaining process.
- **Minimum resolution.** The OS enforces a hardware-dependent minimum timer interval (typically 0.5 ms). Requests below this floor succeed but are clamped to the minimum supported value.

### Platform notes

- **Windows only.** `NtSetTimerResolution` is an undocumented NT-native API exported by `ntdll.dll`, linked via the `#[link(name = "ntdll")]` extern block at the top of `winapi.rs`.
- This API is the same mechanism used by `timeBeginPeriod` / `timeEndPeriod` from `winmm.dll`, but without requiring the multimedia library.

### Unsafe

The function body is wrapped in an `unsafe` block because `NtSetTimerResolution` is a foreign function call through the raw `ntdll` FFI binding.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `winapi.rs` |
| **Callers** | `main.rs` — called unconditionally during startup; the zero-resolution check is handled internally. |
| **Callees** | `NtSetTimerResolution` (ntdll), `log!` macro → [`log_message`](../logging.rs/log_message.md) |
| **API** | NT Native API — `NtSetTimerResolution` |
| **Privileges** | None explicitly required, but may be limited by group policy on locked-down systems. |
| **Platform** | Windows |

## See Also

| Topic | Link |
|-------|------|
| logging module | [logging.rs](../logging.rs/README.md) |
| enable_debug_privilege | [enable_debug_privilege](enable_debug_privilege.md) |
| winapi module overview | [README](README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*