# ThreadLevelConfig struct (config.rs)

Per-process thread-level configuration that controls prime thread scheduling and ideal processor assignment. Unlike [ProcessLevelConfig](ProcessLevelConfig.md) which is applied once when a process is first seen, `ThreadLevelConfig` rules are evaluated every polling iteration to track thread activity and dynamically reassign CPU resources.

## Syntax

```rust
#[derive(Debug, Clone)]
pub struct ThreadLevelConfig {
    pub name: String,
    pub prime_threads_cpus: List<[u32; CONSUMER_CPUS]>,
    pub prime_threads_prefixes: Vec<PrimePrefix>,
    pub track_top_x_threads: i32,
    pub ideal_processor_rules: Vec<IdealProcessorRule>,
}
```

## Members

| Member | Type | Description |
|--------|------|-------------|
| `name` | `String` | Lowercase process name (e.g. `"game.exe"`) used as the lookup key in the thread-level config map. |
| `prime_threads_cpus` | `List<[u32; CONSUMER_CPUS]>` | Union of all CPU indices eligible for prime thread pinning. This is the combined set across all [PrimePrefix](PrimePrefix.md) entries. When a thread is promoted to prime status, its CPU set is restricted to indices from this list (or from the prefix-specific subset if prefix matching is active). |
| `prime_threads_prefixes` | `Vec<PrimePrefix>` | List of [PrimePrefix](PrimePrefix.md) rules that control which threads are eligible for prime scheduling and which CPU subset each prefix group receives. An empty prefix string matches all threads. Multiple entries allow routing threads from different modules to different CPU sets. |
| `track_top_x_threads` | `i32` | Controls how many top threads (by CPU cycle consumption) to track. Positive values enable prime thread scheduling for the top N threads. Negative values enable tracking-only mode (metrics collected but no CPU pinning occurs). Zero disables thread tracking entirely. Parsed from `?N` (positive) or `??N` (negative) prefixes in the prime field. |
| `ideal_processor_rules` | `Vec<IdealProcessorRule>` | List of [IdealProcessorRule](IdealProcessorRule.md) entries that assign ideal processor hints to threads based on their start module prefix. Evaluated independently from prime thread scheduling. |

## Remarks

### Relationship with ProcessLevelConfig

A single config rule line can produce both a [ProcessLevelConfig](ProcessLevelConfig.md) and a `ThreadLevelConfig` for the same process. The [parse_and_insert_rules](parse_and_insert_rules.md) function determines whether thread-level fields are active (non-zero prime CPUs, non-zero tracking count, or non-empty ideal processor rules) and only creates a `ThreadLevelConfig` when at least one thread-level feature is in use.

### Prime thread scheduling

The prime thread system identifies the most CPU-intensive threads in a process and pins them to high-performance cores. The selection uses hysteresis controlled by [ConfigConstants](ConfigConstants.md) to avoid frequent toggling:

1. Each iteration, threads are ranked by CPU cycle delta.
2. Threads exceeding `entry_threshold` relative share begin accumulating an active streak.
3. Once a thread's streak reaches `min_active_streak`, it is promoted to prime status and pinned to `prime_threads_cpus`.
4. A prime thread is demoted only when its share drops below `keep_threshold`.

The `track_top_x_threads` field limits how many threads participate in this ranking. On systems with many threads, this avoids measuring every thread's cycles.

### Tracking-only mode

When `track_top_x_threads` is negative (parsed from `??N` syntax), the scheduler collects thread cycle statistics and logs them but does not perform any CPU set changes. This is useful for profiling thread behavior before committing to a prime configuration.

### Prefix-based CPU routing

The `prime_threads_prefixes` list enables routing different threads to different CPU subsets based on their start module. For example, a game's rendering threads (starting from `d3d11.dll`) could be pinned to P-cores while audio threads (starting from `xaudio2.dll`) go to E-cores. Each [PrimePrefix](PrimePrefix.md) can also carry an optional [ThreadPriority](../priority.rs/ThreadPriority.md) boost.

### Ideal processor assignment

The `ideal_processor_rules` field operates independently from prime scheduling. It sets the ideal processor hint on threads matching specific module prefixes, which the Windows scheduler uses as a preference (not a hard constraint). This is a lighter-weight alternative to prime thread pinning.

### Config field format

Thread-level settings are parsed from fields 4 (prime), and 7 (ideal processor) of a config rule line:

```
process.exe:priority:affinity:cpuset:prime_spec:io:memory:ideal_spec:grade
                                      ^field4                ^field7
```

The prime spec supports several forms:
- `*alias` — pin prime threads to alias CPUs
- `?8x*alias` — track top 8, pin to alias
- `??16` — track top 16, no pinning
- `*p@engine.dll;render.dll*e@audio.dll` — prefix-based routing with per-prefix CPU sets

### Storage structure

`ThreadLevelConfig` instances are stored in `ConfigResult.thread_level_configs`, a `HashMap<u32, HashMap<String, ThreadLevelConfig>>` where the outer key is the grade (polling frequency tier) and the inner key is the lowercase process name.

## Requirements

| | |
|---|---|
| **Module** | `src/config.rs` |
| **Constructed by** | [parse_and_insert_rules](parse_and_insert_rules.md) |
| **Consumed by** | [apply_prime_threads](../apply.rs/apply_prime_threads.md), [apply_ideal_processors](../apply.rs/apply_ideal_processors.md), main polling loop |
| **Dependencies** | [PrimePrefix](PrimePrefix.md), [IdealProcessorRule](IdealProcessorRule.md), [List](../collections.rs/README.md) |
| **Privileges** | None (data struct) |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [config.rs](README.md) |
| Process-level counterpart | [ProcessLevelConfig](ProcessLevelConfig.md) |
| Prime prefix rule | [PrimePrefix](PrimePrefix.md) |
| Ideal processor rule | [IdealProcessorRule](IdealProcessorRule.md) |
| Hysteresis constants | [ConfigConstants](ConfigConstants.md) |
| Prime thread scheduler state | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| Rule parser | [parse_and_insert_rules](parse_and_insert_rules.md) |
| Config file reader | [read_config](read_config.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*