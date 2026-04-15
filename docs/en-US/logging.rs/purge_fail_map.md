# purge_fail_map function (logging.rs)

Removes stale entries from the per-PID apply-failure tracking map ([`PID_MAP_FAIL_ENTRY_SET`](statics.md#pid_map_fail_entry_set)). This function implements a mark-and-sweep garbage collection strategy: it marks all entries as dead, re-marks entries belonging to currently running processes as alive, and then removes any entries that remain dead. This prevents the failure tracking map from growing unboundedly as processes start and stop over time.

## Syntax

```rust
pub fn purge_fail_map(pids_and_names: &[(u32, &str)])
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pids_and_names` | `&[(u32, &str)]` | A slice of `(pid, process_name)` tuples representing the currently running processes that should be retained in the failure tracking map. Entries not matching any tuple in this slice are considered stale and will be removed. |

## Return value

This function does not return a value.

## Remarks

### Algorithm

The function implements a three-phase mark-and-sweep approach:

1. **Mark all dead.** Iterate over every entry in every PID's failure set and set its `alive` flag to `false`.

2. **Re-mark alive.** For each `(pid, name)` in `pids_and_names`:
   - Look up the PID in the failure map.
   - If a failure set exists for that PID **and** at least one entry in the set has a `process_name` matching the provided `name`, mark the first entry in the set as alive (`true`). The name check ensures that PID reuse (where a new process gets the same PID as a terminated one with a different name) does not incorrectly keep stale entries alive.

3. **Sweep.** Call `HashMap::retain` to remove all PID entries where **no** failure entry has its `alive` flag set to `true`. This removes entries for PIDs that are no longer running or whose process names have changed.

### Locking

The function acquires the `PID_MAP_FAIL_ENTRY_SET` mutex via the `get_pid_map_fail_entry_set!()` macro and holds the lock for the entire duration of the purge operation. This ensures consistency between the mark and sweep phases.

### Interaction with is_new_error

This function complements [`is_new_error`](is_new_error.md). While `is_new_error` **adds** entries to the failure map when new failures are encountered, `purge_fail_map` **removes** entries that are no longer relevant. Together, they implement a bounded error-deduplication system:

- `is_new_error` ensures each unique failure is logged only once.
- `purge_fail_map` ensures the tracking data does not accumulate indefinitely.

### Call frequency

This function is typically called once per scheduling loop iteration, after the process snapshot has been taken, with the list of currently active processes that match configuration rules. This ensures that failure tracking data for exited processes is cleaned up promptly.

### Edge cases

- If `pids_and_names` is empty, all entries in the failure map are marked dead and subsequently removed during the sweep phase.
- If a PID exists in the failure map but the process name does not match the name in `pids_and_names` (e.g., PID reuse), the entries for that PID are **not** re-marked as alive and will be swept. When `is_new_error` is called later for the new process occupying that PID, it will clear any stale entries with mismatched names and start fresh.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `logging.rs` |
| **Callers** | `scheduler.rs` — main scheduling loop cleanup phase |
| **Callees** | `get_pid_map_fail_entry_set!()` macro → `PID_MAP_FAIL_ENTRY_SET.lock()` |
| **Statics** | [`PID_MAP_FAIL_ENTRY_SET`](statics.md#pid_map_fail_entry_set) |
| **Platform** | Platform-independent logic (data structures are Windows-specific in context) |

## See Also

| Topic | Link |
|-------|------|
| is_new_error function | [is_new_error](is_new_error.md) |
| ApplyFailEntry struct | [ApplyFailEntry](ApplyFailEntry.md) |
| Operation enum | [Operation](Operation.md) |
| PID_MAP_FAIL_ENTRY_SET static | [statics](statics.md#pid_map_fail_entry_set) |
| logging module overview | [README](README.md) |

---
> Commit SHA: `b0df9da35213b050501fab02c3020ad4dbd6c4e0`
