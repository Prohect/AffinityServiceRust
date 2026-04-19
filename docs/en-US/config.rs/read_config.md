# read_config function (config.rs)

Main configuration file reader. Opens the specified file, iterates through all lines, and dispatches each line to the appropriate sub-parser based on its prefix character. Returns a fully populated [ConfigResult](ConfigResult.md) containing all parsed rules, aliases, constants, groups, errors, and warnings.

## Syntax

```rust
pub fn read_config<P: AsRef<Path>>(path: P) -> ConfigResult
```

## Parameters

`path: P`

A file system path to the configuration file. Accepts any type that implements `AsRef<Path>`, including `&str`, `String`, and `PathBuf`. The file is expected to be UTF-8 encoded with one directive per line.

## Return value

[ConfigResult](ConfigResult.md) — A struct containing:

- `process_level_configs` and `thread_level_configs` maps populated with parsed rules, keyed by grade and process name.
- `constants` populated from `@CONSTANT = value` directives (or defaults if none are specified).
- Counters for aliases, groups, group members, process rules, redundant rules, and thread-level configs.
- `errors` — fatal parse errors that render the config invalid.
- `warnings` — non-fatal issues that are logged but do not prevent the config from being used.

If the file cannot be opened, the function returns a `ConfigResult` with a single error message and no rules.

## Remarks

### Line dispatch

The function reads all lines into memory, then iterates with an index-based loop (to support multi-line group blocks that advance the index by more than one). Each non-empty, non-comment line is dispatched based on its leading character:

| Prefix | Handler | Description |
|--------|---------|-------------|
| `#` | (skip) | Comment line — ignored entirely. |
| `@` | [parse_constant](parse_constant.md) | Expects `@NAME = value`. Parses tuning constants like `MIN_ACTIVE_STREAK`, `KEEP_THRESHOLD`, `ENTRY_THRESHOLD`. |
| `*` | [parse_alias](parse_alias.md) | Expects `*name = cpu_spec`. Defines a named CPU alias that can be referenced by later rule lines. |
| `{` present | Group block | The line contains a group definition. If `}` is on the same line, it is an inline group; otherwise [collect_group_block](collect_group_block.md) reads subsequent lines until `}` is found. |
| (other) | Single rule | The line is split on `:` and the first segment is the process name. The remaining segments are forwarded to [parse_and_insert_rules](parse_and_insert_rules.md). |

### Parsing order

Directives are processed in file order. This means:

1. **Constants** (`@`) should appear before any rules that depend on tuning behavior, though they can appear anywhere.
2. **Aliases** (`*`) must be defined before any rule line that references them. A forward reference to an undefined alias produces an error.
3. **Groups and rules** can appear in any order; later definitions for the same process name overwrite earlier ones with a warning.

### Group handling

Group blocks come in two forms:

**Inline group** (single line):
```
group_name { proc1.exe: proc2.exe }:normal:*ecore:0:0:low:none:0:1
```

**Multi-line group:**
```
group_name {
    proc1.exe: proc2.exe
    proc3.exe
}:normal:*ecore:0:0:low:none:0:1
```

In both cases, the group name is extracted from the text before `{`. If the name is empty, an anonymous label `"anonymous@L{N}"` is generated from the line number. Members are collected by [collect_members](collect_members.md), and the rule suffix after `}:` is split and forwarded to [parse_and_insert_rules](parse_and_insert_rules.md). An unclosed group (missing `}`) produces a fatal error.

### Single-line rule handling

A non-group, non-directive line is expected to have the format:

```
process.exe:priority:affinity:cpuset:prime:io:memory:ideal:grade
```

The minimum required fields are 3 (name, priority, affinity). Fewer fields produce a fatal error. The process name (field 0) is lowercased and passed as a single-element member list to [parse_and_insert_rules](parse_and_insert_rules.md).

### Error handling

- If the config file cannot be opened (permissions, missing file), a single error is added and the function returns immediately.
- Parse errors from sub-parsers (constants, aliases, rules) are accumulated in `ConfigResult.errors` and `ConfigResult.warnings`. The caller should check [is_valid](ConfigResult.md) before using the result.

### CPU alias scope

Aliases are stored in a local `HashMap<String, List<[u32; CONSUMER_CPUS]>>` scoped to the `read_config` call. They are not persisted in the returned `ConfigResult` — only the `aliases_count` is retained. This means aliases are a parse-time concept and are fully resolved into concrete CPU index lists before being stored in rule structs.

## Requirements

| | |
|---|---|
| **Module** | [`src/config.rs`](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/config.rs) |
| **Callers** | [hotreload_config](hotreload_config.md), main service startup |
| **Callees** | [parse_constant](parse_constant.md), [parse_alias](parse_alias.md), [collect_members](collect_members.md), [collect_group_block](collect_group_block.md), [parse_and_insert_rules](parse_and_insert_rules.md) |
| **Dependencies** | `std::fs::File`, `std::io::BufReader`, [ConfigResult](ConfigResult.md), [HashMap](../collections.rs/README.md) |
| **Privileges** | File system read access to the config file path |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [config.rs](README.md) |
| Aggregated parse result | [ConfigResult](ConfigResult.md) |
| Rule insertion logic | [parse_and_insert_rules](parse_and_insert_rules.md) |
| Alias definitions | [parse_alias](parse_alias.md) |
| Constant definitions | [parse_constant](parse_constant.md) |
| Group block collector | [collect_group_block](collect_group_block.md) |
| Hot-reload wrapper | [hotreload_config](hotreload_config.md) |
| Process Lasso converter | [convert](convert.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*