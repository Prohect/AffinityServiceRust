# ProcessPriority enum (priority.rs)

Represents the Windows process priority class levels. Each variant maps to a `PROCESS_CREATION_FLAGS` constant used by the `SetPriorityClass` Win32 API. The `None` variant serves as a sentinel indicating that no priority change is requested for a given process configuration rule.

## Syntax

```priority.rs
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

| Variant | Win32 Constant | Value | Description |
|---------|---------------|-------|-------------|
| `None` | — | — | No priority change requested. Acts as a sentinel/default. |
| `Idle` | `IDLE_PRIORITY_CLASS` | `0x00000040` | Lowest priority class. Threads run only when the system is idle. |
| `BelowNormal` | `BELOW_NORMAL_PRIORITY_CLASS` | `0x00004000` | Priority above Idle but below Normal. |
| `Normal` | `NORMAL_PRIORITY_CLASS` | `0x00000020` | Default priority class for most processes. |
| `AboveNormal` | `ABOVE_NORMAL_PRIORITY_CLASS` | `0x00008000` | Priority above Normal but below High. |
| `High` | `HIGH_PRIORITY_CLASS` | `0x00000080` | High priority. Should be used sparingly as it can starve lower-priority processes. |
| `Realtime` | `REALTIME_PRIORITY_CLASS` | `0x00000100` | Highest possible priority class. Requires `SeIncreaseBasePriorityPrivilege`. Can preempt OS threads. |

## Methods

### as_str

```priority.rs
pub fn as_str(&self) -> &'static str
```

Returns the human-readable string name for this variant (e.g., `"idle"`, `"below normal"`, `"real time"`). Returns `"unknown"` if the variant is not found in the internal lookup table, which should not occur for well-formed values. The `None` variant returns `"none"`.

### as_win_const

```priority.rs
pub fn as_win_const(&self) -> Option<PROCESS_CREATION_FLAGS>
```

Returns the corresponding `PROCESS_CREATION_FLAGS` value for this variant, or `None` if the variant is `ProcessPriority::None` (no change requested).

### from_str

```priority.rs
pub fn from_str(s: &str) -> Self
```

Parses a case-insensitive string into a `ProcessPriority` variant. The input is lowercased before matching against the lookup table. Unrecognized strings return `ProcessPriority::None`.

**Parameters**

| Parameter | Type | Description |
|-----------|------|-------------|
| `s` | `&str` | The priority name to parse. Matched case-insensitively against `"idle"`, `"below normal"`, `"normal"`, `"above normal"`, `"high"`, `"real time"`. |

### from_win_const

```priority.rs
pub fn from_win_const(val: u32) -> &'static str
```

Looks up a raw `u32` value (the inner value of a `PROCESS_CREATION_FLAGS`) and returns the corresponding human-readable string name. Returns `"unknown"` if the value does not match any known priority class.

**Parameters**

| Parameter | Type | Description |
|-----------|------|-------------|
| `val` | `u32` | The raw Win32 priority class constant value to look up. |

## Remarks

- The enum uses an internal `TABLE` constant of `(Self, &str, Option<PROCESS_CREATION_FLAGS>)` tuples that drives all conversion methods. This ensures the string names, enum variants, and Win32 constants stay in sync.
- `from_str` is not the standard library `FromStr` trait — it is an inherent method that returns `ProcessPriority::None` on failure rather than an error.
- `from_win_const` returns `&'static str` rather than `Self`, differing from the other enums' `from_win_const` which also returns `&'static str`. This is used primarily for log output when reading the current priority of a running process.
- Setting `Realtime` priority requires `SeIncreaseBasePriorityPrivilege` and administrative elevation. Without it, the `SetPriorityClass` call will fail with `ERROR_PRIVILEGE_NOT_HELD` (1314).

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `priority` |
| Callers | [parse_and_insert_rules](../config.rs/parse_and_insert_rules.md), [apply_priority](../apply.rs/apply_priority.md) |
| Callees | — |
| Win32 API | `SetPriorityClass`, `GetPriorityClass` (consumers of the constants) |
| Privileges | `SeIncreaseBasePriorityPrivilege` (for `Realtime` variant) |

## See Also

| Topic | Link |
|-------|------|
| I/O priority levels | [IOPriority](IOPriority.md) |
| Memory priority levels | [MemoryPriority](MemoryPriority.md) |
| Thread priority levels | [ThreadPriority](ThreadPriority.md) |
| Per-process configuration record | [ProcessConfig](../config.rs/ProcessConfig.md) |
| Priority application logic | [apply_priority](../apply.rs/apply_priority.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd