# enumerate_process_modules function (winapi.rs)

Enumerates all loaded modules (DLLs and the main executable) in a target process, returning each module's base address, image size, and base name. This is the underlying data-collection function used by [resolve_address_to_module](resolve_address_to_module.md) to populate the [MODULE_CACHE](MODULE_CACHE.md).

## Syntax

```rust
fn enumerate_process_modules(pid: u32) -> Vec<(usize, usize, String)>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The process identifier of the target process whose loaded modules should be enumerated. The function opens a handle to this process internally with `PROCESS_QUERY_INFORMATION \| PROCESS_VM_READ` access. |

## Return value

A `Vec<(usize, usize, String)>` where each tuple represents a single loaded module:

| Index | Type | Description |
|-------|------|-------------|
| `.0` | `usize` | The base address of the module in the target process's virtual address space (`MODULEINFO::lpBaseOfDll`). |
| `.1` | `usize` | The size of the module image in bytes (`MODULEINFO::SizeOfImage`). |
| `.2` | `String` | The base name of the module (e.g., `"kernel32.dll"`, `"game.exe"`), obtained via `GetModuleBaseNameW`. |

If the process cannot be opened or module enumeration fails, an empty vector is returned.

## Remarks

### Algorithm

1. **Open the process** — Calls `OpenProcess` with `PROCESS_QUERY_INFORMATION | PROCESS_VM_READ`. If the call fails or returns an invalid handle, returns an empty vector immediately.

2. **Enumerate module handles** — Calls `EnumProcessModulesEx` with `LIST_MODULES_ALL` to retrieve up to 1024 `HMODULE` handles into a stack-allocated array. The `LIST_MODULES_ALL` flag ensures both 32-bit and 64-bit modules are included (relevant for WoW64 scenarios). If enumeration fails, the process handle is closed and an empty vector is returned.

3. **Query each module** — For each of the `module_count` modules (derived from `cb_needed / size_of::<HMODULE>()`):
   - Calls `GetModuleInformation` to retrieve the `MODULEINFO` struct (base address, image size, entry point).
   - Calls `GetModuleBaseNameW` to retrieve the module's base file name into a 260-character `u16` buffer.
   - If either call fails or the name length is zero, the module is skipped.
   - Otherwise, the `(base, size, name)` tuple is pushed into the result vector.

4. **Cleanup** — Closes the process handle via `CloseHandle` before returning.

### Module limit

The function uses a fixed-size array of 1024 `HMODULE` entries. If a process has more than 1024 loaded modules, only the first 1024 are enumerated. In practice, even large applications rarely exceed this limit — typical processes load 50–200 modules.

### Name buffer

Module names are retrieved into a 260-element `u16` buffer (`MAX_PATH`), which is sufficient for all standard Windows module names. `GetModuleBaseNameW` returns only the file name component (e.g., `"ntdll.dll"`), not the full path.

### Process access requirements

The function requires both `PROCESS_QUERY_INFORMATION` and `PROCESS_VM_READ` on the target process. These are more demanding than the limited-information rights used by [get_process_handle](get_process_handle.md) for general operations. Without `SeDebugPrivilege`, this call will fail for protected processes and system processes. When enumeration fails, the [resolve_address_to_module](resolve_address_to_module.md) function gracefully falls back to raw hexadecimal address formatting.

### Handle management

The function opens and closes its own process handle internally and does not use the [ProcessHandle](ProcessHandle.md) RAII wrapper. This is because module enumeration is an infrequent, self-contained operation triggered only on cache misses, and mixing it with the main apply loop's handle lifecycle would add unnecessary complexity.

### Cross-architecture considerations

The `LIST_MODULES_ALL` flag passed to `EnumProcessModulesEx` ensures correct behavior when the AffinityServiceRust process (64-bit) enumerates modules in a WoW64 (32-bit) target process. Without this flag, only native-architecture modules would be returned.

### Visibility

This function is module-private (`fn`, no `pub`). It is called exclusively by [resolve_address_to_module](resolve_address_to_module.md) during [MODULE_CACHE](MODULE_CACHE.md) population.

## Requirements

| | |
|---|---|
| **Module** | `winapi` (`src/winapi.rs`) |
| **Visibility** | Module-private (`fn`, no `pub`) |
| **Callers** | [resolve_address_to_module](resolve_address_to_module.md) |
| **Callees** | `OpenProcess`, `EnumProcessModulesEx`, `GetModuleInformation`, `GetModuleBaseNameW`, `CloseHandle` (Win32 Process Status API / Threading) |
| **API** | [`EnumProcessModulesEx`](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-enumprocessmodulesex), [`GetModuleInformation`](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getmoduleinformation), [`GetModuleBaseNameW`](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getmodulebasenamew) |
| **Privileges** | `PROCESS_QUERY_INFORMATION \| PROCESS_VM_READ` on the target process; `SeDebugPrivilege` recommended for protected processes |

## See Also

| Topic | Link |
|-------|------|
| Address-to-module resolution (caller) | [resolve_address_to_module](resolve_address_to_module.md) |
| Per-process module cache | [MODULE_CACHE](MODULE_CACHE.md) |
| Cache eviction | [drop_module_cache](drop_module_cache.md) |
| Thread start address query (provides addresses to resolve) | [get_thread_start_address](get_thread_start_address.md) |
| Process handle container | [ProcessHandle](ProcessHandle.md) |
| Debug privilege enablement | [enable_debug_privilege](enable_debug_privilege.md) |
| EnumProcessModulesEx (MSDN) | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-enumprocessmodulesex) |
| GetModuleInformation (MSDN) | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getmoduleinformation) |