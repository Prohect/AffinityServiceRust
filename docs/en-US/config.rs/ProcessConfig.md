# ProcessConfig struct (config.rs)

Holds the complete, parsed configuration for a single process rule. Each field corresponds to one colon-separated segment of a config file rule line. The service loop matches running processes by name (case-insensitive) and applies the settings described by this struct through the Windows API.

## Syntax

```rust
#[derive(Debug, Clone)]
pub struct ProcessConfig {
    pub name: String,
    pub priority: ProcessPriority,
    pub affinity_cpus: Vec<u32>,
    pub cpu_set_cpus: Vec<u32>,
    pub cpu_set_reset_ideal: bool,
    pub prime_threads_cpus: Vec<u32>,
    pub prime_threads_prefixes: Vec<PrimePrefix>,
    pub track_top_x_threads: i32,
    pub io_priority: IOPriority,
    pub memory_priority: MemoryPriority,
    pub ideal_processor_rules: Vec<IdealProcessorRule>,
}
```

## Members

| Member | Type | Description |
|--------|------|-------------|
| `name` | `String` | Lowercased process image name (e.g., `"game.exe"`). Used as the lookup key when matching running processes. |
| `priority` | `ProcessPriority` | Windows process priority class to apply. `ProcessPriority::None` means no change. Corresponds to rule field 1 (e.g., `high`, `above normal`, `none`). |
| `affinity_cpus` | `Vec<u32>` | Sorted list of CPU indices for the process affinity mask. An empty vector means no affinity change. Set via `SetProcessAffinityMask`. Corresponds to rule field 2. |
| `cpu_set_cpus` | `Vec<u32>` | Sorted list of CPU indices for the process default CPU set. An empty vector means no CPU set change. Set via `SetProcessDefaultCpuSets`. Corresponds to rule field 3. |
| `cpu_set_reset_ideal` | `bool` | When `true`, `reset_thread_ideal_processors` is called after applying the CPU set, distributing thread ideal processors across `cpu_set_cpus`. Enabled by prefixing the cpuset field value with `@` in the config rule (e.g., `@*alias` or `@0-7`). |
| `prime_threads_cpus` | `Vec<u32>` | Aggregate CPU indices used for prime-thread pinning. This is the union of all CPU sets referenced in the prime-threads field. An empty vector disables prime-thread scheduling for this process. Corresponds to rule field 4. |
| `prime_threads_prefixes` | `Vec<PrimePrefix>` | List of module-name prefix filters for prime-thread selection. Each entry can carry its own CPU subset and thread priority override. When no `@`-prefixed specs are present, contains a single wildcard entry with an empty prefix and `cpus: None`. |
| `track_top_x_threads` | `i32` | Controls thread tracking granularity. Positive values (e.g., `8`) track and prime the top N threads by cycle delta. Negative values (e.g., `-16`) track without applying prime scheduling (observation-only mode). `0` uses the default behavior. Set by `?N` (positive) or `??N` (negative) prefixes in the prime-threads field. |
| `io_priority` | `IOPriority` | I/O priority hint to apply via `NtSetInformationProcess`. `IOPriority::None` means no change. Corresponds to rule field 5. |
| `memory_priority` | `MemoryPriority` | Memory priority to apply via `SetProcessInformation`. `MemoryPriority::None` means no change. Corresponds to rule field 6. |
| `ideal_processor_rules` | `Vec<IdealProcessorRule>` | List of ideal processor assignment rules parsed from rule field 7. Each rule maps a CPU set to optional module-name prefix filters. An empty vector means no ideal processor management. |

## Remarks

### Rule line format

A config file rule line has the following colon-separated format:

```
name:priority:affinity:cpuset:prime_cpus:io_priority:memory_priority:ideal_processor:grade
```

Fields 1–2 (`priority`, `affinity`) are required; all subsequent fields are optional and default to `None`/empty/`0`/`1` when omitted.

### Grade system

The `grade` field (field 8, defaulting to `1`) is not stored on `ProcessConfig` itself. Instead, it determines which sub-map inside [ConfigResult](ConfigResult.md)`.configs` the entry is placed into. The `configs` field is a `HashMap<u32, HashMap<String, ProcessConfig>>` keyed by grade, allowing the service loop to process higher-grade rules first.

### Group expansion

When a process group is defined in the config file (using `{ member1: member2 }:rule` syntax), [parse_and_insert_rules](parse_and_insert_rules.md) creates one `ProcessConfig` clone per member, each with its own `name` field set to the individual member name.

### Redundant rule detection

If a process name already exists in any grade map when a new rule is parsed, [parse_and_insert_rules](parse_and_insert_rules.md) increments the redundant-rule counter and emits a warning. The previous definition is overwritten.

### CPU alias resolution

The `affinity_cpus`, `cpu_set_cpus`, and `prime_threads_cpus` fields support CPU alias references (e.g., `*perf`) that are resolved during parsing by [resolve_cpu_spec](resolve_cpu_spec.md). The resolved integer indices are stored in the struct.

### Application order

The apply module processes `ProcessConfig` fields in the following order per process:

1. **Process-level** (applied once): `priority` → `affinity_cpus` → `cpu_set_cpus` (+ optional `cpu_set_reset_ideal`) → `io_priority` → `memory_priority`
2. **Thread-level** (applied every loop iteration): `prime_threads_cpus`/`prime_threads_prefixes` → `ideal_processor_rules`

### Default instance

A `ProcessConfig` with all fields set to `None`/empty/`0` applies no changes to the matched process. This is equivalent to a rule line of `name:none:0`.

## Requirements

| | |
|---|---|
| **Module** | `config` (`src/config.rs`) |
| **Constructed by** | [parse_and_insert_rules](parse_and_insert_rules.md) |
| **Stored in** | [ConfigResult](ConfigResult.md)`.configs` |
| **Consumed by** | [apply_config_process_level](../main.rs/apply_config_process_level.md), [apply_config_thread_level](../main.rs/apply_config_thread_level.md), [apply_priority](../apply.rs/apply_priority.md), [apply_affinity](../apply.rs/apply_affinity.md), [apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md), [apply_io_priority](../apply.rs/apply_io_priority.md), [apply_memory_priority](../apply.rs/apply_memory_priority.md), [apply_prime_threads](../apply.rs/apply_prime_threads.md), [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |
| **Depends on** | [PrimePrefix](PrimePrefix.md), [IdealProcessorRule](IdealProcessorRule.md), `ProcessPriority`, `IOPriority`, `MemoryPriority` |

## See Also

| Topic | Link |
|-------|------|
| Prime-thread prefix filter | [PrimePrefix](PrimePrefix.md) |
| Ideal processor rule | [IdealProcessorRule](IdealProcessorRule.md) |
| Rule field parsing | [parse_and_insert_rules](parse_and_insert_rules.md) |
| Config file reader | [read_config](read_config.md) |
| Parsed config aggregate | [ConfigResult](ConfigResult.md) |
| Process priority enum | [ProcessPriority](../priority.rs/ProcessPriority.md) |
| I/O priority enum | [IOPriority](../priority.rs/IOPriority.md) |
| Memory priority enum | [MemoryPriority](../priority.rs/MemoryPriority.md) |
| CPU spec parsing | [parse_cpu_spec](parse_cpu_spec.md) |
| CPU alias resolution | [resolve_cpu_spec](resolve_cpu_spec.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd