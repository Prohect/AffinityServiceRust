# ConfigConstants struct (config.rs)

Holds tunable hysteresis constants that govern the prime-thread scheduler's promotion and demotion decisions. These values control how aggressively threads are promoted to performance cores and how reluctantly they are demoted back to efficiency cores, preventing rapid oscillation ("thrashing") of thread-to-core assignments.

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
| `min_active_streak` | `u8` | `2` | Minimum number of consecutive scheduling cycles a thread must appear in the top-N hottest threads before it becomes eligible for promotion to a performance core. Higher values add latency to promotion but reduce false positives from transient CPU spikes. |
| `keep_threshold` | `f64` | `0.69` | Fractional threshold (0.0–1.0) of the top thread's cycle delta that a currently-promoted thread must sustain to retain its prime status. A promoted thread whose delta drops below `top_delta * keep_threshold` is demoted. Higher values make demotion more aggressive. |
| `entry_threshold` | `f64` | `0.42` | Fractional threshold (0.0–1.0) of the top thread's cycle delta that an unpromoted thread must reach to be considered for promotion. Only threads exceeding `top_delta * entry_threshold` and meeting the `min_active_streak` requirement are promoted. Lower values widen the promotion window. |

## Remarks

### Default values

The `Default` implementation provides values tuned for typical hybrid-core desktop workloads:

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

These defaults keep the scheduler responsive (two-cycle streak) while ensuring that only threads consuming a meaningful share of CPU cycles are promoted (`entry_threshold = 0.42`) and that already-promoted threads are given reasonable headroom before demotion (`keep_threshold = 0.69`).

### Config file syntax

Constants are set in the configuration file using `@CONSTANT = value` lines, which are parsed by [parse_constant](parse_constant.md):

```
@MIN_ACTIVE_STREAK = 3
@KEEP_THRESHOLD = 0.75
@ENTRY_THRESHOLD = 0.50
```

Unknown constant names produce a warning but do not cause a parse error. Invalid values (e.g., non-numeric strings) are recorded as errors in [ConfigResult](ConfigResult.md).

### Interaction with the scheduler

After [read_config](read_config.md) returns a [ConfigResult](ConfigResult.md), the `constants` field is copied into the [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md). During each scheduling cycle the scheduler uses these thresholds to decide which threads to promote or demote:

1. **Streak gate** — A thread's `active_streak` counter (incremented each cycle it appears in the top-N) must reach `min_active_streak` before promotion is considered.
2. **Entry gate** — The thread's cycle delta must exceed `top_delta * entry_threshold`.
3. **Keep gate** — A currently-promoted thread is demoted if its cycle delta falls below `top_delta * keep_threshold`.

Because `keep_threshold > entry_threshold` by default, a thread that barely qualified for promotion has a buffer zone before it is demoted, reducing oscillation.

### Hot-reload behavior

When [hotreload_config](hotreload_config.md) detects a modified configuration file, it re-parses constants and copies the new values into the live scheduler. Changed thresholds take effect on the next scheduling cycle without restarting the service.

## Requirements

| | |
|---|---|
| **Module** | `config` (`src/config.rs`) |
| **Constructed by** | [read_config](read_config.md) (via [parse_constant](parse_constant.md) and `Default::default`) |
| **Consumed by** | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md), [hotreload_config](hotreload_config.md) |
| **Stored in** | [ConfigResult](ConfigResult.md) (field `constants`) |

## See Also

| Topic | Link |
|-------|------|
| Config parse output | [ConfigResult](ConfigResult.md) |
| Constant line parser | [parse_constant](parse_constant.md) |
| Prime-thread scheduler | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| Hot-reload of configuration | [hotreload_config](hotreload_config.md) |
| Module overview | [config module](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd