# IdealProcessorState struct (scheduler.rs)

Tracks the ideal processor assignment state for a single thread. Stores both the current and previous processor group/number pair, enabling the service to detect changes and avoid redundant `SetThreadIdealProcessorEx` calls across polling iterations.

## Syntax

```rust
#[derive(Debug, Clone, Copy)]
pub struct IdealProcessorState {
    pub current_group: u16,
    pub current_number: u8,
    pub previous_group: u16,
    pub previous_number: u8,
    pub is_assigned: bool,
}
```

## Members

| Member | Type | Description |
|--------|------|-------------|
| `current_group` | `u16` | The processor group of the currently assigned ideal processor. On systems with a single processor group (≤64 logical processors), this is always `0`. |
| `current_number` | `u8` | The logical processor number within `current_group` that is currently set as the thread's ideal processor. |
| `previous_group` | `u16` | The processor group of the ideal processor from the prior polling iteration. Used to detect whether the assignment has changed. |
| `previous_number` | `u8` | The logical processor number within `previous_group` from the prior polling iteration. |
| `is_assigned` | `bool` | `true` if this thread has been explicitly assigned an ideal processor by the service. When `false`, the thread retains its OS-default ideal processor. Used by [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) to distinguish managed threads from unmanaged ones. |

## Methods

### new

```rust
pub fn new() -> Self
```

Creates a new `IdealProcessorState` with all fields zeroed and `is_assigned` set to `false`.

**Return value**

`IdealProcessorState` — A default-initialized state representing an unassigned thread.

### Default

```rust
impl Default for IdealProcessorState {
    fn default() -> Self
}
```

Delegates to `IdealProcessorState::new()`.

## Remarks

- The `current_*` / `previous_*` split enables change detection without querying the OS. On each apply cycle, the caller writes new values into `current_group` and `current_number`, then compares against `previous_group` and `previous_number` to decide whether a `SetThreadIdealProcessorEx` call is necessary.
- After a successful update, the caller copies `current_*` into `previous_*` for the next iteration.
- The struct derives `Copy` because it is a small, stack-only value type (10 bytes) with no heap allocations or resource ownership.
- The `is_assigned` flag is checked by `select_top_threads_with_hysteresis` via its `is_currently_assigned` callback to determine whether a thread is already occupying a prime slot. This is a key input to the hysteresis logic that prevents promotion/demotion thrashing.

### Relationship to Windows PROCESSOR_NUMBER

The `current_group`/`current_number` pair maps directly to the Windows `PROCESSOR_NUMBER` structure used by [SetThreadIdealProcessorEx](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadidealprocessorex) and [GetThreadIdealProcessorEx](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadidealprocessorex). The service's wrappers for these APIs are in [set_thread_ideal_processor_ex](../winapi.rs/set_thread_ideal_processor_ex.md) and [get_thread_ideal_processor_ex](../winapi.rs/get_thread_ideal_processor_ex.md).

## Requirements

| | |
|---|---|
| **Module** | `src/scheduler.rs` |
| **Callers** | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md), [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md), [apply_prime_threads_demote](../apply.rs/apply_prime_threads_demote.md) |
| **Dependencies** | None (plain data struct) |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [scheduler.rs](README.md) |
| Parent struct | [ThreadStats](ThreadStats.md) |
| Ideal processor set wrapper | [set_thread_ideal_processor_ex](../winapi.rs/set_thread_ideal_processor_ex.md) |
| Ideal processor get wrapper | [get_thread_ideal_processor_ex](../winapi.rs/get_thread_ideal_processor_ex.md) |
| Thread selection with hysteresis | [PrimeThreadScheduler](PrimeThreadScheduler.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*