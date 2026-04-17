# drop_module_cache function (winapi.rs)

Removes the cached module list for a given process ID from the global [MODULE_CACHE](MODULE_CACHE.md) static. This ensures that stale module information is discarded when a process exits or when a fresh module enumeration is needed on the next call to [resolve_address_to_module](resolve_address_to_module.md).

## Syntax

```rust
pub fn drop_module_cache(pid: u32)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The process identifier whose cached module list should be removed. |

## Return value

This function does not return a value.

## Remarks

- The function acquires a lock on the [MODULE_CACHE](MODULE_CACHE.md) `Mutex<HashMap<u32, Vec<(usize, usize, String)>>>` and calls `HashMap::remove` with the provided `pid` key.

- If the `pid` is not present in the cache (e.g., the process was never queried or was already evicted), the call is a no-op.

- This function is intended to be called during the main scheduling loop when a process is no longer being tracked, or at the start of a new iteration to force a fresh module enumeration. Without explicit eviction, the cache would retain entries for terminated processes indefinitely, consuming memory and potentially returning incorrect results if a new process reuses the same PID.

- The function is lightweight — it performs a single hash-map lookup and removal under the mutex lock, with no system calls or I/O.

### Typical call pattern

1. The scheduler detects that a process has exited (via snapshot comparison or ETW stop event).
2. `drop_module_cache(pid)` is called to discard the stale module list.
3. If the same PID is later reused by a new process, [resolve_address_to_module](resolve_address_to_module.md) will re-enumerate the modules on demand.

### Thread safety

The function acquires the `MODULE_CACHE` mutex, so it is safe to call concurrently from multiple threads. However, callers should be aware that holding other locks (e.g., `CPU_SET_INFORMATION`) simultaneously could create lock-ordering concerns.

## Requirements

| Requirement | Value |
|-------------|-------|
| **Module** | `winapi.rs` |
| **Callers** | `apply.rs`, `scheduler.rs` — process lifecycle management |
| **Callees** | `Mutex::lock`, `HashMap::remove` (standard library) |
| **Win32 API** | None |
| **Privileges** | None required |

## See Also

| Topic | Link |
|-------|------|
| resolve_address_to_module | [resolve_address_to_module](resolve_address_to_module.md) |
| MODULE_CACHE static | [MODULE_CACHE](MODULE_CACHE.md) |
| enumerate_process_modules | [enumerate_process_modules](enumerate_process_modules.md) |
| get_thread_start_address | [get_thread_start_address](get_thread_start_address.md) |
| winapi module overview | [README](README.md) |

---
*Commit: [37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
