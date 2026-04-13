# log_error_if_new function (apply.rs)

Logs an error message to the [ApplyConfigResult](ApplyConfigResult.md) accumulator only if the same pid / tid / operation / error-code combination has not been logged before. This prevents repeated failures—common when a process denies access on every polling cycle—from flooding the log with identical entries.

## Syntax

```AffinityServiceRust/src/apply.rs#L69-81
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
| `pid` | `u32` | Process identifier that the error pertains to. Used together with `tid`, `operation`, and `error_code` to form the deduplication key. |
| `tid` | `u32` | Thread identifier. Pass `0` for process-level operations that are not thread-specific (e.g., `SetPriorityClass`, `SetProcessAffinityMask`). |
| `process_name` | `&str` | Display name of the process, forwarded to [is_new_error](../logging.rs/is_new_error.md) for the deduplication map and included in the formatted message. |
| `operation` | [Operation](../logging.rs/Operation.md) | Enum variant identifying the Windows API call that failed (e.g., `Operation::SetPriorityClass`, `Operation::NtSetInformationProcess2ProcessInformationIOPriority`). |
| `error_code` | `u32` | The raw Win32 error code (`GetLastError().0`) or the unsigned cast of an NTSTATUS value. |
| `apply_config_result` | `&mut` [ApplyConfigResult](ApplyConfigResult.md) | Accumulator that receives the formatted error string via `add_error` when the error is new. |
| `format_msg` | `impl FnOnce() -> String` | Lazy formatting closure. Only evaluated when the error *is* new, avoiding the cost of `format!()` for suppressed duplicates. |

## Return value

None (`()`).

## Remarks

The function delegates deduplication to [is_new_error](../logging.rs/is_new_error.md), which maintains a global `HashMap<u32, HashMap<ApplyFailEntry, bool>>` keyed by pid. If `is_new_error` returns `true`, the `format_msg` closure is invoked and the resulting `String` is pushed into `apply_config_result.errors`. If it returns `false`, neither the closure nor `add_error` is called.

Because `format_msg` is `FnOnce`, the formatting work (which typically involves calls to [error_from_code_win32](../error_codes.rs/error_from_code_win32.md) or [error_from_ntstatus](../error_codes.rs/error_from_ntstatus.md)) is deferred until it is known that the message will actually be recorded. In a steady-state loop where a process continuously denies access, this avoids thousands of wasted allocations per cycle.

The function is marked `#[inline(always)]` because it sits on the hot path of every `apply_*` function's error branch and consists of a single conditional call.

All `apply_*` functions in the [apply](README.md) module route their error handling through `log_error_if_new` rather than calling `add_error` directly. This makes the deduplication policy uniform and centralised.

### Error message convention

Callers format messages following the pattern:

`"fn_name: [OPERATION][error_description] pid-tid-process_name"`

For example:

`"apply_priority: [SET_PRIORITY_CLASS][Access is denied. (0x5)] 1234-chrome"`

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `apply` |
| Visibility | crate-private (`fn`) |
| Callers | [apply_priority](apply_priority.md), [apply_affinity](apply_affinity.md), [reset_thread_ideal_processors](reset_thread_ideal_processors.md), [apply_process_default_cpuset](apply_process_default_cpuset.md), [apply_io_priority](apply_io_priority.md), [apply_memory_priority](apply_memory_priority.md), [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md), [apply_prime_threads_promote](apply_prime_threads_promote.md), [apply_prime_threads_demote](apply_prime_threads_demote.md), [apply_ideal_processors](apply_ideal_processors.md) |
| Callees | [is_new_error](../logging.rs/is_new_error.md), [ApplyConfigResult::add_error](ApplyConfigResult.md) |

## See Also

| Topic | Link |
|-------|------|
| Deduplication map and purging | [is_new_error](../logging.rs/is_new_error.md), [purge_fail_map](../logging.rs/purge_fail_map.md) |
| Operation enum | [Operation](../logging.rs/Operation.md) |
| Error result accumulator | [ApplyConfigResult](ApplyConfigResult.md) |
| Win32 error formatting | [error_from_code_win32](../error_codes.rs/error_from_code_win32.md) |
| NTSTATUS error formatting | [error_from_ntstatus](../error_codes.rs/error_from_ntstatus.md) |