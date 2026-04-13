# log_process_find function (logging.rs)

Logs a discovered process name during `-find` mode, deduplicated per session. On the first call for a given process name, the function writes a `find <process_name>` entry to the find-mode log (via [log_to_find](log_to_find.md)); subsequent calls with the same name are silently ignored. This ensures the find log contains a clean, unique list of all processes observed during the session without repetition from the polling loop.

## Syntax

```logging.rs
#[inline]
pub fn log_process_find(process_name: &str)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `process_name` | `&str` | The name of the discovered process to log. This value is inserted into [FINDS_SET](FINDS_SET.md) for deduplication and formatted into the log line as `"find <process_name>"`. |

## Return value

*(none)*

## Remarks

- The function locks [FINDS_SET](FINDS_SET.md) and calls `HashSet::insert`. If `insert` returns `true` (the name was not already present), the function delegates to [log_to_find](log_to_find.md) with a formatted message of the form `"find <process_name>"`. If `insert` returns `false` (the name was already logged), the function returns without producing any output.
- The deduplication is per-session (per-process lifetime), not per-day. If the service restarts on the same calendar day, the new process begins with an empty [FINDS_SET](FINDS_SET.md) and will re-log all discovered processes. This is by design: each run should produce a self-contained discovery report.
- The function is annotated `#[inline]`, hinting the compiler to inline it at call sites. Because the function body is small (one lock acquisition, one conditional log call), inlining avoids the overhead of a function call in the hot polling path.
- The process name is stored as-is in `FINDS_SET` without lowercasing or normalization. The caller (typically [process_find](../main.rs/README.md)) is responsible for providing the name in the expected format.
- Because [log_to_find](log_to_find.md) internally checks [USE_CONSOLE](USE_CONSOLE.md), the output destination (console or `logs/YYYYMMDD.find.log`) is determined by that flag. The timestamp prefix `[HH:MM:SS]` is added by `log_to_find`.

### Example output

For a first-time discovery of `notepad.exe`, the find log receives a line such as:

```/dev/null/example.log#L1-1
[09:15:42]find notepad.exe
```

A subsequent call with `"notepad.exe"` during the same session produces no output.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `logging` |
| Callers | [process_find](../main.rs/README.md) |
| Callees | [log_to_find](log_to_find.md) |
| Reads | [FINDS_SET](FINDS_SET.md) (locks and inserts) |
| Respects | [USE_CONSOLE](USE_CONSOLE.md) (indirectly, via `log_to_find`) |

## See Also

| Topic | Link |
|-------|------|
| Deduplication set for successful finds | [FINDS_SET](FINDS_SET.md) |
| Timestamped find-mode log writer | [log_to_find](log_to_find.md) |
| Find-mode log file handle | [FIND_LOG_FILE](FIND_LOG_FILE.md) |
| Deduplication set for failed finds | [FINDS_FAIL_SET](FINDS_FAIL_SET.md) |
| Find-mode entry point in main | [process_find](../main.rs/README.md) |
| logging module overview | [logging module](README.md) |