# IdealProcessorState struct (scheduler.rs)

Tracks the ideal processor assignment state for a single thread, recording both the current and previous processor group and number to support reassignment decisions.

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

| Member | Type | Description |
|--------|------|-------------|
| `current_group` | `u16` | The processor group index where the thread is currently ideally scheduled. Corresponds to the `Group` field of a `PROCESSOR_NUMBER` structure. |
| `current_number` | `u8` | The logical processor number within `current_group` where the thread is currently ideally scheduled. Corresponds to the `Number` field of a `PROCESSOR_NUMBER` structure. |
| `previous_group` | `u16` | The processor group index where the thread was ideally scheduled before the most recent reassignment. Used to detect whether a thread's ideal processor actually changed. |
| `previous_number` | `u8` | The logical processor number within `previous_group` where the thread was ideally scheduled before the most recent reassignment. |
| `is_assigned` | `bool` | Indicates whether this thread has been explicitly assigned an ideal processor by the scheduler. When `false`, the `current_*` and `previous_*` fields are at their default zero values and do not represent a deliberate assignment. |

## Remarks

`IdealProcessorState` is embedded in each [ThreadStats](ThreadStats.md) instance as the `ideal_processor` field. It enables the [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) function to track whether a thread's ideal processor has been set, and to compare the current assignment against the previous one when deciding whether to call `SetThreadIdealProcessorEx`.

The `previous_*` fields are updated before writing new values to `current_*`, allowing the caller to detect and log processor reassignment transitions. This is particularly useful for diagnosing scheduling instability — if a thread's ideal processor changes every tick, it may indicate the hysteresis thresholds in [PrimeThreadScheduler](PrimeThreadScheduler.md) need tuning.

### Default state

Calling `IdealProcessorState::new()` or `IdealProcessorState::default()` returns a state with all numeric fields set to `0` and `is_assigned` set to `false`. Group 0, processor 0 as default values do **not** imply an assignment to that processor — the `is_assigned` flag must be checked first.

### Platform notes

Processor group and number values correspond to the Windows `PROCESSOR_NUMBER` structure used by `SetThreadIdealProcessorEx` and `GetThreadIdealProcessorEx`. On systems with a single processor group, `current_group` and `previous_group` will always be `0`.

## Requirements

| | |
|---|---|
| **Module** | `scheduler.rs` |
| **Embedded in** | [ThreadStats](ThreadStats.md) |
| **Consumed by** | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |
| **Platform API** | `SetThreadIdealProcessorEx`, `GetThreadIdealProcessorEx` (`windows` crate) |

## See Also

| Topic | Description |
|-------|-------------|
| [ThreadStats](ThreadStats.md) | Per-thread statistics struct that contains an `IdealProcessorState` instance. |
| [PrimeThreadScheduler](PrimeThreadScheduler.md) | Central scheduler that manages thread-to-processor assignments. |
| [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) | Function that reads and writes `IdealProcessorState` when applying ideal processor rules. |
| [set_thread_ideal_processor_ex](../winapi.rs/set_thread_ideal_processor_ex.md) | Low-level wrapper around the Windows `SetThreadIdealProcessorEx` API. |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd