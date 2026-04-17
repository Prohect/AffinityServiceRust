# ProcessLevelConfig type (config.rs)

The `ProcessLevelConfig` struct holds all process-level settings for a single process rule. Each instance represents the complete set of OS-level attributes that AffinityServiceRust will apply to a matched process, including priority class, hard CPU affinity, soft CPU set preference, I/O priority, and memory priority.

## Syntax

```AffinityServiceRust/src/config.rs#L30-38
#[derive(Debug, Clone)]
pub struct ProcessLevelConfig {
    pub name: String,
    pub priority: ProcessPriority,
    pub affinity_cpus: List<[u32; CONSUMER_CPUS]>,
    pub cpu_set_cpus: List<[u32; CONSUMER_CPUS]>,
    pub cpu_set_reset_ideal: bool,
    pub io_priority: IOPriority,
    pub memory_priority: MemoryPriority,
}
```

## Members

| Member | Type | Description |
|--------|------|-------------|
| `name` | `String` | The lowercase executable name (e.g., `"game.exe"`) that this rule matches against. This is the key used for lookup in the `ConfigResult.process_level_configs` hash map. |
| `priority` | `ProcessPriority` | The Windows process priority class to apply (e.g., `Idle`, `BelowNormal`, `Normal`, `AboveNormal`, `High`, `RealTime`). A value of `ProcessPriority::None` means the priority is left unchanged. |
| `affinity_cpus` | `List<[u32; CONSUMER_CPUS]>` | A sorted list of CPU indices that form the hard affinity mask for the process. When non-empty, the process (and its child threads) are restricted to these logical processors via `SetProcessAffinityMask`. An empty list means no affinity change is applied. |
| `cpu_set_cpus` | `List<[u32; CONSUMER_CPUS]>` | A sorted list of CPU indices that define the soft CPU preference via the Windows CPU Sets API (`SetProcessDefaultCpuSets`). Unlike hard affinity, CPU Sets are advisory — the OS may schedule threads on other cores under load. An empty list means no CPU set is configured. |
| `cpu_set_reset_ideal` | `bool` | When `true`, the ideal processor assignment for all threads in the process is reset after applying the CPU set. This is triggered by prefixing the cpuset field with `@` in the configuration file (e.g., `@*ecore`). |
| `io_priority` | `IOPriority` | The I/O priority level to apply to the process (e.g., `VeryLow`, `Low`, `Normal`, `High`). `IOPriority::None` means no change. Note that `High` typically requires administrator privileges. |
| `memory_priority` | `MemoryPriority` | The memory page priority to apply to the process (e.g., `VeryLow`, `Low`, `Medium`, `BelowNormal`, `Normal`). `MemoryPriority::None` means no change. Lower memory priority causes the process's pages to be reclaimed more aggressively by the working set manager. |

## Remarks

- A `ProcessLevelConfig` instance is created by [`parse_and_insert_rules`](parse_and_insert_rules.md) only when at least one process-level field has a non-default value. If all fields evaluate to `None` or empty, no `ProcessLevelConfig` is inserted (though a [`ThreadLevelConfig`](ThreadLevelConfig.md) may still be created for the same process if thread-level fields are present).

- The struct is stored inside `ConfigResult.process_level_configs`, which is a two-level `HashMap<u32, HashMap<String, ProcessLevelConfig>>`. The outer key is the **grade** (rule application frequency), and the inner key is the lowercase process name. A grade of `1` means the rule runs every polling loop; a grade of `N` means the rule runs every Nth loop.

- Hard affinity (`affinity_cpus`) and soft CPU sets (`cpu_set_cpus`) serve different purposes and can be used independently or together:
  - **Affinity** is a strict constraint inherited by child processes.
  - **CPU sets** are a scheduling hint that is not inherited and can be overridden by the OS.

- The `cpu_set_reset_ideal` flag addresses a specific Windows scheduling behavior where setting a CPU set does not automatically clear previously assigned ideal processors. Enabling this flag ensures that threads are redistributed across the new CPU set rather than remaining pinned to their prior ideal processor.

- The struct derives `Debug` and `Clone`. Cloning is necessary because the same rule may be applied to multiple processes within a group block.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `config.rs` |
| Constructed by | [`parse_and_insert_rules`](parse_and_insert_rules.md) |
| Stored in | [`ConfigResult`](ConfigResult.md)`.process_level_configs` |
| Consumed by | `apply.rs` (process-level application logic) |
| Dependencies | `ProcessPriority`, `IOPriority`, `MemoryPriority` from [`priority.rs`](../priority.rs/README.md); `List` and `CONSUMER_CPUS` from [`collections.rs`](../collections.rs/README.md) |

## See Also

| Resource | Link |
|----------|------|
| ThreadLevelConfig | [ThreadLevelConfig](ThreadLevelConfig.md) |
| ConfigResult | [ConfigResult](ConfigResult.md) |
| parse_and_insert_rules | [parse_and_insert_rules](parse_and_insert_rules.md) |
| read_config | [read_config](read_config.md) |
| priority module | [priority.rs overview](../priority.rs/README.md) |
| config module overview | [README](README.md) |

---
*Commit: [37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*