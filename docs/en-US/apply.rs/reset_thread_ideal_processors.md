# reset_thread_ideal_processors function (apply.rs)

Resets ideal processor assignments for all threads in a process after an affinity or CPU set change. Distributes threads across the new set of CPUs by sorting threads by CPU time (descending) and assigning ideal processors in round-robin order with a random offset to avoid deterministic clustering.

## Syntax

```AffinityServiceRust/src/apply.rs#L219-226
pub fn reset_thread_ideal_processors<'a>(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    cpus: &[u32],
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    apply_config_result: &mut ApplyConfigResult,
)
```

## Parameters

`pid: u32`

The process identifier of the target process.

`config: &ProcessLevelConfig`

The [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md) for the matched process. Used for the `name` field in log messages and to open thread handles.

`dry_run: bool`

When `true`, records what would be changed into `apply_config_result` without calling any Windows APIs.

`cpus: &[u32]`

The set of CPU indices to distribute thread ideal processors across. Callers pass `&config.affinity_cpus` after an affinity change, or `&config.cpu_set_cpus` after a CPU set change (when `cpu_set_reset_ideal` is set).

`threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>`

Lazy accessor returning the map of thread IDs to `SYSTEM_THREAD_INFORMATION` for the process.

`apply_config_result: &mut ApplyConfigResult`

Accumulator for recording changes and errors. See [ApplyConfigResult](ApplyConfigResult.md).

## Return value

This function does not return a value. Results are accumulated in `apply_config_result`.

## Remarks

When Windows changes a process's affinity mask, thread ideal processor assignments may become stale — a thread could retain an ideal processor hint pointing at a CPU no longer in the affinity set. This function corrects that by reassigning ideal processors across the new CPU set.

### Algorithm

1. **Early exit** — If `cpus` is empty, returns immediately. In dry-run mode, records a summary change message and returns.
2. **Collect thread times** — Iterates all threads and pairs each TID with its total CPU time (`KernelTime + UserTime`).
3. **Sort descending** — Sorts threads by total CPU time in descending order so the busiest threads are assigned first.
4. **Random offset** — Generates a random `u8` shift value to randomize the starting position in the CPU list. This prevents the same CPUs from always receiving the highest-activity threads across successive calls.
5. **Round-robin assignment** — For each thread (in sorted order):
   - Opens a thread handle via [`get_thread_handle`](../winapi.rs/get_thread_handle.md).
   - Selects a write handle, preferring `w_handle` over `w_limited_handle`.
   - Computes the target CPU as `cpus[(success_count + random_shift) % cpu_count]`.
   - Calls `SetThreadIdealProcessorEx` with processor group 0 and the target CPU number.
   - On failure, logs via [`log_error_if_new`](log_error_if_new.md). On success, increments the success counter.
6. **Summary** — Records a single change entry: `"reset ideal processor for {N} threads"`.

### Platform notes

- `SetThreadIdealProcessorEx` is a hint to the Windows scheduler, not a hard constraint. The OS may still schedule the thread on other CPUs.
- All assignments use processor group 0. Systems with more than 64 logical processors spanning multiple groups are not fully handled.
- Thread handles opened by this function are dropped immediately after use (not cached in `PrimeThreadScheduler`).

### Edge cases

- If no threads can be opened (e.g., all handles fail), the summary message reports `"reset ideal processor for 0 threads"`.
- The random shift wraps via modulo arithmetic, so it is safe for any value of `random::<u8>()`.

## Requirements

| | |
|---|---|
| **Module** | `src/apply.rs` |
| **Called by** | [apply_affinity](apply_affinity.md) (after successful `SetProcessAffinityMask`), [apply_process_default_cpuset](apply_process_default_cpuset.md) (when `cpu_set_reset_ideal` is `true`) |
| **Calls** | [`get_thread_handle`](../winapi.rs/get_thread_handle.md), [`set_thread_ideal_processor_ex`](../winapi.rs/set_thread_ideal_processor_ex.md), [`log_error_if_new`](log_error_if_new.md) |
| **Win32 API** | [SetThreadIdealProcessorEx](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadidealprocessorex) |
| **Privileges** | `THREAD_SET_INFORMATION` (or `THREAD_SET_LIMITED_INFORMATION`) on each thread |

## See Also

| | |
|---|---|
| [apply_affinity](apply_affinity.md) | Sets process affinity mask; calls this function on success |
| [apply_process_default_cpuset](apply_process_default_cpuset.md) | Sets process default CPU sets; optionally calls this function |
| [apply_ideal_processors](apply_ideal_processors.md) | Rule-based ideal processor assignment for thread-level config |
| [ApplyConfigResult](ApplyConfigResult.md) | Accumulator for changes and errors |
| [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md) | Process-level configuration struct |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*