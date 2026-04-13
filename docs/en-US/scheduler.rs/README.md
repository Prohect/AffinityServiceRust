# scheduler.rs Module (scheduler.rs)

The `scheduler` module implements the prime thread scheduling engine that dynamically assigns high-activity threads to designated "prime" CPU cores. It tracks per-thread CPU cycle deltas across loop iterations and uses hysteresis-based selection to prevent promotion/demotion thrashing.

## Overview

This module provides the stateful scheduler that powers the prime thread feature. It maintains cumulative statistics for every tracked thread across every tracked process, enabling cycle-delta computation between successive polling intervals. The scheduler uses a two-pass hysteresis algorithm to decide which threads qualify for prime CPU assignment:

1. **Keep pass** — Currently-assigned threads retain prime status if their cycle delta exceeds the keep threshold (default 69% of max).
2. **Promote pass** — Remaining slots are filled by unassigned threads whose cycle delta exceeds the entry threshold (default 42% of max) and whose active streak meets the minimum (default 2 consecutive intervals).

The module also provides helper functions for formatting Windows kernel time values and FILETIME timestamps into human-readable strings.

## Items

### Structs

| Name | Description |
| --- | --- |
| [PrimeThreadScheduler](PrimeThreadScheduler.md) | Top-level scheduler managing per-process stats and hysteresis-based thread selection. |
| [ProcessStats](ProcessStats.md) | Per-process container holding thread statistics and liveness tracking. |
| [IdealProcessorState](IdealProcessorState.md) | Tracks the current and previous ideal processor assignment for a thread. |
| [ThreadStats](ThreadStats.md) | Per-thread statistics including cycle counts, handle, streak, and ideal processor state. |

### Functions

| Name | Description |
| --- | --- |
| [format_100ns](format_100ns.md) | Formats a 100-nanosecond time value into `"seconds.milliseconds s"` form. |
| [format_filetime](format_filetime.md) | Converts a Windows FILETIME (100ns since 1601) into a local-time date string. |

## Architecture

### Lifecycle

The scheduler's lifecycle is driven by the main loop in [`apply_config_thread_level`](../main.rs/apply_config_thread_level.md):

1. **`reset_alive()`** — Mark all tracked processes as dead at the start of each iteration.
2. **`set_alive(pid)`** — Mark processes that are still running (matched by config rules).
3. **`set_tracking_info()`** — Update process name and top-thread count from config.
4. **`get_thread_stats(pid, tid)`** — Access or create per-thread stats for cycle prefetching.
5. **`update_active_streaks()`** — Compute streak counters from cycle deltas.
6. **`select_top_threads_with_hysteresis()`** — Two-pass selection of prime threads.
7. **`drop_process_by_pid(pid)`** — Called from the main loop when ETW reports a process exit, removing that process's handles and logging top-N thread reports.

### Hysteresis Constants

The default thresholds are defined in [`ConfigConstants`](../config.rs/ConfigConstants.md) and can be overridden in the configuration file:

| Constant | Default | Purpose |
| --- | --- | --- |
| `entry_threshold` | 0.42 | Minimum cycle ratio (vs. max) for a new thread to be promoted |
| `keep_threshold` | 0.69 | Minimum cycle ratio for an already-assigned thread to keep its slot |
| `min_active_streak` | 2 | Minimum consecutive qualifying intervals before promotion |

The asymmetry between entry and keep thresholds creates a hysteresis band that prevents threads from rapidly toggling between prime and non-prime status.

### Handle Management

[`ThreadStats`](ThreadStats.md) holds an `Option<ThreadHandle>` that caches the opened thread handle for the lifetime of the process. When a process exits (detected via ETW stop event), [`drop_process_by_pid`](PrimeThreadScheduler.md#methods) removes that process's `ThreadStats` entries, which triggers the `Drop` implementation on [`ThreadHandle`](../winapi.rs/ThreadHandle.md) to close the underlying Windows handles.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | `src/scheduler.rs` |
| **Called by** | [`apply_config_thread_level`](../main.rs/apply_config_thread_level.md) in `src/main.rs`, [`apply_prime_threads`](../apply.rs/apply_prime_threads.md) and related functions in `src/apply.rs`, [`main`](../main.rs/main.md) loop (for ETW stop events) |
| **Key dependencies** | [`ConfigConstants`](../config.rs/ConfigConstants.md), [`ThreadPriority`](../priority.rs/ThreadPriority.md), [`ThreadHandle`](../winapi.rs/ThreadHandle.md) |
| **Crate dependencies** | `chrono`, `ntapi` |

## See also

- [apply_prime_threads](../apply.rs/apply_prime_threads.md) — orchestrates the per-process prime thread application
- [apply_prime_threads_select](../apply.rs/apply_prime_threads_select.md) — calls into the scheduler's selection method
- [ConfigConstants](../config.rs/ConfigConstants.md) — hysteresis constants parsed from config
- [priority.rs module](../priority.rs/README.md) — ThreadPriority used for original priority tracking