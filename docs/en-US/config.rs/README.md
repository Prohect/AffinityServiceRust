# config module (AffinityServiceRust)

The `config` module handles parsing, validation, and management of configuration files for AffinityServiceRust. It defines rule structures for process-level and thread-level scheduling policies and implements a multi-section INI-like config parser with support for CPU aliases (`*name = spec`), named groups (`name { members }`), tuning constants (`@NAME = value`), and complex CPU specifications (ranges, hex masks, semicolon-separated indices). The module also provides hot-reload capability for both config and blacklist files, a converter from Process Lasso format, and an auto-grouping utility that merges processes sharing identical rules.

## Structs

| Name | Description |
|------|-------------|
| [PrimePrefix](PrimePrefix.md) | Associates a module name prefix with a CPU set and optional thread priority boost for prime thread matching. |
| [IdealProcessorRule](IdealProcessorRule.md) | Maps thread start module prefixes to ideal CPU assignments. |
| [ProcessLevelConfig](ProcessLevelConfig.md) | Per-process rule applied once: priority, affinity, CPU set, IO priority, memory priority. |
| [ThreadLevelConfig](ThreadLevelConfig.md) | Per-process thread-level rule applied every polling iteration: prime threads, ideal processors, tracking. |
| [ConfigConstants](ConfigConstants.md) | Tuning constants for hysteresis in prime thread selection (streak, thresholds). |
| [ConfigResult](ConfigResult.md) | Aggregate result of config parsing: rule maps by grade, counters, errors, and warnings. |

## Functions

| Name | Description |
|------|-------------|
| [parse_cpu_spec](parse_cpu_spec.md) | Parses CPU specification strings (ranges, hex masks, semicolons) into a sorted list of CPU indices. |
| [mask_to_cpu_indices](mask_to_cpu_indices.md) | Converts a 64-bit bitmask to a sorted list of CPU indices. |
| [cpu_indices_to_mask](cpu_indices_to_mask.md) | Converts a CPU index slice to a `usize` bitmask. |
| [format_cpu_indices](format_cpu_indices.md) | Formats a CPU index slice as a compact range string like `"0-7,12-19"`. |
| [parse_mask](parse_mask.md) | Convenience wrapper: parses a CPU spec string directly to a bitmask. |
| [resolve_cpu_spec](resolve_cpu_spec.md) | Resolves a CPU spec that may be a `*alias` reference or a literal spec. |
| [collect_members](collect_members.md) | Splits colon-separated process names into a member list. |
| [parse_constant](parse_constant.md) | Parses `@NAME = value` constant definitions (MIN_ACTIVE_STREAK, KEEP_THRESHOLD, ENTRY_THRESHOLD). |
| [parse_alias](parse_alias.md) | Parses `*name = cpu_spec` alias definitions. |
| [parse_ideal_processor_spec](parse_ideal_processor_spec.md) | Parses ideal processor specifications like `*alias@prefix1;prefix2`. |
| [collect_group_block](collect_group_block.md) | Collects multi-line group block members between `{` and `}`. |
| [parse_and_insert_rules](parse_and_insert_rules.md) | Main rule parser: splits fields, validates, creates configs, inserts into [ConfigResult](ConfigResult.md) by grade. |
| [read_config](read_config.md) | Main config file reader. Handles constants, aliases, groups, and single-line rules. |
| [read_bleack_list](read_bleack_list.md) | Reads a blacklist file (one process name per line, `#` comments). |
| [read_utf16le_file](read_utf16le_file.md) | Reads a UTF-16LE encoded file and returns it as a Rust `String`. |
| [convert](convert.md) | Converts Process Lasso config format to AffinityServiceRust format. |
| [sort_and_group_config](sort_and_group_config.md) | Auto-groups rules with identical settings into named group blocks. |
| [hotreload_blacklist](hotreload_blacklist.md) | Hot-reloads the blacklist file if it has been modified on disk. |
| [hotreload_config](hotreload_config.md) | Hot-reloads the config file if modified, resetting scheduler state on success. |

## Config File Format

The configuration file uses a line-oriented format with several section types:

```
# Comments start with #

# Constants (tuning parameters)
@MIN_ACTIVE_STREAK = 3
@KEEP_THRESHOLD = 0.05
@ENTRY_THRESHOLD = 0.1

# CPU aliases
*pcore = 0-7
*ecore = 8-19

# Single-line rule
process.exe:priority:affinity:cpuset:prime_cpus:io_priority:memory_priority:ideal_processor:grade

# Named group rule
group_name { proc1.exe: proc2.exe: proc3.exe }:normal:*ecore:0:0:low:none:0:1

# Multi-line group
group_name {
    proc1.exe: proc2.exe
    proc3.exe
}:normal:*ecore:0:0:low:none:0:1
```

## See Also

| Topic | Link |
|-------|------|
| Enforcement engine | [apply.rs](../apply.rs/README.md) |
| Priority enums | [ProcessPriority](../priority.rs/ProcessPriority.md), [IOPriority](../priority.rs/IOPriority.md), [MemoryPriority](../priority.rs/MemoryPriority.md), [ThreadPriority](../priority.rs/ThreadPriority.md) |
| Prime thread scheduler | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| CLI arguments | [CliArgs](../cli.rs/CliArgs.md) |
| Collection types | [List / HashMap](../collections.rs/README.md) |
| Main service loop | [main.rs](../main.rs/README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*