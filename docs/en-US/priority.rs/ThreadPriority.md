# ThreadPriority enum (priority.rs)

Type-safe representation of Windows thread priority levels. Maps between human-readable string names and Win32 `i32` thread priority constants. Provides an additional `boost_one` method for incrementally raising a thread's priority level, used by the prime thread promotion algorithm.

## Syntax

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadPriority {
    None,
    ErrorReturn,         // 0x7FFFFFFF
    ModeBackgroundBegin, // 0x00010000
    ModeBackgroundEnd,   // 0x00020000
    Idle,                // -15
    Lowest,              // -2
    BelowNormal,         // -1
    Normal,              // 0
    AboveNormal,         // 1
    Highest,             // 2
    TimeCritical,        // 15
}
```

## Members

| Variant | String Name | Win32 Value | Description |
|---------|-------------|-------------|-------------|
| `None` | `"none"` | *(None)* | No thread priority configured. Sentinel value that produces no Win32 API call. |
| `ErrorReturn` | `"error"` | `0x7FFFFFFF` | The value returned by `GetThreadPriority` on failure (`THREAD_PRIORITY_ERROR_RETURN`). |
| `ModeBackgroundBegin` | `"background begin"` | `0x00010000` | Enters background processing mode. Only valid for the calling thread. |
| `ModeBackgroundEnd` | `"background end"` | `0x00020000` | Exits background processing mode. Only valid for the calling thread. |
| `Idle` | `"idle"` | `-15` | `THREAD_PRIORITY_IDLE` — base priority 1 for `IDLE_PRIORITY_CLASS`, 16 for `REALTIME_PRIORITY_CLASS`. |
| `Lowest` | `"lowest"` | `-2` | `THREAD_PRIORITY_LOWEST` — 2 levels below normal. |
| `BelowNormal` | `"below normal"` | `-1` | `THREAD_PRIORITY_BELOW_NORMAL` — 1 level below normal. |
| `Normal` | `"normal"` | `0` | `THREAD_PRIORITY_NORMAL` — default thread priority. |
| `AboveNormal` | `"above normal"` | `1` | `THREAD_PRIORITY_ABOVE_NORMAL` — 1 level above normal. |
| `Highest` | `"highest"` | `2` | `THREAD_PRIORITY_HIGHEST` — 2 levels above normal. |
| `TimeCritical` | `"time critical"` | `15` | `THREAD_PRIORITY_TIME_CRITICAL` — base priority 15 for `IDLE_PRIORITY_CLASS`, 31 for `REALTIME_PRIORITY_CLASS`. |

## Methods

### as_str

```rust
pub fn as_str(&self) -> &'static str
```

Returns the human-readable string name for the variant (e.g., `"above normal"`). Returns `"unknown"` if the variant is not found in `TABLE` (should not happen for well-constructed values).

### as_win_const

```rust
pub fn as_win_const(&self) -> Option<i32>
```

Returns the Win32 `i32` thread priority constant, or `None` for `ThreadPriority::None`.

### from_str

```rust
pub fn from_str(s: &str) -> Self
```

Parses a case-insensitive string into a `ThreadPriority` variant. Returns `ThreadPriority::None` if the string does not match any known name.

### from_win_const

```rust
pub fn from_win_const(val: i32) -> Self
```

Converts a Win32 `i32` thread priority value back to a `ThreadPriority` variant. Returns `ThreadPriority::None` if the value does not match any known constant.

**Note:** Unlike [`ProcessPriority::from_win_const`](ProcessPriority.md), [`IOPriority::from_win_const`](IOPriority.md), and [`MemoryPriority::from_win_const`](MemoryPriority.md) which return `&'static str`, this method returns a `ThreadPriority` enum variant directly. This difference exists because thread priorities are used programmatically (e.g., in `boost_one`) rather than only for display.

### boost_one

```rust
pub fn boost_one(&self) -> Self
```

Returns the next higher priority level in the standard priority ladder. Used by the prime thread promotion algorithm in [`apply_prime_threads_promote`](../apply.rs/apply_prime_threads_promote.md) to incrementally raise a thread's priority.

**Promotion ladder:**

| Input | Output |
|-------|--------|
| `Idle` | `Lowest` |
| `Lowest` | `BelowNormal` |
| `BelowNormal` | `Normal` |
| `Normal` | `AboveNormal` |
| `AboveNormal` | `Highest` |
| `Highest` | `Highest` *(capped)* |
| `TimeCritical` | `TimeCritical` *(unchanged)* |
| `None` | `None` *(unchanged)* |
| `ErrorReturn` | `ErrorReturn` *(unchanged)* |
| `ModeBackgroundBegin` | `ModeBackgroundBegin` *(unchanged)* |
| `ModeBackgroundEnd` | `ModeBackgroundEnd` *(unchanged)* |

The boost is capped at `Highest` — it never promotes to `TimeCritical`, which could cause system instability. Special variants (`None`, `ErrorReturn`, `ModeBackgroundBegin`, `ModeBackgroundEnd`) are returned unchanged.

### to_thread_priority_struct

```rust
pub fn to_thread_priority_struct(self) -> THREAD_PRIORITY
```

Converts the enum to a `windows::Win32::System::Threading::THREAD_PRIORITY` newtype wrapper for direct use in Win32 FFI calls. Falls back to `THREAD_PRIORITY(0)` (normal priority) if `as_win_const()` returns `None`.

## Remarks

### TABLE constant

All conversions are driven by a single `TABLE` constant:

```rust
const TABLE: &'static [(Self, &'static str, Option<i32>)] = &[...];
```

This array of `(variant, name, win32_value)` tuples is the single source of truth for all bidirectional mappings, following the same DRY pattern as the other priority enums in this module.

### Background mode variants

`ModeBackgroundBegin` and `ModeBackgroundEnd` are special thread priority values that switch the calling thread into or out of background processing mode. In background mode, the system reduces the thread's scheduling priority, IO priority, and memory priority simultaneously. These values are only valid when applied to the **current** thread — using them with `SetThreadPriority` on a remote thread will fail. AffinityServiceRust does not typically set these values on remote threads; they are included for completeness and for `from_win_const` round-tripping.

### Platform notes

- Thread priority values are `i32` signed integers, unlike process priority classes which are `u32` flags.
- The `THREAD_PRIORITY` newtype in the `windows` crate wraps an `i32`. The `to_thread_priority_struct` method produces this wrapper for callers that need the typed FFI struct.
- `SetThreadPriority` requires `THREAD_SET_INFORMATION` access on the target thread handle.

## Requirements

| | |
|---|---|
| **Module** | [`src/priority.rs`](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/priority.rs) |
| **Used by** | [`apply_prime_threads_promote`](../apply.rs/apply_prime_threads_promote.md), [`apply_prime_threads_demote`](../apply.rs/apply_prime_threads_demote.md), [config parsing](../config.rs/README.md) |
| **Win32 type** | [`THREAD_PRIORITY`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadpriority) |
| **Privileges** | `THREAD_SET_INFORMATION` (when setting via `SetThreadPriority`) |

## See Also

| Topic | Link |
|-------|------|
| Process priority enum | [ProcessPriority](ProcessPriority.md) |
| IO priority enum | [IOPriority](IOPriority.md) |
| Memory priority enum | [MemoryPriority](MemoryPriority.md) |
| Prime thread promotion | [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md) |
| Prime thread demotion | [apply_prime_threads_demote](../apply.rs/apply_prime_threads_demote.md) |
| Module overview | [priority.rs](README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*