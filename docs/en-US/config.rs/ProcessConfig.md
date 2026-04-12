# ProcessConfig struct (config.rs)

Complete configuration for a single process, defining priority, CPU affinity, CPU sets, prime thread scheduling, I/O and memory priority, and ideal processor assignment rules.

## Syntax

```rust
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

## Fields

`name`

The lowercase process executable name (e.g., `"chrome.exe"`). Used as the lookup key when matching running processes.

`priority`

The desired [ProcessPriority](../priority.rs/ProcessPriority.md) class to apply. Set to `ProcessPriority::None` to leave priority unchanged.

`affinity_cpus`

List of CPU indices for the process affinity mask. Applied via `SetProcessAffinityMask`. An empty vector means no affinity change. Parsed from the affinity field in the config rule via [resolve_cpu_spec](resolve_cpu_spec.md).

`cpu_set_cpus`

List of CPU indices for the process default CPU set. Applied via `SetProcessDefaultCpuSets`. An empty vector means no CPU set change. Parsed from the cpuset field in the config rule.

`cpu_set_reset_ideal`

When `true`, [reset_thread_ideal_processors](../apply.rs/reset_thread_ideal_processors.md) is called after applying the CPU set, distributing thread ideal processors across `cpu_set_cpus`. Enabled by prefixing the cpuset field value with `@` in the config rule.

`prime_threads_cpus`

List of CPU indices to which top threads are promoted by the prime thread scheduler. An empty vector disables prime thread scheduling.

`prime_threads_prefixes`

List of [PrimePrefix](PrimePrefix.md) rules that control which threads are eligible for prime scheduling based on their start module. When only a default entry (empty prefix) is present, all threads are eligible.

`track_top_x_threads`

Number of top threads to track by CPU cycle usage. A positive value enables both tracking and prime scheduling. A negative value enables tracking only (no promotion/demotion). Zero disables tracking.

`io_priority`

The desired [IOPriority](../priority.rs/IOPriority.md) to apply. Set to `IOPriority::None` to leave I/O priority unchanged.

`memory_priority`

The desired [MemoryPriority](../priority.rs/MemoryPriority.md) to apply. Set to `MemoryPriority::None` to leave memory priority unchanged.

`ideal_processor_rules`

List of [IdealProcessorRule](IdealProcessorRule.md) entries for assigning ideal processors to threads. Rules are evaluated in order; the first matching rule (by module prefix) determines the CPU set for round-robin ideal processor assignment.

## Remarks

`ProcessConfig` instances are constructed by [parse_and_insert_rules](parse_and_insert_rules.md) during config file parsing and stored in [ConfigResult](ConfigResult.md).configs, keyed by grade and then by process name.

The config rule format parsed by [read_config](read_config.md) is:

```
name:priority:affinity:cpuset:prime:io_priority:memory_priority:ideal:grade
```

Each field maps directly to a `ProcessConfig` field. Fields beyond `affinity` are optional and default to no-op values.

At runtime, [apply_config](../main.rs/apply_config.md) passes this struct to the individual apply functions in the [apply](../apply.rs/README.md) module: [apply_priority](../apply.rs/apply_priority.md), [apply_affinity](../apply.rs/apply_affinity.md), [apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md), [apply_prime_threads](../apply.rs/apply_prime_threads.md), [apply_io_priority](../apply.rs/apply_io_priority.md), [apply_memory_priority](../apply.rs/apply_memory_priority.md), and [apply_ideal_processors](../apply.rs/apply_ideal_processors.md).

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/config.rs |
| **Defined at** | Line 36 |
| **Derives** | `Debug`, `Clone` |
| **Used by** | [apply_config](../main.rs/apply_config.md), [parse_and_insert_rules](parse_and_insert_rules.md), [ConfigResult](ConfigResult.md) |