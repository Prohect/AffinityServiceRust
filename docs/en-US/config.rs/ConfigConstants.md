# ConfigConstants type (config.rs)

The `ConfigConstants` struct holds tunable numeric constants that control the behavior of the prime-thread scheduler. These values are read from the configuration file using `@NAME = value` syntax and influence how aggressively threads are promoted into or demoted from the prime-thread set.

## Syntax

```AffinityServiceRust/src/config.rs#L49-63
#[derive(Debug, Clone)]
pub struct ConfigConstants {
    pub min_active_streak: u8,
    pub keep_threshold: f64,
    pub entry_threshold: f64,
}

impl Default for ConfigConstants {
    fn default() -> Self {
        ConfigConstants {
            min_active_streak: 2,
            keep_threshold: 0.69,
            entry_threshold: 0.42,
        }
    }
}
```

## Members

| Member | Type | Default | Description |
|--------|------|---------|-------------|
| `min_active_streak` | `u8` | `2` | The minimum number of consecutive polling intervals during which a thread must remain active (CPU-consuming) before it becomes eligible for prime-thread promotion. Higher values make the scheduler more conservative, reducing churn from transient spikes. |
| `keep_threshold` | `f64` | `0.69` | The CPU utilization fraction (0.0–1.0) that a thread already in the prime set must maintain to keep its prime status. When a prime thread's utilization drops below this threshold, it becomes a candidate for demotion. |
| `entry_threshold` | `f64` | `0.42` | The CPU utilization fraction (0.0–1.0) that a non-prime thread must exceed to be considered for promotion into the prime set. This value should be lower than `keep_threshold` to provide hysteresis and prevent rapid toggling. |

## Remarks

- **Hysteresis design**: The default values establish a hysteresis band (`entry_threshold = 0.42` < `keep_threshold = 0.69`). A thread must exceed 42% utilization to enter the prime set but must drop below 69% to leave it. This prevents rapid promotion/demotion cycling for threads hovering near a single threshold.

- **Configuration file syntax**: Constants are defined in the config file with the `@` prefix:
  ```/dev/null/example.ini#L1-3
  @MIN_ACTIVE_STREAK = 3
  @KEEP_THRESHOLD = 0.75
  @ENTRY_THRESHOLD = 0.50
  ```
  Parsing is handled by [parse_constant](parse_constant.md), which validates types and reports errors for invalid values.

- **Derives**: The struct derives `Debug` and `Clone`. The `Default` implementation provides the production-tuned defaults shown above rather than Rust's zero-initialization.

- **Validation**: `min_active_streak` is parsed as `u8` (range 0–255). The threshold fields are parsed as `f64` without explicit range clamping; values outside [0.0, 1.0] are technically accepted but produce undefined scheduling behavior.

- **Hot-reload**: When the configuration file is modified at runtime, [hotreload_config](hotreload_config.md) replaces the scheduler's constants with the newly parsed values from the reloaded [ConfigResult](ConfigResult.md).

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `config.rs` |
| Constructed by | `Default::default()`, [read_config](read_config.md) via [parse_constant](parse_constant.md) |
| Consumed by | `scheduler::PrimeThreadScheduler`, [hotreload_config](hotreload_config.md) |
| Stored in | [ConfigResult](ConfigResult.md)`.constants` |
| Privileges | None |

## See Also

| Resource | Link |
|----------|------|
| ConfigResult | [ConfigResult](ConfigResult.md) |
| parse_constant | [parse_constant](parse_constant.md) |
| hotreload_config | [hotreload_config](hotreload_config.md) |
| read_config | [read_config](read_config.md) |
| scheduler module | [scheduler.rs overview](../scheduler.rs/README.md) |
| config module overview | [README](README.md) |

---
*Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
