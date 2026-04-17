# priority module (AffinityServiceRust)

The `priority` module defines strongly-typed Rust enumerations and helper types that map Windows process, thread, I/O, and memory priority levels to their corresponding Win32 API constants. Each type provides bidirectional conversion between human-readable string names, Rust enum variants, and the raw numeric values expected by the Windows API (`PROCESS_CREATION_FLAGS`, `THREAD_PRIORITY`, `MEMORY_PRIORITY`, and undocumented I/O priority integers). The module is consumed by the configuration parser to deserialize user-supplied priority strings and by the apply engine to pass correct constants to `SetPriorityClass`, `SetThreadPriority`, `NtSetInformationProcess`, and related Win32 calls.

## Enums

| Enum | Description |
|------|-------------|
| [ProcessPriority](ProcessPriority.md) | Maps the six Windows process priority classes (`Idle` through `Realtime`) plus a `None` sentinel to `PROCESS_CREATION_FLAGS` values. |
| [IOPriority](IOPriority.md) | Maps Windows I/O priority hints (`VeryLow`, `Low`, `Normal`, `High`) plus a `None` sentinel to the raw `u32` values used by `NtSetInformationProcess`. |
| [MemoryPriority](MemoryPriority.md) | Maps Windows memory priority levels (`VeryLow` through `Normal`) plus a `None` sentinel to `MEMORY_PRIORITY` constants. |
| [ThreadPriority](ThreadPriority.md) | Maps the full set of Windows thread priority levels (`Idle` through `TimeCritical`, including background-mode tokens) plus a `None` sentinel to `i32` values used by `SetThreadPriority`. Also provides a `boost_one` method for single-step priority elevation. |

## Structs

| Struct | Description |
|--------|-------------|
| [MemoryPriorityInformation](MemoryPriorityInformation.md) | A `#[repr(C)]` newtype wrapper around `u32` that matches the layout of the `MEMORY_PRIORITY_INFORMATION` structure expected by `NtSetInformationProcess`. |

## See Also

| Reference | Link |
|-----------|------|
| main module | [main.rs](../main.rs/README.md) |
| scheduler module | [scheduler.rs](../scheduler.rs/README.md) |
| apply module | [apply.rs](../apply.rs/README.md) |
| config module | [config.rs](../config.rs/README.md) |

---
*Commit: [37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
