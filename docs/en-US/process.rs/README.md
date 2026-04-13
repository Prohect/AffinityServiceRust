# process module (AffinityServiceRust)

Provides an RAII-based process snapshot mechanism built on the Windows Native API function `NtQuerySystemInformation` with `SystemProcessInformation` information class. This module captures a point-in-time view of all running processes and their threads on the system, exposing them through safe Rust abstractions. The snapshot data is stored in global buffers guarded by `Mutex`, and the `ProcessSnapshot` struct ensures cleanup on drop. Individual processes are represented by `ProcessEntry`, which lazily parses raw thread arrays into `HashMap` collections for efficient TID-based lookup.

## Statics

| Name | Type | Description |
|------|------|-------------|
| [SNAPSHOT_BUFFER](SNAPSHOT_BUFFER.md) | `Lazy<Mutex<Vec<u8>>>` | Global byte buffer used by `NtQuerySystemInformation` to receive process/thread data. |
| [PID_TO_PROCESS_MAP](PID_TO_PROCESS_MAP.md) | `Lazy<Mutex<HashMap<u32, ProcessEntry>>>` | Global map from process ID to `ProcessEntry`, populated during each snapshot. |

## Structs

| Name | Description |
|------|-------------|
| [ProcessSnapshot](ProcessSnapshot.md) | RAII wrapper that captures and owns a process/thread snapshot. Clears all data on drop. |
| [ProcessEntry](ProcessEntry.md) | Represents a single process with its `SYSTEM_PROCESS_INFORMATION` and lazily-parsed thread map. |

## See Also

| Link | Description |
|------|-------------|
| [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) | Consumes thread cycle data from `ProcessEntry` to drive prime-thread scheduling decisions. |
| [ProcessConfig](../config.rs/ProcessConfig.md) | Per-process configuration (priority, affinity, CPU set, prime threads) applied to entries from the snapshot. |
| [apply_prime_threads](../apply.rs/apply_prime_threads.md) | Reads thread data from `ProcessEntry` and feeds it into the scheduler for promotion/demotion. |
| [prefetch_all_thread_cycles](../apply.rs/prefetch_all_thread_cycles.md) | Iterates `ProcessEntry` threads to query and cache cycle times before scheduling. |