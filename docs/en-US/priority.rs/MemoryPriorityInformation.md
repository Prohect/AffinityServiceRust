# MemoryPriorityInformation type (priority.rs)

A `#[repr(C)]` newtype wrapper around a `u32` value that matches the binary layout of the `MEMORY_PRIORITY_INFORMATION` structure expected by the Windows `NtSetInformationProcess` API. This struct is used when setting the memory priority of a process through the undocumented `ProcessMemoryPriority` information class, ensuring that the Rust-side data is layout-compatible with the C structure the kernel expects.

## Syntax

```rust
#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct MemoryPriorityInformation(pub u32);
```

## Members

| Member | Type | Description |
|--------|------|-------------|
| `0` (tuple field) | `u32` | The raw memory priority value. Corresponds to one of the `MEMORY_PRIORITY_*` constants defined by the Windows SDK: `MEMORY_PRIORITY_VERY_LOW` (1), `MEMORY_PRIORITY_LOW` (2), `MEMORY_PRIORITY_MEDIUM` (3), `MEMORY_PRIORITY_BELOW_NORMAL` (4), or `MEMORY_PRIORITY_NORMAL` (5). The value is typically obtained by calling [`MemoryPriority::as_win_const()`](MemoryPriority.md) and extracting the inner `.0` field from the resulting `MEMORY_PRIORITY` wrapper. |

## Remarks

- The `#[repr(C)]` attribute guarantees that the struct's memory layout matches a C `struct` containing a single `ULONG` field, which is the layout Windows expects when calling `NtSetInformationProcess` with the `ProcessMemoryPriority` information class.
- The struct derives `PartialEq`, `Eq`, `Clone`, and `Copy`, making it suitable for comparison and value-type semantics.
- Unlike the other priority types in this module, `MemoryPriorityInformation` is a struct rather than an enum because it serves as a direct FFI-boundary type. The corresponding enum for user-facing logic is [`MemoryPriority`](MemoryPriority.md), which provides string conversion and lookup table functionality.
- This type does **not** derive `Debug`. If debug output is needed, the inner `u32` value can be accessed directly via `.0`.
- The struct is passed by pointer to `NtSetInformationProcess` in the `apply` module. The size of the struct (`std::mem::size_of::<MemoryPriorityInformation>()`) is passed as the information length parameter to the NT API call.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `priority.rs` |
| Callers | `apply` module (used as the data buffer for `NtSetInformationProcess` calls when setting process memory priority) |
| Win32 API | Corresponds to `MEMORY_PRIORITY_INFORMATION` passed to `NtSetInformationProcess` with information class `ProcessMemoryPriority` |
| Privileges | `SeDebugPrivilege` may be required to set memory priority on processes owned by other users |

## See Also

| Reference | Link |
|-----------|------|
| MemoryPriority enum | [MemoryPriority](MemoryPriority.md) |
| ProcessPriority | [ProcessPriority](ProcessPriority.md) |
| IOPriority | [IOPriority](IOPriority.md) |
| ThreadPriority | [ThreadPriority](ThreadPriority.md) |
| priority module overview | [README](README.md) |
| apply_process_level | [apply_process_level](../main.rs/apply_process_level.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
