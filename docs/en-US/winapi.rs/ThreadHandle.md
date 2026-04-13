# ThreadHandle struct (winapi.rs)

RAII container that holds up to four Windows thread handles opened at different access levels. The `r_limited_handle` field is always valid (its successful acquisition is required during construction); the remaining three handles are attempted but may hold invalid sentinel values if the corresponding `OpenThread` call failed. When the `ThreadHandle` is dropped, all valid handles are automatically closed via `CloseHandle`.

## Syntax

```rust
#[derive(Debug)]
pub struct ThreadHandle {
    pub r_limited_handle: HANDLE,
    pub r_handle: HANDLE,
    pub w_limited_handle: HANDLE,
    pub w_handle: HANDLE,
}
```

## Members

| Member | Type | Access right | Description |
|--------|------|-------------|-------------|
| `r_limited_handle` | `HANDLE` | `THREAD_QUERY_LIMITED_INFORMATION` | Always valid. Required handle — if this open fails, [get_thread_handle](get_thread_handle.md) returns `None` rather than constructing a `ThreadHandle`. Sufficient for lightweight queries such as `QueryThreadCycleTime`. |
| `r_handle` | `HANDLE` | `THREAD_QUERY_INFORMATION` | Valid or `HANDLE::default()` (invalid sentinel). Required for `NtQueryInformationThread` calls such as [get_thread_start_address](get_thread_start_address.md) and for [get_thread_ideal_processor_ex](get_thread_ideal_processor_ex.md). |
| `w_limited_handle` | `HANDLE` | `THREAD_SET_LIMITED_INFORMATION` | Valid or `HANDLE::default()` (invalid sentinel). Required for `SetThreadSelectedCpuSets` when pinning prime threads to specific CPU sets. |
| `w_handle` | `HANDLE` | `THREAD_SET_INFORMATION` | Valid or `HANDLE::default()` (invalid sentinel). Required for [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) and `SetThreadPriority`. |

## Remarks

### Handle validity model

Unlike [ProcessHandle](ProcessHandle.md), which uses `Option<HANDLE>` for its non-required handles, `ThreadHandle` stores raw `HANDLE` values and relies on `HANDLE::is_invalid()` to distinguish success from failure. This design reflects the fact that thread handles are created in bulk (one per thread per process), so the lighter representation reduces allocation pressure.

Callers should check `is_invalid()` before using `r_handle`, `w_limited_handle`, or `w_handle`:

```rust
if !thread_handle.w_handle.is_invalid() {
    set_thread_ideal_processor_ex(thread_handle.w_handle, group, number)?;
}
```

### Drop behavior

The `Drop` implementation closes each handle individually, skipping any that are invalid:

1. `r_limited_handle` — always closed (always valid).
2. `r_handle` — closed only if `!is_invalid()`.
3. `w_limited_handle` — closed only if `!is_invalid()`.
4. `w_handle` — closed only if `!is_invalid()`.

Each `CloseHandle` call is wrapped in `unsafe` and its return value is discarded, consistent with the Windows convention that closing a valid handle cannot meaningfully fail.

### Typical lifetime

`ThreadHandle` instances are typically stored inside [ThreadStats](../scheduler.rs/ThreadStats.md) for the duration of a process's tracked lifetime. They are created by [get_thread_handle](get_thread_handle.md) when a thread is first encountered by the prime-thread scheduler, and dropped when the thread exits or the owning `ProcessStats` is removed.

### Access level rationale

The four access levels cover the full range of thread operations AffinityServiceRust performs:

| Access right | Used for |
|-------------|----------|
| `THREAD_QUERY_LIMITED_INFORMATION` | `QueryThreadCycleTime` |
| `THREAD_QUERY_INFORMATION` | `NtQueryInformationThread` (start address), `GetThreadIdealProcessorEx` |
| `THREAD_SET_LIMITED_INFORMATION` | `SetThreadSelectedCpuSets` |
| `THREAD_SET_INFORMATION` | `SetThreadIdealProcessorEx`, `SetThreadPriority` |

Protected processes may deny the full-access variants while still granting limited access, so splitting the handles allows partial functionality even when elevated rights are insufficient for full control.

## Requirements

| | |
|---|---|
| **Module** | `winapi` (`src/winapi.rs`) |
| **Constructed by** | [get_thread_handle](get_thread_handle.md) |
| **Stored in** | [ThreadStats](../scheduler.rs/ThreadStats.md) (field `handle`) |
| **Consumed by** | [apply_prime_threads](../apply.rs/apply_prime_threads.md), [apply_ideal_processors](../apply.rs/apply_ideal_processors.md), [prefetch_all_thread_cycles](../apply.rs/prefetch_all_thread_cycles.md) |
| **API** | `OpenThread`, `CloseHandle` (Win32 Threading) |
| **Privileges** | `SeDebugPrivilege` recommended for cross-process thread access |

## See Also

| Topic | Link |
|-------|------|
| Thread handle constructor | [get_thread_handle](get_thread_handle.md) |
| Single-access thread open helper | [try_open_thread](try_open_thread.md) |
| Process handle counterpart | [ProcessHandle](ProcessHandle.md) |
| Thread statistics storage | [ThreadStats](../scheduler.rs/ThreadStats.md) |
| Thread start address query | [get_thread_start_address](get_thread_start_address.md) |
| Ideal processor setter | [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) |
| OpenThread (MSDN) | [Microsoft Learn — OpenThread](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openthread) |