# config module (AffinityServiceRust)

The `config` module is the configuration engine for AffinityServiceRust. It is responsible for parsing, validating, and hot-reloading the INI-style configuration file that defines CPU affinity, process priority, CPU set, I/O priority, memory priority, prime-thread scheduling, and ideal-processor rules for Windows processes. The module also provides utilities for converting Process Lasso configurations and auto-grouping duplicate rules.

## Functions

| Function | Description |
|----------|-------------|
| [parse_cpu_spec](parse_cpu_spec.md) | Parses a CPU specification string (ranges, hex masks, individual indices) into a sorted list of CPU indices. |
| [mask_to_cpu_indices](mask_to_cpu_indices.md) | Converts a 64-bit bitmask into a list of CPU index positions where bits are set. |
| [cpu_indices_to_mask](cpu_indices_to_mask.md) | Converts a slice of CPU indices into a `usize` bitmask (≤64 cores). |
| [format_cpu_indices](format_cpu_indices.md) | Formats a slice of CPU indices into a human-readable compact range string. |
| [resolve_cpu_spec](resolve_cpu_spec.md) | Resolves a CPU specification that may reference a `*alias` or a literal spec into CPU indices. |
| [collect_members](collect_members.md) | Splits a colon-delimited string of process names into a list of lowercase member names. |
| [parse_constant](parse_constant.md) | Parses and applies a `@NAME = value` constant definition to the configuration result. |
| [parse_alias](parse_alias.md) | Parses and registers a `*name = cpu_spec` alias definition. |
| [parse_ideal_processor_spec](parse_ideal_processor_spec.md) | Parses an ideal-processor specification string with optional module-prefix filtering into a list of rules. |
| [collect_group_block](collect_group_block.md) | Collects process names from a multi-line `{ … }` group block until the closing brace. |
| [parse_and_insert_rules](parse_and_insert_rules.md) | Parses rule fields from a config line and inserts process-level and thread-level config entries for all group members. |
| [read_config](read_config.md) | Reads and parses an entire configuration file, returning a fully populated `ConfigResult`. |
| [read_list](read_list.md) | Reads a text file and returns non-empty, non-comment lines as a list of lowercase strings. |
| [read_utf16le_file](read_utf16le_file.md) | Reads a file encoded as UTF-16 LE and returns its content as a Rust `String`. |
| [parse_mask](parse_mask.md) | Convenience function that parses a CPU spec string and returns the corresponding `usize` bitmask. |
| [convert](convert.md) | Converts a Process Lasso configuration file into AffinityServiceRust's native format. |
| [sort_and_group_config](sort_and_group_config.md) | Auto-groups processes that share identical rule settings to reduce configuration duplication. |
| [hotreload_blacklist](hotreload_blacklist.md) | Watches the blacklist file for modifications and reloads it when changed. |
| [hotreload_config](hotreload_config.md) | Watches the configuration file for modifications and hot-reloads it when changed. |

## Structs

| Struct | Description |
|--------|-------------|
| [PrimePrefix](PrimePrefix.md) | Associates a thread start-module prefix with optional CPU affinity and thread priority for prime-thread scheduling. |
| [IdealProcessorRule](IdealProcessorRule.md) | Maps a set of CPUs to optional module-name prefixes for ideal-processor assignment. |
| [ProcessLevelConfig](ProcessLevelConfig.md) | Holds all process-level settings for a single process rule: priority, affinity, CPU set, I/O priority, and memory priority. |
| [ThreadLevelConfig](ThreadLevelConfig.md) | Holds all thread-level settings for a single process rule: prime-thread CPUs, prefixes, tracking count, and ideal-processor rules. |
| [ConfigConstants](ConfigConstants.md) | Tunable numeric constants that control the prime-thread scheduler's behavior. |
| [ConfigResult](ConfigResult.md) | Aggregated output of the configuration parser, containing all parsed rules, constants, statistics, errors, and warnings. |

## See Also

| Resource | Link |
|----------|------|
| cli module | [cli.rs overview](../cli.rs/README.md) |
| scheduler module | [scheduler.rs overview](../scheduler.rs/README.md) |
| priority module | [priority.rs overview](../priority.rs/README.md) |
| apply module | [apply.rs overview](../apply.rs/README.md) |

---
*Commit: [37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*