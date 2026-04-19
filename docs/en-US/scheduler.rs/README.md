# scheduler module (AffinityServiceRust)

The `scheduler` module implements the prime thread scheduling engine — the core algorithm that identifies the hottest threads in a process and tracks their activity over time for CPU pinning decisions. It uses hysteresis-based selection to prevent promotion/demotion thrashing, where threads must sustain activity above an entry threshold for a minimum streak before being promoted, and must drop below a separate (lower) keep threshold before being demoted.

The module maintains per-process and per-thread statistics across polling iterations, including cycle counters, streak counters, thread handles, ideal processor assignments, and cached priority information.

## Structs

| Name | Description |
|------|-------------|
| [PrimeThreadScheduler](PrimeThreadScheduler.md) | Top-level scheduler struct that owns per-process statistics and hysteresis constants. Provides methods for alive tracking, streak updates, and hysteresis-based thread selection. |
| [ProcessStats](ProcessStats.md) | Per-process state container holding the thread stats map, alive flag, tracking configuration, and process metadata. |
| [IdealProcessorState](IdealProcessorState.md) | Per-thread ideal processor assignment state, tracking current and previous group/number assignments. |
| [ThreadStats](ThreadStats.md) | Per-thread state container with cycle/time counters, handle cache, CPU set pinning, active streak, start address, priority, and ideal processor state. |

## Functions

| Name | Description |
|------|-------------|
| [format_100ns](format_100ns.md) | Formats a 100-nanosecond time value into a human-readable `"seconds.milliseconds s"` string. |
| [format_filetime](format_filetime.md) | Converts a Windows FILETIME value (100ns units since 1601-01-01) into a local date-time string. |

## See Also

| Topic | Link |
|-------|------|
| Configuration constants for thresholds | [ConfigConstants](../config.rs/ConfigConstants.md) |
| Prime thread application logic | [apply_prime_threads](../apply.rs/apply_prime_threads.md) |
| Thread selection pass | [apply_prime_threads_select](../apply.rs/apply_prime_threads_select.md) |
| Thread handle management | [ThreadHandle](../winapi.rs/ThreadHandle.md) |
| Process snapshot data | [ProcessEntry](../process.rs/ProcessEntry.md) |
| Module address resolution | [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*