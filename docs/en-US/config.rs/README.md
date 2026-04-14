# config module (AffinityServiceRust)

The `config` module is the configuration engine for AffinityServiceRust. It parses an INI-like configuration file into strongly-typed rule structures that the service loop uses to manage process priority, CPU affinity, CPU sets, I/O priority, memory priority, prime-thread scheduling, and ideal processor assignment. The module supports CPU aliases, named process groups, tuning constants, multi-field rules with per-process grading, and hot-reload of both the configuration file and the blacklist at runtime. It also provides utilities for converting Process Lasso configurations and auto-grouping redundant rules.

## Structs

| Struct | Description |
|--------|-------------|
| [PrimePrefix](PrimePrefix.md) | Associates a module-name prefix filter with optional CPU set and thread priority for prime-thread scheduling. |
| [IdealProcessorRule](IdealProcessorRule.md) | Maps a set of CPUs to optional module-name prefix filters for ideal processor assignment. |
| [ProcessConfig](ProcessConfig.md) | Complete per-process configuration record holding all rule fields parsed from a single config entry. |
| [ConfigConstants](ConfigConstants.md) | Tunable hysteresis constants that control prime-thread promotion and demotion behavior. |
| [ConfigResult](ConfigResult.md) | Aggregate output of a config parse pass, containing the rule map, constants, statistics, errors, and warnings. |

## Functions

| Function | Description |
|----------|-------------|
| [parse_cpu_spec](parse_cpu_spec.md) | Parses a CPU specification string (ranges, hex masks, semicolon-separated indices) into a sorted `Vec<u32>`. |
| [mask_to_cpu_indices](mask_to_cpu_indices.md) | Converts a 64-bit bitmask into a sorted vector of CPU indices. |
| [cpu_indices_to_mask](cpu_indices_to_mask.md) | Converts a slice of CPU indices into a `usize` bitmask. |
| [format_cpu_indices](format_cpu_indices.md) | Formats a CPU index slice as a compact, human-readable range string. |
| [resolve_cpu_spec](resolve_cpu_spec.md) | Resolves a CPU spec string that may reference a `*alias`, falling back to [parse_cpu_spec](parse_cpu_spec.md). |
| [collect_members](collect_members.md) | Splits a colon-delimited text line into trimmed, lowercased member names. |
| [parse_constant](parse_constant.md) | Parses an `@CONSTANT = value` line and updates [ConfigResult](ConfigResult.md) constants. |
| [parse_alias](parse_alias.md) | Parses a `*alias = cpu_spec` line and inserts the alias into the CPU alias map. |
| [parse_ideal_processor_spec](parse_ideal_processor_spec.md) | Parses an ideal processor specification string into a vector of [IdealProcessorRule](IdealProcessorRule.md). |
| [collect_group_block](collect_group_block.md) | Collects process names from a multi-line `{ ... }` group block until the closing brace. |
| [parse_and_insert_rules](parse_and_insert_rules.md) | Parses the colon-separated rule fields and inserts a [ProcessConfig](ProcessConfig.md) for each member. |
| [read_config](read_config.md) | Main entry point: reads and parses a configuration file into a [ConfigResult](ConfigResult.md). |
| [read_list](read_list.md) | Reads a simple line-per-entry list file (used for blacklists). |
| [read_utf16le_file](read_utf16le_file.md) | Reads a UTF-16 LE encoded file and returns its content as a Rust `String`. |
| [parse_mask](parse_mask.md) | Convenience wrapper that parses a CPU spec string directly to a `usize` bitmask. |
| [convert](convert.md) | Converts a Process Lasso configuration file to AffinityServiceRust native format. |
| [sort_and_group_config](sort_and_group_config.md) | Auto-groups processes that share identical rule settings to reduce config duplication. |
| [hotreload_blacklist](hotreload_blacklist.md) | Checks the blacklist file's modification time and reloads it if changed. |
| [hotreload_config](hotreload_config.md) | Checks the config file's modification time and reloads it if changed, preserving the previous config on error. |

## See Also

| Topic | Link |
|-------|------|
| Process priority / IO priority / memory priority enums | [priority module](../priority.rs/README.md) |
| Rule application to live processes | [apply module](../apply.rs/README.md) |
| Prime-thread scheduler state | [scheduler module](../scheduler.rs/README.md) |
| CLI argument parsing and help text | [cli module](../cli.rs/README.md) |
| Windows API wrappers (CPU sets, handles) | [winapi module](../winapi.rs/README.md) |
| Service main loop | [main module](../main.rs/README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd