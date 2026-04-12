# get_thread_start_address function (winapi.rs)

Queries the start address of a thread using `NtQueryInformationThread`, returning the memory address where the thread's entry point function resides.

## Syntax

```rust
pub fn get_thread_start_address(thread_handle: HANDLE) -> usize
```

## Parameters

`thread_handle`

A valid thread `HANDLE` opened with at least `THREAD_QUERY_INFORMATION` access. This corresponds to the `r_handle` field of a [`ThreadHandle`](ThreadHandle.md).

## Return value

Returns a `usize` representing the virtual memory address of the thread's start routine. Returns `0` if the query fails or the thread handle is invalid.

## Remarks

This function calls the NT-layer API `NtQueryInformationThread` with the `ThreadQuerySetWin32StartAddress` information class to retrieve the address of the function that was passed to `CreateThread` (or equivalent) when the thread was created.

The returned address is used by the ideal processor assignment system in [`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md) to determine which module a thread originated from. The address is subsequently passed to [`resolve_address_to_module`](resolve_address_to_module.md), which maps it to a `"module.dll+0xABC"` formatted string. This string is then matched against the prefix rules defined in [`IdealProcessorRule`](../config.rs/IdealProcessorRule.md) to assign threads to specific processors based on their originating module.

The start address is queried once per thread lifetime and cached in [`ThreadStats.start_address`](../scheduler.rs/ThreadStats.md) to avoid redundant system calls on subsequent loop iterations.

### Limitations

- The start address may not always reflect the actual current instruction pointer of the thread — it is the *original* entry point, not the current execution location.
- For threads created by the runtime (e.g., thread pool threads), the start address typically points to a generic runtime stub rather than user code.
- If the thread handle lacks `THREAD_QUERY_INFORMATION` access, the function returns `0`.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/winapi.rs |
| **Source lines** | L637–L656 |
| **Called by** | [`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md), [`prefetch_all_thread_cycles`](../apply.rs/prefetch_all_thread_cycles.md) |
| **Calls** | `NtQueryInformationThread` |
| **Windows API** | [NtQueryInformationThread](https://learn.microsoft.com/en-us/windows/win32/api/winternl/nf-winternl-ntqueryinformationthread) |

## See also

- [resolve_address_to_module](resolve_address_to_module.md)
- [ThreadHandle struct](ThreadHandle.md)
- [apply_ideal_processors](../apply.rs/apply_ideal_processors.md)
- [winapi.rs module overview](README.md)