# process_find function (main.rs)

Enumerates all running processes via a Win32 toolhelp snapshot and logs any process that has a default (full) CPU affinity mask and is not already covered by the loaded configuration or blacklist. This is the per-iteration companion to `-find` mode, called at the end of each main loop iteration.

## Syntax

```rust
fn process_find(
    cli: &CliArgs,
    configs: &ConfigResult,
    blacklist: &[String],
) -> Result<(), windows::core::Error>
```

## Parameters

`cli: &CliArgs`

The parsed [CLI arguments](../cli.rs/CliArgs.md). Only the `find_mode` flag is inspected — if `false`, the function returns `Ok(())` immediately with no work performed.

`configs: &ConfigResult`

The loaded [ConfigResult](../config.rs/ConfigResult.md). Both `process_level_configs` and `thread_level_configs` are searched across all grades to determine whether a process name is already managed.

`blacklist: &[String]`

A list of lowercase process names that should be silently ignored during discovery. Processes in this list are never logged even if they have default affinity.

## Return value

`Result<(), windows::core::Error>` — Returns `Ok(())` on success. Returns an error only if `CreateToolhelp32Snapshot` fails.

## Remarks

The function performs the following steps when `cli.find_mode` is `true`:

1. **Snapshot** — Calls `CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)` to capture a point-in-time list of all processes.
2. **Iterate** — Walks through each `PROCESSENTRY32W` entry using `Process32FirstW` / `Process32NextW`.
3. **Normalize** — Converts the process name from the null-terminated UTF-16 `szExeFile` field to a lowercase `String`.
4. **Filter — already managed** — Checks whether the process name exists in any grade of `configs.process_level_configs` or `configs.thread_level_configs`. If found, the process is skipped.
5. **Filter — blacklisted** — Checks whether the process name appears in the `blacklist` vector. If found, the process is skipped.
6. **Filter — already logged** — Checks the global `fail_find_set` to avoid logging the same process name repeatedly within a session.
7. **Filter — affinity check** — Calls [`is_affinity_unset`](../winapi.rs/is_affinity_unset.md) to determine whether the process has a full (default) affinity mask. Only processes with unmodified affinity are considered "unmanaged" and worth logging.
8. **Log** — Calls [`log_process_find`](../logging.rs/log_process_find.md) to write the discovered process name to the `.find.log` file.
9. **Cleanup** — Closes the snapshot handle via `CloseHandle`.

### Deduplication

The `fail_find_set` global prevents the same process name from being logged on every polling iteration. A process name is added to this set the first time it is logged and is not removed until the service restarts. This keeps `.find.log` files concise for later analysis by [`process_logs`](process_logs.md).

### Affinity heuristic

The assumption is that any process still running with the system-default full affinity mask has not been managed by any external tool or by AffinityServiceRust itself. This is a simple heuristic; processes that intentionally use full affinity will also be flagged.

## Requirements

| | |
|---|---|
| **Module** | `src/main.rs` |
| **Callers** | [main](main.md) (end of each loop iteration) |
| **Callees** | `CreateToolhelp32Snapshot`, `Process32FirstW`, `Process32NextW`, `CloseHandle`, [`is_affinity_unset`](../winapi.rs/is_affinity_unset.md), [`log_process_find`](../logging.rs/log_process_find.md) |
| **Win32 API** | [`CreateToolhelp32Snapshot`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-createtoolhelp32snapshot), [`Process32FirstW`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-process32firstw), [`Process32NextW`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-process32nextw) |
| **Privileges** | None beyond what was already acquired at startup (debug privilege enables broader process visibility) |

## See Also

| Topic | Link |
|-------|------|
| Post-session log analysis | [process_logs](process_logs.md) |
| Affinity check helper | [is_affinity_unset](../winapi.rs/is_affinity_unset.md) |
| Find-mode logger | [log_process_find](../logging.rs/log_process_find.md) |
| CLI flags | [CliArgs](../cli.rs/CliArgs.md) |
| Module overview | [main.rs](README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*