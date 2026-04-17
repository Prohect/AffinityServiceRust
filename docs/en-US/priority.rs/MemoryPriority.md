# MemoryPriority enum (priority.rs)

Represents the Windows memory priority levels that can be assigned to a process via `NtSetInformationProcess` with the `ProcessMemoryPriority` information class. Each variant maps to a `MEMORY_PRIORITY` constant from the Windows SDK. The `None` sentinel variant indicates that no memory priority change should be applied.

## Syntax

```AffinityServiceRust/src/priority.rs#L108-L116
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryPriority {
    None,
    VeryLow,
    Low,
    Medium,
    BelowNormal,
    Normal,
}
```

## Members

| Variant | Win32 Constant | Numeric Value | Description |
|---------|---------------|---------------|-------------|
| `None` | *(none)* | N/A | Sentinel value indicating that no memory priority change is requested. `as_win_const()` returns `None`. |
| `VeryLow` | `MEMORY_PRIORITY_VERY_LOW` | 1 | Lowest memory priority. Pages from this process are trimmed first under memory pressure. Suitable for background or maintenance processes. |
| `Low` | `MEMORY_PRIORITY_LOW` | 2 | Low memory priority. Pages are trimmed before normal-priority processes. |
| `Medium` | `MEMORY_PRIORITY_MEDIUM` | 3 | Medium memory priority. A middle tier between low and below-normal. |
| `BelowNormal` | `MEMORY_PRIORITY_BELOW_NORMAL` | 4 | Below-normal memory priority. Pages are slightly more likely to be trimmed than those of normal-priority processes. |
| `Normal` | `MEMORY_PRIORITY_NORMAL` | 5 | Default memory priority. The standard level assigned to processes unless explicitly changed. |

## Methods

### `as_str`

```AffinityServiceRust/src/priority.rs#L127-L132
pub fn as_str(&self) -> &'static str
```

Returns the human-readable string name of the variant (e.g. `"very low"`, `"normal"`). Returns `"unknown"` if the variant is not found in the internal lookup table (which cannot happen for well-formed values).

### `as_win_const`

```AffinityServiceRust/src/priority.rs#L134-L136
pub fn as_win_const(&self) -> Option<MEMORY_PRIORITY>
```

Returns the corresponding `MEMORY_PRIORITY` Windows SDK constant wrapped in `Some`, or `None` for the `MemoryPriority::None` sentinel variant. The returned value is suitable for passing to `NtSetInformationProcess` via the [`MemoryPriorityInformation`](MemoryPriorityInformation.md) wrapper struct.

### `from_str`

```AffinityServiceRust/src/priority.rs#L138-L144
pub fn from_str(s: &str) -> Self
```

Parses a case-insensitive string into the corresponding `MemoryPriority` variant. The input is lowercased before comparison against the lookup table. Returns `MemoryPriority::None` if the string does not match any known variant name.

**Recognized strings:** `"none"`, `"very low"`, `"low"`, `"medium"`, `"below normal"`, `"normal"`

### `from_win_const`

```AffinityServiceRust/src/priority.rs#L146-L152
pub fn from_win_const(val: u32) -> &'static str
```

Looks up the human-readable string name for a raw `u32` memory priority value as returned by the Windows API. Returns `"unknown"` if the value does not match any known constant. Note that this method returns a `&'static str` rather than a `MemoryPriority` variant.

## Remarks

- The internal lookup table (`TABLE`) is defined as a `&'static` array of `(Self, &'static str, Option<MEMORY_PRIORITY>)` tuples, providing a single source of truth for the mapping between enum variants, string names, and Win32 constants.
- Memory priority affects the page-trimming order of the Windows memory manager. Processes with lower memory priority have their pages reclaimed first when the system is under memory pressure. This does not affect the speed of memory allocation itself, only the likelihood of pages being paged out.
- The `MEMORY_PRIORITY` type is imported from `windows::Win32::System::Threading` and wraps a `u32` value internally.
- The `from_str` method does **not** implement the standard `std::str::FromStr` trait. It is a standalone associated function that returns the `None` variant instead of an error on unrecognized input.
- Unlike [`ProcessPriority`](ProcessPriority.md) and [`IOPriority`](IOPriority.md), memory priority is set through `NtSetInformationProcess` using the [`MemoryPriorityInformation`](MemoryPriorityInformation.md) `#[repr(C)]` wrapper rather than a dedicated Win32 function.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `priority.rs` |
| Callers | `config` module (deserialization), `apply` module (`apply_memory_priority`) |
| Dependencies | `windows::Win32::System::Threading::MEMORY_PRIORITY`, `MEMORY_PRIORITY_VERY_LOW`, `MEMORY_PRIORITY_LOW`, `MEMORY_PRIORITY_MEDIUM`, `MEMORY_PRIORITY_BELOW_NORMAL`, `MEMORY_PRIORITY_NORMAL` |
| Platform | Windows |

## See Also

| Reference | Link |
|-----------|------|
| MemoryPriorityInformation | [MemoryPriorityInformation](MemoryPriorityInformation.md) |
| ProcessPriority | [ProcessPriority](ProcessPriority.md) |
| IOPriority | [IOPriority](IOPriority.md) |
| ThreadPriority | [ThreadPriority](ThreadPriority.md) |
| priority module overview | [README](README.md) |
| apply_process_level | [apply_process_level](../main.rs/apply_process_level.md) |

---
*Commit: [37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
