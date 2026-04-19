# get_config_help_lines function (cli.rs)

Returns a vector of static string slices containing the configuration file format documentation template. This template is used both for interactive help display and for embedding as a header comment block in converted configuration files.

## Syntax

```rust
pub fn get_config_help_lines() -> Vec<&'static str>
```

## Parameters

This function takes no parameters.

## Return value

`Vec<&'static str>` — A vector containing one or more static string slices, each holding a multi-line comment block that documents the configuration file format. The strings use `##` comment prefixes so they can be written directly into `.ini` configuration files.

## Remarks

The returned template covers the following sections:

| Section | Description |
|---------|-------------|
| **Terminology** | Defines P-core, E-core, and thread notation (`p`, `pp`, `e`) for Intel hybrid CPU topologies. |
| **Config Format** | Documents the colon-delimited rule syntax: `process_name:priority:affinity:cpuset:prime_cpus:io_priority:memory_priority:ideal_processor:grade`. |
| **CPU Specification Formats** | Explains all supported CPU specification syntaxes: ranges (`0-7`), individual cores (`0;4;8`), single core (`7`), hex bitmask (`0xFF`), and alias references (`*pcore`). Warns that `7` means core 7, not a bitmask for cores 0–2. |
| **Priority Levels** | Lists valid values for `priority`, `io_priority`, and `memory_priority` fields. |
| **Ideal Processor Syntax** | Documents the `*alias[@prefix1;prefix2]` format for thread-to-CPU assignment based on start module matching. |
| **Process Groups** | Explains the `{ }` syntax for grouping multiple processes under a single rule, with named and anonymous variants. |

### Usage contexts

- **[print_config_help](print_config_help.md)** iterates over the returned vector and logs each element to the console.
- **[print_help_all](print_help_all.md)** calls `print_config_help` as the second half of the full help output.
- The `convert` function in `config.rs` embeds these lines as header comments in generated configuration files so that users have inline documentation.

### Design note

The function returns `Vec<&'static str>` rather than a single `&'static str` to allow callers to iterate and process individual blocks independently. Currently, the vector contains a single element, but the signature supports future expansion into multiple logical sections.

## Requirements

| | |
|---|---|
| **Module** | `src/cli.rs` |
| **Callers** | [print_config_help](print_config_help.md), [print_help_all](print_help_all.md), `config::convert` |
| **Callees** | None |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [cli.rs](README.md) |
| Prints the config help to console | [print_config_help](print_config_help.md) |
| Full help printer | [print_help_all](print_help_all.md) |
| CLI argument parser | [parse_args](parse_args.md) |
| Configuration parser | [ConfigResult](../config.rs/ConfigResult.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*