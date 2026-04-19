# IOPriority enum (priority.rs)

Type-safe representation of Windows NT IO priority levels. Maps between human-readable string names and the undocumented `u32` IO priority values used by `NtSetInformationProcess` / `NtQueryInformationProcess` with the `ProcessIoPriority` information class.

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

| Variant | String name | Win32 constant | Description |
|---------|-------------|----------------|-------------|
| `None` | `"none"` | *(none)* | No IO priority configured. `as_win_const()` returns `None`. |
| `VeryLow` | `"very low"` | `0` | Lowest IO priority. Background-level IO scheduling. |
| `Low` | `"low"` | `1` | Low IO priority. Reduced IO bandwidth allocation. |
| `Normal` | `"normal"` | `2` | Default IO priority for most processes. |
| `High` | `"high"` | `3` | Highest IO priority. Requires `SeIncreaseBasePriorityPrivilege` and administrator rights. |

## Methods

### as_str

```rust
pub fn as_str(&self) -> &'static str
```

Returns the human-readable string name for this IO priority variant. Returns `"unknown"` if the variant is not found in the internal table (unreachable for valid variants).

### as_win_const

```rust
pub fn as_win_const(&self) -> Option<u32>
```

Returns the NT IO priority value for use with `NtSetInformationProcess`, or `None` for the `IOPriority::None` variant. The caller should skip the API call when this returns `None`.

### from_str

```rust
pub fn from_str(s: &str) -> Self
```

Parses a case-insensitive string into an `IOPriority` variant. Returns `IOPriority::None` if the string does not match any known IO priority name. The input is lowercased before comparison.

**Parameters**

| Parameter | Type | Description |
|-----------|------|-------------|
| `s` | `&str` | The string to parse (e.g., `"very low"`, `"Normal"`, `"HIGH"`). |

### from_win_const

```rust
pub fn from_win_const(val: u32) -> &'static str
```

Converts an NT IO priority value back to its human-readable string name. Returns `"unknown"` if the value does not match any known constant. This is used for logging the current IO priority when comparing against the desired value.

**Parameters**

| Parameter | Type | Description |
|-----------|------|-------------|
| `val` | `u32` | The NT IO priority value (0–3). |

## Remarks

### TABLE-driven conversions

Like all priority enums in this module, `IOPriority` uses a single `const TABLE` array to drive all four conversion methods. Each entry is a `(Self, &'static str, Option<u32>)` tuple, ensuring that string names and numeric constants are defined in exactly one place.

### Undocumented API

Unlike process priority (which uses the documented `SetPriorityClass` API), IO priority is managed through the NT-native `NtSetInformationProcess` with `ProcessIoPriority` (information class 33). The `u32` values 0–3 are not officially documented by Microsoft but are well-established through reverse engineering and community documentation.

### High IO priority

Setting `IOPriority::High` (value `3`) requires the calling process to hold `SeIncreaseBasePriorityPrivilege` and be running with administrator rights. Without these, the `NtSetInformationProcess` call will fail with `STATUS_PRIVILEGE_NOT_HELD`. The service acquires this privilege at startup via [`enable_inc_base_priority_privilege`](../winapi.rs/enable_inc_base_priority_privilege.md).

### Note on from_str

This `from_str` is an inherent method, not a `std::str::FromStr` trait implementation. It does not return a `Result` — unrecognized strings silently map to `IOPriority::None`.

## Requirements

| | |
|---|---|
| **Module** | [`src/priority.rs`](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/priority.rs) |
| **Used by** | [`apply_io_priority`](../apply.rs/apply_io_priority.md), config parser ([`read_config`](../config.rs/read_config.md)) |
| **Win32 API** | `NtQueryInformationProcess` / `NtSetInformationProcess` with `ProcessIoPriority` |
| **Privileges** | `SeIncreaseBasePriorityPrivilege` required for `High` variant |

## See Also

| Topic | Link |
|-------|------|
| Process priority enum | [ProcessPriority](ProcessPriority.md) |
| Memory priority enum | [MemoryPriority](MemoryPriority.md) |
| Thread priority enum | [ThreadPriority](ThreadPriority.md) |
| IO priority application | [apply_io_priority](../apply.rs/apply_io_priority.md) |
| Module overview | [priority.rs](README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*