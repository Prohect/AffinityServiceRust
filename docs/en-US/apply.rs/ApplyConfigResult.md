# ApplyConfigResult struct (apply.rs)

Collects change descriptions and error messages produced during the application of a single process configuration pass.

## Syntax

```rust
#[derive(Debug, Default)]
pub struct ApplyConfigResult {
    pub changes: Vec<String>,
    pub errors: Vec<String>,
}
```

## Members

`changes`

A vector of human-readable strings describing each configuration change that was successfully applied (or would be applied in dry-run mode). Each entry follows the format `"$operation details"` and is later prefixed with `"{pid:>5}::{config.name}::"` by the caller.

`errors`

A vector of human-readable strings describing errors encountered during the apply pass. Each entry follows the format `"$fn_name: [$operation][$error_message] details"`.

## Methods

| Method | Signature | Description |
| --- | --- | --- |
| **new** | `pub fn new() -> Self` | Creates an empty result via `Default`. |
| **add_change** | `pub fn add_change(&mut self, change: String)` | Appends a change description to `changes`. |
| **add_error** | `pub fn add_error(&mut self, error: String)` | Appends an error description to `errors`. |
| **is_empty** | `pub fn is_empty(&self) -> bool` | Returns `true` when both `changes` and `errors` are empty. |

## Remarks

`ApplyConfigResult` is the primary feedback mechanism for every `apply_*` function in the module. The top-level orchestrator [`apply_config`](../main.rs/apply_config.md) in `main.rs` creates a single instance per process per loop iteration, passes it by mutable reference through the entire apply chain, and then inspects it afterward to decide whether to log changes.

An empty result (both vectors empty) indicates that the process was already in the desired state and no action was taken. The caller uses [`is_empty`](#methods) to skip unnecessary log output.

The struct derives `Default`, so the `new()` constructor is simply a convenience alias.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/apply.rs |
| **Lines** | L30–L56 |
| **Returned by** | [`apply_config`](../main.rs/apply_config.md) in main.rs |
| **Consumed by** | All `apply_*` functions in this module via `&mut ApplyConfigResult` |

## See also

- [apply.rs module overview](README.md)
- [apply_priority](apply_priority.md)
- [apply_affinity](apply_affinity.md)
- [log_error_if_new](log_error_if_new.md)