# scheduler module (AffinityServiceRust)

The `scheduler` module implements hysteresis-based prime thread scheduling for the AffinityServiceRust service. It tracks per-process and per-thread CPU cycle statistics across polling intervals, maintains active streak counters to filter out transiently busy threads, and selects the top N threads for promotion to performance cores. The two-pass selection algorithm (retain → fill) prevents promotion/demotion thrashing by applying separate keep and entry thresholds. The module also provides utility functions for formatting Windows timing values.

## Structs

| Struct | Description |
|--------|-------------|
| [PrimeThreadScheduler](PrimeThreadScheduler.md) | Central scheduler that owns per-process stats maps and exposes hysteresis-based thread selection. |
| [ProcessStats](ProcessStats.md) | Per-process container holding liveness state, thread stats, and tracking configuration. |
| [IdealProcessorState](IdealProcessorState.md) | Tracks current and previous ideal processor assignments for a single thread. |
| [ThreadStats](ThreadStats.md) | Per-thread statistics including cycle counts, active streak, thread handle, CPU set pins, and ideal processor state. |

## Functions

| Function | Description |
|----------|-------------|
| [format_100ns](format_100ns.md) | Formats a 100-nanosecond interval count as a human-readable `seconds.milliseconds s` string. |
| [format_filetime](format_filetime.md) | Converts a Windows FILETIME 64-bit value to a local date-time string (`YYYY-MM-DD HH:MM:SS.mmm`). |

## See Also

| Topic | Link |
|-------|------|
| Process snapshot and enumeration | [process module](../process.rs/README.md) |
| Configuration constants (thresholds) | [ConfigConstants](../config.rs/ConfigConstants.md) |
| Prime thread promotion and demotion | [apply_prime_threads](../apply.rs/apply_prime_threads.md) |
| Thread handle management | [ThreadHandle](../winapi.rs/ThreadHandle.md) |
| Thread priority levels | [ThreadPriority](../priority.rs/ThreadPriority.md) |