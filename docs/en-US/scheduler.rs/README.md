# scheduler module (AffinityServiceRust)

The `scheduler` module implements the core thread-scheduling engine for AffinityServiceRust. It tracks per-process and per-thread CPU cycle statistics, manages thread handle lifetimes, and uses a hysteresis-based algorithm to select "prime" threads—those that deserve preferential CPU core placement. The module also provides ideal-processor state tracking for thread-to-core pinning, and utility functions for formatting Windows kernel time values.

## Structs

| Struct | Description |
|--------|-------------|
| [PrimeThreadScheduler](PrimeThreadScheduler.md) | Top-level scheduler that owns per-process statistics and exposes methods for streak tracking, hysteresis-based thread selection, and process cleanup. |
| [ProcessStats](ProcessStats.md) | Per-process bookkeeping: liveness flag, thread-stats map, top-thread tracking count, and process metadata. |
| [IdealProcessorState](IdealProcessorState.md) | Tracks the current and previous ideal-processor assignment (group + number) for a single thread. |
| [ThreadStats](ThreadStats.md) | Per-thread statistics including cycle counters, cached timing data, thread handle, CPU-set pin list, active-streak counter, priority snapshot, and ideal-processor state. |

## Functions

| Function | Description |
|----------|-------------|
| [format_100ns](format_100ns.md) | Converts a Windows 100-nanosecond time value into a human-readable `seconds.milliseconds` string. |
| [format_filetime](format_filetime.md) | Converts a Windows `FILETIME` 100-nanosecond value into a local-time date-time string. |

## See Also

| Resource | Link |
|----------|------|
| main module | [main.rs README](../main.rs/README.md) |
| priority module | [priority.rs README](../priority.rs/README.md) |
| apply module | [apply.rs README](../apply.rs/README.md) |
| config module | [config.rs README](../config.rs/README.md) |
| winapi module | [winapi.rs README](../winapi.rs/README.md) |

---
*Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
