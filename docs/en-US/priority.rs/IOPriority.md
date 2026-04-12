# IOPriority enum (priority.rs)

Represents the I/O priority level for a process, controlling how the Windows I/O scheduler prioritizes disk and device requests from the process relative to others.

## Syntax

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IOPriority {
    None,
    VeryLow,
    Low,
    Normal,
    High,
}
```

## Members

`None`

No I/O priority change requested. When this variant is active, the apply logic skips I/O priority modification entirely. This is the default when no `io=` directive is present in the configuration.

`VeryLow`

Maps to Windows I/O priority constant `0`. Suitable for background tasks such as indexing services, search crawlers, or defragmentation utilities where I/O latency is unimportant.

`Low`

Maps to Windows I/O priority constant `1`. A step above `VeryLow`, appropriate for low-importance background processes that should still make forward progress under moderate system load.

`Normal`

Maps to Windows I/O priority constant `2`. The default I/O priority for most processes. Provides fair scheduling without special treatment.

`High`

Maps to Windows I/O priority constant `3`. Elevates the process's I/O requests above all other priority levels. **Requires administrator privileges and the `SeIncreaseBasePriorityPrivilege` privilege** to set successfully. Without both, the `NtSetInformationProcess` call will fail with `STATUS_PRIVILEGE_NOT_HELD`.

## Methods

| Method | Signature | Description |
| --- | --- | --- |
| **as_str** | `pub fn as_str(&self) -> &'static str` | Returns the human-readable name (`"none"`, `"very low"`, `"low"`, `"normal"`, `"high"`). Returns `"unknown"` if the variant is somehow unmatched. |
| **as_win_const** | `pub fn as_win_const(&self) -> Option<u32>` | Returns the Windows I/O priority constant (`0`–`3`), or `None` for the `None` variant. Used by [`apply_io_priority`](../apply.rs/apply_io_priority.md) when calling `NtSetInformationProcess`. |
| **from_str** | `pub fn from_str(s: &str) -> Self` | Parses a case-insensitive string to an `IOPriority` variant. Unrecognized strings return `IOPriority::None`. Used by the config parser in [`config.rs`](../config.rs/ProcessConfig.md). |
| **from_win_const** | `pub fn from_win_const(val: u32) -> &'static str` | Converts a raw Windows I/O priority integer back to a human-readable name string. Used for display and logging when querying current I/O priority. |

## Remarks

`IOPriority` follows the same static lookup table pattern as all other priority enums in this module ([ProcessPriority](ProcessPriority.md), [MemoryPriority](MemoryPriority.md), [ThreadPriority](ThreadPriority.md)). A `const TABLE` array stores `(Self, &str, Option<u32>)` tuples that power all four conversion methods via linear search.

The `None` variant acts as a sentinel meaning "do not change the I/O priority." When `as_win_const()` returns `None`, the apply function in [`apply_io_priority`](../apply.rs/apply_io_priority.md) short-circuits and takes no action.

### Windows API mapping

I/O priority is applied through `NtSetInformationProcess` with the `ProcessInformationIoPriority` information class, and queried through `NtQueryInformationProcess` with the same class. These are undocumented NT API calls accessed via the `ntapi` crate.

### Security requirements for High

Setting `IOPriority::High` requires two conditions:

1. The process must be running with **administrator** elevation (UAC).
2. The **SeIncreaseBasePriorityPrivilege** must be enabled in the process token.

The service enables this privilege at startup via [`enable_inc_base_priority_privilege`](../winapi.rs/enable_inc_base_priority_privilege.md) unless `--no-inc-base-priority` is passed on the command line.

### Configuration syntax

In the configuration file, I/O priority is specified with the `io=` directive:

```
process.exe  io=normal
background_svc.exe  io=very low
critical_app.exe  io=high
```

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | `src/priority.rs` |
| **Source lines** | L60–L107 |
| **Used by** | [`ProcessConfig`](../config.rs/ProcessConfig.md), [`apply_io_priority`](../apply.rs/apply_io_priority.md) |
| **Windows API** | `NtSetInformationProcess` / `NtQueryInformationProcess` with `ProcessInformationIoPriority` |

## See also

- [priority.rs module overview](README.md)
- [ProcessPriority](ProcessPriority.md)
- [MemoryPriority](MemoryPriority.md)
- [ThreadPriority](ThreadPriority.md)
- [apply_io_priority](../apply.rs/apply_io_priority.md)