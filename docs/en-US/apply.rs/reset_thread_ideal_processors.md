# reset_thread_ideal_processors function (apply.rs)

Redistributes thread ideal processors across a specified set of CPUs after an affinity mask or CPU set change. When Windows changes process affinity, it may reset thread ideal processors to values that no longer make sense. This function re-assigns ideal processors by sorting threads by CPU time (descending) and distributing them round-robin across the target CPUs with a random offset to avoid always packing the highest-consuming threads onto the same core.

## Syntax

```AffinityServiceRust/src/apply.rs#L219-313
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

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | Process identifier of the target process. Used for error logging and thread handle acquisition. |
| `config` | `&ProcessConfig` | The parsed [ProcessConfig](../config.rs/ProcessConfig.md) for the process. The `name` field is used in error messages. |
| `dry_run` | `bool` | When `true`, records what *would* change in `apply_config_result` without calling any Windows APIs. |
| `cpus` | `&[u32]` | The set of CPU indices to distribute thread ideal processors across. Callers pass `&config.affinity_cpus` after an affinity change, or `&config.cpu_set_cpus` after a CPU-set change (when `cpu_set_reset_ideal` is enabled). |
| `process` | `&mut ProcessEntry` | The [ProcessEntry](../process.rs/ProcessEntry.md) for the target process. Provides access to the thread list and per-thread kernel/user time data from the most recent process snapshot. |
| `apply_config_result` | `&mut` [ApplyConfigResult](ApplyConfigResult.md) | Accumulator for change descriptions and error messages. |

## Return value

None (`()`).

## Remarks

### Algorithm

1. **Early exit** — If `cpus` is empty, the function returns immediately. In dry-run mode, a single summary change is recorded and the function returns.

2. **Collect and sort threads** — All threads from `process.get_threads()` are collected into a `Vec<(tid, total_cpu_time)>` where `total_cpu_time = KernelTime + UserTime` (in 100-ns units). The vector is sorted in descending order of CPU time so the busiest threads are assigned first.

3. **Open thread handles** — For every thread id in sorted order, a [ThreadHandle](../winapi.rs/ThreadHandle.md) is obtained via [get_thread_handle](../winapi.rs/get_thread_handle.md). The write handle (`w_handle`, falling back to `w_limited_handle`) is selected for each thread. Threads whose handles could not be opened are skipped.

4. **Round-robin with random shift** — A random `u8` offset is generated via `rand::random::<u8>()`. For thread at sorted index `i`, the target CPU is `cpus[(i + random_shift) % cpus.len()]`. The random shift ensures that across successive calls the distribution is not deterministic, preventing pathological packing where the highest-time thread always lands on CPU 0.

5. **Set ideal processor** — [set_thread_ideal_processor_ex](../winapi.rs/set_thread_ideal_processor_ex.md) is called with group `0` and the target CPU number. Failures are routed through [log_error_if_new](log_error_if_new.md); successes increment a counter.

6. **Report** — A single change entry `"reset ideal processor for N threads"` is recorded with the count of successful assignments.

### When this function is called

- **After affinity change** — [apply_affinity](apply_affinity.md) calls this function with `&config.affinity_cpus` immediately after a successful `SetProcessAffinityMask`. This compensates for Windows resetting ideal processors when the affinity mask changes.

- **After CPU set change** — [apply_process_default_cpuset](apply_process_default_cpuset.md) calls this function with `&config.cpu_set_cpus` when `config.cpu_set_reset_ideal` is `true`, just before applying `SetProcessDefaultCpuSets`. CPU sets are a soft preference and do not force ideal-processor resets, so this opt-in behaviour lets the user choose whether to redistribute.

### Thread handle lifetime

Thread handles are opened into a local `Vec<(u32, Option<ThreadHandle>)>`. When this vector goes out of scope at function exit, the `Drop` implementation on [ThreadHandle](../winapi.rs/ThreadHandle.md) automatically closes all OS handles.

### Processor group limitation

The function always passes group `0` to `set_thread_ideal_processor_ex`. This is correct for systems with up to 64 logical processors (a single processor group). Systems with more than 64 logical processors that span multiple groups are not currently supported by this function.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `apply` |
| Visibility | `pub` (crate-public) |
| Callers | [apply_affinity](apply_affinity.md), [apply_process_default_cpuset](apply_process_default_cpuset.md) |
| Callees | [get_thread_handle](../winapi.rs/get_thread_handle.md), [set_thread_ideal_processor_ex](../winapi.rs/set_thread_ideal_processor_ex.md), [log_error_if_new](log_error_if_new.md) |
| API | [SetThreadIdealProcessorEx](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadidealprocessorex) (Win32) |
| Privileges | `SeDebugPrivilege` for cross-process thread handle access (obtained at service startup via [enable_debug_privilege](../winapi.rs/enable_debug_privilege.md)) |

## See Also

| Topic | Link |
|-------|------|
| Hard affinity mask application | [apply_affinity](apply_affinity.md) |
| Soft CPU set application | [apply_process_default_cpuset](apply_process_default_cpuset.md) |
| Thread handle acquisition | [get_thread_handle](../winapi.rs/get_thread_handle.md) |
| Ideal processor Win32 wrapper | [set_thread_ideal_processor_ex](../winapi.rs/set_thread_ideal_processor_ex.md) |
| Rule-based ideal processor assignment | [apply_ideal_processors](apply_ideal_processors.md) |
| Process thread enumeration | [ProcessEntry](../process.rs/ProcessEntry.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd