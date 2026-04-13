# ApplyConfigResult struct (apply.rs)

Accumulates human-readable change descriptions and error messages produced while applying a single configuration pass to a process. Every `apply_*` function in the [apply](README.md) module receives an `&mut ApplyConfigResult` and pushes entries into it rather than logging directly, giving callers in [main.rs](../main.rs/README.md) a consolidated view of what happened.

## Syntax

```AffinityServiceRust/src/apply.rs#L29-33
#[derive(Debug, Default)]
pub struct ApplyConfigResult {
    pub changes: Vec<String>,
    pub errors: Vec<String>,
}
```

## Members

| Member | Type | Description |
|--------|------|-------------|
| `changes` | `Vec<String>` | Successful modifications applied to the process or its threads. Each entry is a short, human-readable description such as `"Priority: Normal -> High"` or `"Thread 1234 -> (promoted, [4,5], cycles=98000, start=ntdll.dll)"`. The caller prefixes the process id and name before writing these to the log. |
| `errors` | `Vec<String>` | Errors encountered during the apply pass. Entries follow the format `"fn_name: [OPERATION][error_message] details"`. Only *new* errors (those not previously seen for the same pid/operation/error-code triple) are added, because all `apply_*` functions route through [log_error_if_new](log_error_if_new.md) before calling `add_error`. |

## Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `new` | `pub fn new() -> Self` | Creates an empty result. Equivalent to `Self::default()`. |
| `add_change` | `pub fn add_change(&mut self, change: String)` | Pushes a change description onto the `changes` vector. |
| `add_error` | `pub fn add_error(&mut self, error: String)` | Pushes an error description onto the `errors` vector. |
| `is_empty` | `pub fn is_empty(&self) -> bool` | Returns `true` when both `changes` and `errors` are empty, allowing callers to skip logging when nothing happened. |

## Remarks

`ApplyConfigResult` is created once per process per apply cycle in [apply_config_process_level](../main.rs/apply_config_process_level.md) and [apply_config_thread_level](../main.rs/apply_config_thread_level.md). After all `apply_*` calls return, the caller inspects `is_empty()` to decide whether to emit a log line. Changes and errors are printed together, giving operators a single consolidated summary per process per cycle.

The struct deliberately uses `String` rather than structured error types. This keeps the apply functions simple—they format context (pid, thread id, operation, Win32 error message) at the call site—and avoids coupling the logging layer to specific error enumerations.

The `#[derive(Default)]` implementation produces an instance with two empty `Vec`s, so `new()` is a thin wrapper provided for readability.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `apply` |
| Callers | [apply_config_process_level](../main.rs/apply_config_process_level.md), [apply_config_thread_level](../main.rs/apply_config_thread_level.md) |
| Passed to | Every `apply_*` function in [apply](README.md) |

## See Also

| Topic | Link |
|-------|------|
| apply module overview | [apply](README.md) |
| Error deduplication helper | [log_error_if_new](log_error_if_new.md) |
| Process-level orchestration | [apply_config_process_level](../main.rs/apply_config_process_level.md) |
| Thread-level orchestration | [apply_config_thread_level](../main.rs/apply_config_thread_level.md) |