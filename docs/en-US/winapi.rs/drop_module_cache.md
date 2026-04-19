# drop_module_cache function (winapi.rs)

Removes the cached module enumeration data for a specific process from the global [MODULE_CACHE](README.md) static. Called when a process exits to free memory and ensure stale module data is not used if the PID is later reused by the OS.

## Syntax

```rust
pub fn drop_module_cache(pid: u32)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `pid` | `u32` | The process identifier whose cached module data should be removed. |

## Return value

This function does not return a value.

## Remarks

- The function acquires the `MODULE_CACHE` mutex lock, calls `HashMap::remove(&pid)`, and releases the lock. If no entry exists for the given PID, the call is a no-op.
- `MODULE_CACHE` is a `Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>>` that maps each PID to a list of `(base_address, size, module_name)` tuples. Entries are populated on-demand by [resolve_address_to_module](resolve_address_to_module.md) the first time an address resolution is requested for a given process.
- This function is called by [PrimeThreadScheduler::drop_process_by_pid](../scheduler.rs/PrimeThreadScheduler.md) during process exit cleanup. Dropping the cache at this point ensures that:
  1. Memory used by the module list is freed promptly when the process is no longer tracked.
  2. If the OS reuses the same PID for a new process, the stale module list from the old process will not be returned by [resolve_address_to_module](resolve_address_to_module.md).
- The function does **not** close any handles — module enumeration uses a temporary handle opened and closed within [enumerate_process_modules](enumerate_process_modules.md).

### Timing

In the typical lifecycle, `drop_module_cache` is called once per tracked process exit. It is not called during normal operation while the process is alive, so the cache persists for the entire tracking period of a process to avoid redundant `EnumProcessModulesEx` calls.

## Requirements

| | |
|---|---|
| **Module** | `src/winapi.rs` |
| **Callers** | [PrimeThreadScheduler::drop_process_by_pid](../scheduler.rs/PrimeThreadScheduler.md) |
| **Callees** | None (operates on `MODULE_CACHE` static only) |
| **Win32 API** | None |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [winapi.rs](README.md) |
| Address resolution that populates the cache | [resolve_address_to_module](resolve_address_to_module.md) |
| Module enumeration that fills cache entries | [enumerate_process_modules](enumerate_process_modules.md) |
| Process cleanup that calls this function | [PrimeThreadScheduler::drop_process_by_pid](../scheduler.rs/PrimeThreadScheduler.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*