# ConfigConstants struct (config.rs)

Tuning constants that control the hysteresis behavior of the prime thread selection algorithm. These values determine how aggressively threads are promoted to and demoted from prime status, preventing rapid oscillation when thread utilization hovers near decision boundaries.

## Syntax

```rust
#[derive(Debug, Clone)]
pub struct ConfigConstants {
    pub min_active_streak: u8,
    pub keep_threshold: f64,
    pub entry_threshold: f64,
}
```

## Members

| Member | Type | Default | Description |
|--------|------|---------|-------------|
| `min_active_streak` | `u8` | `2` | Minimum number of consecutive polling iterations a thread must exceed `entry_threshold` before it is promoted to prime status. Higher values increase stability but delay responsiveness. |
| `keep_threshold` | `f64` | `0.69` | Fraction of total CPU cycle share below which a currently-prime thread is demoted. Must be less than or equal to `entry_threshold` to prevent promote/demote oscillation. |
| `entry_threshold` | `f64` | `0.42` | Fraction of total CPU cycle share a non-prime thread must exceed (for `min_active_streak` consecutive iterations) to be promoted to prime status. |

## Default Implementation

```rust
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

The `Default` implementation provides conservative values suitable for most gaming and real-time workloads. The gap between `keep_threshold` (0.69) and `entry_threshold` (0.42) creates a hysteresis band that prevents a thread oscillating near a single threshold from being repeatedly promoted and demoted.

## Remarks

### Hysteresis mechanism

The prime thread scheduler uses a two-threshold hysteresis system:

1. **Entry:** A thread must sustain a CPU cycle share above `entry_threshold` for at least `min_active_streak` consecutive iterations before being promoted.
2. **Retention:** Once promoted, a thread remains prime as long as its cycle share stays above `keep_threshold`.
3. **Demotion:** When a prime thread's cycle share drops below `keep_threshold`, it is immediately demoted.

Because `entry_threshold` is higher than `keep_threshold` (in the default configuration `keep_threshold` is the larger value acting as the retention bar, while `entry_threshold` is the lower initial qualification bar), a thread that barely qualifies for prime status is not immediately at risk of demotion.

### Configuration syntax

Constants are defined at the top of the config file using the `@NAME = value` syntax:

```
@MIN_ACTIVE_STREAK = 3
@KEEP_THRESHOLD = 0.05
@ENTRY_THRESHOLD = 0.1
```

Parsing is handled by [parse_constant](parse_constant.md). Unknown constant names produce a warning but do not cause a parse error.

### Propagation on hot-reload

When the configuration file is hot-reloaded via [hotreload_config](hotreload_config.md), the new `ConfigConstants` are copied into the [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) instance so that the updated thresholds take effect on the next polling iteration.

## Requirements

| | |
|---|---|
| **Module** | `src/config.rs` |
| **Callers** | [read_config](read_config.md) (populates via [parse_constant](parse_constant.md)), [hotreload_config](hotreload_config.md) (propagates to scheduler) |
| **Consumers** | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| **Dependencies** | None (plain data struct) |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [config.rs](README.md) |
| Constant parser | [parse_constant](parse_constant.md) |
| Prime thread scheduler | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| Thread-level config | [ThreadLevelConfig](ThreadLevelConfig.md) |
| Hot-reload mechanism | [hotreload_config](hotreload_config.md) |
| Config result container | [ConfigResult](ConfigResult.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*