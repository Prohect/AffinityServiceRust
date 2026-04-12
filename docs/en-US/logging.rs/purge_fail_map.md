# purge_fail_map function (logging.rs)

Removes stale error deduplication entries from [`PID_MAP_FAIL_ENTRY_SET`](PID_MAP_FAIL_ENTRY_SET.md) for processes that are no longer running, preventing unbounded memory growth.

## Syntax

```rust
pub fn purge_fail_map(pids_and_names: &[(u32, String)])
```

## Parameters

`pids_and_names`

A slice of `(pid, process_name)` tuples representing the currently alive processes. Any PID in [`PID_MAP_FAIL_ENTRY_SET`](PID_MAP_FAIL_ENTRY_SET.md) that is **not** present in this list will have its error entries removed.

## Return value

This function does not return a value.

## Remarks

The error deduplication system in [`PID_MAP_FAIL_ENTRY_SET`](PID_MAP_FAIL_ENTRY_SET.md) accumulates entries over time as new errors are encountered for each process/thread/operation combination. Without periodic cleanup, this map would grow without bound as processes start and stop throughout the lifetime of the application.

`purge_fail_map` is called once per main loop iteration from [`main`](../main.rs/main.md), after the current process snapshot has been taken. The caller provides the list of currently alive PIDs, and the function removes all entries for PIDs not in that list.

### Algorithm

1. Acquire a lock on [`PID_MAP_FAIL_ENTRY_SET`](PID_MAP_FAIL_ENTRY_SET.md).
2. Collect the set of PIDs currently present as outer keys in the map.
3. For each PID in the map, check whether it exists in the `pids_and_names` input.
4. Remove the entire inner `HashMap<ApplyFailEntry, bool>` for any PID that is no longer alive.
5. Release the lock.

This approach ensures that when a process terminates, all of its accumulated error history is discarded. If the same executable is later relaunched (with a new PID), errors will be logged fresh for the new instance, which is the desired behavior since the new process may have different access characteristics.

### Timing

It is important that this function is called **after** the process snapshot is taken each loop iteration. Calling it before the snapshot could remove entries for processes that are still alive but not yet enumerated. The snapshot provides the authoritative list of running processes used as the retention set.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/logging.rs |
| **Source lines** | L151–L172 |
| **Called by** | [`main`](../main.rs/main.md) loop |
| **Modifies** | [`PID_MAP_FAIL_ENTRY_SET`](PID_MAP_FAIL_ENTRY_SET.md) |

## See also

- [PID_MAP_FAIL_ENTRY_SET static](PID_MAP_FAIL_ENTRY_SET.md)
- [is_new_error function](is_new_error.md)
- [ApplyFailEntry struct](ApplyFailEntry.md)
- [logging.rs module overview](README.md)