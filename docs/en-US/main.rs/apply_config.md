# apply_config function (main.rs)

Orchestrates all configuration application steps for a single process in a defined order, collecting changes and errors into an [`ApplyConfigResult`](../apply.rs/ApplyConfigResult.md).

## Syntax

```rust
pub fn apply_config(
    pid: u32,
    config: &ProcessConfig,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    process: &mut ProcessEntry,
    dry_run: bool,
) -> ApplyConfigResult
```

## Parameters

`pid`

The process identifier of the target process to configure.

`config`

Reference to a [`ProcessConfig`](../config.rs/ProcessConfig.md) containing all desired settings for this process — priority, affinity, CPU sets, I/O priority, memory priority, prime thread rules, and ideal processor rules.

`prime_core_scheduler`

Mutable reference to the [`PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md) that tracks per-thread cycle counts and manages prime thread selection state across loop iterations.

`process`

Mutable reference to a [`ProcessEntry`](../process.rs/ProcessEntry.md) representing the process from the current system snapshot. Provides access to thread information and is updated with applied state.

`dry_run`

When `true`, the function records what changes *would* be made without calling any Windows APIs to apply them. All change descriptions are still collected in the result with a dry-run indicator.

## Return value

Returns an [`ApplyConfigResult`](../apply.rs/ApplyConfigResult.md) containing:

- `changes` — a vector of human-readable strings describing each configuration change that was applied (or would be applied in dry-run mode).
- `errors` — a vector of human-readable strings describing errors encountered during the apply pass.

An empty result (both vectors empty) indicates the process was already in the desired state and no action was needed.

## Remarks

`apply_config` is the top-level orchestrator that calls each `apply_*` function in the [`apply.rs`](../apply.rs/README.md) module in a strict, defined order. This ordering is important because some steps depend on the results of earlier steps (e.g., prime thread scheduling depends on affinity being set first).

### Execution order

1. **[`apply_priority`](../apply.rs/apply_priority.md)** — sets the process priority class (e.g., `High`, `AboveNormal`).
2. **[`apply_affinity`](../apply.rs/apply_affinity.md)** — sets the hard CPU affinity mask, constraining which processors the process can use.
3. **[`apply_process_default_cpuset`](../apply.rs/apply_process_default_cpuset.md)** — sets the soft CPU set preference for the process.
4. **[`apply_io_priority`](../apply.rs/apply_io_priority.md)** — sets the I/O priority via `NtSetInformationProcess`.
5. **[`apply_memory_priority`](../apply.rs/apply_memory_priority.md)** — sets the memory page priority via `SetProcessInformation`.
6. **[`apply_prime_threads`](../apply.rs/apply_prime_threads.md)** — identifies and pins the highest-activity threads to designated fast cores, including cycle prefetching, selection, promotion, and demotion.
7. **[`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md)** — assigns ideal processors to threads based on their start module prefix rules.
8. **[`update_thread_stats`](../apply.rs/update_thread_stats.md)** — persists cached cycle and time data for delta calculation in the next iteration.

The function opens a [`ProcessHandle`](../winapi.rs/ProcessHandle.md) via [`get_process_handle`](../winapi.rs/get_process_handle.md) at the start. If the handle cannot be acquired, the function returns early with an empty result (or an error entry), since no operations can be performed without a valid handle.

### Caller context

This function is called from the main loop in [`main`](main.md) for each process that matches a configuration rule. The caller logs the contents of the returned [`ApplyConfigResult`](../apply.rs/ApplyConfigResult.md) — if the result is empty, no log output is produced for that process, keeping the log clean during steady-state operation.

### Dry-run mode

When `dry_run` is `true`, every `apply_*` function records what it *would* do without making actual Windows API calls. This allows the user to preview the effects of a new configuration before committing to it. The change descriptions in the result include a dry-run marker so the user can distinguish simulated changes from real ones.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/main.rs |
| **Source lines** | L46–L104 |
| **Called by** | [`main`](main.md) loop |
| **Calls** | [`get_process_handle`](../winapi.rs/get_process_handle.md), [`apply_priority`](../apply.rs/apply_priority.md), [`apply_affinity`](../apply.rs/apply_affinity.md), [`apply_process_default_cpuset`](../apply.rs/apply_process_default_cpuset.md), [`apply_io_priority`](../apply.rs/apply_io_priority.md), [`apply_memory_priority`](../apply.rs/apply_memory_priority.md), [`apply_prime_threads`](../apply.rs/apply_prime_threads.md), [`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md), [`update_thread_stats`](../apply.rs/update_thread_stats.md) |

## See also

- [ApplyConfigResult struct](../apply.rs/ApplyConfigResult.md)
- [apply.rs module overview](../apply.rs/README.md)
- [ProcessConfig struct](../config.rs/ProcessConfig.md)
- [PrimeThreadScheduler struct](../scheduler.rs/PrimeThreadScheduler.md)
- [main function](main.md)
- [main.rs module overview](README.md)