# MemoryPriority enum (priority.rs)

Type-safe representation of Windows memory priority levels. Maps between human-readable string names and Win32 `MEMORY_PRIORITY` constants, enabling bidirectional conversion for configuration parsing and status display.

## Syntax

```rust
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

| Variant | String name | Win32 constant | Description |
|---------|-------------|----------------|-------------|
| `None` | `"none"` | *(none)* | No memory priority configured. Sentinel value — `as_win_const()` returns `None`. |
| `VeryLow` | `"very low"` | `MEMORY_PRIORITY_VERY_LOW` | Lowest memory priority. Pages are the first candidates for trimming. |
| `Low` | `"low"` | `MEMORY_PRIORITY_LOW` | Low memory priority. |
| `Medium` | `"medium"` | `MEMORY_PRIORITY_MEDIUM` | Medium memory priority. |
| `BelowNormal` | `"below normal"` | `MEMORY_PRIORITY_BELOW_NORMAL` | Below-normal memory priority. |
| `Normal` | `"normal"` | `MEMORY_PRIORITY_NORMAL` | Default memory priority. Pages have standard lifetime in the working set. |

## Constants

### TABLE

```rust
const TABLE: &'static [(Self, &'static str, Option<MEMORY_PRIORITY>)]
```

Private lookup table containing all `(variant, string_name, win32_constant)` tuples. All conversion methods iterate this table, ensuring a single source of truth for the mapping. The `MEMORY_PRIORITY` values are imported from `windows::Win32::System::Threading`.

## Methods

### as_str

```rust
pub fn as_str(&self) -> &'static str
```

Returns the human-readable string representation of this memory priority level (e.g., `"very low"`, `"normal"`). Returns `"unknown"` if the variant is not found in `TABLE` (should not occur for well-constructed values).

**Return value:** A `&'static str` suitable for configuration display and log output.

### as_win_const

```rust
pub fn as_win_const(&self) -> Option<MEMORY_PRIORITY>
```

Returns the corresponding Win32 `MEMORY_PRIORITY` constant, or `None` for the `None` variant. The returned value is used with [`SetProcessInformation`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessinformation) via the [`MemoryPriorityInformation`](MemoryPriorityInformation.md) wrapper struct.

**Return value:** `Some(MEMORY_PRIORITY)` for configured levels, `None` for `MemoryPriority::None`.

### from_str

```rust
pub fn from_str(s: &str) -> Self
```

Parses a string into a `MemoryPriority` variant. The comparison is case-insensitive (the input is lowercased before matching against `TABLE`). Unrecognized strings map to `MemoryPriority::None`.

**Parameters**

| Parameter | Type | Description |
|-----------|------|-------------|
| `s` | `&str` | The string to parse (e.g., `"Very Low"`, `"below normal"`, `"Normal"`). |

**Return value:** The matching `MemoryPriority` variant, or `None` if unrecognized.

### from_win_const

```rust
pub fn from_win_const(val: u32) -> &'static str
```

Converts a raw `u32` memory priority value (as returned by [`GetProcessInformation`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessinformation)) into a human-readable string. Matches against the `.0` field of `MEMORY_PRIORITY` constants in `TABLE`.

**Parameters**

| Parameter | Type | Description |
|-----------|------|-------------|
| `val` | `u32` | The raw memory priority value read from the process. |

**Return value:** A `&'static str` such as `"very low"` or `"normal"`, or `"unknown"` if the value does not match any known constant.

## Remarks

- The memory priority level controls how aggressively the Windows Memory Manager trims pages from a process's working set. Lower priorities cause pages to be trimmed sooner under memory pressure.
- Unlike [`ProcessPriority`](ProcessPriority.md) which maps to `PROCESS_CREATION_FLAGS`, memory priority is set via the `ProcessMemoryPriority` information class through `SetProcessInformation` / `NtSetInformationProcess`. The [`MemoryPriorityInformation`](MemoryPriorityInformation.md) `repr(C)` struct provides the FFI layout required for this call.
- The `from_str` method does **not** implement the standard `std::str::FromStr` trait. It is a standalone associated function that returns a default (`None`) rather than an error on parse failure.
- All conversion methods perform a linear scan of `TABLE`. With only 6 entries, this is negligible in cost.

## Requirements

| | |
|---|---|
| **Module** | [`src/priority.rs`](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/priority.rs) |
| **Callers** | [`apply_memory_priority`](../apply.rs/apply_memory_priority.md), configuration parsing in [`config.rs`](../config.rs/README.md) |
| **Dependencies** | `windows::Win32::System::Threading::{MEMORY_PRIORITY, MEMORY_PRIORITY_VERY_LOW, MEMORY_PRIORITY_LOW, MEMORY_PRIORITY_MEDIUM, MEMORY_PRIORITY_BELOW_NORMAL, MEMORY_PRIORITY_NORMAL}` |
| **Win32 API** | [`SetProcessInformation`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessinformation), [`GetProcessInformation`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessinformation) |
| **Privileges** | None for setting memory priority on processes the caller has `PROCESS_SET_INFORMATION` access to |

## See Also

| Topic | Link |
|-------|------|
| FFI wrapper struct | [`MemoryPriorityInformation`](MemoryPriorityInformation.md) |
| Memory priority apply function | [`apply_memory_priority`](../apply.rs/apply_memory_priority.md) |
| Process priority enum | [`ProcessPriority`](ProcessPriority.md) |
| IO priority enum | [`IOPriority`](IOPriority.md) |
| Thread priority enum | [`ThreadPriority`](ThreadPriority.md) |
| Module overview | [priority.rs](README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*