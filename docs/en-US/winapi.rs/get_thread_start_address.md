# get_thread_start_address function (winapi.rs)

Queries the start address of a thread by calling `NtQueryInformationThread` with the `ThreadQuerySetWin32StartAddress` information class (9). The start address identifies which function — and by extension which module — the thread was created to execute, enabling module-based thread classification for ideal processor and prime-thread scheduling.

## Syntax

```rust
pub fn get_thread_start_address(thread_handle: HANDLE) -> usize
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `thread_handle` | `HANDLE` | A valid thread handle opened with at least `THREAD_QUERY_INFORMATION` access. This is typically the `r_handle` field from a [ThreadHandle](ThreadHandle.md). Using `r_limited_handle` (`THREAD_QUERY_LIMITED_INFORMATION`) is **not** sufficient — `NtQueryInformationThread` with information class 9 requires the full query right. |

## Return value

| Value | Meaning |
|-------|---------|
| Non-zero `usize` | The virtual memory address where the thread's entry-point function resides. This is the address passed to `CreateThread` / `CreateRemoteThread` (or equivalent) when the thread was created. |
| `0` | The query failed. This can happen if the handle lacks `THREAD_QUERY_INFORMATION` access, the thread has already exited, or the `NtQueryInformationThread` call returns a failing `NTSTATUS`. |

## Remarks

### Information class

The function uses information class `9`, which corresponds to `ThreadQuerySetWin32StartAddress`. This is an undocumented but stable information class supported on all modern Windows versions (Windows 7 and later). It returns the Win32 start address of the thread, which is the address of the user-supplied thread procedure (e.g., the function pointer passed to `CreateThread`).

### Output format

The output is a raw pointer-sized value (`usize`). On 64-bit Windows, this is an 8-byte address. On 32-bit Windows, it would be a 4-byte address. The function passes `size_of::<usize>()` as the output length to `NtQueryInformationThread`, ensuring portability across pointer widths.

### Module resolution

The returned address is typically passed to [resolve_address_to_module](resolve_address_to_module.md), which maps it to a module name plus offset string such as `"engine.dll+0x1A30"`. This module name is then used by the prime-thread scheduler and ideal processor rules to match threads against [PrimePrefix](../config.rs/PrimePrefix.md) and [IdealProcessorRule](../config.rs/IdealProcessorRule.md) configurations.

### Failure handling

The function silently returns `0` on failure. The caller should treat `0` as "unknown start address". The [resolve_address_to_module](resolve_address_to_module.md) function handles a `0` address by returning the string `"0x0"`, so the failure propagates gracefully through the module resolution pipeline.

### Typical call sequence

```
thread_handle = get_thread_handle(tid, pid, process_name)
start_address = get_thread_start_address(thread_handle.r_handle)
module_name = resolve_address_to_module(pid, start_address)
// Use module_name for prefix matching
```

### NtQueryInformationThread internals

The function performs a single unsafe FFI call:

```rust
NtQueryInformationThread(
    thread_handle,
    9,                                          // ThreadQuerySetWin32StartAddress
    &mut start_address as *mut _ as *mut c_void,
    size_of::<usize>() as u32,
    &mut return_len,
)
```

The `return_len` output parameter receives the number of bytes written but is not checked — only the `NTSTATUS` return value is inspected. If the status is success (non-negative), the `start_address` value is returned; otherwise `0` is returned.

### Thread start address vs. instruction pointer

The start address is **not** the thread's current instruction pointer (`RIP`/`EIP`). It is the address of the function the thread was created with. This value remains constant for the lifetime of the thread, even as the thread executes through different functions and modules. This stability makes it suitable as a persistent thread identifier for classification purposes.

## Requirements

| | |
|---|---|
| **Module** | `winapi` (`src/winapi.rs`) |
| **Visibility** | `pub` |
| **Callers** | [prefetch_all_thread_cycles](../apply.rs/prefetch_all_thread_cycles.md), [apply_ideal_processors](../apply.rs/apply_ideal_processors.md), [ThreadStats](../scheduler.rs/ThreadStats.md) initialization |
| **Callees** | `NtQueryInformationThread` (ntdll.dll FFI) |
| **API** | `NtQueryInformationThread` with `ThreadQuerySetWin32StartAddress` (information class 9) |
| **Privileges** | Requires `THREAD_QUERY_INFORMATION` access on the thread handle; `SeDebugPrivilege` may be needed for cross-process threads |

## See Also

| Topic | Link |
|-------|------|
| Module address resolution | [resolve_address_to_module](resolve_address_to_module.md) |
| Module cache management | [MODULE_CACHE](MODULE_CACHE.md) |
| Thread handle container | [ThreadHandle](ThreadHandle.md) |
| Thread handle acquisition | [get_thread_handle](get_thread_handle.md) |
| Prime-thread prefix matching | [PrimePrefix](../config.rs/PrimePrefix.md) |
| Ideal processor rule matching | [IdealProcessorRule](../config.rs/IdealProcessorRule.md) |
| Thread statistics (stores start address) | [ThreadStats](../scheduler.rs/ThreadStats.md) |
| NtQueryInformationThread (unofficial docs) | [ntdll undocumented functions](https://learn.microsoft.com/en-us/windows/win32/api/winternl/) |