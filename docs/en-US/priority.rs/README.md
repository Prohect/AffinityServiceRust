# priority.rs Module (priority.rs)

The `priority` module defines enumeration types and helper structures for Windows process, thread, I/O, and memory priority levels. Each type provides bidirectional conversion between human-readable string names and their corresponding Windows API constants.

## Overview

This module is a pure data-mapping layer with no side effects. It defines the vocabulary types that the rest of the application uses to express priority configuration, and translates between three representations:

1. **Enum variant** — used in Rust code for type-safe matching.
2. **String name** — used in configuration files and log output (e.g. `"above normal"`).
3. **Windows constant** — used when calling Win32 or NT APIs (e.g. `ABOVE_NORMAL_PRIORITY_CLASS`).

Every enum follows the same internal pattern: a static lookup `TABLE` of `(Variant, &str, Option<WinConst>)` tuples, with `as_str`, `as_win_const`, `from_str`, and `from_win_const` methods that perform linear scans over the table.

All enums include a `None` variant whose Windows constant is `None`, meaning "do not change this setting."

## Items

### Enums

| Name | Description |
| --- | --- |
| [ProcessPriority](ProcessPriority.md) | Windows process priority class (`Idle` through `Realtime`). |
| [IOPriority](IOPriority.md) | I/O priority level (`VeryLow` through `High`). |
| [MemoryPriority](MemoryPriority.md) | Memory page priority level (`VeryLow` through `Normal`). |
| [ThreadPriority](ThreadPriority.md) | Thread scheduling priority (`Idle` through `TimeCritical`), plus special modes. |

### Structs

| Name | Description |
| --- | --- |
| [MemoryPriorityInformation](MemoryPriorityInformation.md) | `#[repr(C)]` wrapper for the `MEMORY_PRIORITY_INFORMATION` structure used with `GetProcessInformation` / `SetProcessInformation`. |

## Design Pattern

Each priority enum implements:

| Method | Direction | Description |
| --- | --- | --- |
| `as_str(&self)` | Variant → String | Returns the human-readable name of the variant. |
| `as_win_const(&self)` | Variant → Win32 | Returns the Windows API constant, or `None` for the `None` variant. |
| `from_str(s)` | String → Variant | Parses a case-insensitive string into a variant; unrecognized strings map to `None`. |
| `from_win_const(val)` | Win32 → Variant/String | Resolves a raw Windows constant back to a variant or name. |

[ThreadPriority](ThreadPriority.md) additionally provides [`boost_one`](ThreadPriority.md#remarks) and [`to_thread_priority_struct`](ThreadPriority.md#remarks) for prime-thread priority boosting.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | `src/priority.rs` |
| **Source lines** | L1–L248 |
| **Used by** | [`ProcessConfig`](../config.rs/ProcessConfig.md) fields, [`apply_priority`](../apply.rs/apply_priority.md), [`apply_io_priority`](../apply.rs/apply_io_priority.md), [`apply_memory_priority`](../apply.rs/apply_memory_priority.md), [`PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md) |
| **Dependencies** | `windows::Win32::System::Threading` (priority class and memory priority constants) |

## See also

- [apply_priority](../apply.rs/apply_priority.md)
- [apply_io_priority](../apply.rs/apply_io_priority.md)
- [apply_memory_priority](../apply.rs/apply_memory_priority.md)
- [ProcessConfig](../config.rs/ProcessConfig.md)