# resolve_address_to_module function (winapi.rs)

Resolves a memory address within a process to a human-readable string in `"module.dll+0xABC"` format by looking up the address against the process's loaded module list.

## Syntax

```rust
pub fn resolve_address_to_module(pid: u32, address: usize) -> String
```

## Parameters

`pid`

The process identifier of the target process whose module list should be searched.

`address`

The memory address to resolve, typically a thread start address obtained from [`get_thread_start_address`](get_thread_start_address.md).

## Return value

Returns a `String` in the format `"module.dll+0xABC"` where `module.dll` is the name of the module containing the address and `0xABC` is the hexadecimal offset from the module's base address. If the address does not fall within any known module, a fallback string representation of the raw address is returned.

## Remarks

This function is used to identify which module (DLL or EXE) a thread's start address belongs to. This information is critical for the ideal processor assignment feature, which matches threads to CPU cores based on the module prefix of their start address (see [`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md)).

The function uses the [`MODULE_CACHE`](MODULE_CACHE.md) static to avoid repeatedly enumerating a process's modules on every call. The resolution flow is:

1. Lock [`MODULE_CACHE`](MODULE_CACHE.md) and check whether the given `pid` already has a cached module list.
2. If not cached, call [`enumerate_process_modules`](enumerate_process_modules.md) to populate the cache entry for this process.
3. Search the cached module list for an entry whose base–end address range contains the target `address`.
4. If found, compute the offset (`address - base`) and format the result as `"module_name+0xOFFSET"`.
5. If no module range contains the address, return a raw address representation.

The module cache is per-process and persists across loop iterations for performance. When a process exits, [`drop_module_cache`](drop_module_cache.md) should be called to free the stale entry.

### Output format

The output format `"module.dll+0xABC"` is designed to be both human-readable in log files and useful for pattern matching in configuration rules. The module name is extracted from the full path (e.g., `ntdll.dll` rather than `C:\Windows\System32\ntdll.dll`), and the offset uses lowercase hexadecimal with a `0x` prefix.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/winapi.rs |
| **Source lines** | L682–L708 |
| **Called by** | [`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md), [`prefetch_all_thread_cycles`](../apply.rs/prefetch_all_thread_cycles.md) |
| **Calls** | [`enumerate_process_modules`](enumerate_process_modules.md) |
| **Uses** | [`MODULE_CACHE`](MODULE_CACHE.md) |

## See also

- [MODULE_CACHE static](MODULE_CACHE.md)
- [enumerate_process_modules](enumerate_process_modules.md)
- [drop_module_cache](drop_module_cache.md)
- [get_thread_start_address](get_thread_start_address.md)
- [winapi.rs module overview](README.md)