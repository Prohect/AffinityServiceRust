# process_find function (main.rs)

Enumerates all running processes using the Win32 Toolhelp API and logs any process whose CPU affinity matches the system default (all cores), indicating it is not managed by any external tool or policy. This function implements the `-find` mode and is also called at the end of every polling iteration when `-find` is active, continuously discovering new unmanaged processes.

## Syntax

```rust
fn process_find(
    cli: &CliArgs,
    configs: &HashMap<u32, HashMap<String, ProcessConfig>>,
    blacklist: &[String],
) -> Result<(), windows::core::Error>
```

## Parameters

`cli`

A reference to the [CliArgs](../cli.rs/CliArgs.md) structure. The function checks `cli.find_mode`; if `false`, the function is a no-op and returns `Ok(())` immediately.

`configs`

A reference to the grade-keyed configuration map. The outer `HashMap<u32, ...>` is keyed by grade value, and the inner `HashMap<String, ProcessConfig>` is keyed by lowercase process name. Processes that appear in any grade level are considered already managed and are excluded from find results.

`blacklist`

A slice of lowercase process names that should be excluded from discovery. Blacklisted processes are intentionally unmanaged and should not be logged.

## Return value

Returns `Ok(())` on success, or a `windows::core::Error` if `CreateToolhelp32Snapshot` fails. Errors from individual process queries (such as access denied when checking affinity) do not cause the function to return an error; those processes are silently skipped via the fail-find set.

## Remarks

### Enumeration method

Unlike the main polling loop which uses `NtQuerySystemInformation` (via [ProcessSnapshot](../process.rs/ProcessSnapshot.md)) for detailed thread-level data, this function uses the lighter-weight Toolhelp32 snapshot API (`CreateToolhelp32Snapshot` with `TH32CS_SNAPPROCESS`, then `Process32FirstW`/`Process32NextW`). This is sufficient because find mode only needs process names and PIDs — no thread enumeration is required.

The snapshot handle is closed via `CloseHandle` after enumeration completes.

### Filtering pipeline

For each process in the snapshot, the function applies the following filters in order:

1. **Fail-find set** — The process name is checked against a static `HashSet` (`FINDS_FAIL_SET` from the `logging` module). Processes that previously failed affinity queries (e.g., due to access denied) are excluded to avoid repeated log noise.
2. **Config membership** — The process name is checked against all grade levels in `configs`. If a rule exists at any grade, the process is already managed and is skipped.
3. **Blacklist membership** — The process name is checked against `blacklist`. Blacklisted processes are skipped.
4. **Affinity check** — [is_affinity_unset](../winapi.rs/is_affinity_unset.md) is called to determine whether the process's CPU affinity mask matches the full system mask (i.e., all logical processors). Only processes with the default affinity are considered unmanaged.

If a process passes all four filters, it is logged via [log_process_find](../logging.rs/log_process_find.md), which writes to the `.find.log` file and adds the process name to a deduplication set so it is only logged once per session.

### Process name normalization

Process names are extracted from the `PROCESSENTRY32W.szExeFile` field (a null-terminated UTF-16 array), converted to a Rust `String` via `String::from_utf16_lossy`, and lowercased. This ensures case-insensitive matching against config rules and the blacklist.

### Invocation context

This function is called at the end of every polling iteration inside the main loop, after all process-level and thread-level settings have been applied. It is also effectively the sole purpose of the service when running in pure `-find` mode with an empty configuration — the service still polls, but only discovers processes without applying any settings.

### Interaction with `-processlogs`

The `.find.log` files produced by this function's logging are the input for the [process_logs](process_logs.md) function (invoked via `-processlogs` mode). The typical workflow is:

1. Run with `-find` for a period to accumulate `.find.log` data.
2. Run with `-processlogs` to analyze the logs, filter against config/blacklist, and resolve executable paths via Everything search.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `main` |
| Callers | [main](main.md) (end of each polling iteration) |
| Callees | `CreateToolhelp32Snapshot`, `Process32FirstW`, `Process32NextW`, `CloseHandle`, [is_affinity_unset](../winapi.rs/is_affinity_unset.md), [log_process_find](../logging.rs/log_process_find.md) |
| API | Win32 Toolhelp32 (`TH32CS_SNAPPROCESS`), `GetProcessAffinityMask` (via `is_affinity_unset`) |
| Privileges | `SeDebugPrivilege` (recommended for querying affinity of protected processes) |

## See Also

| Topic | Link |
|-------|------|
| Log processing for find results | [process_logs](process_logs.md) |
| Affinity check helper | [is_affinity_unset](../winapi.rs/is_affinity_unset.md) |
| Find logging function | [log_process_find](../logging.rs/log_process_find.md) |
| CLI arguments and find mode flag | [CliArgs](../cli.rs/CliArgs.md) |
| Main entry point and polling loop | [main](main.md) |