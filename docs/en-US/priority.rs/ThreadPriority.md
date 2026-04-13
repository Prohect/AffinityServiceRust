# ThreadPriority enum (priority.rs)

Represents the priority level of a Windows thread. This enum wraps the signed integer constants used by `SetThreadPriority` and `GetThreadPriority`, including standard scheduling levels from `Idle` (−15) through `TimeCritical` (15), as well as special sentinels for background processing mode transitions and error returns. A `None` variant indicates no thread priority change is requested. The enum provides round-trip conversion between Rust variants, display strings, and Win32 integer values, plus a `boost_one` method for incremental priority promotion used by the prime-thread scheduler.

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

| Variant | Win32 value | Description |
|---------|-------------|-------------|
| `None` | *(none)* | Sentinel — no thread priority change requested. |
| `ErrorReturn` | `0x7FFFFFFF` | Value returned by `GetThreadPriority` on failure (`THREAD_PRIORITY_ERROR_RETURN`). |
| `ModeBackgroundBegin` | `0x00010000` | Lowers the thread into background processing mode. Must only be used on the calling thread. |
| `ModeBackgroundEnd` | `0x00020000` | Ends background processing mode for the calling thread. Must only be used on the calling thread. |
| `Idle` | `−15` | `THREAD_PRIORITY_IDLE` — lowest scheduling priority. |
| `Lowest` | `−2` | `THREAD_PRIORITY_LOWEST` — two levels below normal. |
| `BelowNormal` | `−1` | `THREAD_PRIORITY_BELOW_NORMAL` — one level below normal. |
| `Normal` | `0` | `THREAD_PRIORITY_NORMAL` — default scheduling priority. |
| `AboveNormal` | `1` | `THREAD_PRIORITY_ABOVE_NORMAL` — one level above normal. |
| `Highest` | `2` | `THREAD_PRIORITY_HIGHEST` — two levels above normal. |
| `TimeCritical` | `15` | `THREAD_PRIORITY_TIME_CRITICAL` — highest scheduling priority. |

## Methods

### as_str

```rust
pub fn as_str(&self) -> &'static str
```

Returns the human-readable string representation of the variant (e.g., `"below normal"`, `"time critical"`). Returns `"unknown"` if the variant is not found in the internal lookup table (should not occur for valid variants).

### as_win_const

```rust
pub fn as_win_const(&self) -> Option<i32>
```

Returns the Win32 integer constant for this variant, or `None` for the `None` sentinel variant.

### from_str

```rust
pub fn from_str(s: &str) -> Self
```

Parses a case-insensitive string into a `ThreadPriority` variant. Returns `ThreadPriority::None` if the string does not match any known priority name. This method performs a lowercase comparison against the internal lookup table.

### from_win_const

```rust
pub fn from_win_const(val: i32) -> Self
```

Looks up a `ThreadPriority` variant by its Win32 integer value. Returns `ThreadPriority::None` if the value does not match any known constant.

### boost_one

```rust
pub fn boost_one(&self) -> Self
```

Returns the next higher standard priority level, capped at `Highest`. This method is used by the prime-thread scheduler to incrementally promote threads that sustain high CPU utilization.

**Promotion chain:** `Idle` → `Lowest` → `BelowNormal` → `Normal` → `AboveNormal` → `Highest` → `Highest` (capped).

The following variants are identity-mapped (returned unchanged): `None`, `ErrorReturn`, `ModeBackgroundBegin`, `ModeBackgroundEnd`, `TimeCritical`.

### to_thread_priority_struct

```rust
pub fn to_thread_priority_struct(self) -> THREAD_PRIORITY
```

Converts the variant into a `windows::Win32::System::Threading::THREAD_PRIORITY` struct for direct use with Win32 APIs. Falls back to `THREAD_PRIORITY(0)` (normal) if `as_win_const` returns `None`.

## Remarks

- The internal lookup table (`TABLE`) stores all variant-to-string-to-constant mappings in a single `&'static` slice, ensuring that all four conversion methods share one authoritative source of truth.
- `ModeBackgroundBegin` and `ModeBackgroundEnd` are not standard scheduling levels; they are control codes that change the thread's scheduling and I/O behavior. The Win32 documentation states these must only be applied to the current thread — applying them to a remote thread is undefined behavior. AffinityServiceRust does not use these variants for remote thread manipulation.
- `boost_one` never promotes past `Highest`. Promotion to `TimeCritical` is intentionally excluded because `TimeCritical` preempts most system threads and could cause system instability if applied broadly.
- Unlike `ProcessPriority::from_win_const` and `IOPriority::from_win_const` which return `&'static str`, `ThreadPriority::from_win_const` returns `Self`. This allows the caller to further manipulate the variant (e.g., call `boost_one`).

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `priority` |
| Callers | [apply_prime_threads](../apply.rs/apply_prime_threads.md), [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md), [apply_prime_threads_demote](../apply.rs/apply_prime_threads_demote.md), [parse_and_insert_rules](../config.rs/parse_and_insert_rules.md) |
| Callees | *(none — pure data mapping)* |
| Win32 API | `SetThreadPriority`, `GetThreadPriority` (consumed indirectly via [apply module](../apply.rs/README.md)) |
| Privileges | Setting thread priority above `Normal` for processes in other sessions may require `SeDebugPrivilege`. |

## See Also

| Topic | Link |
|-------|------|
| Process-level priority class | [ProcessPriority](ProcessPriority.md) |
| I/O priority levels | [IOPriority](IOPriority.md) |
| Memory priority levels | [MemoryPriority](MemoryPriority.md) |
| Prime-thread scheduling logic | [scheduler module](../scheduler.rs/README.md) |
| Thread handle acquisition | [get_thread_handle](../winapi.rs/get_thread_handle.md) |
| Configuration parsing | [config module](../config.rs/README.md) |