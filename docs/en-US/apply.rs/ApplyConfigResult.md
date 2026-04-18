# ApplyConfigResult type (apply.rs)

The `ApplyConfigResult` struct accumulates human-readable change descriptions and error messages produced during a single configuration-apply pass for one process. Each function in the `apply` module receives a mutable reference to an `ApplyConfigResult` and appends entries to record what was changed or what failed. After all apply functions have run, the caller inspects the result to emit log output or take corrective action.

## Syntax

```AffinityServiceRust/src/apply.rs#L32-35
#[derive(Debug, Default)]
pub struct ApplyConfigResult {
    pub changes: Vec<String>,
    pub errors: Vec<String>,
}
```

## Members

| Member | Type | Description |
|--------|------|-------------|
| `changes` | `Vec<String>` | A list of human-readable strings describing each configuration change that was successfully applied (or would be applied in dry-run mode). Each entry follows the format `"$operation details"` and is intended to be prefixed by the caller with the process ID and config name. |
| `errors` | `Vec<String>` | A list of human-readable strings describing each error that occurred during the apply pass. Each entry follows the format `"$fn_name: [$operation][$error_message] details"`. Errors are deduplicated at the call site via [`log_error_if_new`](log_error_if_new.md) so that repeated failures for the same pid/operation/error-code are not re-added. |

## Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `new` | `pub fn new() -> Self` | Creates a new, empty `ApplyConfigResult`. Delegates to `Default::default()`. |
| `add_change` | `pub fn add_change(&mut self, change: String)` | Appends a change description string to the `changes` vector. Marked `#[inline(always)]`. |
| `add_error` | `pub fn add_error(&mut self, error: String)` | Appends an error description string to the `errors` vector. Marked `#[inline(always)]`. |
| `is_empty` | `pub fn is_empty(&self) -> bool` | Returns `true` if both `changes` and `errors` are empty, indicating no work was performed and no failures occurred. |

## Remarks

- `ApplyConfigResult` derives `Debug` and `Default`. The `Default` implementation produces an instance with two empty `Vec`s, which is identical to calling `new()`.
- The struct is not thread-safe on its own; callers pass it as `&mut ApplyConfigResult` through the sequential apply pipeline for a single process.
- Change strings are designed to be concatenated with a process-identifying prefix (e.g., `"{pid:>5}::{config.name}::"`) by the caller. The `apply` functions themselves do **not** include the prefix.
- Error strings are self-contained and include the originating function name, the Windows API operation that failed, and the decoded error code for direct logging.
- The `is_empty` method is used by callers to avoid emitting empty log entries when a process was already in the desired state.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `apply.rs` |
| Crate | `AffinityServiceRust` |
| Dependencies | Standard library only (`Vec<String>`) |
| Callers | All `apply_*` functions in the module; orchestrator code in `scheduler.rs` and `main.rs` |
| Platform | Windows (content is platform-specific, but the struct itself is platform-independent) |

## See Also

| Reference | Link |
|-----------|------|
| apply module overview | [`README`](README.md) |
| log_error_if_new | [`log_error_if_new`](log_error_if_new.md) |
| apply_priority | [`apply_priority`](apply_priority.md) |
| apply_affinity | [`apply_affinity`](apply_affinity.md) |
| apply_io_priority | [`apply_io_priority`](apply_io_priority.md) |
| apply_memory_priority | [`apply_memory_priority`](apply_memory_priority.md) |
| ProcessLevelConfig | [`config.rs/ProcessLevelConfig`](../config.rs/ProcessLevelConfig.md) |

---
*Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*