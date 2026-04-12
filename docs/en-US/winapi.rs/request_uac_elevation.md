# request_uac_elevation function (winapi.rs)

Spawns a new elevated (administrator) instance of the application via PowerShell's `Start-Process -Verb RunAs`, triggering the Windows UAC consent dialog.

## Syntax

```rust
pub fn request_uac_elevation(console: bool) -> io::Result<()>
```

## Parameters

`console`

When `true`, the elevated process is launched with a visible console window. When `false`, the elevated process may be launched without a console, depending on the PowerShell invocation flags.

## Return value

Returns `Ok(())` if the elevated process was successfully spawned. Returns an `io::Error` if PowerShell could not be launched or the `Start-Process` command failed (e.g., if the user declined the UAC prompt).

## Remarks

This function is the mechanism by which the application self-elevates to administrator when it detects that it is running without admin privileges. The flow is:

1. The function constructs the current executable path and its command-line arguments.
2. It invokes PowerShell with `Start-Process -Verb RunAs` to re-launch the application with elevation.
3. If the user accepts the UAC consent dialog, a new elevated process starts and the current (non-elevated) process is expected to exit.
4. If the user declines the UAC dialog, PowerShell reports an error and the function returns an `io::Error`.

After successful elevation, the original (non-elevated) process should call [`terminate_child_processes`](terminate_child_processes.md) to clean up any orphaned console host processes that were spawned as part of the PowerShell invocation.

The `console` parameter controls whether the re-launched process gets a visible console window, which is relevant for interactive use versus background/service operation.

**Security note:** UAC elevation is only requested when [`is_running_as_admin`](is_running_as_admin.md) returns `false` and the `--no-uac` CLI flag is not set. The `--no-uac` flag in [`CliArgs`](../cli.rs/CliArgs.md) allows users to suppress elevation for environments where UAC prompts are undesirable.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/winapi.rs |
| **Source lines** | L468–L501 |
| **Called by** | [`main`](../main.rs/main.md) |
| **Calls** | PowerShell `Start-Process -Verb RunAs` |
| **Windows API** | [ShellExecuteW](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shellexecutew) (via PowerShell) |

## See also

- [is_running_as_admin](is_running_as_admin.md)
- [terminate_child_processes](terminate_child_processes.md)
- [CliArgs](../cli.rs/CliArgs.md) (`no_uac` flag)
- [winapi.rs module overview](README.md)