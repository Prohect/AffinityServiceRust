# apply_process_default_cpuset function (apply.rs)

Applies a soft CPU set preference to a process via the Windows `SetProcessDefaultCpuSets` API. Unlike a hard affinity mask (which restricts *all* threads unconditionally), a default CPU set establishes a preferred set of logical processors that threads will schedule on unless overridden at the thread level by `SetThreadSelectedCpuSets`. This makes CPU sets the preferred mechanism for steering work without preventing individual threads (such as prime threads) from being pinned elsewhere.

## Syntax

```AffinityServiceRust/src/apply.rs#L315-323
pub fn apply_process_default_cpuset(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    process: &mut ProcessEntry,
    apply_config_result: &mut ApplyConfigResult,
)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | Process identifier of the target process. Used for logging and as a key in the error-deduplication map. |
| `config` | `&`[ProcessConfig](../config.rs/ProcessConfig.md) | The parsed configuration rule matched to this process. The `cpu_set_cpus` field supplies the desired CPU indices; the `cpu_set_reset_ideal` flag controls whether thread ideal processors are redistributed after the CPU set changes. |
| `dry_run` | `bool` | When `true`, the function records the intended change in `apply_config_result` but does not call any Windows API. |
| `process_handle` | `&`[ProcessHandle](../winapi.rs/ProcessHandle.md) | OS handles opened for the target process. Passed through [get_handles](get_handles.md) to obtain the best available read and write `HANDLE`s. |
| `process` | `&mut`[ProcessEntry](../process.rs/ProcessEntry.md) | Mutable reference to the cached process/thread snapshot. Forwarded to [reset_thread_ideal_processors](reset_thread_ideal_processors.md) when `cpu_set_reset_ideal` is set. |
| `apply_config_result` | `&mut`[ApplyConfigResult](ApplyConfigResult.md) | Accumulator for change descriptions and error messages produced during this call. |

## Return value

None (`()`).

## Remarks

### CPU set identifiers

Windows CPU sets are identified by opaque 32-bit IDs (not the same as logical processor indices). The function uses [cpusetids_from_indices](../winapi.rs/cpusetids_from_indices.md) to translate the user-facing CPU indices in `config.cpu_set_cpus` into the system CPU set IDs that the API expects. If the system-wide CPU set information is empty (e.g. on older Windows builds that do not support CPU sets), the function returns immediately without making any changes.

### Query-then-set strategy

The function performs a two-phase query before deciding to set:

1. **First query** — calls `GetProcessDefaultCpuSets` with `None` for the buffer. If this succeeds, the process currently has *no* default CPU set, so `toset` is set to `true`.
2. **Second query** — if the first query fails with `ERROR_INSUFFICIENT_BUFFER` (Win32 error 122), the function allocates a buffer of the reported size and queries again to retrieve the current CPU set IDs. If the current set already matches the target, no write is performed.

Any error code other than 122 on the first query is treated as a real failure and logged through [log_error_if_new](log_error_if_new.md).

### Ideal processor reset

When `config.cpu_set_reset_ideal` is `true` and a change is about to be written, the function calls [reset_thread_ideal_processors](reset_thread_ideal_processors.md) with `config.cpu_set_cpus` *before* applying the new CPU set. This redistributes thread ideal processors across the new CPU set so that the OS scheduler spreads threads evenly rather than clustering them on whatever processors happened to be ideal before the change.

### Change message format

On success the change message follows the pattern:

`"CPU Set: [0,1,2] -> [4,5,6]"`

where the left side shows CPU indices decoded from the previously active CPU set IDs (via [indices_from_cpusetids](../winapi.rs/indices_from_cpusetids.md)), and the right side shows the indices from the configuration. When the process had no previous CPU set, the left side is an empty list `[]`.

### Dry-run behaviour

In dry-run mode the function unconditionally records the target CPU set as a change without querying the current state, producing a message like:

`"CPU Set: -> [4,5,6]"`

### Edge cases

- If `config.cpu_set_cpus` is empty, the function returns immediately — it never *clears* an existing CPU set.
- If `cpusetids_from_indices` returns an empty vector (no matching CPU set IDs found for the given indices), the write is skipped.
- The function does not modify the process affinity mask; CPU sets and affinity masks are independent constraints applied by [apply_affinity](apply_affinity.md) and this function respectively.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `apply` |
| Visibility | `pub` (crate-public) |
| Callers | [apply_config_process_level](../main.rs/apply_config_process_level.md) |
| Callees | [get_handles](get_handles.md), [cpusetids_from_indices](../winapi.rs/cpusetids_from_indices.md), [indices_from_cpusetids](../winapi.rs/indices_from_cpusetids.md), [get_cpu_set_information](../winapi.rs/get_cpu_set_information.md), [format_cpu_indices](../config.rs/format_cpu_indices.md), [reset_thread_ideal_processors](reset_thread_ideal_processors.md), [log_error_if_new](log_error_if_new.md) |
| Win32 API | `GetProcessDefaultCpuSets`, `SetProcessDefaultCpuSets` |
| Privileges | `PROCESS_QUERY_LIMITED_INFORMATION` (read), `PROCESS_SET_LIMITED_INFORMATION` (write) |

## See Also

| Topic | Link |
|-------|------|
| Hard affinity mask | [apply_affinity](apply_affinity.md) |
| Ideal processor redistribution | [reset_thread_ideal_processors](reset_thread_ideal_processors.md) |
| Thread-level CPU set pinning (prime threads) | [apply_prime_threads_promote](apply_prime_threads_promote.md) |
| CPU set ID translation | [cpusetids_from_indices](../winapi.rs/cpusetids_from_indices.md), [indices_from_cpusetids](../winapi.rs/indices_from_cpusetids.md) |
| Configuration model | [ProcessConfig](../config.rs/ProcessConfig.md) |
| apply module overview | [apply](README.md) |