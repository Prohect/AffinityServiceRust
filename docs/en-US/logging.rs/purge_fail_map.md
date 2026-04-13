# purge_fail_map function (logging.rs)

Removes stale entries from the per-PID operation failure tracking map ([PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md)). This function implements a mark-and-sweep garbage collection strategy: it first marks all entries as dead, then re-marks entries belonging to currently running processes as alive, and finally removes any entries that remain dead. This prevents unbounded growth of the failure map as processes start and stop over the lifetime of the service.

## Syntax

```logging.rs
#[inline]
pub fn purge_fail_map(pids_and_names: &[(u32, String)])
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pids_and_names` | `&[(u32, String)]` | A slice of `(pid, process_name)` tuples representing the processes that are currently running. Each PID whose failure entries should be retained must appear in this list with a matching process name. |

## Return value

This function does not return a value.

## Remarks

### Algorithm

The function executes a three-phase mark-and-sweep on the global [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) map:

1. **Mark all dead:** Iterates over every inner `HashMap<ApplyFailEntry, bool>` and sets all `bool` values (the "alive" flags) to `false`.
2. **Re-mark alive:** For each `(pid, name)` pair in `pids_and_names`, looks up the PID in the outer map. If found, and if any entry in that PID's inner map has a `process_name` matching `name`, it marks the first entry in the inner map as alive (`true`). This confirms that the PID is still associated with the same process and its failure records should be retained.
3. **Sweep:** Calls `map.retain(…)` to remove any outer map entries whose inner maps contain no alive entries. This drops all failure tracking state for processes that have exited.

### Design rationale

- The service's polling loop calls `purge_fail_map` periodically (typically once per iteration) with the current snapshot of running processes. This ensures that failure entries for terminated processes do not accumulate indefinitely.
- The alive flag mechanism avoids the need to rebuild the entire map on each iteration. Most entries are simply re-marked as alive, and only entries for exited processes are removed.
- The function acquires the [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) mutex via the [get_pid_map_fail_entry_set!](get_pid_map_fail_entry_set.md) macro and holds it for the duration of the purge. Because this operation touches every entry in the map, the lock is held for a duration proportional to the number of tracked PIDs and their failure entries.

### Process name matching

The re-mark phase checks that at least one [ApplyFailEntry](ApplyFailEntry.md) in the PID's inner map has a `process_name` field matching the name from `pids_and_names`. This guards against PID reuse: if a PID is recycled by the OS and assigned to a different process, the old failure entries (with the previous process name) will not match the new name, will remain marked dead, and will be swept away.

### Interaction with is_new_error

[is_new_error](is_new_error.md) is responsible for inserting new entries and performing deduplication checks, while `purge_fail_map` is responsible for cleaning up after processes exit. Together, they form a complete lifecycle for failure tracking entries:

- `is_new_error` → inserts entries, marks them alive, returns `true` for new errors
- `purge_fail_map` → marks entries dead, re-marks alive ones, removes dead ones

### Inline hint

The function is annotated with `#[inline]`, suggesting to the compiler that it may benefit from inlining at the call site. In practice, the function is called from a single location in the main loop, so this hint primarily serves as a documentation signal that the function is small and performance-sensitive.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `logging` |
| Reads / writes | [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) via [get_pid_map_fail_entry_set!](get_pid_map_fail_entry_set.md) |
| Callers | Main service loop in [main module](../main.rs/README.md) |
| Callees | *(none — operates on in-memory data structures only)* |
| Related insert logic | [is_new_error](is_new_error.md) |

## See Also

| Topic | Link |
|-------|------|
| Error deduplication check and insertion | [is_new_error](is_new_error.md) |
| Global failure tracking map | [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) |
| Failure entry key struct | [ApplyFailEntry](ApplyFailEntry.md) |
| Windows API operation identifiers | [Operation](Operation.md) |
| logging module overview | [logging module](README.md) |