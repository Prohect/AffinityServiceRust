# process module (AffinityServiceRust)

The `process` module provides a high-performance process snapshot mechanism built on top of `NtQuerySystemInformation(SystemProcessInformation)`. It captures a point-in-time view of all running processes and their threads into reusable buffers, minimizing allocation overhead across polling iterations. The module exposes an RAII-based [ProcessSnapshot](ProcessSnapshot.md) type that automatically clears shared state on drop, and a [ProcessEntry](ProcessEntry.md) struct that wraps per-process data including thread arrays parsed from raw kernel structures.

## Statics

| Name | Type | Description |
|------|------|-------------|
| `SNAPSHOT_BUFFER` | `Lazy<Mutex<Vec<u8>>>` | Shared byte buffer used by [ProcessSnapshot::take](ProcessSnapshot.md) to receive raw `SYSTEM_PROCESS_INFORMATION` data from the kernel. Reused across iterations to avoid repeated allocation; dynamically grown when the kernel reports `STATUS_INFO_LENGTH_MISMATCH`. **Do not access directly** — use [ProcessSnapshot](ProcessSnapshot.md). |
| `PID_TO_PROCESS_MAP` | `Lazy<Mutex<HashMap<u32, ProcessEntry>>>` | Shared map of PID → [ProcessEntry](ProcessEntry.md) populated during each snapshot. Cleared on [ProcessSnapshot](ProcessSnapshot.md) drop. **Do not access directly** — use [ProcessSnapshot](ProcessSnapshot.md). |

## Structs

| Name | Description |
|------|-------------|
| [ProcessSnapshot](ProcessSnapshot.md) | RAII guard that captures a system-wide process snapshot into the shared buffer and process map. Clears both on drop. |
| [ProcessEntry](ProcessEntry.md) | Per-process data wrapper containing the `SYSTEM_PROCESS_INFORMATION` record, a pointer to the raw thread array, and the lowercased process name. |

## See Also

| Topic | Link |
|-------|------|
| Snapshot consumer — main loop | [main.rs](../main.rs/README.md) |
| Thread scheduling engine | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| Process handle management | [ProcessHandle](../winapi.rs/ProcessHandle.md) |
| Configuration application | [apply.rs](../apply.rs/README.md) |
| Windows API wrappers | [winapi.rs](../winapi.rs/README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*