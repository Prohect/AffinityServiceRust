# convert function (config.rs)

Converts a Process Lasso configuration file into AffinityServiceRust native format. Reads a UTF-16LE encoded Process Lasso INI-style config and produces a UTF-8 config file with CPU aliases and per-process rule lines.

## Syntax

```rust
pub fn convert(in_file: Option<String>, out_file: Option<String>)
```

## Parameters

`in_file: Option<String>`

Path to the input Process Lasso configuration file. The file must be UTF-16LE encoded (the default encoding used by Process Lasso). If `None`, the function logs an error and returns immediately.

`out_file: Option<String>`

Path to the output AffinityServiceRust configuration file. The file is created or overwritten with UTF-8 encoded content. If `None`, the function logs an error and returns immediately.

## Return value

This function does not return a value. Success and failure are communicated through log messages.

## Remarks

### Process Lasso config format

The converter recognizes three INI-style key-value pairs from the Process Lasso configuration:

| Key | Format | Description |
|-----|--------|-------------|
| `NamedAffinities` | `alias,cpus,alias,cpus,...` | Named CPU affinity aliases (comma-separated pairs of name and CPU spec) |
| `DefaultPriorities` | `process,priority,process,priority,...` | Process priority assignments (comma-separated pairs of process name and numeric/string priority) |
| `DefaultAffinitiesEx` | `process,mask,cpuset,process,mask,cpuset,...` | Process CPU affinity assignments (comma-separated triples of process name, legacy mask, and CPU range) |

### Conversion pipeline

1. **Read input:** The file is read as UTF-16LE via [read_utf16le_file](read_utf16le_file.md).
2. **Parse sections:** Each recognized key is parsed into intermediate hash maps (`priorities`, `affinities`) and a named affinities list.
3. **Build alias reverse map:** A `spec_to_alias` map is constructed so that when a process's CPU spec matches a known named affinity, the output uses the `*alias` reference instead of the raw spec.
4. **Generate header:** The output begins with config help lines from `get_config_help_lines()` (the CLI module) followed by a conversion comment.
5. **Emit CPU aliases:** Each `NamedAffinities` entry is emitted as a `*name = cpu_spec` alias line.
6. **Emit process rules:** All unique process names (from both priorities and affinities maps) are sorted alphabetically and emitted as single-line rules in the format `name:priority:affinity:0:0:none:none`.
7. **Write output:** The generated lines are written to the output file.

### Priority mapping

Process Lasso uses both string and numeric priority identifiers. The converter maps them to AffinityServiceRust priority names:

| Process Lasso Value | AffinityServiceRust Value |
|---------------------|--------------------------|
| `"idle"` or `"1"` | `idle` |
| `"below normal"` or `"2"` | `below normal` |
| `"normal"` or `"3"` | `normal` |
| `"above normal"` or `"4"` | `above normal` |
| `"high"` or `"5"` | `high` |
| `"realtime"`, `"real time"`, or `"6"` | `real time` |
| Unrecognized | `none` |

### Limitations

- The converter only handles process-level settings (priority and affinity). Thread-level features like prime thread scheduling, ideal processor assignment, and IO/memory priorities are not present in Process Lasso configs and default to `0`/`none` in the output.
- CPU set information from `DefaultAffinitiesEx` is placed in the affinity field, not the CPU set field. The output format uses `name:priority:affinity:0:0:none:none` where the third field is the affinity and the CPU set field is `0` (unchanged).
- The legacy mask field in `DefaultAffinitiesEx` triples is ignored; only the CPU range (third element of each triple) is used.
- Named affinity alias matching is based on exact string comparison of the raw CPU spec. If a process's affinity spec doesn't exactly match a named affinity string, the raw spec is emitted instead of an alias reference.

### Error handling

- If either `in_file` or `out_file` is `None`, an error is logged and the function returns.
- If the input file cannot be read (via [read_utf16le_file](read_utf16le_file.md)), the error is logged and the function returns.
- If the output file cannot be created or written, the error is logged and the function returns.
- On success, a summary line is logged showing the number of aliases, priorities, and affinities parsed.

## Requirements

| | |
|---|---|
| **Module** | [`src/config.rs`](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/config.rs) |
| **Callers** | CLI dispatch (when `-convert` flag is used) |
| **Callees** | [read_utf16le_file](read_utf16le_file.md), `get_config_help_lines` from [cli.rs](../cli.rs/README.md) |
| **Dependencies** | [HashMap](../collections.rs/README.md), [HashSet](../collections.rs/README.md) |
| **Privileges** | File system read/write access |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [config.rs](README.md) |
| UTF-16LE file reader | [read_utf16le_file](read_utf16le_file.md) |
| Auto-grouping utility | [sort_and_group_config](sort_and_group_config.md) |
| CPU spec parser | [parse_cpu_spec](parse_cpu_spec.md) |
| Config file reader | [read_config](read_config.md) |
| CLI arguments | [CliArgs](../cli.rs/CliArgs.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*