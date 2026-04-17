# process_find function (main.rs)

Enumerates all running processes using the Windows Toolhelp32 snapshot API and logs any process that is not already covered by the loaded configuration, not present in the blacklist, and whose affinity mask has not been explicitly set. This function implements the `-find` CLI mode, which helps administrators discover processes that may benefit from affinity or priority tuning.

## Syntax

```rust
fn process_find(
    cli: &CliArgs,
    configs: &ConfigResult,
    blacklist: &[String],
) -> Result<(), windows::core::Error>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `cli` | `&CliArgs` | Parsed command-line arguments. The function checks `cli.find_mode`; if `false`, the function returns immediately without enumerating processes. |
| `configs` | `&ConfigResult` | The fully-parsed configuration result containing both `process_level_configs` and `thread_level_configs` keyed by grade and process name. Used to determine whether a discovered process is already managed. |
| `blacklist` | `&[String]` | A slice of lowercase process-name strings that should be silently ignored during discovery. |

## Return value

Returns `Ok(())` on success. Returns `Err(windows::core::Error)` if the call to `CreateToolhelp32Snapshot` fails.

## Remarks

- The function only executes its body when `cli.find_mode` is `true`. Otherwise it is a no-op that returns `Ok(())`.
- Internally the function calls `CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)` to obtain a snapshot of all running processes, then iterates with `Process32FirstW` / `Process32NextW`.
- Each process name is converted to lowercase via `String::from_utf16_lossy` and truncated at the first null character in the `szExeFile` buffer.
- A process is logged (via `log_process_find`) only if **all** of the following conditions are met:
  1. The process name is **not** in the internal fail-find set (a set of names that previously failed discovery and are suppressed to reduce log noise).
  2. The process name does **not** appear in any grade of `configs.process_level_configs` or `configs.thread_level_configs`.
  3. The process name is **not** contained in `blacklist`.
  4. `is_affinity_unset(pid, name)` returns `true`, meaning the process still has the system-default affinity mask and has not been modified by another tool.
- The snapshot handle is closed with `CloseHandle` after iteration completes.
- This function is called once per main-loop iteration regardless of grade, so discovery output can appear at every polling interval.
- All Win32 calls within this function are made inside an `unsafe` block.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `main.rs` |
| Callers | [main](main.md) (called once per polling iteration) |
| Callees | `CreateToolhelp32Snapshot`, `Process32FirstW`, `Process32NextW`, `CloseHandle`, [is_affinity_unset](../winapi.rs/is_affinity_unset.md), [log_process_find](../logging.rs/log_process_find.md) |
| Privileges | `SeDebugPrivilege` is recommended for full process enumeration visibility |
| Platform | Windows only (`windows` crate, `Win32::System::Diagnostics::ToolHelp`) |

## See Also

| Topic | Link |
|-------|------|
| main entry point | [main](main.md) |
| process_logs (companion discovery mode) | [process_logs](process_logs.md) |
| apply_config | [apply_config](apply_config.md) |
| scheduler module | [scheduler.rs README](../scheduler.rs/README.md) |
| priority module | [priority.rs README](../priority.rs/README.md) |

---
*Commit: [37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
