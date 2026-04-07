# Priority Module Documentation

Priority level enums and Windows API mappings.

## Overview

This module defines enums for all Windows priority types with bidirectional conversion to/from Windows constants and string representations.

## Called By

- `config.rs` - Parsing priority strings from config
- `apply.rs` - Converting to Windows API constants
- `scheduler.rs` - Thread priority manipulation
- `logging.rs` - Converting back to strings for output

## Enums

### ProcessPriority

Windows process priority classes.

```rust
pub enum ProcessPriority {
    None,           // Don't change
    Idle,           // IDLE_PRIORITY_CLASS
    BelowNormal,    // BELOW_NORMAL_PRIORITY_CLASS
    Normal,         // NORMAL_PRIORITY_CLASS
    AboveNormal,    // ABOVE_NORMAL_PRIORITY_CLASS
    High,           // HIGH_PRIORITY_CLASS
    Realtime,       // REALTIME_PRIORITY_CLASS
}
```

**String Values:**
| Enum | String | Windows Constant |
|------|--------|------------------|
| `None` | `"none"` | `None` |
| `Idle` | `"idle"` | `IDLE_PRIORITY_CLASS` |
| `BelowNormal` | `"below normal"` | `BELOW_NORMAL_PRIORITY_CLASS` |
| `Normal` | `"normal"` | `NORMAL_PRIORITY_CLASS` |
| `AboveNormal` | `"above normal"` | `ABOVE_NORMAL_PRIORITY_CLASS` |
| `High` | `"high"` | `HIGH_PRIORITY_CLASS` |
| `Realtime` | `"real time"` | `REALTIME_PRIORITY_CLASS` |

**Methods:**

```rust
// Convert to display string
pub fn as_str(&self) -> &'static str

// Convert to Windows API constant
pub fn as_win_const(&self) -> Option<PROCESS_CREATION_FLAGS>

// Parse from string (case-insensitive)
pub fn from_str(s: &str) -> Self

// Convert from Windows constant value
pub fn from_win_const(val: u32) -> &'static str
```

**Examples:**
```rust
let p = ProcessPriority::from_str("High");        // → ProcessPriority::High
p.as_str();                                        // → "high"
p.as_win_const();                                  // → Some(HIGH_PRIORITY_CLASS)

ProcessPriority::from_str("invalid");              // → ProcessPriority::None
ProcessPriority::from_win_const(0x8000);           // → "normal"
```

### IOPriority

I/O priority levels.

```rust
pub enum IOPriority {
    None,       // Don't change
    VeryLow,    // Priority 0
    Low,        // Priority 1
    Normal,     // Priority 2
    High,       // Priority 3 (requires admin + SeIncreaseBasePriorityPrivilege)
}
```

**String Values:**
| Enum | String | Value |
|------|--------|-------|
| `None` | `"none"` | `None` |
| `VeryLow` | `"very low"` | `Some(0)` |
| `Low` | `"low"` | `Some(1)` |
| `Normal` | `"normal"` | `Some(2)` |
| `High` | `"high"` | `Some(3)` |

**Privilege Requirements:**
- `High` requires:
  - Administrator token
  - `SeIncreaseBasePriorityPrivilege`

**Methods:** Same pattern as `ProcessPriority`

### MemoryPriority

Memory page priority levels.

```rust
pub enum MemoryPriority {
    None,         // Don't change
    VeryLow,      // MEMORY_PRIORITY_VERY_LOW
    Low,          // MEMORY_PRIORITY_LOW
    Medium,       // MEMORY_PRIORITY_MEDIUM
    BelowNormal,  // MEMORY_PRIORITY_BELOW_NORMAL
    Normal,       // MEMORY_PRIORITY_NORMAL
}
```

**String Values:**
| Enum | String | Windows Constant |
|------|--------|------------------|
| `None` | `"none"` | `None` |
| `VeryLow` | `"very low"` | `MEMORY_PRIORITY_VERY_LOW` |
| `Low` | `"low"` | `MEMORY_PRIORITY_LOW` |
| `Medium` | `"medium"` | `MEMORY_PRIORITY_MEDIUM` |
| `BelowNormal` | `"below normal"` | `MEMORY_PRIORITY_BELOW_NORMAL` |
| `Normal` | `"normal"` | `MEMORY_PRIORITY_NORMAL` |

**Note:** Memory priority affects page replacement - lower priority pages are paged out first under memory pressure.

**Struct Wrapper:**
```rust
#[repr(C)]
pub struct MemoryPriorityInformation(pub u32);
```

Used with `GetProcessInformation`/`SetProcessInformation`.

### ThreadPriority

Thread priority levels.

```rust
pub enum ThreadPriority {
    None,                // Don't change
    ErrorReturn,         // 0x7FFFFFFF (error indicator)
    ModeBackgroundBegin, // 0x00010000 (current thread only)
    ModeBackgroundEnd,   // 0x00020000 (current thread only)
    Idle,                // -15
    Lowest,              // -2
    BelowNormal,         // -1
    Normal,              // 0
    AboveNormal,         // 1
    Highest,             // 2
    TimeCritical,        // 15
}
```

**String Values:**
| Enum | String | Value |
|------|--------|-------|
| `None` | `"none"` | `None` |
| `ErrorReturn` | `"error"` | `Some(0x7FFFFFFF)` |
| `ModeBackgroundBegin` | `"background begin"` | `Some(0x00010000)` |
| `ModeBackgroundEnd` | `"background end"` | `Some(0x00020000)` |
| `Idle` | `"idle"` | `Some(-15)` |
| `Lowest` | `"lowest"` | `Some(-2)` |
| `BelowNormal` | `"below normal"` | `Some(-1)` |
| `Normal` | `"normal"` | `Some(0)` |
| `AboveNormal` | `"above normal"` | `Some(1)` |
| `Highest` | `"highest"` | `Some(2)` |
| `TimeCritical` | `"time critical"` | `Some(15)` |

**Special Values:**
- `ErrorReturn` (0x7FFFFFFF) - Indicates error from `GetThreadPriority`
- `ModeBackgroundBegin/End` - Can only be used on calling thread (for background mode)

**Methods:**

```rust
// Standard conversions
pub fn as_str(&self) -> &'static str
pub fn as_win_const(&self) -> Option<i32>
pub fn from_str(s: &str) -> Self
pub fn from_win_const(val: i32) -> Self

// Returns next higher priority level, capped at Highest
pub fn boost_one(&self) -> Self

// Convert to Windows THREAD_PRIORITY struct
pub fn to_thread_priority_struct(self) -> THREAD_PRIORITY
```

**boost_one Hierarchy:**
```
None → None
Idle → Lowest
Lowest → BelowNormal
BelowNormal → Normal
Normal → AboveNormal
AboveNormal → Highest
Highest → Highest
TimeCritical → TimeCritical
(ErrorReturn, ModeBackground*) → Same
```

**Examples:**
```rust
let p = ThreadPriority::Normal;
p.boost_one();  // → ThreadPriority::AboveNormal

ThreadPriority::Highest.boost_one();  // → ThreadPriority::Highest (capped)

// Parse and convert for API
let p = ThreadPriority::from_str("above normal");
SetThreadPriority(handle, p.to_thread_priority_struct());
```

## Design Pattern

All enums follow the same pattern:

1. **Static Lookup Table:**
```rust
const TABLE: &'static [(Self, &'static str, Option<NativeType>)] = &[
    (Self::Variant, "string", Some(NATIVE_VALUE)),
    ...
];
```

2. **Bidirectional Conversion:**
- `as_str()` - For display/logging
- `as_win_const()` - For API calls
- `from_str()` - For config parsing (case-insensitive)
- `from_win_const()` - For reading current values

3. **None Variant:**
All enums have a `None` variant with `"none"` string representation that maps to `None` Windows constant. This indicates "don't change this setting."

## Dependencies

- `windows::Win32::System::Threading` - Windows priority constants

## Platform Notes

### Priority Class Effects

| Priority Class | Base Priority | Dynamic Range |
|----------------|---------------|---------------|
| Idle | 4 | 1-6 |
| Below Normal | 6 | 1-8 |
| Normal | 8 | 1-15 |
| Above Normal | 10 | 1-15 |
| High | 13 | 1-15 |
| Realtime | 24 | 16-31 |

### I/O Priority Mapping

I/O priority is independent of CPU priority:
- Most processes run at Normal I/O priority
- VeryLow intended for background tasks
- High intended for time-critical I/O

### Memory Priority Effects

Lower memory priority causes pages to be preferentially moved to the modified/standby list under pressure. Useful for:
- Background processes (set lower)
- Interactive processes (set higher)
