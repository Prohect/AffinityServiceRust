# PID_MAP_FAIL_ENTRY_SET static (logging.rs)

Per-PID map of deduplicated error entries used to suppress repeated logging of the same error across loop iterations. This is the core data structure behind the error deduplication system.

## Syntax

```rust
static PID_MAP_FAIL_ENTRY_SET: Lazy<Mutex<HashMap<u32, HashMap<ApplyFailEntry, bool>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
```

## Members

The static holds a `HashMap<u32, HashMap<ApplyFailEntry, bool>>` behind a `Mutex`:

- **Outer key** (`u32`) — the process identifier (PID).
- **Inner key** ([`ApplyFailEntry`](ApplyFailEntry.md)) — the composite error key combining pid, tid, process name, operation, and error code.
- **Inner value** (`bool`) — presence marker; the value itself is not semantically meaningful — only key membership matters.

## Remarks

Every time a Windows API call fails during configuration application, the error details are checked against this map via [`is_new_error`](is_new_error.md). If the exact combination of `(pid, tid, process_name, operation, error_code)` has already been recorded, the error is considered a duplicate and is not logged again. This prevents the log file from being flooded with the same error message every loop iteration (e.g., `ERROR_ACCESS_DENIED` for a protected system process that will never become accessible).

The two-level map structure (PID → set of fail entries) enables efficient per-process cleanup via [`purge_fail_map`](purge_fail_map.md). When a process terminates and is no longer present in the process snapshot, its entire inner map is removed in one operation. This prevents unbounded memory growth as processes come and go over time.

### Lifecycle

1. **Insertion** — [`is_new_error`](is_new_error.md) inserts a new [`ApplyFailEntry`](ApplyFailEntry.md) when an error is seen for the first time and returns `true`.
2. **Lookup** — [`is_new_error`](is_new_error.md) checks for existing entries and returns `false` if the error has already been recorded.
3. **Eviction** — [`purge_fail_map`](purge_fail_map.md) is called each loop iteration with the list of currently alive PIDs. Entries for PIDs not in the alive list are removed.

### Thread safety

All access is synchronized through the `Mutex`. The lock is acquired for the minimum duration necessary — typically a single lookup-or-insert in [`is_new_error`](is_new_error.md) or a single purge pass in [`purge_fail_map`](purge_fail_map.md).

### Memory growth

Without periodic purging, this map would grow without bound as new processes are encountered and new errors accumulate. The [`purge_fail_map`](purge_fail_map.md) function is the primary mechanism for controlling memory usage. It is critical that the main loop calls it each iteration to remove entries for terminated processes.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/logging.rs |
| **Source line** | L70 |
| **Written by** | [`is_new_error`](is_new_error.md) |
| **Purged by** | [`purge_fail_map`](purge_fail_map.md) |

## See also

- [ApplyFailEntry struct](ApplyFailEntry.md)
- [is_new_error function](is_new_error.md)
- [purge_fail_map function](purge_fail_map.md)
- [logging.rs module overview](README.md)