# reset_thread_ideal_processors function (apply.rs)

Redistributes thread ideal processors across a specified set of CPUs after an affinity mask or CPU-set change. The function sorts all threads by their accumulated CPU time (kernel + user) in descending order and assigns each thread an ideal processor from the target CPU list using round-robin distribution with a random starting offset. This prevents Windows from concentrating all threads onto the same CPU after a process-wide affinity or CPU-set change.

## Syntax

```AffinityServiceRust/src/apply.rs#L219-231
pub fn reset_thread_ideal_processors(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    cpus: &[u32],
    threads: &HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    apply_config_result: &mut ApplyConfigResult,
)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The process ID of the target process. Used for error deduplication in log messages. |
| `config` | `&ProcessLevelConfig` | The process-level configuration. The `name` field is used in error log messages and passed to `get_thread_handle` for handle acquisition. |
| `dry_run` | `bool` | When `true`, a synthetic change message is recorded describing how many threads would be redistributed, without opening any thread handles or calling `SetThreadIdealProcessorEx`. When `false`, the actual reassignment is performed. |
| `cpus` | `&[u32]` | The set of CPU indices to distribute thread ideal processors across. Callers pass `&config.affinity_cpus` after an affinity mask change, or `&config.cpu_set_cpus` after a CPU-set change (when `cpu_set_reset_ideal` is enabled). If empty, the function returns immediately. |
| `threads` | `&HashMap<u32, SYSTEM_THREAD_INFORMATION>` | A map of thread IDs to their `SYSTEM_THREAD_INFORMATION` snapshots from the most recent system process information query. The `KernelTime` and `UserTime` fields are summed to determine each thread's total CPU time for sorting. |
| `apply_config_result` | `&mut ApplyConfigResult` | Accumulator for change descriptions and error messages produced during execution. |

## Return value

This function does not return a value. All outcomes are communicated through the `apply_config_result` parameter.

## Remarks

### Algorithm

1. **Early exit**: If `cpus` is empty, the function returns immediately. If `dry_run` is `true`, a change message is recorded and the function returns without performing any API calls.
2. **Collect CPU times**: For each thread in the `threads` map, the total CPU time is computed as `KernelTime + UserTime` (both in 100-nanosecond units). The results are collected into a fixed-capacity list (`List<[(u32, i64); TIDS_FULL]>`).
3. **Sort**: The thread list is sorted in descending order of total CPU time using `sort_unstable_by_key` with `Reverse`. This ensures the most CPU-active threads are assigned ideal processors first.
4. **Random offset**: A random `u8` value is generated via `rand::random::<u8>()` and used as a starting offset into the CPU array. This avoids always assigning the first thread to the first CPU in the list, providing a degree of load balancing across apply cycles.
5. **Round-robin assignment**: Each thread is assigned an ideal processor by cycling through the `cpus` array: `target_cpu = cpus[(success_count + random_shift) % cpus.len()]`. The function calls `set_thread_ideal_processor_ex` with group `0` and the computed CPU index.
6. **Handle resolution**: For each thread, a handle is obtained via `get_thread_handle`. The function prefers the `w_limited_handle` over `w_handle` (using whichever is not invalid). Threads for which a handle cannot be obtained are silently skipped.
7. **Result recording**: On completion, a change message of the form `"reset ideal processor for N threads"` is appended, where N is the count of threads that were successfully reassigned.

### Edge cases

- If all `set_thread_ideal_processor_ex` calls fail, the success counter remains zero and the change message reports `"reset ideal processor for 0 threads"`.
- Threads that have exited between the snapshot and the handle-open attempt are silently skipped; `get_thread_handle` returns `None` and the function continues with the next thread.
- The `random_shift` is a `u8` cast to `usize`, so values wrap modulo 256 which is fine because the modulus is always `cpus.len()`.

### Callers

This function is called from two sites:
- [`apply_affinity`](apply_affinity.md) — immediately after a successful `SetProcessAffinityMask`, passing `&config.affinity_cpus`.
- [`apply_process_default_cpuset`](apply_process_default_cpuset.md) — immediately before `SetProcessDefaultCpuSets` when `config.cpu_set_reset_ideal` is `true`, passing `&config.cpu_set_cpus`.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `apply.rs` |
| Crate | `AffinityServiceRust` |
| Visibility | `pub` |
| Windows APIs | `SetThreadIdealProcessorEx` (via `winapi::set_thread_ideal_processor_ex`), `GetLastError` |
| Callers | [`apply_affinity`](apply_affinity.md), [`apply_process_default_cpuset`](apply_process_default_cpuset.md) |
| Callees | [`log_error_if_new`](log_error_if_new.md), `winapi::get_thread_handle`, `winapi::set_thread_ideal_processor_ex`, `error_codes::error_from_code_win32`, `rand::random` |
| Privileges | Requires thread handles with `THREAD_SET_INFORMATION` or `THREAD_SET_LIMITED_INFORMATION` access rights. |

## See Also

| Reference | Link |
|-----------|------|
| apply module overview | [`README`](README.md) |
| ApplyConfigResult | [`ApplyConfigResult`](ApplyConfigResult.md) |
| apply_affinity | [`apply_affinity`](apply_affinity.md) |
| apply_process_default_cpuset | [`apply_process_default_cpuset`](apply_process_default_cpuset.md) |
| apply_ideal_processors | [`apply_ideal_processors`](apply_ideal_processors.md) |
| log_error_if_new | [`log_error_if_new`](log_error_if_new.md) |
| ProcessLevelConfig | [`config.rs/ProcessLevelConfig`](../config.rs/ProcessLevelConfig.md) |

---
*Commit: 7221ea0694670265d4eb4975582d8ed2ae02439d*