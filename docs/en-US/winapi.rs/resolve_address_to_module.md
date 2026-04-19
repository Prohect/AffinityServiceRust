# resolve_address_to_module function (winapi.rs)

Resolves a virtual memory address to a human-readable module name with offset string (e.g., `"kernel32.dll+0x1A40"`). Uses a per-PID module cache to avoid re-enumerating loaded modules on every call.

## Syntax

```rust
pub fn resolve_address_to_module(pid: u32, address: usize) -> String
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The process identifier whose module list should be searched. Used as the key into the `MODULE_CACHE` static. |
| `address` | `usize` | The virtual address to resolve. Typically the Win32 start address of a thread, as obtained from [get_thread_start_address](get_thread_start_address.md). |

## Return value

`String` — One of three formats depending on the resolution result:

| Condition | Format | Example |
|-----------|--------|---------|
| `address == 0` | `"0x0"` | `"0x0"` |
| Address falls within a known module | `"{module_name}+0x{offset:X}"` | `"game.dll+0x1A40"` |
| Address does not match any module | `"0x{address:X}"` | `"0x7FF612340000"` |

## Remarks

### Module cache

The function is backed by the static `MODULE_CACHE: Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>>`. On the first call for a given PID, the function calls [enumerate_process_modules](enumerate_process_modules.md) to build a list of `(base_address, size, module_name)` tuples, which is then stored in the cache. Subsequent calls for the same PID reuse the cached data without re-enumerating.

The cache entry for a PID is removed when:

- The process exits and [drop_module_cache](drop_module_cache.md) is called from [PrimeThreadScheduler::drop_process_by_pid](../scheduler.rs/PrimeThreadScheduler.md).

### Address matching

The function performs a linear search over the cached module list, looking for the first module where `base <= address < base + size`. If a match is found, the offset within the module (`address - base`) is appended to the module name. If no module spans the address, the raw hex address is returned.

### Performance

The `MODULE_CACHE` mutex is acquired once per call. The module list is cloned out of the cache before performing the linear search, releasing the lock before the (potentially slow) string formatting step. This minimizes lock contention when multiple calls occur in rapid succession.

### Zero address shortcut

An address of `0` is returned immediately as the string `"0x0"` without acquiring the cache lock. This is a common case for threads whose start address could not be queried (e.g., due to insufficient handle access rights).

### Usage contexts

This function is called from two primary locations:

1. **ThreadStats custom `Debug` implementation** — resolves `start_address` for human-readable debug output.
2. **PrimeThreadScheduler::drop_process_by_pid** — resolves thread start addresses in the exit report when `track_top_x_threads` is configured.

## Requirements

| | |
|---|---|
| **Module** | `src/winapi.rs` |
| **Callers** | [ThreadStats::fmt (Debug)](../scheduler.rs/ThreadStats.md), [PrimeThreadScheduler::drop_process_by_pid](../scheduler.rs/PrimeThreadScheduler.md) |
| **Callees** | [enumerate_process_modules](enumerate_process_modules.md) (on cache miss) |
| **Win32 API** | None directly; relies on [enumerate_process_modules](enumerate_process_modules.md) for module data |
| **Privileges** | `PROCESS_QUERY_INFORMATION` and `PROCESS_VM_READ` are required by the underlying module enumeration |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [winapi.rs](README.md) |
| Module cache cleanup | [drop_module_cache](drop_module_cache.md) |
| Module enumeration | [enumerate_process_modules](enumerate_process_modules.md) |
| Thread start address retrieval | [get_thread_start_address](get_thread_start_address.md) |
| Thread stats that use this for Debug output | [ThreadStats](../scheduler.rs/ThreadStats.md) |
| Process exit reporting | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*