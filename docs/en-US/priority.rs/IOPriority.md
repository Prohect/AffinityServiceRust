# IOPriority enum (priority.rs)

Represents the I/O priority level assigned to a process via `NtSetInformationProcess` with the `ProcessInformationClass` value for I/O priority. Each variant maps to a numeric constant consumed by the NT native API. The `None` variant serves as a sentinel indicating that no I/O priority change was requested in the configuration.

Setting `IOPriority::High` requires the `SeIncreaseBasePriorityPrivilege` privilege and an elevated (administrator) token.

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

| Variant | Win32 value | String name | Description |
|---------|-------------|-------------|-------------|
| `None` | *(none)* | `"none"` | Sentinel — no I/O priority change requested. `as_win_const` returns `None`. |
| `VeryLow` | `0` | `"very low"` | Lowest I/O priority. Background-class I/O. |
| `Low` | `1` | `"low"` | Low I/O priority. |
| `Normal` | `2` | `"normal"` | Default I/O priority for most processes. |
| `High` | `3` | `"high"` | Highest I/O priority. Requires `SeIncreaseBasePriorityPrivilege` and administrator elevation. |

## Methods

### as_str

```rust
pub fn as_str(&self) -> &'static str
```

Returns the human-readable string name for this variant (e.g., `"very low"`, `"normal"`). Returns `"unknown"` if the variant is somehow absent from the internal lookup table.

### as_win_const

```rust
pub fn as_win_const(&self) -> Option<u32>
```

Returns the numeric I/O priority value for use with `NtSetInformationProcess`, or `None` for the `IOPriority::None` sentinel.

### from_str

```rust
pub fn from_str(s: &str) -> Self
```

Parses a case-insensitive string into an `IOPriority` variant. Unrecognized strings return `IOPriority::None`. This is **not** the `std::str::FromStr` trait; it is an inherent method.

### from_win_const

```rust
pub fn from_win_const(val: u32) -> &'static str
```

Maps a raw numeric I/O priority value back to its human-readable string name. Returns `"unknown"` for unrecognized values. Note that this returns a `&'static str`, not an `IOPriority` variant.

## Remarks

The Win32 constant values (`0` through `3`) correspond to the `IO_PRIORITY_HINT` enumeration defined in `ntddk.h`. AffinityServiceRust uses the `NtSetInformationProcess` / `NtQueryInformationProcess` native API rather than the documented Win32 `SetProcessInformation` surface because the latter does not expose I/O priority directly.

The lookup is driven by a `const TABLE` array of `(Self, &str, Option<u32>)` tuples. All four public methods perform a linear scan of this table; for five variants the cost is negligible.

`from_str` performs a case-insensitive comparison by lowercasing the input before matching.

## Requirements

| | |
|---|---|
| **Module** | `priority` |
| **Callers** | [parse_and_insert_rules](../config.rs/parse_and_insert_rules.md), [apply_io_priority](../apply.rs/apply_io_priority.md) |
| **Callees** | *(none — pure data mapping)* |
| **Win32 API** | `NtSetInformationProcess` (`ProcessInformationClass` = I/O priority), `NtQueryInformationProcess` |
| **Privileges** | `SeIncreaseBasePriorityPrivilege` required for `High` variant |

## See Also

| Topic | Link |
|-------|------|
| Process priority class enum | [ProcessPriority](ProcessPriority.md) |
| Memory priority level enum | [MemoryPriority](MemoryPriority.md) |
| Thread priority level enum | [ThreadPriority](ThreadPriority.md) |
| I/O priority application logic | [apply_io_priority](../apply.rs/apply_io_priority.md) |
| priority module overview | [README](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd