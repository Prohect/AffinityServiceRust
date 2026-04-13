# apply_affinity function (apply.rs)

Sets the hard CPU affinity mask on a process via `SetProcessAffinityMask`. A hard affinity mask restricts the process to run only on the specified logical processors. After a successful affinity change, the function automatically calls [reset_thread_ideal_processors](reset_thread_ideal_processors.md) to redistribute thread ideal processors across the new CPU set.

## Syntax

```AffinityServiceRust/src/apply.rs#L132-141
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

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | Process identifier of the target process. Used for error logging and passed through to [reset_thread_ideal_processors](reset_thread_ideal_processors.md). |
| `config` | `&`[ProcessConfig](../config.rs/ProcessConfig.md) | Parsed configuration for this process. The `affinity_cpus` field contains the list of CPU indices to include in the mask. The `prime_threads_cpus` field is also inspected — if either field is non-empty, the current affinity mask is queried. |
| `dry_run` | `bool` | When `true`, the change is recorded in `apply_config_result` but `SetProcessAffinityMask` is not called and the OS state is not modified. |
| `current_mask` | `&mut usize` | In/out parameter that receives the process's current affinity mask (queried via `GetProcessAffinityMask`). On successful set, it is updated to the new mask value. This value is consumed downstream by [apply_prime_threads_promote](apply_prime_threads_promote.md) to filter prime CPU indices against the effective affinity. |
| `process_handle` | `&`[ProcessHandle](../winapi.rs/ProcessHandle.md) | OS handle wrapper for the target process. Both a read handle (for `GetProcessAffinityMask`) and a write handle (for `SetProcessAffinityMask`) are extracted via [get_handles](get_handles.md). |
| `process` | `&mut`[ProcessEntry](../process.rs/ProcessEntry.md) | Snapshot entry for the process, providing thread enumeration. Passed through to [reset_thread_ideal_processors](reset_thread_ideal_processors.md) when the affinity mask changes. |
| `apply_config_result` | `&mut`[ApplyConfigResult](ApplyConfigResult.md) | Accumulator for change descriptions and errors produced during this operation. |

## Return value

None (`()`). Results are communicated through `apply_config_result` and `current_mask`.

## Remarks

### Algorithm

1. **Early exit.** If both `config.affinity_cpus` and `config.prime_threads_cpus` are empty, the function returns immediately — there is nothing to query or set.

2. **Query current mask.** Calls `GetProcessAffinityMask` to read the current process affinity and the system affinity mask. The current mask is written into `*current_mask` so that later functions (especially [apply_prime_threads_promote](apply_prime_threads_promote.md)) can filter their CPU indices against the process's effective affinity.
   - If the query fails and `dry_run` is `false`, the error is logged via [log_error_if_new](log_error_if_new.md) with `Operation::GetProcessAffinityMask` and the function returns without attempting a set.

3. **Compare and set.** The desired mask is computed by [cpu_indices_to_mask](../config.rs/cpu_indices_to_mask.md) from `config.affinity_cpus`. The set is skipped if:
   - `config.affinity_cpus` is empty (the query was only needed for `current_mask`).
   - The computed mask is `0` (no valid CPUs resolved).
   - The computed mask already equals `*current_mask` (the affinity is already correct).

4. **Apply.** In `dry_run` mode, the change message is recorded without calling the API. Otherwise, `SetProcessAffinityMask` is called. On success, `*current_mask` is updated to the new mask and [reset_thread_ideal_processors](reset_thread_ideal_processors.md) is invoked with `config.affinity_cpus` to redistribute thread ideal processors.

### Side effects

- **`current_mask` is always written** after a successful `GetProcessAffinityMask`, even when no set is performed. This is intentional — [apply_prime_threads](apply_prime_threads.md) and [apply_prime_threads_promote](apply_prime_threads_promote.md) depend on the queried value.
- **Thread ideal processors are reset** after a successful affinity change. Windows may internally clear or reassign ideal processors when the affinity mask changes, so [reset_thread_ideal_processors](reset_thread_ideal_processors.md) re-distributes them deterministically with a random shift to avoid CPU hotspotting.

### Change message format

```/dev/null/example.txt#L1
Affinity: 0xFF -> 0xF0
```

The message shows the previous mask and the new mask in hexadecimal.

### Edge cases

- If the process has exited or the handle is invalid, [get_handles](get_handles.md) returns `None` and the function returns silently.
- A mask of `0` (e.g., from an empty or invalid CPU spec that resolves to no bits) is treated as "no change requested" and is never passed to `SetProcessAffinityMask`.
- When only `prime_threads_cpus` is configured (without `affinity_cpus`), the function queries the current mask but does not set a new one. This populates `current_mask` for the downstream prime-thread pipeline.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `apply` |
| Visibility | `pub` (crate-public) |
| Callers | [apply_config_process_level](../main.rs/apply_config_process_level.md) |
| Callees | [get_handles](get_handles.md), [cpu_indices_to_mask](../config.rs/cpu_indices_to_mask.md), [log_error_if_new](log_error_if_new.md), [reset_thread_ideal_processors](reset_thread_ideal_processors.md) |
| Win32 API | [`GetProcessAffinityMask`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getprocessaffinitymask), [`SetProcessAffinityMask`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-setprocessaffinitymask) |
| Privileges | `PROCESS_QUERY_LIMITED_INFORMATION` (read), `PROCESS_SET_INFORMATION` (write). The service typically holds `SeDebugPrivilege` which grants both. |

## See Also

| Topic | Link |
|-------|------|
| apply module overview | [apply](README.md) |
| Soft CPU set (alternative to hard affinity) | [apply_process_default_cpuset](apply_process_default_cpuset.md) |
| Ideal processor redistribution after affinity change | [reset_thread_ideal_processors](reset_thread_ideal_processors.md) |
| Prime thread CPU pinning (uses `current_mask`) | [apply_prime_threads_promote](apply_prime_threads_promote.md) |
| CPU index ↔ mask conversion | [cpu_indices_to_mask](../config.rs/cpu_indices_to_mask.md), [format_cpu_indices](../config.rs/format_cpu_indices.md) |
| Process handle acquisition | [get_process_handle](../winapi.rs/get_process_handle.md) |