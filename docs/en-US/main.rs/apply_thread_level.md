# apply_thread_level function (main.rs)

Applies thread-level settings to a single process on every polling iteration. This includes prime-thread scheduling (selecting the highest-activity threads and pinning them to preferred CPU cores), ideal-processor assignment, and per-thread CPU cycle-time tracking. The function is called repeatedly across iterations so that the scheduler can react to workload changes over time.

## Syntax

```rust
fn apply_thread_level<'a>(
    pid: u32,
    config: &ThreadLevelConfig,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    process: &'a ProcessEntry,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    dry_run: bool,
    apply_configs: &mut ApplyConfigResult,
)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The Windows process identifier of the target process. |
| `config` | `&ThreadLevelConfig` | The thread-level configuration block for this process, containing prime-thread CPU lists, thread-name prefix filters, ideal-processor rules, and the `track_top_x_threads` debugging setting. |
| `prime_core_scheduler` | `&mut PrimeThreadScheduler` | Mutable reference to the scheduler that tracks per-thread cycle statistics and manages hysteresis-based prime-thread selection across iterations. |
| `process` | `&'a ProcessEntry` | A shared reference (with lifetime `'a`) to the process snapshot entry, used when applying prime-thread rules to resolve thread information. |
| `threads` | `&impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>` | A lazy closure that returns a reference to the thread map. The closure is backed by a `OnceCell` so the actual `get_threads()` call is deferred until first use and cached thereafter. This allows `apply_process_level` and `apply_thread_level` to share a single thread enumeration when called together via `apply_config`. |
| `dry_run` | `bool` | When `true`, no Win32 API calls that modify thread state are issued; changes are recorded in `apply_configs` for logging only. |
| `apply_configs` | `&mut ApplyConfigResult` | Accumulator for change descriptions and error messages produced during the apply operation. |

## Return value

This function does not return a value. All results (successes and errors) are recorded in the `apply_configs` accumulator.

## Remarks

The function short-circuits immediately if none of the following thread-level features are configured for the process:

- `prime_threads_cpus` — CPU cores to pin prime threads to.
- `prime_threads_prefixes` — Thread start-address module prefixes used to filter candidate threads.
- `ideal_processor_rules` — Rules for setting ideal processors on threads.
- `track_top_x_threads` — Nonzero value enables diagnostic logging of top threads on process exit.

When thread-level features are active, the function performs the following steps in order:

1. **Affinity mask query** — If `prime_threads_cpus` is non-empty, the current process affinity mask is obtained via `GetProcessAffinityMask` so that prime-thread CPU filtering respects the process-level mask. Unlike the previous implementation, the `affinity_cpus` field is no longer checked in this condition since affinity is now a process-level concern handled exclusively by `apply_process_level`.
2. **Module cache reset** — `drop_module_cache` is called to ensure thread start-address resolution uses fresh data.
3. **Scheduler liveness** — `prime_core_scheduler.set_alive(pid)` marks the process as alive in the scheduler so dead-process cleanup skips it.
4. **Cycle prefetch** — `prefetch_all_thread_cycles` queries per-thread cycle times and populates the scheduler's `ThreadStats` entries. The `threads` closure is passed through so thread enumeration is deferred until actually needed.
5. **Prime-thread application** — `apply_prime_threads` uses hysteresis-based selection to choose top threads and pin them to the configured CPU set. Both `process` and the `threads` closure are passed to this function.
6. **Ideal-processor assignment** — `apply_ideal_processors` sets the ideal processor for threads matching the configured rules. This function receives the `threads` closure directly (rather than a `process` reference).
7. **Stats update** — `update_thread_stats` caches the current cycle counts so that the next iteration can compute deltas.

Because this function is invoked every polling iteration (not just once per process), the scheduler accumulates multi-iteration history that feeds into the hysteresis algorithm, preventing thread promotion/demotion thrashing.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `main.rs` |
| Callers | [apply_config](apply_config.md), standalone thread-level apply loop in [main](main.md) |
| Callees | `apply::prefetch_all_thread_cycles`, `apply::apply_prime_threads`, `apply::apply_ideal_processors`, `apply::update_thread_stats`, `winapi::get_process_handle`, `winapi::drop_module_cache`, [PrimeThreadScheduler::set_alive](../scheduler.rs/PrimeThreadScheduler.md) |
| Win32 API | `GetProcessAffinityMask` |
| Privileges | `SeDebugPrivilege` (for opening thread/process handles of other sessions) |

## See Also

| Topic | Link |
|-------|------|
| apply_process_level | [apply_process_level](apply_process_level.md) |
| apply_config | [apply_config](apply_config.md) |
| PrimeThreadScheduler | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| ThreadLevelConfig | [config module](../config.rs/README.md) |
| main | [main](main.md) |

---
> Commit SHA: `b0df9da35213b050501fab02c3020ad4dbd6c4e0`
