# ConfigConstants struct (config.rs)

Scheduler behavior constants that control prime thread promotion and demotion thresholds. These values are parsed from the constants section of the configuration file and have sensible defaults.

## Syntax

```rust
pub struct ConfigConstants {
    pub min_active_streak: u8,
    pub keep_threshold: f64,
    pub entry_threshold: f64,
}
```

## Members

`min_active_streak`

Minimum number of consecutive scheduling cycles a thread must be active before it is eligible for prime thread promotion. Default value is `2`.

`keep_threshold`

Fraction of total CPU cycle share a promoted thread must maintain to keep its prime status. If the thread's relative cycle usage drops below this threshold, it is demoted. Default value is `0.69`.

`entry_threshold`

Fraction of total CPU cycle share a thread must reach to be considered for prime thread promotion. Only threads exceeding this threshold are candidates. Default value is `0.42`.

## Remarks

`ConfigConstants` implements the `Default` trait, providing the following default values:

| Field | Default |
| --- | --- |
| `min_active_streak` | `2` |
| `keep_threshold` | `0.69` |
| `entry_threshold` | `0.42` |

These constants are set in the configuration file using the `@` prefix in the constants section:

```
@MIN_ACTIVE_STREAK = 2
@KEEP_THRESHOLD = 0.69
@ENTRY_THRESHOLD = 0.42
```

The constants are parsed by [parse_constant](parse_constant.md) during [read_config](read_config.md) and stored in the [ConfigResult](ConfigResult.md)`.constants` field. They are then passed to the `PrimeThreadScheduler` in `src/scheduler.rs` to govern prime thread scheduling decisions.

- `keep_threshold` should be greater than `entry_threshold` to provide hysteresis and prevent rapid promote/demote cycling.
- Setting `min_active_streak` higher makes promotion more conservative, requiring threads to prove sustained activity.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/config.rs |
| **Derives** | `Debug`, `Clone` |
| **Used by** | [ConfigResult](ConfigResult.md), `PrimeThreadScheduler` (src/scheduler.rs) |
| **Parsed by** | [parse_constant](parse_constant.md) |