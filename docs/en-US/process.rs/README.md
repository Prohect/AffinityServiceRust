# process module (AffinityServiceRust)

The `process` module provides efficient process and thread enumeration by wrapping the native `NtQuerySystemInformation` API. It captures point-in-time snapshots of every running process and its threads on the system, storing them in a reusable buffer and a PID-keyed lookup map. The snapshot model uses RAII semantics — the buffer and map are automatically cleared when the `ProcessSnapshot` is dropped, preventing stale data from persisting between scheduling cycles.

## Functions

This module does not export standalone functions. All functionality is exposed through `ProcessSnapshot` and `ProcessEntry` methods.

## Structs

| Struct | Description |
|--------|-------------|
| [ProcessSnapshot](ProcessSnapshot.md) | RAII wrapper that captures a point-in-time snapshot of all processes and threads via `NtQuerySystemInformation`, with automatic cleanup on drop. |
| [ProcessEntry](ProcessEntry.md) | Represents a single process from the snapshot, wrapping `SYSTEM_PROCESS_INFORMATION` with cached process name and on-demand thread enumeration. |

## Statics

| Static | Description |
|--------|-------------|
| [SNAPSHOT_BUFFER](SNAPSHOT_BUFFER.md) | `Lazy<Mutex<Vec<u8>>>` — Shared backing buffer for `NtQuerySystemInformation` results. Must not be accessed directly; use `ProcessSnapshot` instead. |
| [PID_TO_PROCESS_MAP](PID_TO_PROCESS_MAP.md) | `Lazy<Mutex<HashMap<u32, ProcessEntry>>>` — Shared PID-to-process lookup map populated by `ProcessSnapshot::take`. Must not be accessed directly; use `ProcessSnapshot` instead. |

## See Also

| Link | Description |
|------|-------------|
| [winapi.rs](../winapi.rs/README.md) | Low-level Windows API wrappers including handle management and CPU set operations. |
| [collections.rs](../collections.rs/README.md) | Type aliases (`HashMap`, `HashSet`, `List`) and capacity constants used throughout the project. |
| [event_trace.rs](../event_trace.rs/README.md) | ETW-based real-time process monitoring that complements polling-based snapshots. |

---
*Commit: [37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
