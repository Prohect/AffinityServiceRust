# ApplyConfigResult struct (apply.rs)

Accumulator for collecting changes and errors that occur during configuration application. Every `apply_*` function in the module receives a mutable reference to this struct and appends human-readable messages describing what was changed or what failed.

## Syntax

```AffinityServiceRust/src/apply.rs#L31-35
#[derive(Debug, Default)]
pub struct ApplyConfigResult {
    pub changes: Vec<String>,
    pub errors: Vec<String>,
}
```

## Members

| Member | Type | Description |
|--------|------|-------------|
| `changes` | `Vec<String>` | Descriptions of settings that were successfully applied or would be applied in dry-run mode. Format: `"$operation details"`, automatically prefixed by the caller with `"{pid:>5}::{config.name}::"`. |
| `errors` | `Vec<String>` | Descriptions of failures encountered during application. Format: `"$fn_name: [$operation][$error_message] details"`. Only unique errors are recorded (see [log_error_if_new](log_error_if_new.md)). |

## Methods

### new

```AffinityServiceRust/src/apply.rs#L38-40
pub fn new() -> Self
```

Creates a new `ApplyConfigResult` with empty `changes` and `errors` vectors. Delegates to `Default::default()`.

### add_change

```AffinityServiceRust/src/apply.rs#L45-47
pub fn add_change(&mut self, change: String)
```

Appends a change description to the `changes` vector.

**Parameters**

| Parameter | Type | Description |
|-----------|------|-------------|
| `change` | `String` | A human-readable description of the applied (or dry-run) change. |

### add_error

```AffinityServiceRust/src/apply.rs#L51-53
pub fn add_error(&mut self, error: String)
```

Appends an error description to the `errors` vector.

**Parameters**

| Parameter | Type | Description |
|-----------|------|-------------|
| `error` | `String` | A human-readable description of the failure, including operation tag and Win32/NTSTATUS error text. |

### is_empty

```AffinityServiceRust/src/apply.rs#L55-57
pub fn is_empty(&self) -> bool
```

Returns `true` if both `changes` and `errors` are empty, indicating that no observable action occurred for this process during the current apply cycle.

**Return value**

`bool` — `true` when neither changes nor errors were recorded.

## Remarks

- All `apply_*` functions take `&mut ApplyConfigResult` as their last parameter, following a consistent convention throughout the module.
- The caller (in `main.rs`) uses `is_empty()` to skip log output for processes that required no changes.
- `add_change` and `add_error` are marked `#[inline(always)]` because they are called on every hot path in the apply loop.

## Requirements

| | |
|---|---|
| **Module** | `src/apply.rs` |
| **Callers** | All `apply_*` functions, [log_error_if_new](log_error_if_new.md), main apply loop |
| **Dependencies** | None (plain data struct) |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [apply.rs](README.md) |
| Error deduplication helper | [log_error_if_new](log_error_if_new.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*