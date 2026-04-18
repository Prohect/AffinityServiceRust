# read_config function (config.rs)

Reads and parses an entire AffinityServiceRust configuration file, returning a fully populated [`ConfigResult`](ConfigResult.md) containing all process-level rules, thread-level rules, constants, aliases, group expansions, and any errors or warnings produced during parsing. This is the primary entry point for loading configuration from disk.

## Syntax

```AffinityServiceRust/src/config.rs#L743-875
pub fn read_config<P: AsRef<Path>>(path: P) -> ConfigResult
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `path` | `P: AsRef<Path>` | The file system path to the configuration file to read (e.g., `"config.ini"`). Accepts any type that implements `AsRef<Path>`, including `&str`, `String`, and `PathBuf`. |

## Return value

Type: [`ConfigResult`](ConfigResult.md)

A fully populated configuration result struct containing:

- `process_level_configs` — All parsed process-level rules organized by grade.
- `thread_level_configs` — All parsed thread-level rules organized by grade.
- `constants` — Tunable scheduler constants parsed from `@NAME = value` lines (or defaults).
- Alias, group, and rule counts for diagnostic reporting.
- `errors` — A list of fatal errors. When non-empty, the configuration is considered invalid (`is_valid()` returns `false`).
- `warnings` — A list of non-fatal warnings.

If the file cannot be opened, the function returns a `ConfigResult` with a single error message in the `errors` vector and all other fields at their default values.

## Remarks

### File format overview

The configuration file is a line-oriented text format. Each line is one of:

| Line type | Prefix | Handler | Description |
|-----------|--------|---------|-------------|
| Comment | `#` | Skipped | Comments and blank lines are ignored. |
| Constant | `@` | [`parse_constant`](parse_constant.md) | Scheduler tuning constants (e.g., `@MIN_ACTIVE_STREAK = 3`). |
| Alias | `*` | [`parse_alias`](parse_alias.md) | Named CPU specifications (e.g., `*pcore = 0-7`). |
| Group block | `{` | [`collect_group_block`](collect_group_block.md) + [`parse_and_insert_rules`](parse_and_insert_rules.md) | Multi-process group rules enclosed in braces. |
| Process rule | *(other)* | [`parse_and_insert_rules`](parse_and_insert_rules.md) | Individual `name:priority:affinity:...` rule lines. |

### Parsing algorithm

1. The file is opened and read into a buffered reader. All lines are collected into a `Vec<String>`.
2. A local `cpu_aliases` hash map is initialized to store alias definitions encountered during the pass.
3. The parser iterates through lines sequentially using an index variable `i`:
   - **Blank lines and comments** (`#`-prefixed) are skipped.
   - **Constants** (`@`-prefixed) are split on `=` and dispatched to [`parse_constant`](parse_constant.md).
   - **Aliases** (`*`-prefixed) are split on `=` and dispatched to [`parse_alias`](parse_alias.md). The parsed alias is stored in `cpu_aliases` for use by subsequent rule lines.
   - **Group blocks** (lines containing `{`) are handled in two sub-cases:
     - *Single-line groups*: Both `{` and `}` appear on the same line. Members are extracted inline.
     - *Multi-line groups*: The opening `{` is on this line but `}` is not. The parser calls [`collect_group_block`](collect_group_block.md) to scan forward through subsequent lines until `}` is found, advancing `i` past the closing brace.
     - In both cases, the group name (text before `{`) is captured for diagnostics. An empty name produces the label `"anonymous@L{line_number}"`.
     - The collected members and rule suffix are passed to [`parse_and_insert_rules`](parse_and_insert_rules.md).
   - **Individual rules**: Lines not matching any of the above patterns are split on `:`, with the first element as the process name and the rest as rule fields. At least 3 colon-separated parts are required (name, priority, affinity). The name is lowercased and passed to [`parse_and_insert_rules`](parse_and_insert_rules.md).
4. After all lines are processed, the populated `ConfigResult` is returned.

### Error handling strategy

The parser is designed to be **resilient** — it collects all errors and warnings rather than aborting on the first problem. This allows users to see every issue in their config file in a single validation pass. Fatal errors (e.g., unclosed groups, too few fields, invalid constant syntax) are appended to `result.errors`. Non-fatal issues (e.g., unknown priority strings, empty groups, redundant rules) are appended to `result.warnings`.

### Key error conditions

| Condition | Severity | Message pattern |
|-----------|----------|-----------------|
| File cannot be opened | Error | `"Cannot open config file: {io_error}"` |
| Constant line missing `=` | Error | `"Line {n}: Invalid constant - expected '@NAME = value'"` |
| Alias line missing `=` | Error | `"Line {n}: Invalid alias - expected '*name = cpu_spec'"` |
| Unclosed group block | Error | `"Line {n}: Unclosed group '{label}' - missing }"` |
| Group with no members | Warning | `"Line {n}: Group '{label}' has no members"` |
| Group with no rule suffix | Error | `"Line {n}: Group '{label}' missing rule - use }:priority:affinity,..."` |
| Individual line with < 3 fields | Error | `"Line {n}: Too few fields - expected name:priority:affinity,..."` |
| Empty process name | Error | `"Line {n}: Empty process name"` |

### Line numbering

Line numbers in error and warning messages are 1-based (`line_number = i + 1`), matching what users see in their text editors.

### Ordering constraints

- **Aliases must be defined before use.** The parser processes lines sequentially, so a `*alias` reference in a rule will fail to resolve if the corresponding `*alias = cpu_spec` definition appears later in the file.
- **Constants** can appear anywhere and are applied immediately; they do not affect rule parsing.
- **Rules** can appear in any order. Redundant definitions overwrite earlier ones with a warning.

### Hot-reload usage

`read_config` is called both at startup and during hot-reload by [`hotreload_config`](hotreload_config.md). During hot-reload, the function parses the modified file into a fresh `ConfigResult`. If `is_valid()` is `true`, the new config replaces the active one; otherwise, the previous config is retained and errors are logged.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `config.rs` |
| Visibility | `pub` |
| Callers | `main.rs` (startup), [`hotreload_config`](hotreload_config.md) (runtime reload) |
| Callees | [`parse_constant`](parse_constant.md), [`parse_alias`](parse_alias.md), [`collect_group_block`](collect_group_block.md), [`collect_members`](collect_members.md), [`parse_and_insert_rules`](parse_and_insert_rules.md) |
| Dependencies | [`ConfigResult`](ConfigResult.md), `HashMap`, `List`, `CONSUMER_CPUS` from [`collections.rs`](../collections.rs/README.md) |
| I/O | Reads a file via `std::fs::File` and `std::io::BufReader` |
| Privileges | File system read access to the config file path |

## See Also

| Resource | Link |
|----------|------|
| ConfigResult | [ConfigResult](ConfigResult.md) |
| parse_constant | [parse_constant](parse_constant.md) |
| parse_alias | [parse_alias](parse_alias.md) |
| collect_group_block | [collect_group_block](collect_group_block.md) |
| parse_and_insert_rules | [parse_and_insert_rules](parse_and_insert_rules.md) |
| hotreload_config | [hotreload_config](hotreload_config.md) |
| convert | [convert](convert.md) |
| cli module | [cli.rs overview](../cli.rs/README.md) |
| config module overview | [README](README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*