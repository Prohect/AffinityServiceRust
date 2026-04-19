# enumerate_process_modules function (winapi.rs)

Enumerates all loaded modules in a target process, returning each module's base address, size, and name. Used internally by [resolve_address_to_module](resolve_address_to_module.md) to populate the per-PID module cache for address-to-module resolution.

## Syntax

```rust
fn enumerate_process_modules(pid: u32) -> Vec<(usize, usize, String)>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The process identifier of the target process whose modules should be enumerated. |

## Return value

`Vec<(usize, usize, String)>` — A vector of tuples where each element represents a loaded module:

| Tuple index | Type | Description |
|-------------|------|-------------|
| `.0` | `usize` | Base address of the module in the target process's virtual address space (`MODULEINFO::lpBaseOfDll`). |
| `.1` | `usize` | Size of the module image in bytes (`MODULEINFO::SizeOfImage`). |
| `.2` | `String` | The base name of the module (e.g., `"kernel32.dll"`, `"ntdll.dll"`), obtained via `GetModuleBaseNameW`. |

Returns an empty `Vec` if:
- The process cannot be opened (e.g., access denied, invalid PID).
- The process handle is invalid after opening.
- `EnumProcessModulesEx` fails (e.g., 32-bit process querying a 64-bit process without WOW64 support).

## Remarks

### Implementation steps

1. **Open process** — Opens the target process with `PROCESS_QUERY_INFORMATION | PROCESS_VM_READ` via `OpenProcess`. Both access rights are required: `PROCESS_QUERY_INFORMATION` for module enumeration, and `PROCESS_VM_READ` for reading module names from the target process's address space.

2. **Enumerate modules** — Calls `EnumProcessModulesEx` with `LIST_MODULES_ALL` to retrieve module handles for both 32-bit and 64-bit modules. The function uses a fixed-size array of 1024 `HMODULE` slots, which is sufficient for the vast majority of processes.

3. **Query each module** — For each returned module handle:
   - `GetModuleInformation` retrieves the `MODULEINFO` structure containing `lpBaseOfDll` (base address) and `SizeOfImage` (module size).
   - `GetModuleBaseNameW` retrieves the module's base name as a UTF-16 string, which is converted to a Rust `String` via `String::from_utf16_lossy`.
   - Modules where either call fails are silently skipped.

4. **Cleanup** — The process handle is closed via `CloseHandle` before returning, on both success and early-exit paths.

### Module limit

The function allocates space for 1024 module handles on the stack. If a process has more than 1024 loaded modules, only the first 1024 are returned. In practice, even complex applications rarely exceed a few hundred modules.

### Visibility

This function is module-private (`fn`, not `pub fn`) and is only called by [resolve_address_to_module](resolve_address_to_module.md) during module cache population. External code should use `resolve_address_to_module` instead of calling this function directly.

### Error handling

All Win32 API failures are handled by returning an empty result or skipping the failing module — no errors are logged or propagated. This is intentional because module enumeration is a best-effort diagnostic feature; failure does not affect the service's core functionality.

### Access requirements

The combination of `PROCESS_QUERY_INFORMATION | PROCESS_VM_READ` is more restrictive than the limited-access handles used elsewhere in the service. This means module enumeration may fail for processes where [get_process_handle](get_process_handle.md) succeeds with limited access. [SeDebugPrivilege](enable_debug_privilege.md) typically resolves access issues for most processes.

## Requirements

| | |
|---|---|
| **Module** | `src/winapi.rs` |
| **Callers** | [resolve_address_to_module](resolve_address_to_module.md) (via [MODULE_CACHE](README.md) population) |
| **Callees** | `OpenProcess`, `EnumProcessModulesEx`, `GetModuleInformation`, `GetModuleBaseNameW`, `CloseHandle` (Win32) |
| **Win32 API** | [OpenProcess](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess), [EnumProcessModulesEx](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-enumprocessmodulesex), [GetModuleInformation](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getmoduleinformation), [GetModuleBaseNameW](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getmodulebasenamew), [CloseHandle](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle) |
| **Privileges** | `PROCESS_QUERY_INFORMATION` and `PROCESS_VM_READ` on the target process; [SeDebugPrivilege](enable_debug_privilege.md) recommended |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [winapi.rs](README.md) |
| Address resolution consumer | [resolve_address_to_module](resolve_address_to_module.md) |
| Module cache cleanup | [drop_module_cache](drop_module_cache.md) |
| Thread start address query | [get_thread_start_address](get_thread_start_address.md) |
| ThreadStats that stores start_address | [ThreadStats](../scheduler.rs/ThreadStats.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*