# IdealProcessorState type (scheduler.rs)

Tracks the current and previous ideal-processor assignment for a single thread. The ideal processor is a Windows scheduling hint that tells the kernel which logical processor a thread should preferentially run on. This struct records both the current assignment and the previous one so that the apply engine can detect changes and avoid redundant `SetThreadIdealProcessorEx` calls.

## Syntax

```rust
#[derive(Debug, Clone, Copy)]
pub struct IdealProcessorState {
    pub current_group: u16,
    pub current_number: u8,
    pub previous_group: u16,
    pub previous_number: u8,
    pub is_assigned: bool,
}
```

## Members

| Field | Type | Description |
|-------|------|-------------|
| `current_group` | `u16` | The processor group number of the currently assigned ideal processor. On systems with 64 or fewer logical processors there is only group 0. |
| `current_number` | `u8` | The zero-based processor number within `current_group` that is the thread's current ideal processor. |
| `previous_group` | `u16` | The processor group of the previously assigned ideal processor. Used to detect whether a reassignment has occurred since the last iteration. |
| `previous_number` | `u8` | The processor number within `previous_group` that was the thread's ideal processor before the most recent change. |
| `is_assigned` | `bool` | Indicates whether this thread has ever been assigned an ideal processor by the service. When `false`, the `current_*` and `previous_*` fields contain default values (all zero) and do not represent a real assignment. |

## Remarks

- All fields are initialized to zero/`false` by `IdealProcessorState::new()` and the `Default` implementation.
- The struct derives `Clone` and `Copy`, making it cheap to snapshot and compare across iterations.
- The previous/current pair enables the apply engine to implement change detection: if `(current_group, current_number)` equals `(previous_group, previous_number)` and `is_assigned` is `true`, no Win32 call is needed because the assignment has not changed.
- Processor groups are relevant on systems with more than 64 logical processors (e.g. dual-socket server hardware or heavily multi-core workstations). On typical consumer hardware, `current_group` and `previous_group` will always be `0`.
- This struct is embedded inside [`ThreadStats`](ThreadStats.md) as the `ideal_processor` field.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `scheduler.rs` |
| Used by | [`ThreadStats`](ThreadStats.md), `apply::apply_ideal_processors` |
| Win32 API | Corresponds to the `PROCESSOR_NUMBER` structure used by `SetThreadIdealProcessorEx` / `GetThreadIdealProcessorEx` |
| Privileges | None (data-only struct) |

## See Also

| Reference | Link |
|-----------|------|
| ThreadStats | [ThreadStats](ThreadStats.md) |
| PrimeThreadScheduler | [PrimeThreadScheduler](PrimeThreadScheduler.md) |
| ProcessStats | [ProcessStats](ProcessStats.md) |
| apply_thread_level | [apply_thread_level](../main.rs/apply_thread_level.md) |
| scheduler module overview | [README](README.md) |

---
*Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
