# parse_alias function (config.rs)

Parses a `*alias = cpu_spec` line from the configuration file, resolves the CPU specification into a list of CPU indices, and inserts the alias into the CPU alias map. Aliases provide symbolic names for CPU sets that can be referenced elsewhere in rule fields using the `*alias` syntax.

## Syntax

```rust
fn parse_alias(
    name: &str,
    value: &str,
    line_number: usize,
    cpu_aliases: &mut HashMap<String, Vec<u32>>,
    result: &mut ConfigResult,
)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `name` | `&str` | The alias name extracted from the portion between the leading `*` and the `=` sign, already trimmed and lowercased by the caller. For example, in `*perf = 0-7`, `name` is `"perf"`. |
| `value` | `&str` | The raw CPU specification string from the right-hand side of the `=` sign, already trimmed by the caller. Passed directly to [parse_cpu_spec](parse_cpu_spec.md) for resolution. |
| `line_number` | `usize` | The 1-based line number in the config file where the alias definition appears. Used for error and warning messages. |
| `cpu_aliases` | `&mut HashMap<String, Vec<u32>>` | Mutable reference to the alias map being built during config parsing. On success, the parsed alias is inserted (or overwrites a previous entry with the same name). |
| `result` | `&mut ConfigResult` | Mutable reference to the running [ConfigResult](ConfigResult.md). The `aliases_count` counter is incremented on success. Errors and warnings are pushed to `result.errors` and `result.warnings` respectively. |

## Return value

This function does not return a value. Results are communicated through mutations to `cpu_aliases` and `result`.

## Remarks

### Validation

- **Empty name** — If `name` is empty, an error is pushed to `result.errors` with the message `"Line {N}: Empty alias name"` and the function returns without inserting anything.
- **Empty CPU set** — If [parse_cpu_spec](parse_cpu_spec.md) returns an empty vector and `value` is not the literal string `"0"`, a warning is pushed indicating the alias resolved to an empty CPU set. The alias is still inserted with an empty vector; this allows the config file to define a placeholder alias that disables features referencing it.

### Overwriting

If an alias with the same `name` already exists in `cpu_aliases`, it is silently overwritten. No warning or error is generated for redefinition. This allows later alias lines to override earlier ones.

### Config file syntax

Alias lines in the configuration file follow this format:

```
*alias_name = cpu_spec
```

Where:

- `*` is the required prefix marker identifying the line as an alias definition.
- `alias_name` is a case-insensitive identifier (lowercased during parsing by the caller in [read_config](read_config.md)).
- `cpu_spec` is any format accepted by [parse_cpu_spec](parse_cpu_spec.md): hex mask (`0xFF`), range (`0-7`), semicolon list (`0;4;8`), or mixed (`0-3;8`).

**Examples:**

```
*perf = 0-7
*efficiency = 8-15
*all = 0-15
*gaming = 0;2;4;6
*legacy = 0xFF
```

### Alias usage

Once defined, aliases are referenced in rule fields by prefixing the alias name with `*`:

- **Affinity / CPU set / prime CPUs** — Resolved by [resolve_cpu_spec](resolve_cpu_spec.md) (e.g., `process.exe:high:*perf:*perf`).
- **Ideal processor rules** — Resolved by [parse_ideal_processor_spec](parse_ideal_processor_spec.md) (e.g., `*perf@engine.dll`).

### Parse order

Aliases must be defined before the rule lines that reference them. [read_config](read_config.md) processes the file top-to-bottom, so alias lines placed at the top of the file are available to all subsequent rules.

## Requirements

| | |
|---|---|
| **Module** | `config` (`src/config.rs`) |
| **Visibility** | Crate-private |
| **Called by** | [read_config](read_config.md) |
| **Callees** | [parse_cpu_spec](parse_cpu_spec.md) |
| **API** | No Windows API calls |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| CPU spec parser | [parse_cpu_spec](parse_cpu_spec.md) |
| Alias-aware CPU spec resolution | [resolve_cpu_spec](resolve_cpu_spec.md) |
| Config file reader (caller) | [read_config](read_config.md) |
| Constant parsing (analogous `@` lines) | [parse_constant](parse_constant.md) |
| Ideal processor spec (alias consumer) | [parse_ideal_processor_spec](parse_ideal_processor_spec.md) |
| Parse output aggregate | [ConfigResult](ConfigResult.md) |
| Module overview | [config module](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd