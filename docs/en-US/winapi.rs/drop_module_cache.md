# drop_module_cache function (winapi.rs)

Removes a process's cached module enumeration data from the global [`MODULE_CACHE`](MODULE_CACHE.md), freeing memory and ensuring stale module information is not reused.

## Syntax

```rust
pub fn drop_module_cache(pid: u32)
```

## Parameters

`pid`

The process identifier whose cached module data should be removed from the [`MODULE_CACHE`](MODULE_CACHE.md).

## Return value

This function does not return a value.

## Remarks

The [`MODULE_CACHE`](MODULE_CACHE.md) stores per-process module enumeration results (base address, end address, module name tuples) to avoid repeatedly calling [`enumerate_process_modules`](enumerate_process_modules.md) for the same process across loop iterations. When a process terminates or is otherwise no longer being tracked, this function should be called to remove its entry from the cache.

The function acquires a lock on the [`MODULE_CACHE`](MODULE_CACHE.md) mutex and removes the entry keyed by `pid`. If no entry exists for the given PID, the function is a no-op.

Failure to call this function for terminated processes would cause the cache to grow unboundedly over time as new processes are encountered and old entries are never cleaned up. The caller (typically the main loop in [`main`](../main.rs/main.md)) is responsible for calling this when a tracked process is no longer alive.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/winapi.rs |
| **Source lines** | L710–L713 |
| **Called by** | [`main`](../main.rs/main.md) loop cleanup |
| **Modifies** | [`MODULE_CACHE`](MODULE_CACHE.md) |

## See also

- [MODULE_CACHE static](MODULE_CACHE.md)
- [resolve_address_to_module](resolve_address_to_module.md)
- [enumerate_process_modules](enumerate_process_modules.md)
- [winapi.rs module overview](README.md)