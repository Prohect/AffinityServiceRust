# log_process_find function (logging.rs)

Logs a discovered process name from `-find` mode, deduplicated per session. Uses the `FINDS_SET` static to ensure each unique process name is logged only once during the current application run, preventing log spam from repeatedly discovered processes across scheduling loop iterations.

## Syntax

```rust
#[inline]
pub fn log_process_find(process_name: &str)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `process_name` | `&str` | The name of the discovered process to log. This value is used both as the deduplication key (inserted into `FINDS_SET`) and as the log message payload. |

## Return value

This function does not return a value.

## Remarks

### Deduplication mechanism

The function locks the global [FINDS_SET](statics.md#finds_set) (`Mutex<HashSet<String>>`) and attempts to insert the given `process_name`. Because `HashSet::insert` returns `true` only when the value was not already present, the log write via [`log_to_find`](log_to_find.md) is executed only on the first occurrence of each process name per session.

### Algorithm

1. Lock `FINDS_SET`.
2. Call `insert(process_name.to_string())` on the set.
3. If `insert` returned `true` (the name was new), call [`log_to_find`](log_to_find.md) with the message `"find <process_name>"`.
4. If `insert` returned `false` (the name was already in the set), do nothing.

### Log output format

When a new process name is logged, the output line has the form:

```text
[HH:MM:SS]find <process_name>
```

The timestamp prefix is added by [`log_to_find`](log_to_find.md). The message is written to either the `.find` log file or stdout, depending on the value of [USE_CONSOLE](statics.md#use_console).

### Performance

The function is marked `#[inline]`, allowing the compiler to inline the call at the call site. The deduplication check is O(1) amortized (hash-set lookup), making it cheap to call on every scheduling cycle for every discovered process.

### Session scope

The `FINDS_SET` is never cleared during normal operation — it accumulates process names for the entire lifetime of the application. This means that if a process exits and restarts, it will **not** be re-logged in the same session. A new session (application restart) begins with an empty set.

### Relationship to `-find` mode

The `-find` CLI mode scans for processes whose CPU affinity has not been explicitly configured (i.e., whose affinity matches the system default). For each such process, `log_process_find` is called so the user can see which processes are "unconfigured" without being flooded by repeated entries on every polling cycle.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `logging.rs` |
| **Callers** | `apply.rs`, `scheduler.rs` — find-mode scanning logic |
| **Callees** | [`log_to_find`](log_to_find.md) |
| **Statics** | [FINDS_SET](statics.md#finds_set) |
| **Platform** | Cross-platform (no direct Windows API calls) |

## See Also

| Topic | Link |
|-------|------|
| log_to_find function | [log_to_find](log_to_find.md) |
| log_message function | [log_message](log_message.md) |
| FINDS_SET static | [statics](statics.md#finds_set) |
| is_affinity_unset function | [is_affinity_unset](../winapi.rs/is_affinity_unset.md) |
| logging module overview | [README](README.md) |

---
*Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
