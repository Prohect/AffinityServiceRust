# resolve_address_to_module function (winapi.rs)

Resolves a virtual memory address to a human-readable module name with offset (e.g., `kernel32.dll+0x345`). Uses a per-PID cache of loaded module information to avoid repeated process module enumeration on every call.

## Syntax

```AffinityServiceRust/src/winapi.rs#L688-688
pub fn resolve_address_to_module(pid: u32, address: usize) -> String
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The process identifier whose address space contains the target address. Used as the cache key for module enumeration results. |
| `address` | `usize` | The virtual memory address to resolve. Typically a thread start address obtained from [get_thread_start_address](get_thread_start_address.md). |

## Return value

Returns a `String` in one of three formats:

| Condition | Format | Example |
|-----------|--------|---------|
| `address` is `0` | `"0x0"` | `0x0` |
| Address falls within a known module's range | `"<module_name>+0x<offset>"` | `kernel32.dll+0x1A345` |
| Address does not match any loaded module | `"0x<address>"` | `0x7FF8A1230000` |

## Remarks

### Caching strategy

The function maintains a per-PID module cache in the [MODULE_CACHE](MODULE_CACHE.md) static (`Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>>`). On the first call for a given PID, it invokes [enumerate_process_modules](enumerate_process_modules.md) to populate the cache with a list of `(base_address, size, module_name)` tuples. Subsequent calls for the same PID reuse the cached data without re-enumerating.

The cache should be cleared periodically via [drop_module_cache](drop_module_cache.md) when a process exits or at the beginning of each main-loop iteration, to prevent stale data from accumulating.

### Address resolution algorithm

1. If `address` is `0`, return `"0x0"` immediately (fast path for unknown/null addresses).
2. Acquire the [MODULE_CACHE](MODULE_CACHE.md) mutex lock.
3. If the cache contains an entry for `pid`, use the cached module list. Otherwise, call [enumerate_process_modules](enumerate_process_modules.md) to build the list, insert it into the cache, and use it.
4. Search the module list for the first entry where `base <= address < base + size`.
5. If a matching module is found, compute `offset = address - base` and return `"<module_name>+0x<offset>"`.
6. If no match is found, return `"0x<address>"` in uppercase hexadecimal.

### Important side effects

- **First call per PID triggers module enumeration.** This involves opening the target process with `PROCESS_QUERY_INFORMATION | PROCESS_VM_READ` access and calling `EnumProcessModulesEx`, `GetModuleInformation`, and `GetModuleBaseNameW`. This may be slow for processes with many loaded modules.
- **The cache grows unboundedly** if [drop_module_cache](drop_module_cache.md) is never called. Callers are responsible for pruning entries for terminated processes.
- The function acquires the `MODULE_CACHE` mutex, so concurrent calls will serialize on the lock.

### Platform notes

- **Windows only.** Module enumeration relies on Win32 Process Status API (`psapi`) functions.
- The function clones the cached module vector out of the mutex before performing the search, minimizing lock hold time.
- For 64-bit processes, addresses and offsets are formatted using `usize` (8 bytes on x86-64).

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `winapi.rs` |
| **Callers** | `apply.rs` — ideal processor assignment logic that logs which module a thread's start address belongs to. |
| **Callees** | [enumerate_process_modules](enumerate_process_modules.md) (on cache miss) |
| **Statics** | [MODULE_CACHE](MODULE_CACHE.md) |
| **Win32 API** | Indirectly: `OpenProcess`, `EnumProcessModulesEx`, `GetModuleInformation`, `GetModuleBaseNameW`, `CloseHandle` (via `enumerate_process_modules`) |
| **Privileges** | `PROCESS_QUERY_INFORMATION \| PROCESS_VM_READ` on the target process. `SeDebugPrivilege` recommended for cross-session access. |

## See Also

| Topic | Link |
|-------|------|
| drop_module_cache | [drop_module_cache](drop_module_cache.md) |
| enumerate_process_modules | [enumerate_process_modules](enumerate_process_modules.md) |
| MODULE_CACHE static | [MODULE_CACHE](MODULE_CACHE.md) |
| get_thread_start_address | [get_thread_start_address](get_thread_start_address.md) |
| set_thread_ideal_processor_ex | [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) |
| winapi module overview | [README](README.md) |

---
*Commit: [37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
