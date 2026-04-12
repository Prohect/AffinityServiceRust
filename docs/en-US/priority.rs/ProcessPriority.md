# ProcessPriority enum (priority.rs)

Represents the Windows process priority class. Used to configure the scheduling priority of an entire process via `SetPriorityClass`.

## Syntax

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessPriority {
    None,
    Idle,
    BelowNormal,
    Normal,
    AboveNormal,
    High,
    Realtime,
}
```

## Members

`None`

No priority change. When a [ProcessConfig](../config.rs/ProcessConfig.md) specifies `None`, the [apply_priority](../apply.rs/apply_priority.md) function skips the process without modifying its priority class.

`Idle`

Maps to `IDLE_PRIORITY_CLASS` (Windows constant `0x40`). Threads in this process run only when the system is idle. Suitable for background maintenance tasks.

`BelowNormal`

Maps to `BELOW_NORMAL_PRIORITY_CLASS` (Windows constant `0x4000`). Lower than `Normal` but higher than `Idle`.

`Normal`

Maps to `NORMAL_PRIORITY_CLASS` (Windows constant `0x20`). The default priority class for most processes.

`AboveNormal`

Maps to `ABOVE_NORMAL_PRIORITY_CLASS` (Windows constant `0x8000`). Higher than `Normal` but lower than `High`.

`High`

Maps to `HIGH_PRIORITY_CLASS` (Windows constant `0x80`). Should be used sparingly — processes at this level can starve lower-priority processes of CPU time.

`Realtime`

Maps to `REALTIME_PRIORITY_CLASS` (Windows constant `0x100`). The highest possible priority class. Threads in this process preempt all other threads including OS threads. Requires administrator privileges and should be used with extreme caution.

## Methods

| Method | Signature | Description |
| --- | --- | --- |
| **as_str** | `pub fn as_str(&self) -> &'static str` | Returns the human-readable name of the variant (e.g. `"above normal"`, `"real time"`). Returns `"unknown"` if the variant is not found in the lookup table. |
| **as_win_const** | `pub fn as_win_const(&self) -> Option<PROCESS_CREATION_FLAGS>` | Returns the corresponding Windows `PROCESS_CREATION_FLAGS` constant, or `None` for the `None` variant. |
| **from_str** | `pub fn from_str(s: &str) -> Self` | Parses a case-insensitive string into a `ProcessPriority`. Unrecognised strings return `None`. |
| **from_win_const** | `pub fn from_win_const(val: u32) -> &'static str` | Converts a raw Windows priority class `u32` value back to a human-readable name. Returns `"unknown"` for unrecognised values. |

## Remarks

`ProcessPriority` follows the same bidirectional conversion pattern used by all priority enums in this module ([IOPriority](IOPriority.md), [MemoryPriority](MemoryPriority.md), [ThreadPriority](ThreadPriority.md)). Internally it uses a static lookup table (`TABLE`) that maps each variant to its string representation and optional Windows constant. All conversion methods iterate this table.

The `None` variant acts as a sentinel meaning "don't change the current value." It is the default when a configuration rule does not specify a priority, and it maps to `None` in the Windows constant column so that downstream code can detect and skip it.

String representations use lowercase with spaces (e.g. `"below normal"`, `"real time"`) and are matched case-insensitively during parsing.

### Lookup table

| Variant | String | Windows Constant |
| --- | --- | --- |
| None | `"none"` | *(none)* |
| Idle | `"idle"` | `IDLE_PRIORITY_CLASS` |
| BelowNormal | `"below normal"` | `BELOW_NORMAL_PRIORITY_CLASS` |
| Normal | `"normal"` | `NORMAL_PRIORITY_CLASS` |
| AboveNormal | `"above normal"` | `ABOVE_NORMAL_PRIORITY_CLASS` |
| High | `"high"` | `HIGH_PRIORITY_CLASS` |
| Realtime | `"real time"` | `REALTIME_PRIORITY_CLASS` |

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/priority.rs |
| **Source lines** | L8–L58 |
| **Used by** | [ProcessConfig](../config.rs/ProcessConfig.md), [apply_priority](../apply.rs/apply_priority.md) |
| **Windows API** | [SetPriorityClass](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setpriorityclass), [GetPriorityClass](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getpriorityclass) |

## See also

- [priority.rs module overview](README.md)
- [IOPriority](IOPriority.md)
- [MemoryPriority](MemoryPriority.md)
- [ThreadPriority](ThreadPriority.md)
- [apply_priority](../apply.rs/apply_priority.md)