# ThreadPriority enum (priority.rs)

Maps the full set of Windows thread priority levels to strongly-typed Rust enum variants, with bidirectional conversion between human-readable string names, enum variants, and the raw `i32` values expected by the Win32 `SetThreadPriority` API. The enum covers the standard scheduling levels from `Idle` (−15) through `TimeCritical` (15), the special background-mode tokens `ModeBackgroundBegin` / `ModeBackgroundEnd`, the `ErrorReturn` sentinel, and a `None` variant indicating that no priority change is requested. A `boost_one` method supports single-step priority elevation for prime-thread boosting.

## Syntax

```AffinityServiceRust/src/priority.rs#L159-L172
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadPriority {
    None,
    ErrorReturn,         // 0x7FFFFFFF
    ModeBackgroundBegin, // 0x00010000 (use only for current thread)
    ModeBackgroundEnd,   // 0x00020000 (use only for current thread)
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

| Variant | Win32 value | String key | Description |
|---------|-------------|------------|-------------|
| `None` | *(no API call)* | `"none"` | Sentinel indicating no priority change should be applied. The `as_win_const` method returns `None` for this variant. |
| `ErrorReturn` | `0x7FFFFFFF` | `"error"` | Represents the `THREAD_PRIORITY_ERROR_RETURN` value returned by `GetThreadPriority` on failure. Not normally used as an input to `SetThreadPriority`. |
| `ModeBackgroundBegin` | `0x00010000` | `"background begin"` | Lowers the thread to background processing mode, reducing its scheduling priority, I/O priority, and memory priority. **Must only be set on the calling thread.** |
| `ModeBackgroundEnd` | `0x00020000` | `"background end"` | Restores normal processing mode for a thread that previously entered background mode. **Must only be set on the calling thread.** |
| `Idle` | `-15` | `"idle"` | `THREAD_PRIORITY_IDLE`. The lowest regular scheduling priority. The thread runs only when no other threads are ready. |
| `Lowest` | `-2` | `"lowest"` | `THREAD_PRIORITY_LOWEST`. Two levels below normal. |
| `BelowNormal` | `-1` | `"below normal"` | `THREAD_PRIORITY_BELOW_NORMAL`. One level below normal. |
| `Normal` | `0` | `"normal"` | `THREAD_PRIORITY_NORMAL`. The default priority for most threads. |
| `AboveNormal` | `1` | `"above normal"` | `THREAD_PRIORITY_ABOVE_NORMAL`. One level above normal. |
| `Highest` | `2` | `"highest"` | `THREAD_PRIORITY_HIGHEST`. Two levels above normal. |
| `TimeCritical` | `15` | `"time critical"` | `THREAD_PRIORITY_TIME_CRITICAL`. The highest regular scheduling priority. Use with extreme caution as it can starve other threads. |

## Methods

### `as_str`

```AffinityServiceRust/src/priority.rs#L186-L191
pub fn as_str(&self) -> &'static str
```

Returns the human-readable string name for this variant (e.g. `"normal"`, `"above normal"`). Returns `"unknown"` if the variant is somehow not found in the internal lookup table (should not happen for valid enum values).

### `as_win_const`

```AffinityServiceRust/src/priority.rs#L193-L195
pub fn as_win_const(&self) -> Option<i32>
```

Returns the Win32 `i32` constant corresponding to this variant, or `None` for the `ThreadPriority::None` sentinel. The returned value is suitable for passing to `SetThreadPriority` (after wrapping in `THREAD_PRIORITY`).

### `from_str`

```AffinityServiceRust/src/priority.rs#L197-L204
pub fn from_str(s: &str) -> Self
```

Parses a case-insensitive string (e.g. `"Above Normal"`, `"idle"`) into the corresponding `ThreadPriority` variant. Returns `ThreadPriority::None` if the string does not match any known priority name.

### `from_win_const`

```AffinityServiceRust/src/priority.rs#L206-L212
pub fn from_win_const(val: i32) -> Self
```

Converts a raw `i32` Win32 thread priority value (e.g. as returned by `GetThreadPriority`) back into the corresponding `ThreadPriority` variant. Returns `ThreadPriority::None` if the value does not match any known constant.

### `boost_one`

```AffinityServiceRust/src/priority.rs#L215-L228
pub fn boost_one(&self) -> Self
```

Returns the next higher priority level in the standard scheduling ladder. Used by the prime-thread engine to elevate a thread's priority by one step when it is promoted to prime status. The mapping is:

| Input | Output |
|-------|--------|
| `Idle` | `Lowest` |
| `Lowest` | `BelowNormal` |
| `BelowNormal` | `Normal` |
| `Normal` | `AboveNormal` |
| `AboveNormal` | `Highest` |
| `Highest` | `Highest` *(capped)* |
| `TimeCritical` | `TimeCritical` *(capped)* |
| `None` | `None` |
| `ErrorReturn` | `ErrorReturn` |
| `ModeBackgroundBegin` | `ModeBackgroundBegin` |
| `ModeBackgroundEnd` | `ModeBackgroundEnd` |

The function caps elevation at `Highest` — it will never promote a thread to `TimeCritical`. The special variants (`None`, `ErrorReturn`, `ModeBackgroundBegin`, `ModeBackgroundEnd`) are returned unchanged.

### `to_thread_priority_struct`

```AffinityServiceRust/src/priority.rs#L230-L232
pub fn to_thread_priority_struct(self) -> THREAD_PRIORITY
```

Converts this enum variant into the `windows::Win32::System::Threading::THREAD_PRIORITY` newtype struct expected by the `windows` crate APIs. Calls `as_win_const()` and wraps the result in `THREAD_PRIORITY(...)`, defaulting to `0` if the variant is `None`.

## Remarks

- The internal lookup table `TABLE` is a `&'static` slice of `(Self, &'static str, Option<i32>)` tuples, ensuring that all conversions are zero-allocation, constant-time linear scans over a small fixed-size array (11 entries).
- The `from_str` method performs a case-insensitive comparison by lowercasing the input before matching. All table entries use lowercase string keys.
- `ModeBackgroundBegin` and `ModeBackgroundEnd` are special Win32 values that can only be applied to the **current** thread. Attempting to set these on a remote thread via `SetThreadPriority` with an arbitrary thread handle will fail with `ERROR_ACCESS_DENIED`. The AffinityServiceRust service does not use these variants for remote threads.
- The `original_priority` field in [`ThreadStats`](../scheduler.rs/ThreadStats.md) stores a `Option<ThreadPriority>` so that the service can snapshot and later restore a thread's scheduling priority.
- `boost_one` is designed to be safe by default — it never escalates into `TimeCritical`, which could cause system instability if applied broadly.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `priority.rs` |
| Callers | Config parser (`config.rs`), apply engine (`apply.rs`), [`ThreadStats`](../scheduler.rs/ThreadStats.md) |
| Callees | None (pure data type with conversion methods) |
| Win32 API | Corresponds to values accepted by `SetThreadPriority` and returned by `GetThreadPriority` |
| Dependencies | `windows::Win32::System::Threading::THREAD_PRIORITY` |
| Privileges | `SeIncreaseBasePriorityPrivilege` may be required to set `TimeCritical` or to boost above `Normal` depending on process priority class |

## See Also

| Reference | Link |
|-----------|------|
| ProcessPriority | [ProcessPriority](ProcessPriority.md) |
| IOPriority | [IOPriority](IOPriority.md) |
| MemoryPriority | [MemoryPriority](MemoryPriority.md) |
| MemoryPriorityInformation | [MemoryPriorityInformation](MemoryPriorityInformation.md) |
| ThreadStats | [ThreadStats](../scheduler.rs/ThreadStats.md) |
| priority module overview | [README](README.md) |

---
*Commit: [37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
