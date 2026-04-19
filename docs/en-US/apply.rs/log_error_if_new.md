# log_error_if_new function (apply.rs)

Logs an error message only if the same error has not already been logged for the given process/thread/operation combination. This prevents repetitive error messages from flooding the log when the same operation fails on every polling cycle.

## Syntax

```AffinityServiceRust/src/apply.rs#L71-80
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

`pid: u32`

The process ID associated with the error.

`tid: u32`

The thread ID associated with the error. Pass `0` for process-level operations that are not thread-specific.

`process_name: &str`

The display name of the process from the matched configuration rule. Used as part of the deduplication key and included in log output.

`operation: Operation`

The [Operation](../logging.rs/Operation.md) enum variant identifying which Windows API call failed. Forms part of the uniqueness key for deduplication.

`error_code: u32`

The Win32 error code or NTSTATUS value returned by the failed API call. Forms part of the uniqueness key — the same operation failing with a different error code is treated as a new, distinct error.

`apply_config_result: &mut ApplyConfigResult`

The [ApplyConfigResult](ApplyConfigResult.md) accumulator to which the formatted error message is appended via `add_error()`, but only if the error is new.

`format_msg: impl FnOnce() -> String`

A lazily-evaluated closure that produces the formatted error message string. The closure is only invoked when the error is determined to be new, avoiding the cost of string formatting for suppressed duplicates.

## Return value

This function does not return a value.

## Remarks

This function is a thin wrapper around [`logging::is_new_error`](../logging.rs/is_new_error.md). It delegates deduplication to `is_new_error(pid, tid, process_name, operation, error_code)`, which maintains a per-PID set of `ApplyFailEntry` records. An error is considered "new" if no prior entry with the same `(tid, process_name, operation, error_code)` tuple exists for that PID.

The function is marked `#[inline(always)]` to eliminate call overhead, since it is invoked at every error site throughout the `apply` module.

The lazy `format_msg` closure pattern is important for performance. In steady state, most errors are duplicates of previously seen failures (e.g., access denied on a protected process). By deferring string formatting until after the deduplication check, the hot path avoids allocation entirely.

### Error message format convention

Callers follow a consistent format for the closure output:

```/dev/null/error_format.txt#L1
fn_name: [OPERATION_NAME][error_description] pid-tid-process_name
```

For example:
```/dev/null/error_example.txt#L1
apply_affinity: [SET_PROCESS_AFFINITY_MASK][Access is denied. (0x5)] 1234-my_process
```

### Typical usage

Every `apply_*` function in the module calls `log_error_if_new` after a failed Windows API call, passing the relevant `Operation` variant and the Win32/NTSTATUS error code. This centralizes the dedup-then-log pattern and keeps individual apply functions focused on their core logic.

## Requirements

| | |
|---|---|
| **Module** | `apply` |
| **Callers** | [apply_priority](apply_priority.md), [apply_affinity](apply_affinity.md), [reset_thread_ideal_processors](reset_thread_ideal_processors.md), [apply_process_default_cpuset](apply_process_default_cpuset.md), [apply_io_priority](apply_io_priority.md), [apply_memory_priority](apply_memory_priority.md), [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md), [apply_prime_threads_promote](apply_prime_threads_promote.md), [apply_prime_threads_demote](apply_prime_threads_demote.md), [apply_ideal_processors](apply_ideal_processors.md) |
| **Callees** | [`logging::is_new_error`](../logging.rs/is_new_error.md), [ApplyConfigResult::add_error](ApplyConfigResult.md) |
| **Visibility** | `fn` (crate-private) |

## See Also

| | |
|---|---|
| [ApplyConfigResult](ApplyConfigResult.md) | Accumulator for changes and errors |
| [Operation](../logging.rs/Operation.md) | Enum of Windows API operation identifiers |
| [is_new_error](../logging.rs/is_new_error.md) | Underlying deduplication logic |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*