# ProcessLevelConfig struct (config.rs)

Represents a complete set of process-level tuning parameters for a single Windows process. Each instance captures the desired priority class, CPU affinity mask, CPU set assignment, IO priority, and memory priority. These settings are applied once when a matching process is first discovered by the service and are not re-applied on subsequent polling iterations unless the configuration is hot-reloaded.

## Syntax

```rust
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
| `name` | `String` | Lowercase process name (e.g., `"game.exe"`) used as the lookup key in the config hash map. Matches are performed case-insensitively against running process names. |
| `priority` | [ProcessPriority](../priority.rs/ProcessPriority.md) | Desired Windows priority class for the process. When set to `ProcessPriority::None`, the service does not modify the process's priority. Valid values include `Idle`, `BelowNormal`, `Normal`, `AboveNormal`, `High`, and `RealTime`. |
| `affinity_cpus` | `List<[u32; CONSUMER_CPUS]>` | Sorted list of logical processor indices defining the hard CPU affinity mask. An empty list means no affinity change is applied. When set, the service calls `SetProcessAffinityMask` and immediately redistributes thread ideal processors across the new CPU set. |
| `cpu_set_cpus` | `List<[u32; CONSUMER_CPUS]>` | Sorted list of logical processor indices for the process default CPU set (soft affinity). An empty list means no CPU set change is applied. Uses the Windows CPU Sets API (`SetProcessDefaultCpuSets`) which provides a softer scheduling hint than hard affinity. |
| `cpu_set_reset_ideal` | `bool` | When `true`, resets all thread ideal processor assignments after applying the CPU set. Triggered by prefixing the CPU set specification with `@` in the config file (e.g., `@*ecore`). Useful when combining CPU sets with ideal processor rules to ensure threads are redistributed after the CPU set change. |
| `io_priority` | [IOPriority](../priority.rs/IOPriority.md) | Desired IO priority for the process. Set via `NtSetInformationProcess` with `ProcessIoPriority`. When set to `IOPriority::None`, no IO priority change is applied. Valid values include `VeryLow`, `Low`, `Normal`, and `High`. |
| `memory_priority` | [MemoryPriority](../priority.rs/MemoryPriority.md) | Desired memory priority for the process. Set via `SetProcessInformation` with `ProcessMemoryPriority`. When set to `MemoryPriority::None`, no memory priority change is applied. Valid values include `VeryLow`, `Low`, `Medium`, `BelowNormal`, and `Normal`. |

## Remarks

### Config file format

A `ProcessLevelConfig` is constructed from a config rule line with the following positional fields:

```
process.exe:priority:affinity:cpuset:prime:io_priority:memory_priority:ideal:grade
```

Only the first two fields after the process name (`priority` and `affinity`) are required. Omitted fields default to their "no change" values (`None` / empty list / `false`).

### Validation during parsing

The [parse_and_insert_rules](parse_and_insert_rules.md) function creates a `ProcessLevelConfig` only when at least one process-level field has a non-default value. If all process-level fields are at their defaults (e.g., the rule only specifies thread-level settings like prime CPUs), no `ProcessLevelConfig` is inserted, and only a [ThreadLevelConfig](ThreadLevelConfig.md) is created.

### Affinity vs. CPU set

Both `affinity_cpus` and `cpu_set_cpus` control which processors a process can use, but they differ in enforcement:

| Feature | Affinity (`affinity_cpus`) | CPU Set (`cpu_set_cpus`) |
|---------|---------------------------|--------------------------|
| **API** | `SetProcessAffinityMask` | `SetProcessDefaultCpuSets` |
| **Enforcement** | Hard — threads cannot run on excluded CPUs | Soft — scheduler prefers listed CPUs but may spill |
| **Scope** | Limits the entire process | Sets default hint; individual threads can override |
| **Max CPUs** | 64 (single processor group) | Supports >64 CPUs across processor groups |

### Grade-based organization

`ProcessLevelConfig` instances are stored in a two-level `HashMap<u32, HashMap<String, ProcessLevelConfig>>` within [ConfigResult](ConfigResult.md). The outer key is the *grade* (application frequency), and the inner key is the process name. Grade 1 rules are checked every polling iteration; higher grades are checked less frequently, reducing overhead for low-priority processes.

### Redundancy detection

If multiple rules define a `ProcessLevelConfig` for the same process name, the parser emits a warning and the later definition overwrites the earlier one. The `redundant_rules_count` counter in [ConfigResult](ConfigResult.md) tracks how many such overwrites occurred.

## Requirements

| | |
|---|---|
| **Module** | [`src/config.rs`](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/config.rs) |
| **Created by** | [parse_and_insert_rules](parse_and_insert_rules.md) |
| **Consumed by** | [apply_priority](../apply.rs/apply_priority.md), [apply_affinity](../apply.rs/apply_affinity.md), [apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md), [apply_io_priority](../apply.rs/apply_io_priority.md), [apply_memory_priority](../apply.rs/apply_memory_priority.md) |
| **Dependencies** | [ProcessPriority](../priority.rs/ProcessPriority.md), [IOPriority](../priority.rs/IOPriority.md), [MemoryPriority](../priority.rs/MemoryPriority.md), [List](../collections.rs/README.md) |
| **Privileges** | `PROCESS_SET_INFORMATION` (write), `PROCESS_QUERY_LIMITED_INFORMATION` (read) |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [config.rs](README.md) |
| Thread-level counterpart | [ThreadLevelConfig](ThreadLevelConfig.md) |
| Rule parser | [parse_and_insert_rules](parse_and_insert_rules.md) |
| Config file reader | [read_config](read_config.md) |
| Aggregated parse result | [ConfigResult](ConfigResult.md) |
| CPU spec parser | [parse_cpu_spec](parse_cpu_spec.md) |
| CPU alias resolution | [resolve_cpu_spec](resolve_cpu_spec.md) |
| Priority enums | [ProcessPriority](../priority.rs/ProcessPriority.md), [IOPriority](../priority.rs/IOPriority.md), [MemoryPriority](../priority.rs/MemoryPriority.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*