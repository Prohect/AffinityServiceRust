# apply_ideal_processors function (apply.rs)

Assigns ideal processors to threads based on their start module, using per-rule prefix matching and hysteresis-based selection.

## Syntax

```rust
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

`pid`

The process ID of the target process.

`config`

Reference to the [ProcessConfig](../config.rs/ProcessConfig.md) containing `ideal_processor_rules`, each with a list of CPUs and optional module prefixes.

`dry_run`

When `true`, logs what changes would be made without calling any Windows APIs.

`process`

Mutable reference to the [ProcessEntry](../process.rs/ProcessEntry.md) for enumerating live thread IDs.

`prime_scheduler`

Mutable reference to the [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) that holds per-thread cached cycle counts, handles, start addresses, and [IdealProcessorState](../scheduler.rs/IdealProcessorState.md) tracking.

`apply_config_result`

Mutable reference to [ApplyConfigResult](ApplyConfigResult.md) that accumulates change and error messages.

## Return value

This function does not return a value. Results are recorded in `apply_config_result`.

## Remarks

This function processes each `IdealProcessorRule` in `config.ideal_processor_rules` independently. Each rule specifies a set of CPUs and optional module prefixes to match against thread start addresses.

### Algorithm per rule

1. **Filter threads by module prefix** — For each thread with nonzero cached cycles, resolve the start address to a module name via `resolve_address_to_module`. If the rule has prefixes, only threads whose module name (case-insensitive) starts with one of the prefixes are included. If no prefixes are specified, all threads match.

2. **Select top N using hysteresis** — Calls `select_top_threads_with_hysteresis` with N equal to the number of CPUs in the rule. The hysteresis predicate checks `thread_stats.ideal_processor.is_assigned` to determine whether a thread is currently promoted. This prevents rapid flipping between assigned/unassigned states.

3. **Claim already-assigned CPUs** — Threads that are already marked `is_assigned` have their `current_number` added to a claimed set. Newly selected threads that are not yet assigned have their current ideal processor queried via `GetThreadIdealProcessorEx`; if the current ideal processor happens to already be in the rule's CPU list, it is claimed directly (lazy set optimization — no syscall needed).

4. **Assign from free pool** — CPUs not yet claimed form a free pool. Each newly selected thread that still needs assignment receives the next free CPU via `SetThreadIdealProcessorEx`. On success, the thread's `IdealProcessorState` is updated and `is_assigned` is set to `true`.

5. **Restore demoted threads** — Threads that were previously assigned (`is_assigned == true`) but are no longer in the selected set have their ideal processor restored to the previous value (`previous_group`, `previous_number`) via `SetThreadIdealProcessorEx`. After restoration, `is_assigned` is cleared to `false`.

### Lazy set optimization

If a thread's current ideal processor is already within the rule's CPU pool, the function skips the `SetThreadIdealProcessorEx` syscall and simply marks the thread as assigned. This avoids unnecessary kernel transitions.

### Dry run behavior

When `dry_run` is `true`, logs a summary message for each rule showing the target CPUs, count, and matched prefixes, then returns without making any system calls.

### Change messages

- `"Thread {tid} -> ideal CPU {cpu} (group 0) start={module}"` — on successful assignment.
- `"Thread {tid} -> restored ideal CPU {number} (group {group}) start={module}"` — on successful demotion/restoration.

### Error handling

Errors from `GetThreadIdealProcessorEx` and `SetThreadIdealProcessorEx` are deduplicated via [log_error_if_new](log_error_if_new.md) before being recorded in `apply_config_result`.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/apply.rs |
| **Lines** | L1055–L1325 |
| **Called by** | `apply_config` in [main.rs](../main.rs/README.md) |
| **Depends on** | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md), [ProcessConfig](../config.rs/ProcessConfig.md), [ProcessEntry](../process.rs/ProcessEntry.md), [ApplyConfigResult](ApplyConfigResult.md), [log_error_if_new](log_error_if_new.md) |
| **Windows API** | `SetThreadIdealProcessorEx`, `GetThreadIdealProcessorEx` |

## See also

- [apply_affinity](apply_affinity.md) — hard affinity mask assignment
- [reset_thread_ideal_processors](reset_thread_ideal_processors.md) — round-robin ideal processor redistribution after affinity/CPU set changes
- [apply_prime_threads](apply_prime_threads.md) — CPU Set–based thread pinning (complementary to ideal processor assignment)