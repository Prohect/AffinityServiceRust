# MemoryPriorityInformation struct (priority.rs)

A `#[repr(C)]` newtype wrapper around a `u32` value representing the memory priority level for a process. This struct is used as the data payload when calling `GetProcessInformation` and `SetProcessInformation` with the `ProcessMemoryPriority` information class.

## Syntax

```rust
#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct MemoryPriorityInformation(pub u32);
```

## Members

`0: u32`

The raw memory priority value. This corresponds to one of the `MEMORY_PRIORITY_*` constants defined in the Windows API:

| Value | Constant | [MemoryPriority](MemoryPriority.md) variant |
| --- | --- | --- |
| 0 | `MEMORY_PRIORITY_VERY_LOW` | `VeryLow` |
| 1 | `MEMORY_PRIORITY_LOW` | `Low` |
| 2 | `MEMORY_PRIORITY_MEDIUM` | `Medium` |
| 3 | `MEMORY_PRIORITY_BELOW_NORMAL` | `BelowNormal` |
| 5 | `MEMORY_PRIORITY_NORMAL` | `Normal` |

## Remarks

This struct exists because the Windows `SetProcessInformation` / `GetProcessInformation` APIs for the `ProcessMemoryPriority` information class expect a pointer to a C-compatible structure containing a single `u32` field. The `#[repr(C)]` attribute ensures the Rust struct has the same layout as the C `MEMORY_PRIORITY_INFORMATION` structure expected by the kernel.

The struct is used in [`apply_memory_priority`](../apply.rs/apply_memory_priority.md) as follows:

1. **Read:** A `MemoryPriorityInformation` is passed by pointer to `GetProcessInformation` to query the current memory priority of the target process.
2. **Compare:** The returned value is compared against the configured [MemoryPriority](MemoryPriority.md) enum's `as_win_const()` result to determine whether a change is needed.
3. **Write:** If the priorities differ, a new `MemoryPriorityInformation` is constructed with the target value and passed to `SetProcessInformation`.

The struct derives `PartialEq` and `Eq` to support direct comparison of current vs. desired priority, and `Clone` / `Copy` for value semantics consistent with its use as a small FFI type.

### Relationship to MemoryPriority enum

[MemoryPriority](MemoryPriority.md) is the high-level enum used in configuration parsing and display. `MemoryPriorityInformation` is the low-level FFI wrapper used at the Windows API boundary. Conversion between them goes through `MemoryPriority::as_win_const()` which returns `Option<MEMORY_PRIORITY>`, and the inner `.0` field of `MEMORY_PRIORITY` provides the raw `u32` for constructing a `MemoryPriorityInformation`.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | `src/priority.rs` |
| **Source line** | L109 |
| **Used by** | [`apply_memory_priority`](../apply.rs/apply_memory_priority.md) in `src/apply.rs` |
| **Windows API** | [SetProcessInformation](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessinformation), [GetProcessInformation](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessinformation) |

## See also

- [MemoryPriority enum](MemoryPriority.md)
- [apply_memory_priority](../apply.rs/apply_memory_priority.md)
- [priority.rs module overview](README.md)