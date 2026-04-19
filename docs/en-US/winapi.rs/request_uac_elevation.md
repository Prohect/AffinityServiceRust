# request_uac_elevation function (winapi.rs)

Re-launches the current process with administrator privileges by spawning a PowerShell `Start-Process -Verb RunAs` command, which triggers a Windows UAC consent prompt. After the elevated child process is launched, the current (non-elevated) process exits.

## Syntax

```rust
pub fn request_uac_elevation(console: bool) -> io::Result<()>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `console` | `bool` | Whether the service was launched with the `console` CLI flag. When `true`, a warning is logged that the elevated process's output will appear in a new console window, not the current one. |

## Return value

`io::Result<()>` — On success, this function **does not return** because the current process calls `std::process::exit(0)` after spawning the elevated child. On failure (e.g., PowerShell could not be launched), returns an `io::Error` describing the spawn failure.

## Remarks

### Elevation mechanism

The function constructs a PowerShell command of the form:

```
Start-Process -FilePath '<current_exe_path>' -Verb RunAs -ArgumentList '<original_args> -skip_log_before_elevation'
```

- The current executable path is obtained via `std::env::current_exe()`.
- All original command-line arguments (skipping `argv[0]`) are forwarded to the elevated instance.
- The flag `-skip_log_before_elevation` is appended to prevent the elevated process from duplicating log initialization that already occurred in the non-elevated instance.

### Console mode warning

When `console` is `true`, the function logs a warning explaining that the elevated process will run in a separate console window. This is inherent to the `runas` verb — Windows creates a new console for the elevated process, and the current console session receives no further output from the service.

### Process exit

After a successful `Command::spawn()`, the current process calls `exit(0)` immediately. This ensures there is only one instance of the service running (the newly elevated one). The exit is unconditional — cleanup code after this function call in the caller will not execute.

### Error handling

If `Command::spawn()` fails (e.g., PowerShell is not found, or the user declines the UAC prompt before the process starts), the error is logged and propagated as `io::Result<Err>`. The caller can then decide whether to continue running without elevation or abort.

### UAC prompt behavior

The actual UAC consent dialog is presented by Windows when the elevated PowerShell instance executes the `Start-Process -Verb RunAs` command. If the user clicks **No** or the dialog times out, the `Start-Process` cmdlet fails silently from the perspective of the spawning (non-elevated) process — the `spawn()` call itself succeeds because it only launches PowerShell, not the elevated target. The current process will have already exited by this point.

## Requirements

| | |
|---|---|
| **Module** | `src/winapi.rs` |
| **Callers** | Main startup logic in `src/main.rs`, guarded by `!is_running_as_admin() && !cli.no_uac` |
| **Callees** | `std::env::current_exe`, `std::env::args`, `std::process::Command::spawn`, `std::process::exit` |
| **Win32 API** | None directly; relies on PowerShell's `Start-Process -Verb RunAs` which internally calls [ShellExecuteW](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shellexecutew) |
| **Privileges** | None required to invoke; the UAC prompt is what grants elevation to the child process |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [winapi.rs](README.md) |
| Admin check that gates this call | [is_running_as_admin](is_running_as_admin.md) |
| Debug privilege enabled after elevation | [enable_debug_privilege](enable_debug_privilege.md) |
| Base priority privilege enabled after elevation | [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md) |
| Child process cleanup on startup | [terminate_child_processes](terminate_child_processes.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*