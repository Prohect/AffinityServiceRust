# purge_fail_map function (logging.rs)

Removes stale entries from the apply-failure tracking map to prevent unbounded memory growth. Called periodically by the main polling loop after enumerating running processes.

## Syntax

```rust
pub fn purge_fail_map(pids_and_names: &[(u32, &str)])
```

## Parameters

`pids_and_names: &[(u32, &str)]`

A slice of `(pid, process_name)` tuples representing the currently running processes. Only entries whose PID **and** process name match a tuple in this slice survive the purge.

## Return value

This function does not return a value. It mutates the global `PID_MAP_FAIL_ENTRY_SET` in place.

## Remarks

### Algorithm

The purge follows a mark-and-sweep pattern:

1. **Mark all dead:** Iterates every entry in the global `PID_MAP_FAIL_ENTRY_SET` map and sets every `alive` flag to `false`.
2. **Re-mark living:** For each `(pid, name)` in `pids_and_names`, if the PID exists in the map and at least one entry's `process_name` matches `name`, the first entry's `alive` flag is set back to `true`.
3. **Sweep:** Calls `retain` on the outer map, removing any PID whose inner map contains no entries with `alive == true`.

### PID reuse awareness

Because Windows PIDs can be recycled, the re-mark step checks the `process_name` field in addition to the PID. If a PID has been reused by a different process, none of its entries will match the new name, and the stale entries will be swept away. This prevents error deduplication from incorrectly suppressing errors for a new process that happens to receive a recycled PID.

### Call frequency

This function is called once per main-loop iteration after the full process list has been enumerated. It acquires the `PID_MAP_FAIL_ENTRY_SET` mutex for the duration of the operation, so it should not be called from within a context that already holds this lock.

### Relationship with is_new_error

While [is_new_error](is_new_error.md) adds entries and handles per-PID process-name changes (clearing the inner map when a name mismatch is detected), `purge_fail_map` handles the complementary case of removing entries for processes that have exited entirely. Together they ensure the map stays bounded by the number of currently running monitored processes.

## Requirements

| | |
|---|---|
| **Module** | `src/logging.rs` |
| **Callers** | Main polling loop (after process enumeration) |
| **Callees** | `get_pid_map_fail_entry_set!()` macro |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [logging.rs](README.md) |
| Error deduplication check | [is_new_error](is_new_error.md) |
| Failure entry key | [ApplyFailEntry](ApplyFailEntry.md) |
| Operations enum | [Operation](Operation.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*