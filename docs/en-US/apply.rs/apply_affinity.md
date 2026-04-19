# apply_affinity function (apply.rs)

Sets the CPU affinity mask for a process, restricting it to run only on specified logical processors.

## Syntax

```AffinityServiceRust/src/apply.rs#L134-142
pub fn apply_affinity<'a>(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    current_mask: &mut usize,
    process_handle: &ProcessHandle,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    apply_config_result: &mut ApplyConfigResult,
)
```

## Parameters

`pid: u32`

The process identifier of the target process.

`config: &ProcessLevelConfig`

The process-level configuration containing `affinity_cpus`, the list of CPU indices the process should be restricted to. If `affinity_cpus` is empty, the function returns immediately without action.

`dry_run: bool`

If `true`, records the intended change in *apply_config_result* without calling any Windows APIs.

`current_mask: &mut usize`

**\[out\]** Receives the process's current affinity mask on successful query. Updated to the new mask on successful set. This value is consumed downstream by functions like [apply_prime_threads_promote](apply_prime_threads_promote.md), which uses it to filter prime CPU indices to only those within the process affinity.

`process_handle: &ProcessHandle`

Handle wrapper for the target process. Both read and write handles are required; the function returns early if either is unavailable.

`threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>`

Lazy accessor for the thread map of the target process. Passed through to [reset_thread_ideal_processors](reset_thread_ideal_processors.md) when affinity is successfully changed.

`apply_config_result: &mut ApplyConfigResult`

Accumulator for change messages and errors produced during the operation.

## Return value

This function does not return a value. Results are communicated through `current_mask` (side effect) and `apply_config_result`.

## Remarks

The function converts the configured CPU indices to a bitmask via `cpu_indices_to_mask`. It then queries the current process affinity mask using **GetProcessAffinityMask** and compares it to the target. If they differ, it calls **SetProcessAffinityMask** to apply the new mask.

### Side effects

- **Fills `current_mask`:** Even when the configured affinity already matches, the current mask is written to `*current_mask` via the `GetProcessAffinityMask` out-parameter. This is consumed by later prime-thread logic.
- **Resets thread ideal processors:** On a successful affinity change, the function immediately calls [reset_thread_ideal_processors](reset_thread_ideal_processors.md) with `config.affinity_cpus`. This redistributes thread ideal processors across the new CPU set to prevent stale assignments that would cluster threads on CPUs no longer in the affinity mask.
- **Updates `current_mask` to new value:** After a successful set, `*current_mask` is overwritten with the new affinity mask.

### Error handling

- If **GetProcessAffinityMask** fails, the error is logged once per unique (pid, operation, error\_code) via [log_error_if_new](log_error_if_new.md) and the function exits without attempting to set.
- If **SetProcessAffinityMask** fails, the error is similarly logged and the mask is not updated.
- Both error paths are suppressed in `dry_run` mode (get-path errors are skipped entirely).

### Affinity mask format

The affinity mask is a `usize` bitmask where bit *N* represents logical processor *N*. For example, CPUs `[0, 2, 4]` produce mask `0x15`. A zero mask after conversion causes the function to skip the set operation, as Windows rejects a zero affinity mask.

### Change message format

```/dev/null/example.txt#L1
Affinity: 0xFF -> 0x15
```

## Requirements

| | |
|---|---|
| **Module** | `src/apply.rs` |
| **Callers** | Main polling loop (via process-level config application) |
| **Callees** | [get_handles](get_handles.md), [log_error_if_new](log_error_if_new.md), [reset_thread_ideal_processors](reset_thread_ideal_processors.md), `cpu_indices_to_mask` |
| **Win32 API** | [GetProcessAffinityMask](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getprocessaffinitymask), [SetProcessAffinityMask](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-setprocessaffinitymask) |
| **Privileges** | `PROCESS_QUERY_LIMITED_INFORMATION` (read), `PROCESS_SET_INFORMATION` (write) |

## See Also

| Topic | Description |
|---|---|
| [ApplyConfigResult](ApplyConfigResult.md) | Accumulator for changes and errors |
| [apply_process_default_cpuset](apply_process_default_cpuset.md) | Soft CPU preference via CPU Sets (alternative to hard affinity) |
| [reset_thread_ideal_processors](reset_thread_ideal_processors.md) | Redistributes thread ideal processors after affinity change |
| [apply_prime_threads_promote](apply_prime_threads_promote.md) | Uses `current_mask` to filter prime CPU indices |
| [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md) | Configuration struct containing `affinity_cpus` |
| [ProcessHandle](../winapi.rs/ProcessHandle.md) | Process handle wrapper with read/write access levels |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*