# drop_module_cache function (winapi.rs)

Removes the cached module list for a specific process from the global [MODULE_CACHE](MODULE_CACHE.md), freeing memory and ensuring that the next call to [resolve_address_to_module](resolve_address_to_module.md) for that process will re-enumerate its loaded modules.

## Syntax

```rust
pub fn drop_module_cache(pid: u32)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The process identifier whose cached module list should be removed. If the PID does not exist in the cache, the call is a no-op. |

## Return value

None.

## Remarks

### Purpose

The [MODULE_CACHE](MODULE_CACHE.md) stores per-process module enumeration results (base address, size, and name for each loaded DLL/EXE) so that repeated calls to [resolve_address_to_module](resolve_address_to_module.md) for the same process do not need to re-enumerate modules every time. However, the cache must be invalidated when:

- A process exits, to prevent stale entries from accumulating.
- A new main-loop iteration begins, to pick up any modules that were loaded or unloaded since the last scan.
- The PID is recycled by the OS for a new process, which would have a completely different module layout.

`drop_module_cache` handles all of these cases by removing the entry for the given PID.

### Implementation

The function acquires the [MODULE_CACHE](MODULE_CACHE.md) mutex lock and calls `HashMap::remove` on the PID key. If the PID is not present in the map, `remove` returns `None` and no action is taken. The lock is released when the `MutexGuard` goes out of scope at the end of the function.

### Thread safety

Access to the [MODULE_CACHE](MODULE_CACHE.md) is serialized by a `std::sync::Mutex`. The lock is held only for the duration of the `remove` call, which is O(1) amortized for `HashMap`.

### Relationship to resolve_address_to_module

The typical lifecycle of a cache entry is:

1. [resolve_address_to_module](resolve_address_to_module.md) is called for a PID that is not yet in the cache.
2. The function calls [enumerate_process_modules](enumerate_process_modules.md) and stores the result in [MODULE_CACHE](MODULE_CACHE.md).
3. Subsequent calls to `resolve_address_to_module` for the same PID use the cached data.
4. At the end of the loop iteration or when the process exits, `drop_module_cache` removes the entry.
5. If the process is still running in the next iteration, step 1 repeats with fresh module data.

### Call sites

The function is called from the scheduler module when a process exits (during `PrimeThreadScheduler::drop_process_by_pid`) and from the main loop between iterations to prevent stale data from persisting across polling cycles.

## Requirements

| | |
|---|---|
| **Module** | `winapi` (`src/winapi.rs`) |
| **Visibility** | `pub` |
| **Callers** | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) (process exit cleanup), main loop in [`main.rs`](../main.rs/README.md) |
| **Callees** | `Mutex::lock`, `HashMap::remove` (standard library) |
| **Dependencies** | [MODULE_CACHE](MODULE_CACHE.md) |

## See Also

| Topic | Link |
|-------|------|
| Module address resolution | [resolve_address_to_module](resolve_address_to_module.md) |
| Global module cache | [MODULE_CACHE](MODULE_CACHE.md) |
| Module enumeration | [enumerate_process_modules](enumerate_process_modules.md) |
| Prime-thread scheduler (process exit cleanup) | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| Thread start address query | [get_thread_start_address](get_thread_start_address.md) |