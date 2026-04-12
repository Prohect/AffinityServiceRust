# config.rs Module

Configuration file parsing, CPU specification handling, and config manipulation utilities for AffinityServiceRust.

This module is responsible for reading and interpreting the configuration file format, resolving CPU aliases, parsing process rules (including groups), and providing conversion/auto-grouping utilities.

## Data Structures

| Struct | Description |
| --- | --- |
| [ProcessConfig](ProcessConfig.md) | Complete configuration for a single process rule |
| [PrimePrefix](PrimePrefix.md) | Module-specific prefix rule for prime thread scheduling |
| [IdealProcessorRule](IdealProcessorRule.md) | Rule for assigning ideal processors to threads by module prefix |
| [ConfigConstants](ConfigConstants.md) | Scheduler behavior tuning constants |
| [ConfigResult](ConfigResult.md) | Aggregate result of config file parsing with statistics and diagnostics |

## CPU Specification Functions

| Function | Description |
| --- | --- |
| [parse_cpu_spec](parse_cpu_spec.md) | Parses a CPU specification string into a sorted list of CPU indices |
| [mask_to_cpu_indices](mask_to_cpu_indices.md) | Converts a 64-bit bitmask to a vector of CPU indices |
| [cpu_indices_to_mask](cpu_indices_to_mask.md) | Converts CPU indices to a bitmask (≤64 cores) |
| [format_cpu_indices](format_cpu_indices.md) | Formats CPU indices as a compact human-readable string |
| [parse_mask](parse_mask.md) | Convenience wrapper: parses a CPU spec string directly to a bitmask |

## Config Parsing Functions

| Function | Description |
| --- | --- |
| [read_config](read_config.md) | Main entry point — reads and parses a configuration file |
| [resolve_cpu_spec](resolve_cpu_spec.md) | Resolves a CPU spec that may contain an alias reference |
| [collect_members](collect_members.md) | Splits colon-separated text into group member names |
| [parse_constant](parse_constant.md) | Parses and applies a `@CONSTANT = value` line |
| [parse_alias](parse_alias.md) | Parses and registers a `*alias = cpu_spec` line |
| [parse_ideal_processor_spec](parse_ideal_processor_spec.md) | Parses an ideal processor specification with module prefix filtering |
| [collect_group_block](collect_group_block.md) | Collects members from a multi-line `{ ... }` group block |
| [parse_and_insert_rules](parse_and_insert_rules.md) | Parses rule fields and inserts [ProcessConfig](ProcessConfig.md) entries for all members |

## File I/O Functions

| Function | Description |
| --- | --- |
| [read_list](read_list.md) | Reads a simple line-based list file (e.g. blacklist) |
| [read_utf16le_file](read_utf16le_file.md) | Reads a UTF-16 LE encoded file into a Rust `String` |

## Conversion & Grouping Utilities

| Function | Description |
| --- | --- |
| [convert](convert.md) | Converts a Process Lasso INI config to native format |
| [sort_and_group_config](sort_and_group_config.md) | Auto-groups processes with identical rules to reduce duplication |

## See Also

- [priority.rs](../priority.rs/) — `ProcessPriority`, `IOPriority`, `MemoryPriority`, `ThreadPriority` enums
- [apply.rs](../apply.rs/) — Functions that consume [ProcessConfig](ProcessConfig.md) to apply settings
- [scheduler.rs](../scheduler.rs/) — Prime thread scheduler that uses [ConfigConstants](ConfigConstants.md)
- [cli.rs](../cli.rs/) — Command-line interface that invokes [read_config](read_config.md), [convert](convert.md), and [sort_and_group_config](sort_and_group_config.md)
