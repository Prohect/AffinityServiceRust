# ProcessPriority enum (priority.rs)

Type-safe representation of Windows process priority classes. Provides bidirectional conversion between human-readable string names and Win32 `PROCESS_CREATION_FLAGS` constants, using a single `TABLE` constant for DRY lookup logic.

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

| Variant | String name | Win32 constant | Value |
|---------|-------------|----------------|-------|
| `None` | `"none"` | *(None)* | N/A — sentinel meaning "do not change" |
| `Idle` | `"idle"` | `IDLE_PRIORITY_CLASS` | `0x00000040` |
| `BelowNormal` | `"below normal"` | `BELOW_NORMAL_PRIORITY_CLASS` | `0x00004000` |
| `Normal` | `"normal"` | `NORMAL_PRIORITY_CLASS` | `0x00000020` |
| `AboveNormal` | `"above normal"` | `ABOVE_NORMAL_PRIORITY_CLASS` | `0x00008000` |
| `High` | `"high"` | `HIGH_PRIORITY_CLASS` | `0x00000080` |
| `Realtime` | `"real time"` | `REALTIME_PRIORITY_CLASS` | `0x00000100` |

## Methods

### as_str

```rust
pub fn as_str(&self) -> &'static str
```

Returns the human-readable string name for this variant (e.g., `"above normal"`). Returns `"unknown"` if the variant is not found in `TABLE` (should not occur for well-constructed values).

### as_win_const

```rust
pub fn as_win_const(&self) -> Option<PROCESS_CREATION_FLAGS>
```

Returns the corresponding Win32 `PROCESS_CREATION_FLAGS` constant, or `None` for the `None` variant. A return value of `None` signals to callers (such as [`apply_priority`](../apply.rs/apply_priority.md)) that no priority change should be made.

### from_str

```rust
pub fn from_str(s: &str) -> Self
```

Parses a case-insensitive string into a `ProcessPriority` variant. Unrecognized strings map to `ProcessPriority::None`. This is used during configuration file parsing.

**Parameters**

| Parameter | Type | Description |
|-----------|------|-------------|
| `s` | `&str` | The priority name to parse (e.g., `"High"`, `"below normal"`). Comparison is case-insensitive. |

### from_win_const

```rust
pub fn from_win_const(val: u32) -> &'static str
```

Converts a raw Win32 priority class value (as returned by `GetPriorityClass`) back to a human-readable string name. Returns `"unknown"` if the value does not match any known constant. This is used in change-log messages to display the "before" state.

**Parameters**

| Parameter | Type | Description |
|-----------|------|-------------|
| `val` | `u32` | The raw `PROCESS_CREATION_FLAGS` value (`.0` field). |

## Remarks

### TABLE-driven design

All four conversion methods operate against a single `const TABLE` array:

```rust
const TABLE: &'static [(Self, &'static str, Option<PROCESS_CREATION_FLAGS>)]
```

Each entry is a `(variant, string_name, optional_win32_constant)` tuple. This ensures that adding a new priority level requires only one table entry rather than updates to four separate `match` arms.

### None sentinel

The `None` variant is not a Windows priority class — it represents the absence of a configured priority. When `as_win_const()` returns `None`, apply functions skip the priority-setting step entirely. This is distinct from `Normal`, which actively sets the priority to `NORMAL_PRIORITY_CLASS`.

### Realtime priority

The `Realtime` variant maps to `REALTIME_PRIORITY_CLASS`, which requires `SeIncreaseBasePriorityPrivilege` and Administrator rights. Using this priority class inappropriately can cause system instability. The string representation uses `"real time"` (with a space) to match the Windows Task Manager display name.

## Requirements

| | |
|---|---|
| **Module** | [`src/priority.rs`](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/priority.rs) |
| **Callers** | [`apply_priority`](../apply.rs/apply_priority.md), config parsing ([`read_config`](../config.rs/read_config.md)) |
| **Dependencies** | `windows::Win32::System::Threading::PROCESS_CREATION_FLAGS` and related constants |
| **Privileges** | `SeIncreaseBasePriorityPrivilege` for `Realtime` |

## See Also

| Topic | Link |
|-------|------|
| Apply function that uses this enum | [`apply_priority`](../apply.rs/apply_priority.md) |
| IO priority enum | [`IOPriority`](IOPriority.md) |
| Memory priority enum | [`MemoryPriority`](MemoryPriority.md) |
| Thread priority enum | [`ThreadPriority`](ThreadPriority.md) |
| Module overview | [priority.rs](README.md) |
| Win32 reference | [`SetPriorityClass`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setpriorityclass) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*