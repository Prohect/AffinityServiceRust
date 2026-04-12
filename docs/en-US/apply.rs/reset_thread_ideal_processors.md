# reset_thread_ideal_processors function (apply.rs)

Redistributes thread ideal processors across a specified set of CPUs. Called after affinity changes or CPU set changes to rebalance thread placement and avoid clumping.

## Syntax

```rust
pub fn reset_thread_ideal_processors(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    cpus: &[u32],
    process: &mut ProcessEntry,
    apply_config_result: &mut ApplyConfigResult,
)
```

## Parameters

`pid`

The process ID of the target process.

`config`

Reference to the [ProcessConfig](../config.rs/ProcessConfig.md) containing the process rule. Used for the process name in error messages and for opening thread handles.

`dry_run`

When `true`, logs the intended change without calling any Windows APIs. The logged message includes the number of CPUs being targeted.

`cpus`

Slice of CPU indices to distribute thread ideal processors across. Callers pass `&config.affinity_cpus` after an affinity change, or `&config.cpu_set_cpus` after a CPU set change (when `cpu_set_reset_ideal` is enabled). If empty, the function returns immediately.

`process`

Mutable reference to the [ProcessEntry](../process.rs/ProcessEntry.md) for the target process. Provides the thread list with per-thread timing information used for sorting.

`apply_config_result`

Mutable reference to [ApplyConfigResult](ApplyConfigResult.md) that accumulates change messages and error messages.

## Return value

This function does not return a value. Results are recorded via `apply_config_result`.

## Remarks

### Algorithm

1. **Collect threads**: Iterates all threads in the process snapshot, pairing each thread ID with its total CPU time (kernel + user).
2. **Sort by CPU time**: Sorts threads in descending order of total CPU time so the most active threads are assigned first.
3. **Open handles**: Opens a write handle for each thread via [get_thread_handle](../winapi.rs/get_thread_handle.md). Prefers the full-access write handle (`w_handle`); falls back to the limited write handle (`w_limited_handle`).
4. **Round-robin assignment with random shift**: Assigns each thread an ideal processor from the `cpus` slice using the formula:

   ```
   target_cpu = cpus[(thread_index + random_shift) % cpus.len()]
   ```

   The `random_shift` is a random `u8` value generated once per call. This prevents always assigning the first (highest-CPU-time) thread to the same core, distributing load more evenly across invocations.
5. **Lazy set not used here**: Unlike [apply_ideal_processors](apply_ideal_processors.md), this function does *not* skip the syscall when the thread's current ideal processor is already on the target CPU. Every thread is unconditionally reassigned.
6. **Change logged**: On completion, logs `"reset ideal processor for {N} threads"` where N is the number of successful assignments.

### Handle cleanup

Thread handles are wrapped in `ThreadHandle` structs whose `Drop` implementation automatically closes the underlying OS handles when `tid_handles` goes out of scope at the end of the function.

### Callers

- [apply_affinity](apply_affinity.md) — called immediately after a successful `SetProcessAffinityMask` with `&config.affinity_cpus`.
- [apply_process_default_cpuset](apply_process_default_cpuset.md) — called before `SetProcessDefaultCpuSets` when `config.cpu_set_reset_ideal` is `true`, with `&config.cpu_set_cpus`.
- [apply_config](../main.rs/apply_config.md) in `main.rs` — may call after a CPU set change when `cpu_set_reset_ideal` is set.

### Error handling

If a thread handle is invalid (both `w_handle` and `w_limited_handle`), the error is logged via [log_error_if_new](log_error_if_new.md) with `Operation::OpenThread` and the thread is skipped. If `SetThreadIdealProcessorEx` fails, the error is logged with `Operation::SetThreadIdealProcessorEx` and the thread is skipped; remaining threads continue to be processed.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | `src/apply.rs` (lines 210–313) |
| **Called by** | [apply_affinity](apply_affinity.md), [apply_process_default_cpuset](apply_process_default_cpuset.md), [apply_config](../main.rs/apply_config.md) |
| **Windows API** | `SetThreadIdealProcessorEx` via [set_thread_ideal_processor_ex](../winapi.rs/set_thread_ideal_processor_ex.md) |
| **Related** | [apply_ideal_processors](apply_ideal_processors.md) (rule-based ideal processor assignment) |

## See also

- [apply_affinity](apply_affinity.md)
- [apply_process_default_cpuset](apply_process_default_cpuset.md)
- [apply_ideal_processors](apply_ideal_processors.md)
- [ApplyConfigResult](ApplyConfigResult.md)