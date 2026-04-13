# convert function (config.rs)

Converts a Process Lasso configuration file (UTF-16 LE encoded INI format) into the native AffinityServiceRust configuration format. The function reads named affinities, default priorities, and default affinities from the Process Lasso file, maps them to equivalent AffinityServiceRust rule syntax, and writes the result to an output file in UTF-8 with CPU aliases and per-process rule lines.

## Syntax

```rust
pub fn convert(in_file: Option<String>, out_file: Option<String>)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `in_file` | `Option<String>` | Path to the input Process Lasso configuration file (UTF-16 LE encoded). If `None`, an error message is logged and the function returns immediately. Passed to [read_utf16le_file](read_utf16le_file.md) for decoding. |
| `out_file` | `Option<String>` | Path to the output file where the converted AffinityServiceRust configuration will be written. If `None`, an error message is logged and the function returns immediately. The file is created (or overwritten) using `File::create`. |

## Return value

This function does not return a value. Results are written to the output file and progress/error messages are emitted via the `log!` macro.

## Remarks

### Process Lasso INI format

The function parses three specific INI keys from the Process Lasso configuration:

| Key | Format | Description |
|-----|--------|-------------|
| `NamedAffinities=` | `alias1,cpuspec1,alias2,cpuspec2,...` | Comma-separated pairs of alias name and CPU specification. Each pair is converted to a `*alias = cpu_spec` line in the output. |
| `DefaultPriorities=` | `process1,priority1,process2,priority2,...` | Comma-separated pairs of process name and priority value. Priority values can be human-readable strings (`"idle"`, `"high"`) or numeric codes (`1`–`6`). |
| `DefaultAffinitiesEx=` | `process1,mask1,cpuset1,process2,mask2,cpuset2,...` | Comma-separated triples of process name, legacy affinity mask (often `0`, ignored), and CPU set specification. Only entries where the CPU set is non-empty and not `"0"` are included. |

### Conversion algorithm

1. **Read input** — The input file is read via [read_utf16le_file](read_utf16le_file.md), which handles the UTF-16 LE to UTF-8 conversion.
2. **Parse INI keys** — The function iterates over lines, extracting values from the three recognized keys into intermediate data structures:
   - `named_affinities: Vec<(String, String)>` — alias name/CPU spec pairs.
   - `priorities: HashMap<String, String>` — process name → priority string.
   - `affinities: HashMap<String, String>` — process name → CPU set string.
3. **Build alias reverse map** — A `spec_to_alias` map is built from `named_affinities`, allowing affinity values to be replaced with `*alias` references in the output for readability.
4. **Generate output** — The output file is assembled in memory as a `Vec<String>`:
   a. Config help lines (from `get_config_help_lines()`) are prepended as comments.
   b. A `"# Converted from Process Lasso config"` header is added.
   c. CPU alias definitions (`*alias = cpu_spec`) are written for each named affinity.
   d. For each unique process name (union of `priorities` and `affinities` keys, sorted alphabetically), a rule line is emitted in the format `name:priority:affinity:0:0:none:none`.
5. **Write output** — The assembled lines are written to the output file, one per line, using `writeln!`.

### Priority mapping

Process Lasso priority values are mapped to AffinityServiceRust priority strings:

| Process Lasso value | AffinityServiceRust equivalent |
|---------------------|-------------------------------|
| `idle` or `1` | `idle` |
| `below normal` or `2` | `below normal` |
| `normal` or `3` | `normal` |
| `above normal` or `4` | `above normal` |
| `high` or `5` | `high` |
| `realtime` / `real time` or `6` | `real time` |
| *(anything else)* | `none` |

String matching is case-insensitive (values are lowercased before comparison).

### Affinity alias substitution

When a process's affinity CPU specification exactly matches a known named affinity's CPU specification, the output uses the `*alias` reference instead of the raw spec. This keeps the converted file concise and makes it easier to adjust CPU assignments by editing a single alias line.

### Output format

The generated output is a valid AffinityServiceRust configuration file. Each process rule uses the full 7-field format with explicit defaults for unused fields:

```text
process.exe:priority:affinity:0:0:none:none
```

Where:
- Fields 3–4 (`cpuset`, `prime_cpus`) are set to `0` (disabled).
- Fields 5–6 (`io_priority`, `memory_priority`) are set to `none`.

This provides a clean starting point that users can then customize with cpuset, prime-thread, and other advanced fields.

### Error handling

| Condition | Behavior |
|-----------|----------|
| `in_file` is `None` | Logs `"Error: -in <file> is required for -convert"` and returns. |
| `out_file` is `None` | Logs `"Error: -out <file> is required for -convert"` and returns. |
| Input file cannot be read | Logs `"Failed to read {path}: {error}"` and returns. |
| Output file cannot be created | Logs `"Failed to create {path}: {error}"` and returns. |
| Write failure | Logs `"Failed to write to {path}"` and returns. |

No errors are propagated to the caller — all failures are reported via `log!` and the function returns gracefully.

### Logging

On success, the function logs:

```text
Parsed {N} aliases, {N} priorities, {N} affinities
Converted {in_path} to {out_path}
```

### CLI integration

This function is invoked when the user passes the `-convert` flag on the command line. The `-in` and `-out` arguments supply the `in_file` and `out_file` parameters respectively. See the [cli module](../cli.rs/README.md) for details on argument parsing.

### Limitations

- Only the three recognized INI keys are parsed. Other Process Lasso settings (e.g., I/O priorities, power plans, watchdog rules) are not converted.
- The function does not attempt to merge or deduplicate rules that might have conflicting priorities and affinities for the same process — it simply outputs whatever was found in the source file.
- CPU specifications are passed through as-is from the Process Lasso file without re-normalization. If the Process Lasso file uses unusual formats, manual cleanup of the output may be needed.
- The input file must be UTF-16 LE encoded. UTF-8 or other encodings will produce garbled output without an explicit error.

## Requirements

| | |
|---|---|
| **Module** | `config` (`src/config.rs`) |
| **Visibility** | `pub` |
| **Callers** | [main](../main.rs/main.md) (via `-convert` CLI flag) |
| **Callees** | [read_utf16le_file](read_utf16le_file.md), `get_config_help_lines` (from [cli module](../cli.rs/README.md)), `File::create`, `writeln!` |
| **API** | `std::fs::File::create` for output, [read_utf16le_file](read_utf16le_file.md) for input |
| **Privileges** | Read access to the input file, write access to the output file path |

## See Also

| Topic | Link |
|-------|------|
| UTF-16 LE file reader | [read_utf16le_file](read_utf16le_file.md) |
| Auto-grouping of converted output | [sort_and_group_config](sort_and_group_config.md) |
| Native config file reader | [read_config](read_config.md) |
| CPU spec parser | [parse_cpu_spec](parse_cpu_spec.md) |
| CLI argument parsing | [cli module](../cli.rs/README.md) |
| Config module overview | [README](README.md) |