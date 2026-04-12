# MemoryPriority enum (priority.rs)

Represents the memory page priority level for a process. Used with `SetProcessInformation` / `GetProcessInformation` to control how aggressively the memory manager trims a process's working set pages.

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

`None`

Sentinel value indicating "don't change". When the configured memory priority is `None`, no `SetProcessInformation` call is made. This is the default when no `memory_priority` directive is specified in the configuration.

`VeryLow`

Maps to `MEMORY_PRIORITY_VERY_LOW` (value `1`). Pages are the first candidates for trimming and repurposing. Suitable for background or idle processes whose memory residency is not important.

`Low`

Maps to `MEMORY_PRIORITY_LOW` (value `2`). Pages are trimmed before medium-priority pages but after very-low ones.

`Medium`

Maps to `MEMORY_PRIORITY_MEDIUM` (value `3`). A balanced middle tier. Pages are trimmed before below-normal and normal pages.

`BelowNormal`

Maps to `MEMORY_PRIORITY_BELOW_NORMAL` (value `4`). Pages have moderate protection from trimming. Suitable for processes that benefit from cached pages but are not performance-critical.

`Normal`

Maps to `MEMORY_PRIORITY_NORMAL` (value `5`). The default memory priority for all processes. Pages receive the highest retention priority and are trimmed last.

## Methods

| Method | Signature | Description |
| --- | --- | --- |
| **as_str** | `pub fn as_str(&self) -> &'static str` | Returns the human-readable name (e.g. `"very low"`, `"below normal"`). Returns `"unknown"` for unrecognised variants. |
| **as_win_const** | `pub fn as_win_const(&self) -> Option<MEMORY_PRIORITY>` | Returns the corresponding `MEMORY_PRIORITY` Windows constant, or `None` for the `None` variant. |
| **from_str** | `pub fn from_str(s: &str) -> Self` | Parses a case-insensitive string into a variant. Unrecognised strings map to `None`. |
| **from_win_const** | `pub fn from_win_const(val: u32) -> &'static str` | Converts a raw `u32` Windows constant back to its human-readable name string. Returns `"unknown"` if the value does not match any known constant. |

## Remarks

`MemoryPriority` follows the same bidirectional conversion pattern as all other enums in this module. A static lookup table (`TABLE`) maps each variant to its string representation and optional Windows constant, and all conversion methods scan this table.

The Windows memory priority system has five levels (1–5). Lower-priority pages are reclaimed first when the system is under memory pressure. This enum is used by [`apply_memory_priority`](../apply.rs/apply_memory_priority.md) to set the page priority for a target process via `SetProcessInformation` with `ProcessMemoryPriority` information class.

The actual API call requires a [`MemoryPriorityInformation`](MemoryPriorityInformation.md) struct wrapping the raw `u32` value, which is obtained from `as_win_const()` and then wrapped: `MemoryPriorityInformation(val.0)`.

### Variant-to-constant mapping

| Variant | String | Windows Constant | Raw Value |
| --- | --- | --- | --- |
| `None` | `"none"` | *(none)* | — |
| `VeryLow` | `"very low"` | `MEMORY_PRIORITY_VERY_LOW` | `1` |
| `Low` | `"low"` | `MEMORY_PRIORITY_LOW` | `2` |
| `Medium` | `"medium"` | `MEMORY_PRIORITY_MEDIUM` | `3` |
| `BelowNormal` | `"below normal"` | `MEMORY_PRIORITY_BELOW_NORMAL` | `4` |
| `Normal` | `"normal"` | `MEMORY_PRIORITY_NORMAL` | `5` |

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | `src/priority.rs` |
| **Source lines** | L112–L160 |
| **Used by** | [`ProcessConfig`](../config.rs/ProcessConfig.md), [`apply_memory_priority`](../apply.rs/apply_memory_priority.md) |
| **Windows API** | [SetProcessInformation](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessinformation), [GetProcessInformation](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessinformation) |

## See also

- [priority.rs module overview](README.md)
- [MemoryPriorityInformation](MemoryPriorityInformation.md)
- [ProcessPriority](ProcessPriority.md)
- [IOPriority](IOPriority.md)
- [apply_memory_priority](../apply.rs/apply_memory_priority.md)