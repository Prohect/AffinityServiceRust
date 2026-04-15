# IOPriority type (priority.rs)

Represents the Windows I/O priority hint levels that can be applied to a process via `NtSetInformationProcess`. Each variant maps to a raw `u32` constant used by the undocumented `ProcessIoPriority` information class. The `None` sentinel indicates that no I/O priority override is configured.

## Syntax

```AffinityServiceRust/src/priority.rs#L63-69
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IOPriority {
    None,
    VeryLow,
    Low,
    Normal,
    High, // Requires SeIncreaseBasePriorityPrivilege + admin
}
```

## Members

| Variant | Win32 Value | Description |
|---------|-------------|-------------|
| `None` | *(no value)* | Sentinel variant indicating that no I/O priority change should be applied. `as_win_const()` returns `None`. |
| `VeryLow` | `0` | Background I/O priority. Operations issued by the process are serviced at the lowest priority by the I/O scheduler, suitable for maintenance or indexing tasks that should not interfere with interactive workloads. |
| `Low` | `1` | Low I/O priority. A step above `VeryLow`, appropriate for processes that perform non-urgent disk work. |
| `Normal` | `2` | Default I/O priority for most processes. This is the level assigned by the Windows kernel unless explicitly changed. |
| `High` | `3` | Elevated I/O priority. Setting this level requires the `SeIncreaseBasePriorityPrivilege` privilege and administrator rights. Appropriate only for latency-sensitive applications whose I/O should be serviced ahead of normal-priority traffic. |

## Methods

### `as_str`

```AffinityServiceRust/src/priority.rs#L80-85
pub fn as_str(&self) -> &'static str
```

Returns the human-readable lowercase string representation of the variant (e.g. `"very low"`, `"normal"`). Returns `"unknown"` if the variant is not found in the internal lookup table (unreachable for well-formed values).

### `as_win_const`

```AffinityServiceRust/src/priority.rs#L87-89
pub fn as_win_const(&self) -> Option<u32>
```

Returns the raw `u32` value suitable for passing to `NtSetInformationProcess` with the `ProcessIoPriority` information class. Returns `None` for the `IOPriority::None` sentinel, signaling that no API call should be made.

### `from_str`

```AffinityServiceRust/src/priority.rs#L91-98
pub fn from_str(s: &str) -> Self
```

Parses a case-insensitive string into the corresponding `IOPriority` variant. Recognized strings are `"none"`, `"very low"`, `"low"`, `"normal"`, and `"high"`. Unrecognized input falls back to `IOPriority::None`.

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `s` | `&str` | The string to parse. Comparison is case-insensitive (the input is lowercased before matching). |

### `from_win_const`

```AffinityServiceRust/src/priority.rs#L100-106
pub fn from_win_const(val: u32) -> &'static str
```

Converts a raw `u32` I/O priority constant back to its human-readable string name. Returns `"unknown"` if the value does not match any known constant. This is used for display and logging purposes when reading the current I/O priority of a process.

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `val` | `u32` | The raw Win32 I/O priority value to look up. |

## Remarks

- The I/O priority constants (`0`–`3`) are not part of the public Windows SDK; they are used with the undocumented `NtSetInformationProcess` / `NtQueryInformationProcess` information class `ProcessIoPriority` (value `33`). The values are stable across all modern Windows versions (Vista through Windows 11).
- Setting `IOPriority::High` on a process that the caller does not own, or without the `SeIncreaseBasePriorityPrivilege` privilege enabled, will fail with `STATUS_PRIVILEGE_NOT_HELD`.
- All conversion methods use a compile-time constant lookup table (`TABLE`) that pairs each variant with its string name and optional raw value. This ensures zero heap allocation and O(n) lookup over a fixed-size array (n ≤ 5).
- The `from_str` method is used by the configuration parser to deserialize user-supplied strings from the config file. The `as_win_const` method is used by the apply engine to obtain the value passed to the Win32 API.
- `IOPriority` derives `Clone`, `Copy`, `PartialEq`, and `Eq`, making it suitable for cheap comparisons and storage in configuration structs.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `priority.rs` |
| Callers | Configuration parser (`config.rs`), apply engine (`apply.rs`), [apply_process_level](../main.rs/apply_process_level.md) |
| Win32 API | `NtSetInformationProcess` (`ProcessIoPriority`, information class 33) |
| Privileges | `SeIncreaseBasePriorityPrivilege` + administrator (for `High` variant only) |

## See Also

| Reference | Link |
|-----------|------|
| ProcessPriority | [ProcessPriority](ProcessPriority.md) |
| MemoryPriority | [MemoryPriority](MemoryPriority.md) |
| ThreadPriority | [ThreadPriority](ThreadPriority.md) |
| MemoryPriorityInformation | [MemoryPriorityInformation](MemoryPriorityInformation.md) |
| priority module overview | [README](README.md) |
| apply_process_level | [apply_process_level](../main.rs/apply_process_level.md) |

---
> Commit SHA: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
