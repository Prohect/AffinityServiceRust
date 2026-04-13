# FINDS_FAIL_SET static (logging.rs)

A global, lazily initialized, mutex-guarded `HashSet<String>` that tracks process names for which `-find` mode discovery has already failed during the current session. This prevents repeated logging of the same failed find operation, keeping log output concise when processes cannot be matched.

## Syntax

```logging.rs
pub static FINDS_FAIL_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));
```

## Members

| Component | Type | Description |
|-----------|------|-------------|
| Outer | `Lazy<…>` | Deferred initialization via `once_cell::sync::Lazy`. The `HashSet` is created empty on first access. |
| Middle | `Mutex<…>` | Provides interior mutability and thread-safe access from the service loop. |
| Inner | `HashSet<String>` | Contains process name strings for which a find failure has already been logged. |

## Remarks

- `FINDS_FAIL_SET` complements [FINDS_SET](FINDS_SET.md), which deduplicates *successful* process discoveries. Together, they ensure that both successful and failed find operations are logged at most once per session for each unique process name.
- The set is accessed through the [get_fail_find_set!](get_fail_find_set.md) macro, which acquires the mutex lock and returns a `MutexGuard<HashSet<String>>`.
- The set is never explicitly cleared during normal operation — it accumulates entries for the lifetime of the service process. This is acceptable because the number of distinct process names in a typical configuration is small (hundreds at most).
- Entries are plain `String` values representing lowercased or as-configured process names, matching the format used by the configuration parser.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `logging` |
| Crate dependencies | `once_cell` (`Lazy`), `std::sync::Mutex`, `std::collections::HashSet` |
| Accessor macro | [get_fail_find_set!](get_fail_find_set.md) |
| Callers | [process_find](../main.rs/README.md) |

## See Also

| Topic | Link |
|-------|------|
| Successful find deduplication set | [FINDS_SET](FINDS_SET.md) |
| Per-PID operation failure tracking | [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) |
| Find-mode log output function | [log_to_find](log_to_find.md) |
| Find-mode process logging | [log_process_find](log_process_find.md) |
| logging module overview | [logging module](README.md) |