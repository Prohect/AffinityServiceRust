# apply_ideal_processors function (apply.rs)

Assigns ideal processors to threads based on configurable rules that match thread start-module prefixes to dedicated CPU sets. For each rule, the function identifies threads whose start module matches one of the rule's prefixes, selects the top *N* threads by CPU cycle delta (where *N* equals the number of CPUs in the rule) using the same hysteresis algorithm as prime-thread selection, and pins each selected thread to a dedicated CPU via `SetThreadIdealProcessorEx`. When a thread drops out of the top *N*, its ideal processor is restored to the value it had before assignment.

## Syntax

```AffinityServiceRust/src/apply.rs#L1061-1072
pub fn apply_ideal_processors(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process: &mut ProcessEntry,
    prime_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | Process identifier of the target process. Used as the key into the [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) state maps and for logging. |
| `config` | `&`[ProcessConfig](../config.rs/ProcessConfig.md) | Parsed configuration for this process. The `ideal_processor_rules` field (a `Vec<`[IdealProcessorRule](../config.rs/IdealProcessorRule.md)`>`) contains zero or more rules, each specifying a set of CPU indices and a list of module-name prefixes. |
| `dry_run` | `bool` | When `true`, the function records a summary of what each rule *would* do in `apply_config_result` without calling any Windows APIs or modifying scheduler state. |
| `process` | `&mut`[ProcessEntry](../process.rs/ProcessEntry.md) | Snapshot entry for the target process. Provides the thread list (thread IDs) used to enumerate candidates. |
| `prime_scheduler` | `&mut`[PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) | Persistent scheduler state that holds per-thread [ThreadStats](../scheduler.rs/ThreadStats.md), including cached cycle counts, thread handles, start addresses, and [IdealProcessorState](../scheduler.rs/IdealProcessorState.md). The function reads `cached_cycles`, `last_cycles`, `start_address`, and `ideal_processor` fields and writes to `ideal_processor.current_group`, `ideal_processor.current_number`, `ideal_processor.previous_group`, `ideal_processor.previous_number`, and `ideal_processor.is_assigned`. |
| `apply_config_result` | `&mut`[ApplyConfigResult](ApplyConfigResult.md) | Accumulator for change descriptions and error messages produced during this operation. |

## Return value

None (`()`). Results are communicated through `apply_config_result` and side effects on `prime_scheduler`.

## Remarks

### Algorithm

The function processes each [IdealProcessorRule](../config.rs/IdealProcessorRule.md) independently. For each rule:

**Step 1 — Collect thread info.**
All threads with non-zero `cached_cycles` in the scheduler are collected into a `Vec<(tid, delta_cycles, start_address, start_module)>`. The start module is resolved from the thread's start address via [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md). This collection is computed once and shared across all rules.

**Step 2 — Filter by prefix.**
The thread list is filtered to include only threads whose start module (lowercased) starts with one of the rule's prefixes (also lowercased). If the rule's `prefixes` list is empty, *all* threads match — this allows a "catch-all" rule that assigns ideal processors across the entire process without module filtering.

**Step 3 — Select top N via hysteresis.**
A `Vec<(tid, delta_cycles, is_selected)>` is built from the filtered threads and passed to `prime_scheduler.select_top_threads_with_hysteresis()`. The selection uses the same [hysteresis algorithm](../scheduler.rs/PrimeThreadScheduler.md#hysteresis-algorithm) as [apply_prime_threads_select](apply_prime_threads_select.md), but the "is currently assigned" predicate checks `thread_stats.ideal_processor.is_assigned` instead of `pinned_cpu_set_ids`. *N* equals `rule.cpus.len()` — the number of CPUs available for ideal-processor assignment in this rule.

**Step 4 — Claim CPUs already held.**
For each selected thread that already has `is_assigned == true`, the CPU it currently holds is added to a "claimed" set. For newly-selected threads (not yet assigned), the function reads the thread's current ideal processor via [get_thread_ideal_processor_ex](../winapi.rs/get_thread_ideal_processor_ex.md) and saves it into `ideal_processor.previous_group` and `ideal_processor.previous_number`. If the thread's current ideal processor happens to already be in `rule.cpus`, it is claimed in-place without needing reassignment.

**Step 5 — Assign from free pool.**
CPUs in `rule.cpus` that are not in the claimed set form the "free pool". Newly-selected threads that are not yet assigned are allocated a CPU from this pool in order. [set_thread_ideal_processor_ex](../winapi.rs/set_thread_ideal_processor_ex.md) is called with group `0` and the target CPU number. On success, `ideal_processor.current_group`, `ideal_processor.current_number`, and `ideal_processor.is_assigned` are updated. A change message is recorded:

`"Thread 1234 -> ideal CPU 5 (group 0) start=game.dll!WorkerThread"`

If the free pool is exhausted before all selected threads are assigned, the remaining threads are skipped.

**Step 6 — Restore unselected threads.**
For each thread that was previously assigned (`is_assigned == true`) but is no longer in the selected set, the function restores the ideal processor to `(previous_group, previous_number)` if they differ from `(current_group, current_number)`. [set_thread_ideal_processor_ex](../winapi.rs/set_thread_ideal_processor_ex.md) is called with the previous values, and `is_assigned` is set to `false`. A change message is recorded:

`"Thread 1234 -> restored ideal CPU 3 (group 0) start=game.dll!WorkerThread"`

### Dry-run behaviour

In dry-run mode, the function records one change message per rule summarising the intent:

`"Ideal Processor: CPUs [4,5,6] for top 3 threads from [game.dll; render.dll]"`

When the rule's prefix list is empty, the message reads `"from [all modules]"`.

No thread handles are opened, no Win32 calls are made, and no scheduler state is modified.

### Multiple rules

Each rule operates independently. A single thread can potentially match multiple rules if its start module matches prefixes in more than one rule. However, the `is_assigned` flag is shared across rules, so once a thread is assigned by an earlier rule, later rules will see it as already assigned and may claim its CPU or skip it. The order of rules in the configuration therefore matters when rules have overlapping prefix sets.

### Processor group limitation

Like [reset_thread_ideal_processors](reset_thread_ideal_processors.md), this function always operates within processor group `0`. Systems with more than 64 logical processors that span multiple processor groups are not fully supported; only group-0 CPUs are assignable.

### Relationship to prime-thread scheduling

`apply_ideal_processors` and [apply_prime_threads](apply_prime_threads.md) address different use cases:

| Aspect | Prime threads | Ideal processors |
|--------|--------------|------------------|
| Mechanism | Per-thread CPU sets (`SetThreadSelectedCpuSets`) | Ideal processor hint (`SetThreadIdealProcessorEx`) |
| Strength | Hard constraint — thread *cannot* run elsewhere | Soft hint — scheduler *prefers* the indicated CPU |
| Priority boost | Yes (configurable or auto +1 level) | No |
| Module filtering | Via `prime_threads_prefixes` | Via `ideal_processor_rules[].prefixes` |

Both features share the [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) state and the same hysteresis selection algorithm, but they write to different fields in [ThreadStats](../scheduler.rs/ThreadStats.md) (`pinned_cpu_set_ids` vs. `ideal_processor`).

### Error handling

Handle resolution and Win32 API call failures are routed through [log_error_if_new](log_error_if_new.md). The operations logged include:

| Operation | When |
|-----------|------|
| `Operation::OpenThread` | Thread handle is invalid (both `w_handle` and `w_limited_handle`). |
| `Operation::GetThreadIdealProcessorEx` | Reading current ideal processor for a newly-selected thread fails. |
| `Operation::SetThreadIdealProcessorEx` | Setting or restoring an ideal processor fails. |

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `apply` |
| Visibility | `pub` (crate-public) |
| Callers | [apply_config_thread_level](../main.rs/apply_config_thread_level.md) |
| Callees | [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md), [set_thread_ideal_processor_ex](../winapi.rs/set_thread_ideal_processor_ex.md), [get_thread_ideal_processor_ex](../winapi.rs/get_thread_ideal_processor_ex.md), [format_cpu_indices](../config.rs/format_cpu_indices.md), [log_error_if_new](log_error_if_new.md), [PrimeThreadScheduler::select_top_threads_with_hysteresis](../scheduler.rs/PrimeThreadScheduler.md), [PrimeThreadScheduler::get_thread_stats](../scheduler.rs/PrimeThreadScheduler.md) |
| Win32 API | [`SetThreadIdealProcessorEx`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadidealprocessorex), [`GetThreadIdealProcessorEx`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadidealprocessorex) |
| Privileges | `THREAD_SET_INFORMATION` (write), `THREAD_QUERY_INFORMATION` (read ideal processor). `SeDebugPrivilege` provides cross-process thread access. |

## See Also

| Topic | Link |
|-------|------|
| apply module overview | [apply](README.md) |
| Ideal processor rule configuration | [IdealProcessorRule](../config.rs/IdealProcessorRule.md) |
| Ideal processor redistribution after affinity change | [reset_thread_ideal_processors](reset_thread_ideal_processors.md) |
| Prime-thread scheduling (hard CPU set pinning) | [apply_prime_threads](apply_prime_threads.md) |
| Hysteresis selection algorithm | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| Per-thread ideal processor state | [IdealProcessorState](../scheduler.rs/IdealProcessorState.md) |
| Thread start-module resolution | [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md) |
| Win32 ideal processor wrappers | [set_thread_ideal_processor_ex](../winapi.rs/set_thread_ideal_processor_ex.md), [get_thread_ideal_processor_ex](../winapi.rs/get_thread_ideal_processor_ex.md) |
| Thread-level apply orchestration | [apply_config_thread_level](../main.rs/apply_config_thread_level.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd