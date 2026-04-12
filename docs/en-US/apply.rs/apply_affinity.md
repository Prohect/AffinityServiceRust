# apply_affinity function (apply.rs)

Sets the hard CPU affinity mask for a process. As a side effect, fills in the caller's `current_mask` with the process's current affinity mask, which is needed by downstream prime-thread logic.

## Syntax

```rust
pub fn apply_affinity(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    current_mask: &mut usize,
    process_handle: &ProcessHandle,
    process: &mut ProcessEntry,
    apply_config_result: &mut ApplyConfigResult,
)
```

## Parameters

`pid`

The process identifier of the target process.

`config`

Reference to the [ProcessConfig](../config.rs/ProcessConfig.md) containing the desired `affinity_cpus` and `prime_threads_cpus` settings.

`dry_run`

When `true`, records what changes *would* be made without calling any Windows APIs.

`current_mask`

Mutable reference to a `usize` that receives the process's current affinity mask. This value is read by later functions such as [apply_prime_threads](apply_prime_threads.md) to filter CPU sets against the live affinity. The mask is updated to the new value on a successful set operation.

`process_handle`

Reference to a [ProcessHandle](../winapi.rs/ProcessHandle.md) from which read and write `HANDLE` values are extracted via [get_handles](get_handles.md).

`process`

Mutable reference to the [ProcessEntry](../process.rs/ProcessEntry.md) for the target process. Passed through to [reset_thread_ideal_processors](reset_thread_ideal_processors.md) when the affinity mask is changed.

`apply_config_result`

Mutable reference to an [ApplyConfigResult](ApplyConfigResult.md) that accumulates change descriptions and error messages.

## Return value

This function does not return a value. Results are recorded in `apply_config_result` and `current_mask` is updated as a side effect.

## Remarks

The function proceeds only if `config.affinity_cpus` or `config.prime_threads_cpus` is non-empty, because the current affinity mask is needed for both features.

The affinity mask is converted from the CPU index list via `cpu_indices_to_mask()`. A set operation is only performed when the computed mask differs from the current mask and the mask is non-zero.

**Post-action:** When the affinity mask is successfully changed, the function immediately calls [reset_thread_ideal_processors](reset_thread_ideal_processors.md) with `&config.affinity_cpus` to redistribute thread ideal processors across the new CPU set. This prevents threads from being stranded on CPUs that are no longer in the affinity mask.

**Change logged:** `"Affinity: {old:#X} -> {new:#X}"`

**Error deduplication:** All errors are reported through [log_error_if_new](log_error_if_new.md), so the same error for the same PID and operation is only logged once.

### Execution flow

1. Extract read/write handles via [get_handles](get_handles.md); return early if either is `None`.
2. Check whether `affinity_cpus` or `prime_threads_cpus` is configured.
3. Call `GetProcessAffinityMask` to read the current mask into `current_mask`.
4. If the desired mask differs from the current mask:
   - **Dry run:** Record the change message.
   - **Live run:** Call `SetProcessAffinityMask`. On success, update `current_mask` and call [reset_thread_ideal_processors](reset_thread_ideal_processors.md).

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/apply.rs |
| **Lines** | L131–L208 |
| **Called by** | [apply_config](../main.rs/apply_config.md) |
| **Calls** | [get_handles](get_handles.md), [log_error_if_new](log_error_if_new.md), [reset_thread_ideal_processors](reset_thread_ideal_processors.md) |
| **Windows API** | `GetProcessAffinityMask`, `SetProcessAffinityMask` |
| **Config fields** | `affinity_cpus`, `prime_threads_cpus` |

## See also

- [apply_process_default_cpuset](apply_process_default_cpuset.md) — soft CPU preference via CPU Sets
- [apply_prime_threads](apply_prime_threads.md) — uses `current_mask` filled by this function
- [reset_thread_ideal_processors](reset_thread_ideal_processors.md) — redistributes ideal processors after affinity change