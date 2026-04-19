# priority module (AffinityServiceRust)

The `priority` module provides type-safe Rust enums for Windows priority levels with bidirectional conversion between human-readable string names and Win32 numeric constants. Each enum follows an identical pattern: a `None` variant representing "no configured value," a `TABLE` constant for DRY lookup-based conversions, and four standard methods (`as_str`, `as_win_const`, `from_str`, `from_win_const`). These enums are used throughout the configuration parser and apply engine to represent priority settings without exposing raw Win32 numeric values.

## Enums

| Name | Description |
|------|-------------|
| [ProcessPriority](ProcessPriority.md) | Maps process priority class names (`"idle"`, `"normal"`, `"high"`, etc.) to `PROCESS_CREATION_FLAGS` constants. |
| [IOPriority](IOPriority.md) | Maps IO priority level names (`"very low"`, `"low"`, `"normal"`, `"high"`) to NT IO priority `u32` values (0–3). |
| [MemoryPriority](MemoryPriority.md) | Maps memory priority level names to `MEMORY_PRIORITY` constants used with `SetProcessInformation`. |
| [ThreadPriority](ThreadPriority.md) | Maps thread priority level names to `i32` Win32 thread priority values. Includes additional methods for priority boosting and FFI conversion. |

## Structs

| Name | Description |
|------|-------------|
| [MemoryPriorityInformation](MemoryPriorityInformation.md) | `#[repr(C)]` wrapper around a `u32` for Win32 `MEMORY_PRIORITY_INFORMATION` interop via `NtSetInformationProcess`. |

## See Also

| Topic | Link |
|-------|------|
| Apply engine (consumer of these enums) | [apply.rs](../apply.rs/README.md) |
| Configuration parser (creates these enums from strings) | [config.rs](../config.rs/README.md) |
| Process priority application | [apply_priority](../apply.rs/apply_priority.md) |
| IO priority application | [apply_io_priority](../apply.rs/apply_io_priority.md) |
| Memory priority application | [apply_memory_priority](../apply.rs/apply_memory_priority.md) |
| Thread-level scheduling | [apply_prime_threads](../apply.rs/apply_prime_threads.md) |
| Source file | [`src/priority.rs`](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/priority.rs) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*