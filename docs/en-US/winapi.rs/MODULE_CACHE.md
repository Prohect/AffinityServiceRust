# MODULE_CACHE static (winapi.rs)

Per-process cache of enumerated module address ranges and names, used by [resolve_address_to_module](resolve_address_to_module.md) to map thread start addresses to human-readable module names with offsets. The cache is populated lazily on the first address resolution for each process and cleared when a process exits or via [drop_module_cache](drop_module_cache.md).

## Syntax

```rust
#[allow(clippy::type_complexity)]
static MODULE_CACHE: Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
```

## Members

| Member | Type | Description |
|--------|------|-------------|
| Outer key | `u32` | Process ID (PID) used to index into the cache. Each process has its own independent module list. |
| Inner value | `Vec<(usize, usize, String)>` | A vector of module entries for the process. Each tuple contains: **(1)** `usize` — base address of the module in the target process's virtual address space, **(2)** `usize` — size of the module image in bytes, **(3)** `String` — the base name of the module (e.g., `"kernel32.dll"`, `"ntdll.dll"`). |

## Remarks

### Population

The cache is populated on-demand by [resolve_address_to_module](resolve_address_to_module.md). When that function is called for a PID not yet in the cache, it calls [enumerate_process_modules](enumerate_process_modules.md) to walk the target process's loaded module list via `EnumProcessModulesEx`, `GetModuleInformation`, and `GetModuleBaseNameW`. The resulting vector of `(base, size, name)` tuples is inserted into the cache and also returned for immediate use.

### Cache invalidation

The cache entry for a specific PID is removed by calling [drop_module_cache](drop_module_cache.md), which simply calls `cache.remove(&pid)`. This is typically done when:

- A process exits and its [ProcessStats](../scheduler.rs/ProcessStats.md) entry is cleaned up by the scheduler.
- The main loop iterates to a new cycle and stale entries should not persist.

No automatic expiration or LRU eviction is implemented. If a process's module list changes at runtime (e.g., via `LoadLibrary`), the cached data becomes stale. For the prime-thread scheduling use case this is acceptable, as module loads typically happen early in a process's lifetime and the thread start address is fixed at thread creation time.

### Thread safety

The `HashMap` is wrapped in a `std::sync::Mutex`. Both [resolve_address_to_module](resolve_address_to_module.md) and [drop_module_cache](drop_module_cache.md) acquire the lock for the duration of their operation. Because the cache is read/written on the main service loop thread, contention is minimal in practice.

### Memory usage

Each cached process entry contains one tuple per loaded module. A typical Windows process loads 50–200 modules, so each entry is on the order of a few kilobytes. The cache grows linearly with the number of distinct PIDs that have had address resolution performed and shrinks as entries are dropped.

### Clippy suppression

The `#[allow(clippy::type_complexity)]` attribute suppresses the Clippy warning about the deeply nested generic type. The complexity is inherent to the `HashMap<u32, Vec<(usize, usize, String)>>` structure and extracting a type alias would not improve readability in this context.

## Requirements

| | |
|---|---|
| **Module** | `winapi` (`src/winapi.rs`) |
| **Crate dependencies** | `once_cell::sync::Lazy`, `std::sync::Mutex`, `std::collections::HashMap` |
| **Populated by** | [resolve_address_to_module](resolve_address_to_module.md) (via [enumerate_process_modules](enumerate_process_modules.md)) |
| **Invalidated by** | [drop_module_cache](drop_module_cache.md) |
| **Privileges** | `PROCESS_QUERY_INFORMATION | PROCESS_VM_READ` on the target process (required by [enumerate_process_modules](enumerate_process_modules.md)) |

## See Also

| Topic | Link |
|-------|------|
| Address-to-module resolution | [resolve_address_to_module](resolve_address_to_module.md) |
| Cache eviction | [drop_module_cache](drop_module_cache.md) |
| Module enumeration implementation | [enumerate_process_modules](enumerate_process_modules.md) |
| Thread start address query | [get_thread_start_address](get_thread_start_address.md) |
| Prime-thread scheduler (consumer of module names) | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| CPU set topology cache (analogous lazy global) | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) |
| EnumProcessModulesEx (MSDN) | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-enumprocessmodulesex) |