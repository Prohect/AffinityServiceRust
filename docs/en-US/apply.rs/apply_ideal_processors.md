# apply_ideal_processors function (apply.rs)

The `apply_ideal_processors` function assigns ideal processors to threads based on module-prefix matching rules defined in the configuration. For each rule, it identifies threads whose start module matches the specified prefixes, selects the top N threads by cycle-count delta (where N equals the number of CPUs in the rule), and assigns each selected thread to a dedicated CPU via `SetThreadIdealProcessorEx`. When a thread drops out of the top N in a subsequent apply cycle, its ideal processor is restored to its previous value. The function uses the same hysteresis-based selection mechanism as the prime-thread pipeline to prevent rapid oscillation.

## Syntax

```AffinityServiceRust/src/apply.rs#L1047-1058
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
| `pid` | `u32` | The process ID of the target process. Used for scheduler lookups, error deduplication, and log messages. |
| `config` | `&ThreadLevelConfig` | The thread-level configuration containing `ideal_processor_rules`, a list of rules each specifying a set of CPU indices (`cpus`) and optional module prefixes (`prefixes`). If `ideal_processor_rules` is empty, the function returns immediately. |
| `dry_run` | `bool` | When `true`, synthetic change messages are recorded describing what ideal processor assignments would be made, without calling any Windows APIs. When `false`, the actual assignments and restorations are performed. |
| `threads` | `&impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>` | A lazy closure that returns a reference to a map of thread IDs to their `SYSTEM_THREAD_INFORMATION` snapshots from the most recent system process information query. The closure is invoked on demand to enumerate candidate threads, deferring the cost of thread enumeration until it is actually needed. |
| `prime_scheduler` | `&mut PrimeThreadScheduler` | The mutable prime-thread scheduler state that tracks per-thread statistics including cached cycles, start addresses, thread handles, and ideal processor assignment state across apply cycles. |
| `apply_config_result` | `&mut ApplyConfigResult` | Accumulator for change descriptions and error messages produced during execution. |

## Return value

This function does not return a value. All outcomes are communicated through mutations to `prime_scheduler` thread stats and appended entries in `apply_config_result`.

## Remarks

### Early exit

If `config.ideal_processor_rules` is empty, the function returns immediately without performing any work.

### Dry-run mode

In dry-run mode, for each rule, a change message is recorded of the form:
`"Ideal Processor: CPUs [<cpu_list>] for top <N> threads from [<prefixes>]"`
where `<N>` is the number of CPUs in the rule and `<prefixes>` is either the joined list of module prefixes or `"all modules"` if no prefixes are specified.

### Algorithm (non-dry-run)

1. **Resolve module names**: For all threads with `cached_cycles > 0`, the function resolves each thread's start address to a module name via `resolve_address_to_module`. Module names are collected into a shared `Vec<String>` and indexed to avoid redundant resolution. Each thread is represented as a `(tid, delta_cycles, start_address, name_index)` tuple.

2. **Per-rule processing**: For each rule in `config.ideal_processor_rules`:

   a. **Filter by prefix**: If the rule has prefixes, only threads whose start module (lowercased) starts with one of the prefixes are included. If the rule has no prefixes, all threads are candidates.

   b. **Hysteresis-based selection**: The function calls `PrimeThreadScheduler::select_top_threads_with_hysteresis` with `rule.cpus.len()` as the target count and `|ts| ts.ideal_processor.is_assigned` as the "is currently selected" predicate. Threads already assigned an ideal processor receive the more lenient keep threshold; new candidates must exceed the stricter entry threshold and meet the active-streak minimum.

   c. **Claim existing assignments**: For threads selected as prime that already have `ideal_processor.is_assigned == true`, their current CPU number is added to a `claimed` set. For newly-selected threads, `GetThreadIdealProcessorEx` is called to capture the thread's current ideal processor as the `previous_group`/`previous_number` baseline. If the thread's current ideal processor happens to already be one of the rule's CPUs, it is marked as assigned and claimed immediately.

   d. **Assign from free pool**: CPUs from the rule that are not in the `claimed` set form the free pool. Each newly-selected thread that is not yet assigned is given the next available CPU from the free pool via `set_thread_ideal_processor_ex` (group 0, target CPU). On success, the thread's `ideal_processor.current_group` and `current_number` are updated, `is_assigned` is set to `true`, and a change message is recorded:
      `"Thread <tid> -> ideal CPU <cpu> (group 0) start=<module>"`

   e. **Restore unselected threads**: Threads that have `ideal_processor.is_assigned == true` but are no longer in the selected set are restored to their `previous_group`/`previous_number` via `set_thread_ideal_processor_ex`. On success, the thread's `current_group`/`current_number` are updated to match the previous values, and `is_assigned` is cleared to `false`. A change message is recorded:
      `"Thread <tid> -> restored ideal CPU <prev_number> (group <prev_group>) start=<module>"`

### Edge cases

- If a rule's `cpus` list is empty, that rule is skipped entirely.
- If a rule has no prefixes, all threads with cached cycles are eligible candidates for that rule.
- If `GetThreadIdealProcessorEx` fails during the claim phase, the error is logged via [`log_error_if_new`](log_error_if_new.md) with `Operation::GetThreadIdealProcessorEx` and the thread is not assigned.
- If `set_thread_ideal_processor_ex` fails during assignment or restoration, the error is logged via [`log_error_if_new`](log_error_if_new.md) with `Operation::SetThreadIdealProcessorEx`. The thread's `is_assigned` state is left unchanged on assignment failure, but is cleared on restoration failure to avoid infinite retries.
- Threads with `cached_cycles == 0` (i.e., threads whose cycles were not prefetched successfully) are excluded from all rule processing.
- Thread handle validation follows the same pattern as other functions: `w_handle` is preferred, falling back to `w_limited_handle`. If both are invalid, an error is logged and the thread is skipped.
- When restoring an ideal processor, the restoration is only performed when the previous and current values differ (`prev_group != cur_group || prev_number != cur_number`), avoiding unnecessary API calls for threads that were assigned to a CPU they were already on.

### Per-thread ideal processor state

The `ideal_processor` field in `ThreadStats` tracks three pieces of information:
- **`previous_group` / `previous_number`**: The thread's ideal processor at the time it was first selected. This is the value that will be restored on demotion.
- **`current_group` / `current_number`**: The thread's currently assigned ideal processor. Updated after each successful `set_thread_ideal_processor_ex` call.
- **`is_assigned`**: A boolean indicating whether this function has an active ideal processor assignment for the thread.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `apply.rs` |
| Crate | `AffinityServiceRust` |
| Visibility | `pub` |
| Windows APIs | `SetThreadIdealProcessorEx` (via `winapi::set_thread_ideal_processor_ex`), `GetThreadIdealProcessorEx` (via `winapi::get_thread_ideal_processor_ex`), `GetLastError` |
| Callers | Orchestrator code in `scheduler.rs` / `main.rs` that iterates matched processes |
| Callees | [`log_error_if_new`](log_error_if_new.md), `winapi::resolve_address_to_module`, `winapi::get_thread_ideal_processor_ex`, `winapi::set_thread_ideal_processor_ex`, `config::format_cpu_indices`, `error_codes::error_from_code_win32`, `PrimeThreadScheduler::get_thread_stats`, `PrimeThreadScheduler::select_top_threads_with_hysteresis` |
| Privileges | Requires thread handles with `THREAD_SET_INFORMATION` or `THREAD_SET_LIMITED_INFORMATION` (write) and `THREAD_QUERY_INFORMATION` or `THREAD_QUERY_LIMITED_INFORMATION` (read for `GetThreadIdealProcessorEx`). |

## See Also

| Reference | Link |
|-----------|------|
| apply module overview | [`README`](README.md) |
| ApplyConfigResult | [`ApplyConfigResult`](ApplyConfigResult.md) |
| reset_thread_ideal_processors | [`reset_thread_ideal_processors`](reset_thread_ideal_processors.md) |
| apply_prime_threads | [`apply_prime_threads`](apply_prime_threads.md) |
| apply_prime_threads_select | [`apply_prime_threads_select`](apply_prime_threads_select.md) |
| prefetch_all_thread_cycles | [`prefetch_all_thread_cycles`](prefetch_all_thread_cycles.md) |
| update_thread_stats | [`update_thread_stats`](update_thread_stats.md) |
| log_error_if_new | [`log_error_if_new`](log_error_if_new.md) |
| ThreadLevelConfig | [`config.rs/ThreadLevelConfig`](../config.rs/ThreadLevelConfig.md) |
| PrimeThreadScheduler | [`scheduler.rs/PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md) |

---
*Commit: b0df9da35213b050501fab02c3020ad4dbd6c4e0*