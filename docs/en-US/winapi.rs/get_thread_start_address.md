# get_thread_start_address function (winapi.rs)

Retrieves the start address of a thread via `NtQueryInformationThread`. This address identifies the entry point function where the thread began execution, and is used to determine which module a thread belongs to for module-based ideal processor assignment.

## Syntax

```rust
pub fn get_thread_start_address(thread_handle: HANDLE) -> usize
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `thread_handle` | `HANDLE` | A valid thread handle opened with at least `THREAD_QUERY_LIMITED_INFORMATION` access. Typically the `r_limited_handle` or `r_handle` field from a [`ThreadHandle`](ThreadHandle.md). |

## Return value

Returns the start address of the thread as a `usize`. If the query fails (i.e., `NtQueryInformationThread` returns a non-success `NTSTATUS`), returns `0`.

## Remarks

- This function calls `NtQueryInformationThread` with information class `9` (`ThreadQuerySetWin32StartAddress`) to retrieve the Win32 start address of the thread. This is the address passed to `CreateThread` or similar thread-creation APIs, not necessarily the current instruction pointer.

- The returned address can be passed to [`resolve_address_to_module`](resolve_address_to_module.md) to determine which loaded module (DLL or EXE) owns that address, enabling module-aware thread-to-core assignment policies.

- A return value of `0` indicates either a query failure or that the thread has no recorded start address. Callers should treat `0` as an unknown/unresolvable address.

- The function does **not** log errors on failure. It silently returns `0`, leaving it to the caller to decide whether the failure is significant.

- The `NtQueryInformationThread` function is an undocumented (but stable) NTDLL export linked via the `#[link(name = "ntdll")]` extern block at the top of `winapi.rs`.

### Platform notes

- **Windows only.** `NtQueryInformationThread` is an NT-native API not available on other platforms.
- The start address is a virtual memory address in the target thread's process address space. It is only meaningful when combined with module base address information from the same process.
- For threads created by the CRT or runtime libraries, the start address may point to a runtime wrapper (e.g., `KERNEL32!BaseThreadInitThunk`) rather than the user-supplied thread function.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `winapi.rs` |
| **Callers** | `apply.rs` — ideal processor assignment logic |
| **Callees** | `NtQueryInformationThread` (ntdll, information class `9`) |
| **API** | NT Native API — `NtQueryInformationThread` |
| **Privileges** | Requires a valid thread handle with query access. `SeDebugPrivilege` may be needed for threads in other sessions. |

## See Also

| Topic | Link |
|-------|------|
| resolve_address_to_module | [resolve_address_to_module](resolve_address_to_module.md) |
| ThreadHandle struct | [ThreadHandle](ThreadHandle.md) |
| get_thread_handle | [get_thread_handle](get_thread_handle.md) |
| set_thread_ideal_processor_ex | [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) |
| get_thread_ideal_processor_ex | [get_thread_ideal_processor_ex](get_thread_ideal_processor_ex.md) |
| MODULE_CACHE static | [MODULE_CACHE](MODULE_CACHE.md) |

---
*Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
