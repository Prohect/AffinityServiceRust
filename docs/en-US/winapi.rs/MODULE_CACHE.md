# MODULE_CACHE static (winapi.rs)

Per-process cache of enumerated module base addresses, end addresses, and module names. Used by [`resolve_address_to_module`](resolve_address_to_module.md) to avoid re-enumerating process modules on every address resolution.

## Syntax

```rust
static MODULE_CACHE: Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
```

## Members

The static holds a `HashMap<u32, Vec<(usize, usize, String)>>` behind a `Mutex`:

- **Key** (`u32`) — the process identifier (PID).
- **Value** (`Vec<(usize, usize, String)>`) — a vector of tuples, each representing one loaded module:
  - `.0` (`usize`) — the base address of the module in the process's virtual address space.
  - `.1` (`usize`) — the end address (base + size) of the module.
  - `.2` (`String`) — the module file name (e.g., `"ntdll.dll"`, `"kernel32.dll"`).

## Remarks

Module enumeration via [`enumerate_process_modules`](enumerate_process_modules.md) is a relatively expensive operation that involves opening the target process with `PROCESS_QUERY_INFORMATION | PROCESS_VM_READ` access and calling `EnumProcessModulesEx` followed by `GetModuleFileNameExW` for each module. The `MODULE_CACHE` avoids repeating this work by storing the results per PID.

### Cache lifecycle

1. **Population** — when [`resolve_address_to_module`](resolve_address_to_module.md) is called for a PID that has no cache entry, it calls [`enumerate_process_modules`](enumerate_process_modules.md) and inserts the result into the cache.
2. **Lookup** — subsequent calls to [`resolve_address_to_module`](resolve_address_to_module.md) for the same PID perform a simple linear scan of the cached module list, comparing the address against each module's `[base, end)` range.
3. **Eviction** — [`drop_module_cache`](drop_module_cache.md) removes a PID's entry from the cache. This should be called when a process terminates to prevent stale data from accumulating.

### Thread safety

All access to the cache is synchronized through the `Mutex`. The lock is acquired for the duration of each lookup or insertion, then released immediately.

### Memory considerations

Since each entry contains a full module list for a process (which can include hundreds of DLLs for complex applications), it is important that [`drop_module_cache`](drop_module_cache.md) is called when processes exit. Failure to do so would cause the cache to grow unboundedly over time.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/winapi.rs |
| **Source line** | L680 |
| **Populated by** | [`resolve_address_to_module`](resolve_address_to_module.md) via [`enumerate_process_modules`](enumerate_process_modules.md) |
| **Evicted by** | [`drop_module_cache`](drop_module_cache.md) |

## See also

- [resolve_address_to_module](resolve_address_to_module.md)
- [drop_module_cache](drop_module_cache.md)
- [enumerate_process_modules](enumerate_process_modules.md)
- [winapi.rs module overview](README.md)