# MemoryPriority enum (priority.rs)

Represents the memory priority level that can be assigned to a process via the Windows `SetProcessInformation` API with `ProcessMemoryPriority` information class. Memory priority influences how quickly the memory manager trims a process's working set pages and how it prioritizes them in the standby list. Lower memory priority causes pages to be repurposed sooner under memory pressure. The `None` variant acts as a sentinel indicating that no memory priority change was requested in the configuration.

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

| Variant | Win32 constant | Value | Description |
|---------|---------------|-------|-------------|
| `None` | *(none)* | — | Sentinel: no memory priority change requested. `as_win_const` returns `None`. |
| `VeryLow` | `MEMORY_PRIORITY_VERY_LOW` | 1 | Lowest memory priority. Pages are trimmed and repurposed first under memory pressure. |
| `Low` | `MEMORY_PRIORITY_LOW` | 2 | Low memory priority. Pages are trimmed before medium and higher levels. |
| `Medium` | `MEMORY_PRIORITY_MEDIUM` | 3 | Medium memory priority. Balanced trimming behavior. |
| `BelowNormal` | `MEMORY_PRIORITY_BELOW_NORMAL` | 4 | Below-normal memory priority. Pages are trimmed after medium but before normal. |
| `Normal` | `MEMORY_PRIORITY_NORMAL` | 5 | Default memory priority. Pages are retained longest in the standby list. |

## Methods

### as_str

```rust
pub fn as_str(&self) -> &'static str
```

Returns the human-readable string name for this variant (e.g., `"very low"`, `"normal"`). Returns `"unknown"` if the variant is not found in the lookup table (which cannot occur for well-constructed values).

### as_win_const

```rust
pub fn as_win_const(&self) -> Option<MEMORY_PRIORITY>
```

Returns the corresponding `MEMORY_PRIORITY` constant for use with `SetProcessInformation`, or `None` for the `MemoryPriority::None` sentinel variant.

### from_str

```rust
pub fn from_str(s: &str) -> Self
```

Parses a case-insensitive string into a `MemoryPriority` variant. Recognized values are `"none"`, `"very low"`, `"low"`, `"medium"`, `"below normal"`, and `"normal"`. Unrecognized strings default to `MemoryPriority::None`.

**Note:** This is an inherent method, not a `std::str::FromStr` trait implementation. It does not return a `Result`; unrecognized input silently maps to `None`.

### from_win_const

```rust
pub fn from_win_const(val: u32) -> &'static str
```

Looks up a raw `u32` memory priority constant value and returns the corresponding human-readable name string. Returns `"unknown"` if the value does not match any known `MEMORY_PRIORITY` constant. This method is used for diagnostic logging when reading the current memory priority of a process.

## Remarks

The `MemoryPriority` enum works in conjunction with [MemoryPriorityInformation](MemoryPriorityInformation.md), which serves as the `#[repr(C)]` buffer passed to `SetProcessInformation` and `GetProcessInformation`. When applying memory priority, the service calls `as_win_const()` to obtain the `MEMORY_PRIORITY` value, wraps it in a `MemoryPriorityInformation` struct, and passes it to the Windows API.

All conversion methods use a single `const TABLE` array that pairs each variant with its string name and optional Win32 constant. This ensures that string ↔ constant ↔ variant mappings are always consistent.

Memory priority is orthogonal to process priority class and I/O priority; all three can be set independently on the same process.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `priority` |
| Callers | [apply_memory_priority](../apply.rs/apply_memory_priority.md), [read_config](../config.rs/read_config.md), [parse_and_insert_rules](../config.rs/parse_and_insert_rules.md) |
| Win32 API | `SetProcessInformation`, `GetProcessInformation` (`ProcessMemoryPriority`) |
| Privileges | Standard process handle with `PROCESS_SET_INFORMATION` access |

## See Also

| Topic | Link |
|-------|------|
| C-compatible buffer for memory priority API calls | [MemoryPriorityInformation](MemoryPriorityInformation.md) |
| Process priority class enum | [ProcessPriority](ProcessPriority.md) |
| I/O priority enum | [IOPriority](IOPriority.md) |
| Thread priority enum | [ThreadPriority](ThreadPriority.md) |
| Module overview | [priority module](README.md) |