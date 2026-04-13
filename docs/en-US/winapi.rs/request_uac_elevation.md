# request_uac_elevation function (winapi.rs)

Requests User Account Control (UAC) elevation by re-launching the current process with administrator privileges via a PowerShell `Start-Process -Verb RunAs` command. If the elevation request is successfully dispatched, the current (non-elevated) process exits immediately. This function does not return on success.

## Syntax

```rust
pub fn request_uac_elevation(console: bool) -> io::Result<()>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `console` | `bool` | Indicates whether the current process is running with a visible console window. When `true`, an additional warning is logged advising the user that log output after elevation will not appear in the current console session, because the elevated process runs in a new window. |

## Return value

| Value | Description |
|-------|-------------|
| `Ok(())` | Not reachable in practice — the function calls `std::process::exit(0)` after successfully spawning the elevated child process. |
| `Err(io::Error)` | The `powershell.exe` child process could not be spawned (e.g., `powershell.exe` is not on `PATH`, or the system is critically resource-constrained). The error is also logged before being returned. |

## Remarks

### Elevation mechanism

The function constructs a PowerShell command line of the form:

```
powershell.exe -Command "Start-Process -FilePath '<exe_path>' -Verb RunAs -ArgumentList '<args>'"
```

Where:

- `<exe_path>` is the full path to the current executable, obtained via `std::env::current_exe()`.
- `<args>` is the original command-line arguments (minus `argv[0]`) with `-skip_log_before_elevation` appended.

`-Verb RunAs` triggers the Windows UAC consent dialog. If the user approves, Windows launches the new process with a full administrator token. If the user denies elevation, the PowerShell command fails silently (it does not propagate an error back to this process).

### The `-skip_log_before_elevation` flag

Before the elevated child process starts its main loop, it would normally emit startup log messages. The `-skip_log_before_elevation` flag is appended to the argument list to signal to the [CLI parser](../cli.rs/README.md) that this is a re-launch, and certain pre-elevation log entries (such as the "requesting elevation" message) should not be duplicated in the log file.

### Process exit behavior

On success, the function calls `std::process::exit(0)` to terminate the non-elevated parent process. This means:

- **No `Drop` implementations run** for objects alive at the time of the call.
- Any buffered log output that has not been flushed may be lost.
- The caller should ensure critical state is persisted before invoking this function.

### Console warning

When `console` is `true` and the process is not running as administrator without the `noUAC` flag, a warning is logged:

> "Warning: process is running as non-administrator without 'noUAC' flag with 'console' flag, the log after elevation will not be shown in current session."

This is because the elevated process spawns in a new console window, and the user would need to switch to that window to see subsequent output.

### Typical call sequence

In the main loop ([`main.rs`](../main.rs/README.md)):

1. [is_running_as_admin](is_running_as_admin.md) returns `false`.
2. The `noUAC` CLI flag is **not** set.
3. `request_uac_elevation` is called.
4. The current process exits; the elevated child takes over.

### Error conditions

| Scenario | Behavior |
|----------|----------|
| `std::env::current_exe()` fails | Returns `Err(io::Error)` from `current_exe()`. |
| `powershell.exe` cannot be found or spawned | Returns `Err(io::Error)` from `Command::spawn()`. |
| User denies the UAC prompt | The `Start-Process` command fails inside PowerShell, but `Command::spawn()` has already succeeded — the current process has already exited. The elevated child simply does not start. |

## Requirements

| | |
|---|---|
| **Module** | `winapi` (`src/winapi.rs`) |
| **Visibility** | `pub` |
| **Callers** | [`main`](../main.rs/README.md) (startup sequence) |
| **Callees** | `std::env::current_exe`, `std::env::args`, `std::process::Command::spawn`, `std::process::exit` |
| **External programs** | `powershell.exe` (must be on `PATH`) |
| **Privileges** | None required to call; the function **requests** elevation via UAC |

## See Also

| Topic | Link |
|-------|------|
| Admin privilege check | [is_running_as_admin](is_running_as_admin.md) |
| Debug privilege enablement (post-elevation) | [enable_debug_privilege](enable_debug_privilege.md) |
| Base priority privilege enablement (post-elevation) | [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md) |
| CLI argument parsing and flags | [cli module](../cli.rs/README.md) |
| Service main entry point | [main module](../main.rs/README.md) |