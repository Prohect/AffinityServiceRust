# priority module (AffinityServiceRust)

The `priority` module defines the strongly-typed enumerations and helper types that represent Windows process priority classes, I/O priority levels, memory priority levels, and thread priority levels. Each enum carries a lookup table that maps between Rust variants, human-readable string names, and the corresponding Win32 constant values. The module provides uniform `as_str`, `as_win_const`, `from_str`, and `from_win_const` conversion methods on every priority enum, making it straightforward to round-trip between configuration text, Rust logic, and Windows API calls. A `None` variant on each enum acts as a sentinel indicating "no change requested."

## Enums

| Enum | Description |
|------|-------------|
| [ProcessPriority](ProcessPriority.md) | Windows process priority class (`Idle` through `Realtime`), wrapping `PROCESS_CREATION_FLAGS`. |
| [IOPriority](IOPriority.md) | I/O priority level (`VeryLow` through `High`), used with `NtSetInformationProcess`. |
| [MemoryPriority](MemoryPriority.md) | Memory priority level (`VeryLow` through `Normal`), wrapping `MEMORY_PRIORITY` constants. |
| [ThreadPriority](ThreadPriority.md) | Thread priority level (`Idle` through `TimeCritical`), including background-mode sentinels and a `boost_one` helper. |

## Structs

| Struct | Description |
|--------|-------------|
| [MemoryPriorityInformation](MemoryPriorityInformation.md) | `#[repr(C)]` newtype wrapper around `u32` used as the buffer for `SetProcessInformation` / `GetProcessInformation` memory-priority calls. |

## See Also

| Topic | Link |
|-------|------|
| Configuration parsing (reads priority strings) | [config module](../config.rs/README.md) |
| Rule application (calls `as_win_const`) | [apply module](../apply.rs/README.md) |
| Error code translation | [error_codes module](../error_codes.rs/README.md) |
| Logging and failure tracking | [logging module](../logging.rs/README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd