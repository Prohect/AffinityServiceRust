# enumerate_process_modules function (winapi.rs)

Enumerates all loaded modules for a target process, returning a vector of base address, end address, and module name tuples.

## Syntax

```rust
pub fn enumerate_process_modules(pid: u32) -> Vec<(usize, usize, String)>
```

## Parameters

`pid`

The process identifier of the target process whose loaded modules should be enumerated.

## Return value

Returns a `Vec<(usize, usize, String)>` where each tuple represents one loaded module:

- `.0` (`usize`) — the base address of the module in the process's virtual address space.
- `.1` (`usize`) — the end address of the module (base address + module size).
- `.2` (`String`) — the file name of the module (e.g., `"ntdll.dll"`, `"game.exe"`). This is the file name only, not the full path.

Returns an empty vector if the process cannot be opened, if module enumeration fails, or if the process has exited.

## Remarks

This function performs the following steps:

1. Opens the target process with `PROCESS_QUERY_INFORMATION | PROCESS_VM_READ` access via `OpenProcess`.
2. Calls `EnumProcessModulesEx` with `LIST_MODULES_ALL` to retrieve handles for all loaded modules (both 32-bit and 64-bit).
3. For each module handle, calls `GetModuleInformation` to obtain the base address and size, and `GetModuleFileNameExW` to obtain the module name.
4. Extracts the file name component from the full module path.
5. Collects the results into a vector of `(base, base + size, name)` tuples.

The returned data is typically cached in [`MODULE_CACHE`](MODULE_CACHE.md) by [`resolve_address_to_module`](resolve_address_to_module.md) to avoid repeating this expensive enumeration on every address resolution call. Direct callers of this function should be aware that it performs multiple Windows API calls per module and may be slow for processes with many loaded DLLs.

### Error handling

If `OpenProcess` fails (e.g., access denied for protected processes, or the process has already exited), the function returns an empty vector without logging an error. Module enumeration is considered best-effort — the caller (typically [`resolve_address_to_module`](resolve_address_to_module.md)) handles missing module data gracefully by falling back to raw address formatting.

If `EnumProcessModulesEx` or individual module queries fail, the function returns whatever modules were successfully enumerated up to that point.

### Access requirements

The function requires `PROCESS_QUERY_INFORMATION | PROCESS_VM_READ` access to the target process. These are relatively high-privilege access rights — without [`SeDebugPrivilege`](enable_debug_privilege.md) enabled, enumeration will fail for processes owned by other users or running at higher integrity levels.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/winapi.rs |
| **Source lines** | L767–L820 |
| **Called by** | [`resolve_address_to_module`](resolve_address_to_module.md) |
| **Populates** | [`MODULE_CACHE`](MODULE_CACHE.md) (indirectly, via caller) |
| **Windows API** | [OpenProcess](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess), [EnumProcessModulesEx](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-enumprocessmodulesex), [GetModuleInformation](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getmoduleinformation), [GetModuleFileNameExW](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getmodulefilenameexw) |

## See also

- [MODULE_CACHE static](MODULE_CACHE.md)
- [resolve_address_to_module](resolve_address_to_module.md)
- [drop_module_cache](drop_module_cache.md)
- [get_thread_start_address](get_thread_start_address.md)
- [winapi.rs module overview](README.md)