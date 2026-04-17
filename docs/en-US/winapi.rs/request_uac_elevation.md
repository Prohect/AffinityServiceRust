# request_uac_elevation function (winapi.rs)

Restarts the current process with administrator privileges by launching an elevated copy via PowerShell's `Start-Process -Verb RunAs`, which triggers a Windows UAC (User Account Control) prompt. The current process exits after spawning the elevated child.

## Syntax

```rust
pub fn request_uac_elevation(console: bool) -> io::Result<()>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `console` | `bool` | Indicates whether the process is running in console mode. When `true`, a warning is logged that elevated output will not appear in the current console session. |

## Return value

Returns `io::Result<()>`. On success, this function **does not return** — it calls `std::process::exit(0)` after spawning the elevated child process. On failure (e.g., PowerShell could not be launched), returns an `io::Error` describing the spawn failure.

## Remarks

### Elevation mechanism

The function constructs a PowerShell command of the form:

```text
powershell.exe -Command "Start-Process -FilePath '<exe_path>' -Verb RunAs -ArgumentList '<args>'"
```

Where `<exe_path>` is the path of the currently running executable (obtained via `std::env::current_exe()`) and `<args>` are all command-line arguments forwarded from the current invocation.

### Skip-log flag

Before spawning the elevated child, the function appends `-skip_log_before_elevation` to the argument list. This flag prevents the elevated instance from duplicating any startup log messages that the non-elevated instance already wrote.

### Console mode warning

When `console` is `true`, the function logs a warning that the elevated process will run in a new window/session, so subsequent log output will not be visible in the original console. This is an inherent limitation of UAC elevation — the new process gets a new console host.

### Process lifecycle

1. The function logs `"Requesting UAC elevation..."`.
2. It spawns `powershell.exe` as a child process with the constructed command.
3. On successful spawn, it logs a confirmation message and calls `exit(0)`.
4. On spawn failure, it logs the error and returns the `io::Error`.

### Edge cases

- If `std::env::current_exe()` fails (e.g., the executable was deleted while running), the function returns the resulting `io::Error` before attempting to spawn PowerShell.
- If the user denies the UAC prompt, the PowerShell `Start-Process` command fails silently from the perspective of the original process (which has already exited).
- The function does **not** wait for the elevated child to start or confirm success before exiting.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `winapi.rs` |
| **Callers** | `main.rs` — startup logic when admin privileges are required but not held |
| **Callees** | `std::env::current_exe`, `std::env::args`, `std::process::Command::spawn`, `std::process::exit`, [`log_message`](../logging.rs/log_message.md) (via `log!` macro) |
| **External** | `powershell.exe` must be available on the system PATH |
| **Platform** | Windows only |
| **Privileges** | None required to call; triggers UAC prompt for the user to grant admin privileges to the new process |

## See Also

| Topic | Link |
|-------|------|
| is_running_as_admin | [is_running_as_admin](is_running_as_admin.md) |
| enable_debug_privilege | [enable_debug_privilege](enable_debug_privilege.md) |
| enable_inc_base_priority_privilege | [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md) |
| terminate_child_processes | [terminate_child_processes](terminate_child_processes.md) |
| logging module | [logging.rs](../logging.rs/README.md) |

---
*Commit: [37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
