# is_new_error function (logging.rs)

Determines whether a particular operation failure is being seen for the first time for a given process, enabling the caller to log only novel errors and suppress repeated identical failures.

## Syntax

```rust
pub fn is_new_error(
    pid: u32,
    tid: u32,
    process_name: &str,
    operation: Operation,
    error_code: u32,
) -> bool
```

## Parameters

`pid: u32`

The process identifier that experienced the failure. Used as the top-level key in the global `PID_MAP_FAIL_ENTRY_SET` map.

`tid: u32`

The thread identifier associated with the failure. For process-level operations where no specific thread is involved, callers typically pass `0`.

`process_name: &str`

The executable name of the process (e.g. `"explorer.exe"`). Used both as part of the deduplication key and to detect PID reuse — if the stored entries for a PID have a different process name, the entire entry set for that PID is cleared before inserting the new entry.

`operation: Operation`

The [Operation](Operation.md) variant identifying which Windows API call failed.

`error_code: u32`

The Win32 or NTSTATUS error code returned by the failed API call. If no error code is available from context, callers pass `0` or a custom discriminator to distinguish different failure modes.

## Return value

`bool` — Returns `true` if this is the first time this exact `(pid, tid, process_name, operation, error_code)` combination has been recorded, meaning the caller should log it. Returns `false` if the failure was already tracked, meaning the caller should suppress the log.

## Remarks

### Deduplication strategy

The function maintains a two-level map via the global static `PID_MAP_FAIL_ENTRY_SET`:

```
HashMap<u32, HashMap<ApplyFailEntry, bool>>
  ^pid         ^(tid, process_name, operation, error_code) -> alive flag
```

Each [ApplyFailEntry](ApplyFailEntry.md) is equality-compared by all four fields (`tid`, `process_name`, `operation`, `error_code`). The `bool` value is the "alive" flag used by [purge_fail_map](purge_fail_map.md) during garbage collection.

### PID reuse detection

**Invariant:** All entries in a PID's fail-entry set are expected to share the same `process_name`. When the function finds that existing entries for a PID have a different process name than the one provided, it clears the entire entry set for that PID before inserting the new entry. This handles the Windows PID-reuse scenario where a terminated process's PID is reassigned to a new, unrelated process.

### Alive flag management

When an existing entry is found (duplicate), its alive flag is set to `true`. This ensures that the entry survives the next [purge_fail_map](purge_fail_map.md) cycle, which marks all entries dead and then re-marks active ones.

### Thread safety

The function acquires the global `PID_MAP_FAIL_ENTRY_SET` mutex via the `get_pid_map_fail_entry_set!()` macro. The lock is held for the duration of the lookup and potential insertion.

## Requirements

| | |
|---|---|
| **Module** | `src/logging.rs` |
| **Callers** | [log_error_if_new](../apply.rs/log_error_if_new.md), apply functions in [apply.rs](../apply.rs/README.md) |
| **Callees** | `get_pid_map_fail_entry_set!()` macro |
| **Win32 API** | None |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [logging.rs](README.md) |
| Failure entry key struct | [ApplyFailEntry](ApplyFailEntry.md) |
| Operation enum | [Operation](Operation.md) |
| Garbage collection for stale entries | [purge_fail_map](purge_fail_map.md) |
| Caller-side error logging helper | [log_error_if_new](../apply.rs/log_error_if_new.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*