# log_error_if_new function (apply.rs)

Logs an error message only if it has not been previously logged for the same process/thread/operation combination. This deduplication prevents repetitive error spam in the log output when the same operation fails repeatedly across polling iterations.

## Syntax

```rust
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

`pid`

The process ID associated with the error.

`tid`

The thread ID associated with the error. Use `0` for process-level operations that are not thread-specific.

`process_name`

The name of the process, used as part of the deduplication key and included in error messages.

`operation`

The [Operation](../logging.rs/Operation.md) enum variant identifying which Windows API call or logical operation failed.

`error_code`

The Win32 error code or NTSTATUS value returned by the failed operation. Combined with `pid`, `tid`, `process_name`, and `operation` to form the deduplication key.

`apply_config_result`

Mutable reference to the [ApplyConfigResult](ApplyConfigResult.md) that collects errors for the current application cycle. If the error is new, the formatted message is added via `add_error()`.

`format_msg`

A closure (`FnOnce() -> String`) that produces the error message string. This is lazily evaluated — only called if the error is determined to be new, avoiding the cost of formatting when the error has already been logged.

## Return value

This function does not return a value.

## Remarks

This is a helper function used extensively throughout the `apply.rs` module by every function that interacts with the Windows API. It wraps [is_new_error](../logging.rs/is_new_error.md) from the logging module.

The deduplication logic works as follows:

1. The function calls `is_new_error(pid, tid, process_name, operation, error_code)` to check whether this exact combination has been seen before.
2. If the error is **new**, `format_msg()` is invoked and the resulting string is added to `apply_config_result.errors` via `add_error()`.
3. If the error has **already been logged**, the function returns immediately without evaluating `format_msg`, saving the cost of string formatting.

The lazy evaluation pattern using `impl FnOnce() -> String` is important for performance because error formatting involves `format!()` calls with Win32 error code lookups, which would be wasteful if the error is a known duplicate.

**Error message convention:** Callers typically format messages as:

`"fn_name: [OPERATION_NAME][error_description] pid-tid-process_name"`

For example:

`"apply_priority: [SET_PRIORITY_CLASS][Access is denied (5)] 1234-game.exe"`

The function is marked `#[inline(always)]` to eliminate call overhead since it is invoked on every error path in the module.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/apply.rs |
| **Line** | L67–L81 |
| **Visibility** | Private (`fn`) |
| **Called by** | [apply_priority](apply_priority.md), [apply_affinity](apply_affinity.md), [reset_thread_ideal_processors](reset_thread_ideal_processors.md), [apply_process_default_cpuset](apply_process_default_cpuset.md), [apply_io_priority](apply_io_priority.md), [apply_memory_priority](apply_memory_priority.md), [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md), [apply_prime_threads_promote](apply_prime_threads_promote.md), [apply_prime_threads_demote](apply_prime_threads_demote.md), [apply_ideal_processors](apply_ideal_processors.md) |
| **Dependencies** | [is_new_error](../logging.rs/is_new_error.md), [ApplyConfigResult](ApplyConfigResult.md) |