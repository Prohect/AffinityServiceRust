# convert function (config.rs)

Converts a Process Lasso configuration file (UTF-16 LE encoded INI format) into the native AffinityServiceRust configuration format. The function reads named affinities, default priorities, and default affinities from the input file, maps them to equivalent AffinityServiceRust rule syntax, and writes the result to the specified output file with a full configuration reference header prepended.

## Syntax

```AffinityServiceRust/src/config.rs#L908-1063
pub fn convert(in_file: Option<String>, out_file: Option<String>)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `in_file` | `Option<String>` | The path to the input Process Lasso configuration file. This file must be encoded as UTF-16 LE (the default encoding used by Process Lasso). If `None`, the function logs an error and returns immediately. Corresponds to the `-in <file>` CLI argument. |
| `out_file` | `Option<String>` | The path to the output file where the converted AffinityServiceRust configuration will be written. If `None`, the function logs an error and returns immediately. Corresponds to the `-out <file>` CLI argument. |

## Return value

This function does not return a value. Output is written to the file specified by `out_file`. Diagnostic messages are emitted via the `log!` macro.

## Remarks

### Process Lasso INI keys parsed

The function scans the input file line by line and extracts data from three specific INI keys:

| Key | Format | Description |
|-----|--------|-------------|
| `NamedAffinities=` | `alias,cpus,alias,cpus,...` | Comma-separated pairs of alias name and CPU specification. Each pair is converted to a `*alias = cpus` alias definition in the output. |
| `DefaultPriorities=` | `process,priority,process,priority,...` | Comma-separated pairs of process name (lowercase) and priority value. Priority values can be string names (`"idle"`, `"normal"`, etc.) or numeric codes (`1`–`6`). |
| `DefaultAffinitiesEx=` | `process,mask,cpuset,process,mask,cpuset,...` | Comma-separated triples of process name, legacy bitmask (usually `0`, ignored), and CPU set specification. Only the CPU set value is used; entries where the CPU set is `"0"` or empty are skipped. |

### Priority mapping

Process Lasso numeric priority codes are mapped to AffinityServiceRust string names:

| Numeric code | String equivalent | AffinityServiceRust value |
|-------------|-------------------|---------------------------|
| `1` | `idle` | `idle` |
| `2` | `below normal` | `below normal` |
| `3` | `normal` | `normal` |
| `4` | `above normal` | `above normal` |
| `5` | `high` | `high` |
| `6` | `realtime` / `real time` | `real time` |
| Other | — | `none` |

### Output structure

The generated output file contains three sections in order:

1. **Configuration reference header** — the full configuration file help template from [`get_config_help_lines`](../cli.rs/get_config_help_lines.md), providing users with syntax documentation.
2. **CPU aliases** — one `*alias = cpu_spec` line for each entry in `NamedAffinities`, under the heading `# CPU Aliases (from Process Lasso NamedAffinities)`.
3. **Process rules** — one rule line per unique process found across `DefaultPriorities` and `DefaultAffinitiesEx`, sorted alphabetically. Each line follows the format: `name:priority:affinity:0:0:none:none`. Where possible, literal CPU specs are replaced with `*alias` references using a reverse lookup from the named affinities.

### Alias-to-spec reverse lookup

The function builds a `spec_to_alias` map that maps raw CPU specification strings back to their `*alias` names. When writing process rules, if a process's affinity spec matches a known named affinity, the `*alias` form is used instead of the raw spec. This produces cleaner, more readable output.

### File encoding

The input file is read via [`read_utf16le_file`](read_utf16le_file.md), which decodes UTF-16 LE byte pairs into a Rust `String` using lossy conversion. The output file is written as UTF-8.

### Error handling

| Condition | Behavior |
|-----------|----------|
| `in_file` is `None` | Logs `"Error: -in <file> is required for -convert"` and returns. |
| `out_file` is `None` | Logs `"Error: -out <file> is required for -convert"` and returns. |
| Input file cannot be read | Logs `"Failed to read {path}: {error}"` and returns. |
| Output file cannot be created | Logs `"Failed to create {path}: {error}"` and returns. |
| Write failure during output | Logs `"Failed to write to {path}"` and returns. |

On success, the function logs a summary: `"Parsed {n} aliases, {n} priorities, {n} affinities"` followed by `"Converted {in_path} to {out_path}"`.

### CLI usage

```/dev/null/example.sh#L1
AffinityServiceRust.exe -convert -in ProcessLassoConfig.ini -out config.ini
```

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `config.rs` |
| Visibility | `pub` |
| Callers | `main.rs` (when `cli.convert_mode` is `true`) |
| Callees | [`read_utf16le_file`](read_utf16le_file.md), [`get_config_help_lines`](../cli.rs/get_config_help_lines.md), `File::create`, `writeln!`, `log!` |
| Dependencies | `HashMap`, `HashSet` from [`collections.rs`](../collections.rs/README.md); `std::fs::File`, `std::io::Write` |
| Privileges | None (file system read/write access required) |

## See Also

| Resource | Link |
|----------|------|
| sort_and_group_config | [sort_and_group_config](sort_and_group_config.md) |
| read_utf16le_file | [read_utf16le_file](read_utf16le_file.md) |
| get_config_help_lines | [get_config_help_lines](../cli.rs/get_config_help_lines.md) |
| read_config | [read_config](read_config.md) |
| CliArgs | [CliArgs](../cli.rs/CliArgs.md) |
| config module overview | [README](README.md) |

---
*Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*