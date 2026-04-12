# ThreadPriority enum (priority.rs)

Represents Windows thread priority levels used with `SetThreadPriority` and `GetThreadPriority`. Each variant maps to a well-known Win32 constant. The `None` variant means "don't change the current priority."

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

`None`

Sentinel value indicating that thread priority should not be changed. Returns `None` from `as_win_const()`.

`ErrorReturn`

Maps to `THREAD_PRIORITY_ERROR_RETURN` (`0x7FFFFFFF`). Represents an error condition returned by `GetThreadPriority`.

`ModeBackgroundBegin`

Maps to `THREAD_MODE_BACKGROUND_BEGIN` (`0x00010000`). Puts the calling thread into background processing mode. **Must only be used on the current thread.**

`ModeBackgroundEnd`

Maps to `THREAD_MODE_BACKGROUND_END` (`0x00020000`). Takes the calling thread out of background processing mode. **Must only be used on the current thread.**

`Idle`

Maps to `THREAD_PRIORITY_IDLE` (`-15`). The lowest schedulable priority for normal threads.

`Lowest`

Maps to `THREAD_PRIORITY_LOWEST` (`-2`). Two levels below normal.

`BelowNormal`

Maps to `THREAD_PRIORITY_BELOW_NORMAL` (`-1`). One level below normal.

`Normal`

Maps to `THREAD_PRIORITY_NORMAL` (`0`). The default thread priority.

`AboveNormal`

Maps to `THREAD_PRIORITY_ABOVE_NORMAL` (`1`). One level above normal.

`Highest`

Maps to `THREAD_PRIORITY_HIGHEST` (`2`). Two levels above normal. This is the cap for [`boost_one`](#boost_one).

`TimeCritical`

Maps to `THREAD_PRIORITY_TIME_CRITICAL` (`15`). The highest priority level. Threads at this level preempt almost everything.

## Methods

| Method | Signature | Description |
| --- | --- | --- |
| **as_str** | `pub fn as_str(&self) -> &'static str` | Returns the human-readable string name (e.g. `"normal"`, `"above normal"`, `"time critical"`). |
| **as_win_const** | `pub fn as_win_const(&self) -> Option<i32>` | Returns the Win32 integer constant, or `None` for the `None` variant. |
| **from_str** | `pub fn from_str(s: &str) -> Self` | Parses a case-insensitive string into a variant. Unrecognized strings map to `None`. |
| **from_win_const** | `pub fn from_win_const(val: i32) -> Self` | Looks up a variant by its Win32 integer value. Unrecognized values map to `None`. |
| **boost_one** | `pub fn boost_one(&self) -> Self` | Returns the next higher priority level, capped at `Highest`. |
| **to_thread_priority_struct** | `pub fn to_thread_priority_struct(self) -> THREAD_PRIORITY` | Wraps `as_win_const()` into a `windows` crate `THREAD_PRIORITY` struct. Defaults to `0` if `None`. |

### boost_one

Returns the next higher schedulable priority variant along the progression:

`Idle` → `Lowest` → `BelowNormal` → `Normal` → `AboveNormal` → `Highest` → `Highest` (capped)

Special variants are identity-mapped:

- `None` → `None`
- `ErrorReturn` → `ErrorReturn`
- `ModeBackgroundBegin` → `ModeBackgroundBegin`
- `ModeBackgroundEnd` → `ModeBackgroundEnd`
- `TimeCritical` → `TimeCritical`

This method is used by the prime thread scheduler to boost a thread's priority by one level when it is promoted to a prime CPU. The cap at `Highest` prevents accidental promotion to `TimeCritical`, which could starve system threads.

### to_thread_priority_struct

Convenience wrapper that converts the variant into the `THREAD_PRIORITY` newtype expected by the `windows` crate API bindings. If `as_win_const()` returns `None` (i.e. the variant is `None`), defaults to `THREAD_PRIORITY(0)` (normal priority).

## Remarks

`ThreadPriority` follows the same lookup-table pattern as [ProcessPriority](ProcessPriority.md), [IOPriority](IOPriority.md), and [MemoryPriority](MemoryPriority.md): a static `TABLE` array enables bidirectional conversion between enum variants, string names, and Win32 constants.

Unlike the other priority enums, `ThreadPriority` returns `Self` (not `&'static str`) from `from_win_const`, because thread priority values are used in further logic (e.g. storing the original priority for later restoration).

**Priority boosting in prime thread scheduling:** When a thread is promoted to a prime CPU via [`apply_prime_threads_promote`](../apply.rs/apply_prime_threads_promote.md), its original priority is saved in [`ThreadStats.original_priority`](../scheduler.rs/ThreadStats.md) and then boosted one level with `boost_one()`. When demoted, the original priority is restored. This mechanism ensures prime threads get a slight scheduling advantage on their assigned fast cores without reaching dangerous priority levels.

**Background mode variants** (`ModeBackgroundBegin` / `ModeBackgroundEnd`) are included for completeness of the Win32 mapping but are only meaningful when applied to the calling thread, not to arbitrary remote threads.

### Lookup table

| Variant | String | Win32 value |
| --- | --- | --- |
| None | `"none"` | *(none)* |
| ErrorReturn | `"error"` | `0x7FFFFFFF` |
| ModeBackgroundBegin | `"background begin"` | `0x00010000` |
| ModeBackgroundEnd | `"background end"` | `0x00020000` |
| Idle | `"idle"` | `-15` |
| Lowest | `"lowest"` | `-2` |
| BelowNormal | `"below normal"` | `-1` |
| Normal | `"normal"` | `0` |
| AboveNormal | `"above normal"` | `1` |
| Highest | `"highest"` | `2` |
| TimeCritical | `"time critical"` | `15` |

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | `src/priority.rs` |
| **Source lines** | L162–L244 |
| **Used by** | [`apply_priority`](../apply.rs/apply_priority.md), [`apply_prime_threads_promote`](../apply.rs/apply_prime_threads_promote.md), [`apply_prime_threads_demote`](../apply.rs/apply_prime_threads_demote.md), [`ThreadStats`](../scheduler.rs/ThreadStats.md), [`ProcessConfig`](../config.rs/ProcessConfig.md) |
| **Windows API** | [SetThreadPriority](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadpriority), [GetThreadPriority](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadpriority) |

## See also

- [priority.rs module overview](README.md)
- [ProcessPriority](ProcessPriority.md)
- [IOPriority](IOPriority.md)
- [MemoryPriority](MemoryPriority.md)
- [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md)