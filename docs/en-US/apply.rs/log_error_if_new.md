# log_error_if_new function (apply.rs)

The `log_error_if_new` function conditionally appends an error message to an `ApplyConfigResult` only when the given pid/tid/operation/error-code combination has not been recorded before. This prevents the same recurring error from flooding the change log on every apply cycle, while still ensuring every distinct failure is reported at least once.

## Syntax

```AffinityServiceRust/src/apply.rs#L71-83
#[inline(always)]
fn log_error_if_new(
    pid: u32,
    tid: u32,
    process_name: &str,
    operation: Operation,
    error_code: u32,
    apply_config_result: &mut ApplyConfigResult,
    format_msg: impl FnOnce() -> String,
)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The process ID associated with the error. Used together with `tid`, `process_name`, `operation`, and `error_code` to form the deduplication key. |
| `tid` | `u32` | The thread ID associated with the error. Pass `0` for process-level operations that are not thread-specific. |
| `process_name` | `&str` | The human-readable name of the process or configuration rule, used as part of the deduplication key. |
| `operation` | `Operation` | An enum variant from the `logging` module that identifies which Windows API call failed (e.g., `Operation::SetPriorityClass`, `Operation::SetThreadSelectedCpuSets`). |
| `error_code` | `u32` | The raw Windows error code (Win32 `GetLastError` result or NTSTATUS cast to `u32`) that was returned by the failing API call. |
| `apply_config_result` | `&mut ApplyConfigResult` | The result accumulator to which the error string is appended if the error is new. |
| `format_msg` | `impl FnOnce() -> String` | A lazily-evaluated closure that produces the formatted error message string. The closure is only invoked when the error is determined to be new, avoiding the cost of string formatting for duplicate errors. |

## Return value

This function does not return a value.

## Remarks

- The function delegates deduplication to `logging::is_new_error`, which maintains a persistent set of previously-seen error keys. If `is_new_error` returns `true`, the `format_msg` closure is called and its result is passed to `ApplyConfigResult::add_error`. Otherwise, the error is silently suppressed.
- The `format_msg` parameter uses `impl FnOnce() -> String` rather than a pre-formatted `String` to defer the cost of `format!()` macro expansion. In high-frequency apply loops where the same error recurs every cycle, this avoids thousands of unnecessary heap allocations.
- The function is marked `#[inline(always)]` because it is called at every error site across the module and the body is small (a single branch plus a closure call).
- This is a module-private function (`fn`, not `pub fn`). It is used exclusively by other functions within `apply.rs`.
- The `tid` parameter is set to `0` by convention when the error is at the process level (e.g., `apply_priority`, `apply_affinity`). Thread-level callers (e.g., `apply_prime_threads_promote`, `apply_prime_threads_demote`, `apply_ideal_processors`) pass the actual thread ID.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `apply.rs` |
| Crate | `AffinityServiceRust` |
| Visibility | Private (crate-internal) |
| Dependencies | `logging::is_new_error`, `logging::Operation` |
| Callers | `apply_priority`, `apply_affinity`, `reset_thread_ideal_processors`, `apply_process_default_cpuset`, `apply_io_priority`, `apply_memory_priority`, `prefetch_all_thread_cycles`, `apply_prime_threads_promote`, `apply_prime_threads_demote`, `apply_ideal_processors` |
| Platform | Windows |

## See Also

| Reference | Link |
|-----------|------|
| apply module overview | [`README`](README.md) |
| ApplyConfigResult | [`ApplyConfigResult`](ApplyConfigResult.md) |
| get_handles | [`get_handles`](get_handles.md) |
| logging module | [`logging.rs`](../logging.rs/README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*