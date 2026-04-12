# IdealProcessorState struct (scheduler.rs)

Tracks the current and previous ideal processor assignment for a single thread, enabling change detection and rollback during ideal processor scheduling.

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

`current_group`

The processor group index of the thread's currently assigned ideal processor. Processor groups are a Windows concept for systems with more than 64 logical processors; on most consumer systems this is `0`.

`current_number`

The processor number (within `current_group`) of the thread's currently assigned ideal processor. This is a zero-based index into the group's logical processor array.

`previous_group`

The processor group index of the thread's *previously* assigned ideal processor. Retained so that the scheduler can detect whether an assignment actually changed between iterations.

`previous_number`

The processor number (within `previous_group`) of the thread's previously assigned ideal processor. Used together with `previous_group` for change detection.

`is_assigned`

Indicates whether this thread has ever been assigned an ideal processor by the scheduler. When `false`, the `current_*` and `previous_*` fields are at their default zero values and should not be interpreted as valid assignments.

## Methods

| Method | Signature | Description |
| --- | --- | --- |
| **new** | `pub fn new() -> Self` | Creates a default state with all fields zeroed and `is_assigned` set to `false`. |
| **default** | `fn default() -> Self` | `Default` trait implementation; delegates to `new()`. |

## Remarks

`IdealProcessorState` is embedded within each [ThreadStats](ThreadStats.md) instance. It serves two purposes in the ideal processor assignment pipeline implemented in [`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md):

1. **Change detection** — Before calling `SetThreadIdealProcessorEx`, the scheduler compares the new target `(group, number)` against `(current_group, current_number)`. If they match, the syscall is skipped entirely, avoiding unnecessary kernel transitions.

2. **History tracking** — When a new assignment is made, the current values are shifted into the `previous_*` fields before being overwritten. This allows logging of transitions (e.g., `"TID 1234: ideal CPU 2→5"`) and could support future rollback logic.

### Processor groups

On systems with ≤64 logical processors (the vast majority of consumer and gaming PCs), there is a single processor group (`group = 0`), so `current_group` and `previous_group` will always be `0`. The struct carries group information for correctness on high-core-count server/workstation systems where Windows creates multiple processor groups.

### Lifetime

The state persists across loop iterations inside [ThreadStats](ThreadStats.md), which itself lives inside [ProcessStats](ProcessStats.md) within the [PrimeThreadScheduler](PrimeThreadScheduler.md). It is cleared when the owning process exits and its `ProcessStats` entry is removed by [`close_dead_process_handles`](PrimeThreadScheduler.md#close_dead_process_handles).

### Derives

The struct derives `Debug`, `Clone`, and `Copy`. `Copy` is appropriate because the struct is 12 bytes of plain data with no heap allocations, making pass-by-value efficient.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/scheduler.rs |
| **Source lines** | L206–L233 |
| **Contained in** | [ThreadStats](ThreadStats.md) `.ideal_processor` field |
| **Used by** | [`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md) |
| **Windows API** | [`SetThreadIdealProcessorEx`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadidealprocessorex), [`GetThreadIdealProcessorEx`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadidealprocessorex) |

## See also

- [ThreadStats](ThreadStats.md)
- [PrimeThreadScheduler](PrimeThreadScheduler.md)
- [apply_ideal_processors](../apply.rs/apply_ideal_processors.md)
- [scheduler.rs module overview](README.md)