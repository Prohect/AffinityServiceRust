# get_thread_start_address function (winapi.rs)

Retrieves the Win32 start address of a thread via `NtQueryInformationThread` with the `ThreadQuerySetWin32StartAddress` information class (class 9). The start address identifies which function the thread was created to execute, enabling module-based ideal processor assignment and diagnostic reporting.

## Syntax

```rust
pub fn get_thread_start_address(thread_handle: HANDLE) -> usize
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `thread_handle` | `HANDLE` | A valid thread handle with at least `THREAD_QUERY_INFORMATION` access. Typically the `r_handle` field from a [ThreadHandle](ThreadHandle.md). Using `r_limited_handle` (`THREAD_QUERY_LIMITED_INFORMATION`) is **not** sufficient for this query — `NtQueryInformationThread` with class 9 requires full query rights. |

## Return value

`usize` — The virtual address at which the thread's entry point resides in the process's address space. Returns `0` if the query fails (e.g., the handle lacks sufficient access or the thread has exited).

## Remarks

### NT API details

The function calls the undocumented `NtQueryInformationThread` FFI binding (linked from `ntdll.dll`) with:

| Parameter | Value |
|-----------|-------|
| `thread_handle` | Passed through from the caller |
| `thread_information_class` | `9` (`ThreadQuerySetWin32StartAddress`) |
| `thread_information` | Pointer to a `usize` output buffer |
| `thread_information_length` | `size_of::<usize>()` (8 bytes on 64-bit) |
| `return_length` | Pointer to a `u32` (unused after the call) |

The function checks `NTSTATUS.is_ok()` on the return value. If the status indicates success, the start address is returned; otherwise, `0` is returned.

### Start address vs. entry point

The Win32 start address returned by this information class is the address of the function passed to `CreateThread` / `_beginthreadex` (or equivalent). This may differ from the actual thread entry point seen by the kernel (`RtlUserThreadStart`), which performs CRT initialization before jumping to the user-specified start function.

### Usage in the service

The start address is used in two key scenarios:

1. **Module-based ideal processor assignment** — The address is passed to [resolve_address_to_module](resolve_address_to_module.md) to determine which module (DLL/EXE) the thread belongs to. Module-prefix matching rules in the configuration then assign specific ideal processors to threads within matching modules.

2. **Diagnostic reporting** — The [ThreadStats](../scheduler.rs/ThreadStats.md) `Debug` implementation and the process exit report in [PrimeThreadScheduler::drop_process_by_pid](../scheduler.rs/PrimeThreadScheduler.md) both resolve the start address to a `"module.dll+0xOffset"` string for human-readable thread identification.

### Failure cases

The function returns `0` in the following situations:

- The thread handle does not have `THREAD_QUERY_INFORMATION` access (only limited access was obtained).
- The thread has already exited and the handle is stale.
- The NT API call returns an unexpected `NTSTATUS` error.

A return value of `0` is treated as "unknown" by downstream consumers. [resolve_address_to_module](resolve_address_to_module.md) returns the string `"0x0"` for a zero address.

### Handle requirements

The `r_handle` field of [ThreadHandle](ThreadHandle.md) (opened with `THREAD_QUERY_INFORMATION`) must be used, not `r_limited_handle`. If `r_handle` is invalid (access was denied during thread handle acquisition), this function will return `0`. The caller should check `r_handle.is_invalid()` before calling this function to avoid a wasted syscall.

## Requirements

| | |
|---|---|
| **Module** | `src/winapi.rs` |
| **Callers** | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md), [prefetch_all_thread_cycles](../apply.rs/prefetch_all_thread_cycles.md), [ThreadStats](../scheduler.rs/ThreadStats.md) |
| **Callees** | `NtQueryInformationThread` (ntdll FFI) |
| **NT API** | `NtQueryInformationThread` with `ThreadQuerySetWin32StartAddress` (class 9) |
| **Privileges** | Requires `THREAD_QUERY_INFORMATION` access on the thread handle; [SeDebugPrivilege](enable_debug_privilege.md) may be needed to obtain that handle for protected processes |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [winapi.rs](README.md) |
| Address-to-module resolution | [resolve_address_to_module](resolve_address_to_module.md) |
| Thread handle management | [ThreadHandle](ThreadHandle.md), [get_thread_handle](get_thread_handle.md) |
| Thread stats that store the start address | [ThreadStats](../scheduler.rs/ThreadStats.md) |
| Ideal processor assignment | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*