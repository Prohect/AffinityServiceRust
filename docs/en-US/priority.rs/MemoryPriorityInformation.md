# MemoryPriorityInformation struct (priority.rs)

A `#[repr(C)]` wrapper around a `u32` value for direct interop with the Win32 `MEMORY_PRIORITY_INFORMATION` structure layout. Used when calling `SetProcessInformation` / `GetProcessInformation` with the `ProcessMemoryPriority` information class, which expects a pointer to a single `ULONG` field.

## Syntax

```rust
#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct MemoryPriorityInformation(pub u32);
```

## Members

| Member | Type | Description |
|--------|------|-------------|
| `0` (tuple field) | `u32` | The raw memory priority value. Corresponds to one of the `MEMORY_PRIORITY_*` constants defined by Windows (e.g., `MEMORY_PRIORITY_VERY_LOW` through `MEMORY_PRIORITY_NORMAL`). |

## Remarks

This struct exists solely to provide a correctly-sized and correctly-aligned FFI type for the `NtSetInformationProcess` / `SetProcessInformation` calls that set memory priority. The `#[repr(C)]` attribute ensures the memory layout matches what the Win32 API expects — a single `ULONG` (4-byte, naturally aligned).

The struct derives `PartialEq` and `Eq` for comparison (e.g., checking whether the current memory priority matches the desired value before making a change), and `Clone` / `Copy` for ergonomic pass-by-value semantics.

### Relationship to MemoryPriority enum

[`MemoryPriority`](MemoryPriority.md) is the type-safe, human-readable enum used in configuration and logging. When a raw `u32` value is needed for Win32 interop, `MemoryPriority::as_win_const()` returns an `Option<MEMORY_PRIORITY>` which can be wrapped in `MemoryPriorityInformation` for the FFI call. The two types serve complementary roles:

| Type | Purpose |
|------|---------|
| [`MemoryPriority`](MemoryPriority.md) | Config parsing, string display, variant matching |
| `MemoryPriorityInformation` | FFI-safe layout for Win32 `SetProcessInformation` calls |

### Win32 correspondence

The Windows SDK defines `MEMORY_PRIORITY_INFORMATION` as:

```c
typedef struct _MEMORY_PRIORITY_INFORMATION {
    ULONG MemoryPriority;
} MEMORY_PRIORITY_INFORMATION;
```

`MemoryPriorityInformation(pub u32)` is a Rust newtype that has the identical layout, making it safe to pass as a pointer to the `lpProcessInformation` parameter of `SetProcessInformation`.

## Requirements

| | |
|---|---|
| **Module** | [`src/priority.rs`](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/priority.rs) |
| **Callers** | [`apply_memory_priority`](../apply.rs/apply_memory_priority.md) |
| **Dependencies** | None (plain newtype wrapper) |
| **Win32 API** | [`SetProcessInformation`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessinformation) with `ProcessMemoryPriority` |
| **Privileges** | `PROCESS_SET_INFORMATION` (required by caller when writing) |

## See Also

| Topic | Link |
|-------|------|
| Type-safe memory priority enum | [`MemoryPriority`](MemoryPriority.md) |
| Memory priority application | [`apply_memory_priority`](../apply.rs/apply_memory_priority.md) |
| Module overview | [priority.rs](README.md) |
| Process priority enum | [`ProcessPriority`](ProcessPriority.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*