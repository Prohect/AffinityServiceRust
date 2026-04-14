# resolve_address_to_module function (winapi.rs)

Resolves a memory address to a human-readable string containing the module name and hexadecimal offset (e.g., `"engine.dll+0x1A3F0"`). This is used by the prime-thread scheduler and ideal processor assignment logic to identify which loaded module a thread's start address belongs to, enabling module-based thread filtering via [PrimePrefix](../config.rs/PrimePrefix.md) rules.

## Syntax

```rust
pub fn resolve_address_to_module(pid: u32, address: usize) -> String
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The process identifier that owns the address space. Used as the key into the [MODULE_CACHE](MODULE_CACHE.md) to look up or populate the process's module list. |
| `address` | `usize` | The virtual memory address to resolve. Typically a thread start address obtained from [get_thread_start_address](get_thread_start_address.md). |

## Return value

A `String` representing the resolved address. The format depends on resolution success:

| Scenario | Return format | Example |
|----------|---------------|---------|
| Address is `0` | `"0x0"` | `"0x0"` |
| Address falls within a known module's range | `"{module_name}+0x{offset:X}"` | `"engine.dll+0x1A3F0"` |
| Address does not match any loaded module | `"0x{address:X}"` | `"0x7FF6A1230000"` |

## Remarks

### Module cache

The function uses the [MODULE_CACHE](MODULE_CACHE.md) global static — a `Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>>` — to avoid enumerating a process's modules on every call. The cache is keyed by PID, and each entry is a vector of `(base_address, size, module_name)` tuples.

On the first call for a given PID:

1. The cache lock is acquired.
2. If no entry exists for the PID, [enumerate_process_modules](enumerate_process_modules.md) is called to populate the cache.
3. The newly enumerated module list is cloned and stored in the cache.

On subsequent calls for the same PID, the cached module list is returned directly without re-enumerating.

### Address resolution algorithm

After obtaining the module list (from cache or fresh enumeration), the function performs a linear search for the first module whose address range `[base, base + size)` contains the target `address`. If found, it computes the offset as `address - base` and formats the result as `"{module_name}+0x{offset:X}"`.

If no module range contains the address, the raw address is returned in hexadecimal format (`"0x{address:X}"`). This can happen when:

- The thread's start address points to dynamically allocated (non-module) memory.
- The process's module list has changed since the cache was populated.
- The enumeration failed (e.g., insufficient access rights).

### Zero-address fast path

If `address` is `0`, the function returns `"0x0"` immediately without accessing the module cache. A zero start address typically indicates that the thread information could not be queried (see [get_thread_start_address](get_thread_start_address.md) failure behavior).

### Cache lifetime

The module cache for a given PID persists until explicitly cleared by [drop_module_cache](drop_module_cache.md), which is called when a process exits or at the start of each main loop iteration. This ensures that module lists remain reasonably current without the cost of re-enumerating on every thread inspection.

### Clone of cached data

The function clones the cached module vector out of the mutex guard before performing the address search. This releases the mutex lock quickly, minimizing contention when multiple threads or successive calls resolve addresses for different processes. The trade-off is a per-call allocation for the cloned vector, which is acceptable given the typical module count (tens to low hundreds).

### Thread safety

The [MODULE_CACHE](MODULE_CACHE.md) mutex is held only for the duration of the cache lookup or insertion. The actual address resolution (linear search through the module list) occurs outside the lock on the cloned data.

## Requirements

| | |
|---|---|
| **Module** | `winapi` (`src/winapi.rs`) |
| **Visibility** | `pub` |
| **Callers** | [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md), [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |
| **Callees** | [enumerate_process_modules](enumerate_process_modules.md) (on cache miss) |
| **Dependencies** | [MODULE_CACHE](MODULE_CACHE.md), [get_thread_start_address](get_thread_start_address.md) (provides the address input) |
| **API** | None directly (module enumeration is delegated to [enumerate_process_modules](enumerate_process_modules.md)) |
| **Privileges** | `PROCESS_QUERY_INFORMATION | PROCESS_VM_READ` on the target process (required by [enumerate_process_modules](enumerate_process_modules.md) on cache miss) |

## See Also

| Topic | Link |
|-------|------|
| Per-process module cache | [MODULE_CACHE](MODULE_CACHE.md) |
| Cache clearing function | [drop_module_cache](drop_module_cache.md) |
| Module enumeration implementation | [enumerate_process_modules](enumerate_process_modules.md) |
| Thread start address query | [get_thread_start_address](get_thread_start_address.md) |
| Module-name prefix filter for prime threads | [PrimePrefix](../config.rs/PrimePrefix.md) |
| Prime thread promotion (uses module resolution) | [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md) |
| Ideal processor assignment (uses module resolution) | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |
| EnumProcessModulesEx (MSDN) | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-enumprocessmodulesex) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd