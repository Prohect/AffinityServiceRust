# MODULE_CACHE static (winapi.rs)

Per-process cache of loaded module information (base address, size, and name) used by [`resolve_address_to_module`](resolve_address_to_module.md) to translate memory addresses into human-readable module-relative offsets (e.g., `kernel32.dll+0x345`).

## Syntax

```rust
static MODULE_CACHE: Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>> =
    Lazy::new(|| Mutex::new(HashMap::default()));
```

## Members

| Component | Type | Description |
|-----------|------|-------------|
| Key | `u32` | The process ID (PID) for which modules have been enumerated. |
| Value | `Vec<(usize, usize, String)>` | A vector of tuples, each containing: **base address** (`usize`), **module size** (`usize`), and **module name** (`String`). |

Each tuple in the value vector represents a single loaded module in the target process's address space:

| Tuple Index | Type | Description |
|-------------|------|-------------|
| `.0` | `usize` | The base virtual address of the module (`MODULEINFO.lpBaseOfDll`). |
| `.1` | `usize` | The size of the module image in bytes (`MODULEINFO.SizeOfImage`). |
| `.2` | `String` | The base name of the module (e.g., `kernel32.dll`), obtained via `GetModuleBaseNameW`. |

## Remarks

- **Lazy initialization.** The cache is initialized as an empty `HashMap` on first access via `once_cell::sync::Lazy`. No system calls are made until [`resolve_address_to_module`](resolve_address_to_module.md) is called for a specific PID.

- **Population strategy.** When `resolve_address_to_module` is called for a PID not yet in the cache, it calls the private function [`enumerate_process_modules`](enumerate_process_modules.md) to enumerate all modules loaded in that process via `EnumProcessModulesEx`, `GetModuleInformation`, and `GetModuleBaseNameW`. The result is stored in the cache and reused for subsequent lookups of the same PID.

- **Cache invalidation.** Entries are removed explicitly by calling [`drop_module_cache`](drop_module_cache.md), which removes the entry for a given PID. This is typically called when a process exits or at the beginning of a new scheduling loop iteration to ensure stale data does not persist.

- **Thread safety.** The cache is wrapped in a `Mutex`, ensuring safe concurrent access. All accesses go through `MODULE_CACHE.lock().unwrap()`.

- **Address lookup algorithm.** When resolving an address, [`resolve_address_to_module`](resolve_address_to_module.md) performs a linear scan of the cached module list for the given PID, searching for a module whose base address range (`base..base+size`) contains the target address. The first matching module is used to produce a `"module_name+0xOFFSET"` string.

- **Memory considerations.** Each process entry holds a `Vec` of module tuples. For typical applications with 50–200 loaded DLLs, this amounts to a few kilobytes per PID entry. The cache grows as new PIDs are encountered and shrinks as entries are explicitly dropped.

- The `HashMap` type used here is the project's custom alias for `FxHashMap` from the [`collections`](../collections.rs/README.md) module, which uses a fast non-cryptographic hash function.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `winapi.rs` |
| **Type** | `Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>>` |
| **Initialized by** | `once_cell::sync::Lazy` (empty on first access) |
| **Populated by** | [`resolve_address_to_module`](resolve_address_to_module.md) → [`enumerate_process_modules`](enumerate_process_modules.md) |
| **Invalidated by** | [`drop_module_cache`](drop_module_cache.md) |
| **Win32 API** | `EnumProcessModulesEx`, `GetModuleInformation`, `GetModuleBaseNameW` (via `enumerate_process_modules`) |
| **Platform** | Windows |

## See Also

| Topic | Link |
|-------|------|
| resolve_address_to_module | [resolve_address_to_module](resolve_address_to_module.md) |
| drop_module_cache | [drop_module_cache](drop_module_cache.md) |
| enumerate_process_modules | [enumerate_process_modules](enumerate_process_modules.md) |
| get_thread_start_address | [get_thread_start_address](get_thread_start_address.md) |
| CPU_SET_INFORMATION static | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) |
| collections module | [collections.rs](../collections.rs/README.md) |

---
> Commit SHA: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
