# MemoryPriorityInformation struct (priority.rs)

A `#[repr(C)]` newtype wrapper around a `u32` value that represents the memory priority of a process. This struct is used as the in-memory buffer passed directly to the Windows `SetProcessInformation` and `GetProcessInformation` APIs when querying or setting `ProcessMemoryPriority`. The C-compatible layout guarantees that the struct's memory representation matches the `MEMORY_PRIORITY_INFORMATION` structure expected by the kernel.

## Syntax

```rust
#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct MemoryPriorityInformation(pub u32);
```

## Members

| Member | Type | Description |
|--------|------|-------------|
| `0` | `u32` | The raw memory priority value. Corresponds to the `MemoryPriority` field of the Win32 `MEMORY_PRIORITY_INFORMATION` structure. Valid values are `0` (`MEMORY_PRIORITY_VERY_LOW`) through `5` (`MEMORY_PRIORITY_NORMAL`). |

## Remarks

This is a tuple struct with a single public `u32` field, designed to be cast to and from a raw pointer when calling `SetProcessInformation` / `GetProcessInformation` with the `ProcessMemoryPriority` information class. The `#[repr(C)]` attribute ensures the struct has a predictable, C-compatible layout with no padding, making it safe for use as a typed buffer in FFI calls.

The numeric values stored in the inner `u32` correspond to the constants defined on the [MemoryPriority](MemoryPriority.md) enum via its `as_win_const` method:

| Value | Constant |
|-------|----------|
| `1` | `MEMORY_PRIORITY_VERY_LOW` |
| `2` | `MEMORY_PRIORITY_LOW` |
| `3` | `MEMORY_PRIORITY_MEDIUM` |
| `4` | `MEMORY_PRIORITY_BELOW_NORMAL` |
| `5` | `MEMORY_PRIORITY_NORMAL` |

The struct derives `PartialEq`, `Eq`, `Clone`, and `Copy` for value semantics and comparison. It does not derive `Debug`; the inner value can be inspected directly via `.0`.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `priority` |
| Callers | [apply_memory_priority](../apply.rs/apply_memory_priority.md) |
| Win32 API | `SetProcessInformation`, `GetProcessInformation` with `ProcessMemoryPriority` |
| Header equivalent | `MEMORY_PRIORITY_INFORMATION` (processthreadsapi.h) |

## See Also

| Topic | Link |
|-------|------|
| Memory priority enum with named levels | [MemoryPriority](MemoryPriority.md) |
| Process priority class enum | [ProcessPriority](ProcessPriority.md) |
| I/O priority enum | [IOPriority](IOPriority.md) |
| Memory priority application logic | [apply_memory_priority](../apply.rs/apply_memory_priority.md) |
| priority module overview | [priority module](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd