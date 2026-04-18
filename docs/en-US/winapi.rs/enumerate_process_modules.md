# enumerate_process_modules function (winapi.rs)

Enumerates all loaded modules (DLLs and the main executable) for a given process, returning each module's base address, size, and name. This is an internal helper used by [`resolve_address_to_module`](resolve_address_to_module.md) to build the per-process module map cached in [`MODULE_CACHE`](MODULE_CACHE.md).

## Syntax

```rust
fn enumerate_process_modules(pid: u32) -> Vec<(usize, usize, String)>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The process identifier of the target process whose loaded modules should be enumerated. |

## Return value

Returns a `Vec<(usize, usize, String)>` where each tuple element represents:

| Index | Type | Description |
|-------|------|-------------|
| `.0` | `usize` | The base address of the module in the target process's virtual address space (`MODULEINFO.lpBaseOfDll`). |
| `.1` | `usize` | The size of the module image in bytes (`MODULEINFO.SizeOfImage`). |
| `.2` | `String` | The base name of the module (e.g., `kernel32.dll`), obtained via `GetModuleBaseNameW`. |

Returns an empty `Vec` if:

- The process could not be opened with the required access rights.
- The opened handle is invalid.
- `EnumProcessModulesEx` fails (e.g., the process is protected or 32-bit/64-bit mismatch).

## Remarks

### Algorithm

1. **Open the process** — Calls `OpenProcess` with `PROCESS_QUERY_INFORMATION | PROCESS_VM_READ` access. Both rights are required: `PROCESS_QUERY_INFORMATION` for module enumeration and `PROCESS_VM_READ` for reading module names and information from the target process's memory.

2. **Enumerate modules** — Calls `EnumProcessModulesEx` with `LIST_MODULES_ALL` to retrieve handles (HMODULEs) for all loaded modules (both 32-bit and 64-bit). The function uses a fixed-size array of 1024 `HMODULE` entries as the output buffer, which is sufficient for virtually all real-world processes.

3. **Gather module info** — For each returned `HMODULE`:
   - Calls `GetModuleInformation` to obtain the `MODULEINFO` struct (base address and image size).
   - Calls `GetModuleBaseNameW` to obtain the module's file name as a UTF-16 string, then converts it to a Rust `String` via `String::from_utf16_lossy`.
   - If either call fails for a given module, that module is silently skipped.

4. **Clean up** — The process handle is closed via `CloseHandle` before returning, regardless of success or failure.

### Capacity limits

The function allocates a stack array of 1024 `HMODULE` entries. If a process has more than 1024 loaded modules, only the first 1024 are enumerated. In practice, even very large applications (e.g., web browsers, game engines) rarely exceed a few hundred modules.

### Module name buffer

Module names are read into a fixed `[u16; 260]` buffer (matching `MAX_PATH`). Module names longer than 260 characters are truncated.

### Error handling

This function does not log errors. If the process cannot be opened or module enumeration fails, it returns an empty vector silently. Error reporting is left to the caller ([`resolve_address_to_module`](resolve_address_to_module.md)), which falls back to formatting the raw address as a hex string.

### Platform notes

- **Windows only.** Uses `EnumProcessModulesEx`, `GetModuleInformation`, and `GetModuleBaseNameW` from `psapi` (linked via the `windows` crate's `Win32::System::ProcessStatus` module).
- `LIST_MODULES_ALL` ensures that both 32-bit and 64-bit modules are included in the results, which is relevant when a WoW64 process is being inspected from a 64-bit host.
- The function opens a **new** process handle each time it is called. Results are cached externally by [`MODULE_CACHE`](MODULE_CACHE.md) to avoid redundant handle opens and module enumerations.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `winapi.rs` |
| **Visibility** | Private (module-internal, no `pub`) |
| **Callers** | [`resolve_address_to_module`](resolve_address_to_module.md) (via [`MODULE_CACHE`](MODULE_CACHE.md) population) |
| **Callees** | `OpenProcess`, `EnumProcessModulesEx`, `GetModuleInformation`, `GetModuleBaseNameW`, `CloseHandle` (Win32 API) |
| **Win32 API** | [EnumProcessModulesEx](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-enumprocessmodulesexw), [GetModuleInformation](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getmoduleinformation), [GetModuleBaseNameW](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getmodulebasenamew) |
| **Access rights** | `PROCESS_QUERY_INFORMATION \| PROCESS_VM_READ` |
| **Privileges** | `SeDebugPrivilege` recommended for opening protected or elevated processes. |

## See Also

| Topic | Link |
|-------|------|
| resolve_address_to_module | [resolve_address_to_module](resolve_address_to_module.md) |
| drop_module_cache | [drop_module_cache](drop_module_cache.md) |
| MODULE_CACHE static | [MODULE_CACHE](MODULE_CACHE.md) |
| get_thread_start_address | [get_thread_start_address](get_thread_start_address.md) |
| get_process_handle | [get_process_handle](get_process_handle.md) |
| winapi module overview | [README](README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
