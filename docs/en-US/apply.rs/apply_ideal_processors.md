# apply_ideal_processors function (apply.rs)

Assigns ideal processor hints to threads based on configurable [`IdealProcessorRule`](../config.rs/IdealProcessorRule.md) entries. Each rule specifies a set of CPUs and optional module-name prefixes; the function matches threads by start-address module, selects the top *N* threads (where *N* = number of CPUs in the rule) using hysteresis, and round-robin assigns each selected thread to a dedicated ideal CPU. Threads that fall out of the top *N* have their ideal processor restored to the value observed before assignment.

## Syntax

```AffinityServiceRust/src/apply.rs#L1048-1057
pub fn apply_ideal_processors<'a>(
    pid: u32,
    config: &ThreadLevelConfig,
    dry_run: bool,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    prime_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | Process ID of the target process. |
| `config` | `&ThreadLevelConfig` | Thread-level configuration containing `ideal_processor_rules`. |
| `dry_run` | `bool` | When `true`, records what would change without calling any Windows API. |
| `threads` | `&impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>` | Lazy accessor returning the thread map for the process. |
| `prime_scheduler` | `&mut PrimeThreadScheduler` | Scheduler that owns per-thread stats (cycle caches, ideal-processor tracking state). |
| `apply_config_result` | `&mut ApplyConfigResult` | Accumulator for change and error messages. |

## Return value

This function does not return a value. All results are recorded in `apply_config_result`.

## Remarks

### Algorithm

For each [`IdealProcessorRule`](../config.rs/IdealProcessorRule.md) in `config.ideal_processor_rules`:

1. **Module matching** — Every thread whose start-address module matches one of the rule's `prefixes` (case-insensitive) is considered a candidate. If `prefixes` is empty, all threads are candidates.

2. **Selection with hysteresis** — Candidates are fed to [`PrimeThreadScheduler::select_top_threads_with_hysteresis`](../scheduler.rs/PrimeThreadScheduler.md) with a slot count equal to `rule.cpus.len()`. The `is_currently_assigned` predicate checks `thread_stats.ideal_processor.is_assigned`, which stabilises thread selection across polling intervals.

3. **Preserve existing assignments** — Threads that were already assigned an ideal CPU from a previous iteration keep their CPU slot if still selected. Their CPU is added to a `claimed` set to avoid double-allocation.

4. **New assignments** — Newly selected threads that are not yet assigned receive a CPU from the free pool (rule CPUs not in `claimed`) via round-robin. The API `SetThreadIdealProcessorEx` is called with processor group 0 and the target CPU number.

5. **Restoration** — Threads that were previously assigned but are no longer selected have their ideal processor restored to the value captured before the first assignment (`previous_group`, `previous_number`). The `is_assigned` flag is cleared.

### Ideal processor state tracking

Each thread's ideal-processor state is tracked in `ThreadStats.ideal_processor`:

| Field | Purpose |
|-------|---------|
| `is_assigned` | Whether this function currently owns the thread's ideal processor. |
| `previous_group` / `previous_number` | The ideal processor before the first assignment, used for restoration. |
| `current_group` / `current_number` | The ideal processor most recently set by this function. |

On first selection, `GetThreadIdealProcessorEx` is called to capture the baseline. If the thread's current ideal processor already falls within `rule.cpus`, it is kept without a redundant `Set` call.

### Dry-run behaviour

When `dry_run` is `true`, the function logs one summary change per rule indicating the CPU set and prefix filter, then returns without opening any thread handles or calling Windows APIs.

### Error handling

Errors from `GetThreadIdealProcessorEx` and `SetThreadIdealProcessorEx` are reported through [`log_error_if_new`](log_error_if_new.md), which deduplicates by `(pid, tid, operation, error_code)`. Invalid thread handles are logged once and the thread is skipped.

### Platform notes

- Ideal processor is a *hint* to the Windows scheduler, not a hard constraint. The OS may still schedule the thread on a different CPU.
- Only processor group 0 is supported for assignment. Threads whose baseline ideal processor is in group 0 and within `rule.cpus` are claimed without a `Set` call.

## Requirements

| | |
|---|---|
| **Module** | `apply` (`src/apply.rs`) |
| **Callers** | Service polling loop (via the top-level apply orchestrator) |
| **Callees** | [`PrimeThreadScheduler::select_top_threads_with_hysteresis`](../scheduler.rs/PrimeThreadScheduler.md), `resolve_address_to_module` (`winapi`), `get_thread_ideal_processor_ex` / `set_thread_ideal_processor_ex` (`winapi`), [`log_error_if_new`](log_error_if_new.md) |
| **Win32 API** | `SetThreadIdealProcessorEx`, `GetThreadIdealProcessorEx` (via `winapi` wrappers) |
| **Privileges** | `THREAD_SET_INFORMATION` (write handle), `THREAD_QUERY_INFORMATION` or `THREAD_QUERY_LIMITED_INFORMATION` (read handle) |

## See Also

| Topic | Link |
|-------|------|
| Result accumulator | [ApplyConfigResult](ApplyConfigResult.md) |
| Thread-level configuration | [ThreadLevelConfig](../config.rs/ThreadLevelConfig.md) |
| Ideal processor rule definition | [IdealProcessorRule](../config.rs/IdealProcessorRule.md) |
| Prime thread scheduler | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| Cycle prefetch (prerequisite) | [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) |
| Thread stats snapshot | [update_thread_stats](update_thread_stats.md) |
| Module overview | [apply.rs](README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*