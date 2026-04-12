# FINDS_FAIL_SET static (logging.rs)

Tracks process names that failed to be found during process discovery, preventing duplicate "not found" messages from being logged on every loop iteration.

## Syntax

```rust
static FINDS_FAIL_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));
```

## Members

The static holds a `HashSet<String>` behind a `Mutex`. Each entry is a process name that was searched for but not found in the system's running process list.

## Remarks

When the application is configured to find specific processes (via find mode or the main loop's process matching), some configured process names may not correspond to any currently running process. Without deduplication, the "not found" message would be logged every loop iteration, creating excessive log noise.

`FINDS_FAIL_SET` stores the names of processes that have already had a "not found" message logged. On subsequent loop iterations, the set is checked before logging, and the message is suppressed if the name is already present.

When a previously-missing process is eventually found running, its name should be removed from this set so that future disappearances can be logged again.

### Thread safety

All access is synchronized through the `Mutex`. The lock is acquired briefly for each check-and-insert operation.

### Relationship to FINDS_SET

While [`FINDS_SET`](FINDS_SET.md) tracks processes that *were* successfully found (for deduplicating positive discovery messages), `FINDS_FAIL_SET` tracks processes that were *not* found (for deduplicating negative discovery messages). Together, they ensure that both positive and negative discovery events are logged exactly once.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/logging.rs |
| **Source line** | L69 |
| **Used by** | [`log_process_find`](log_process_find.md), main loop process matching |

## See also

- [FINDS_SET static](FINDS_SET.md)
- [log_process_find function](log_process_find.md)
- [logging.rs module overview](README.md)