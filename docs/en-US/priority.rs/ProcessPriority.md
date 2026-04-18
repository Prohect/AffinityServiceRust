# ProcessPriority enum (priority.rs)

Represents the Windows process priority class as a strongly-typed Rust enumeration. Each variant maps to a corresponding `PROCESS_CREATION_FLAGS` constant used by the Win32 `SetPriorityClass` API, with a `None` sentinel indicating that no priority change should be applied. The type provides bidirectional conversion between human-readable string names, enum variants, and raw Win32 constant values.

## Syntax

```AffinityServiceRust/src/priority.rs#L10-18
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

| Variant | Win32 Constant | Numeric Value | String Representation | Description |
|---------|---------------|---------------|----------------------|-------------|
| `None` | *(none)* | N/A | `"none"` | Sentinel value indicating no priority change is requested. `as_win_const()` returns `None`. |
| `Idle` | `IDLE_PRIORITY_CLASS` | `0x00000040` | `"idle"` | The process runs only when the system is idle. Threads in an idle-priority process are preempted by threads in any higher priority class. |
| `BelowNormal` | `BELOW_NORMAL_PRIORITY_CLASS` | `0x00004000` | `"below normal"` | Priority above `Idle` but below `Normal`. Suitable for background work that should not interfere with interactive responsiveness. |
| `Normal` | `NORMAL_PRIORITY_CLASS` | `0x00000020` | `"normal"` | The default priority class for most processes. No special scheduling treatment. |
| `AboveNormal` | `ABOVE_NORMAL_PRIORITY_CLASS` | `0x00008000` | `"above normal"` | Priority above `Normal` but below `High`. Useful for latency-sensitive foreground work. |
| `High` | `HIGH_PRIORITY_CLASS` | `0x00000080` | `"high"` | Threads in a high-priority process preempt threads in `Normal` and `BelowNormal` processes. Should be used sparingly to avoid starving other processes. |
| `Realtime` | `REALTIME_PRIORITY_CLASS` | `0x00000100` | `"real time"` | The highest possible priority class. Threads preempt all other threads, including operating system threads performing critical tasks. **Requires `SeIncreaseBasePriorityPrivilege` and administrator rights.** Improper use can cause system instability. |

## Methods

### `as_str`

```AffinityServiceRust/src/priority.rs#L30-35
pub fn as_str(&self) -> &'static str
```

Returns the human-readable string name for this variant (e.g. `"below normal"`, `"high"`). Returns `"unknown"` if the variant is not found in the internal lookup table (should never happen for well-constructed values).

### `as_win_const`

```AffinityServiceRust/src/priority.rs#L37-39
pub fn as_win_const(&self) -> Option<PROCESS_CREATION_FLAGS>
```

Returns the Win32 `PROCESS_CREATION_FLAGS` constant corresponding to this variant, wrapped in `Some`. Returns `None` for the `ProcessPriority::None` sentinel, indicating that no API call should be made.

### `from_str`

```AffinityServiceRust/src/priority.rs#L41-47
pub fn from_str(s: &str) -> Self
```

Parses a case-insensitive string into a `ProcessPriority` variant. The input is lowercased before comparison against the lookup table. Returns `ProcessPriority::None` if the string does not match any known priority name.

**Note:** This is an inherent method, not an implementation of the `std::str::FromStr` trait.

### `from_win_const`

```AffinityServiceRust/src/priority.rs#L49-55
pub fn from_win_const(val: u32) -> &'static str
```

Given a raw `u32` value representing a `PROCESS_CREATION_FLAGS` constant, returns the corresponding human-readable string name. Returns `"unknown"` if the value does not match any known priority class. This method returns a string rather than an enum variant and is primarily used for diagnostic logging of values read back from the OS.

## Remarks

- All conversions are driven by a single `const TABLE` array defined as an associated constant on the enum. This table-driven design ensures that string names, enum variants, and Win32 constants stay in sync and that adding a new variant requires only a single table entry.
- The string representation `"real time"` (two words with a space) matches the display name shown by Windows Task Manager and is the expected format in configuration files.
- Setting a process to `Realtime` priority is **extremely dangerous** on a desktop system. It can starve critical OS threads (including the mouse/keyboard input thread and the disk I/O thread), potentially rendering the system unresponsive. The apply engine in AffinityServiceRust will attempt this call only if explicitly configured and the required privileges are held.
- The `ProcessPriority::None` variant is used as a default/no-op sentinel throughout the configuration system. When the config parser encounters a missing or empty `priority` field, it produces `ProcessPriority::None`, which causes the apply engine to skip the `SetPriorityClass` call entirely.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `priority.rs` |
| Callers | `config` module (deserialization), `apply::apply_priority` (process-level apply) |
| Win32 API | `SetPriorityClass` (indirect — the enum provides constants consumed by the apply module) |
| Privileges | `SeIncreaseBasePriorityPrivilege` required for `Realtime`; administrator recommended for `High` |
| Dependencies | `windows::Win32::System::Threading::PROCESS_CREATION_FLAGS` and related constants |

## See Also

| Reference | Link |
|-----------|------|
| IOPriority | [IOPriority](IOPriority.md) |
| MemoryPriority | [MemoryPriority](MemoryPriority.md) |
| ThreadPriority | [ThreadPriority](ThreadPriority.md) |
| MemoryPriorityInformation | [MemoryPriorityInformation](MemoryPriorityInformation.md) |
| apply_process_level | [apply_process_level](../main.rs/apply_process_level.md) |
| priority module overview | [README](README.md) |

---
*Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
