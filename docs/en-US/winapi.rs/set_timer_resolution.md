# set_timer_resolution function (winapi.rs)

Sets the system timer resolution via `NtSetTimerResolution`.

## Syntax

```rust
pub fn set_timer_resolution(cli: &CliArgs)
```

## Parameters

`cli` — Reference to [`CliArgs`](../cli.rs/CliArgs.md) containing the `time_resolution` value (in 100ns ticks).

## Remarks

Calls `NtSetTimerResolution` with the resolution from CLI args. Logs success with the new and previous resolution values, or logs failure with the NTSTATUS code. Previously inline in `main()`, now extracted for clarity.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/winapi.rs |
| **Source lines** | L823–L837 |
| **Called by** | [`main`](../main.rs/main.md) |

## See also

- [winapi.rs module overview](README.md)