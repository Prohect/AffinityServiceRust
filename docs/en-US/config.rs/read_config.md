# read_config function (config.rs)

Reads and parses an AffinityServiceRust configuration file from disk, returning a complete [ConfigResult](ConfigResult.md) containing all parsed process rules, constants, aliases, and any errors or warnings encountered during parsing.

## Syntax

```rust
pub fn read_config<P: AsRef<Path>>(path: P) -> ConfigResult
```

## Parameters

`path`

A file system path to the configuration file to parse. Accepts any type that implements `AsRef<Path>`, such as `&str`, `String`, or `PathBuf`.

## Return value

Returns a [ConfigResult](ConfigResult.md) struct containing:

- **configs** — A `HashMap<u32, HashMap<String, ProcessConfig>>` mapping grade tiers to process name → [ProcessConfig](ProcessConfig.md) entries.
- **constants** — Parsed [ConfigConstants](ConfigConstants.md), or defaults if none were specified.
- **errors** — A list of fatal parsing errors. If non-empty, the config should not be used.
- **warnings** — A list of non-fatal warnings (redundant rules, unknown values treated as defaults, etc.).
- **counts** — Statistics for constants, aliases, groups, group members, process rules, and redundant rules.

## Remarks

The function processes the configuration file line by line. Blank lines and lines starting with `#` are treated as comments and skipped. The parser recognizes three sections of directives, which may appear in any order:

### Constants section

Lines beginning with `@` define scheduler behavior constants. The format is `@NAME = value`. Recognized constants are parsed via [parse_constant](parse_constant.md):

- `@MIN_ACTIVE_STREAK = 2`
- `@KEEP_THRESHOLD = 0.69`
- `@ENTRY_THRESHOLD = 0.42`

### Aliases section

Lines beginning with `*` define CPU aliases. The format is `*name = cpu_spec`. Aliases are parsed via [parse_alias](parse_alias.md) and can be referenced in later rule fields using `*name` syntax. CPU specifications are resolved by [parse_cpu_spec](parse_cpu_spec.md).

- `*p = 0-7`
- `*e = 8-19`

### Process rules

Remaining lines define individual process rules or process groups.

**Individual rules** follow the format:

```
name:priority:affinity:cpuset:prime:io_priority:memory_priority:ideal:grade
```

Fields beyond `affinity` are optional. The process name and rule fields are split on `:` and delegated to [parse_and_insert_rules](parse_and_insert_rules.md).

**Group rules** use curly braces to define multiple processes sharing the same rule:

```
browsers {
  chrome.exe:
  firefox.exe:
  msedge.exe
}:high:*p:0:0:none:none
```

Single-line groups are also supported: `browsers { chrome.exe: firefox.exe }:high:*p:0:0:none:none`

Group members are extracted via [collect_members](collect_members.md). Multi-line groups are collected via [collect_group_block](collect_group_block.md). The rule suffix after the closing `}:` is then parsed by [parse_and_insert_rules](parse_and_insert_rules.md).

### CPU Set Reset Ideal

If the cpuset field is prefixed with `@` (e.g., `@*p`), the `cpu_set_reset_ideal` flag is set to `true` on the resulting [ProcessConfig](ProcessConfig.md), which triggers thread ideal processor redistribution after applying the CPU set. See [apply_priority](../apply.rs/apply_priority.md).

### Error handling

The parser is resilient — it continues processing after encountering errors on individual lines, collecting all errors and warnings into the result. Callers should check `ConfigResult::is_valid()` before using the parsed configs.

### Example

```
# Constants
@MIN_ACTIVE_STREAK = 3
@KEEP_THRESHOLD = 0.75

# Aliases
*p = 0-7
*e = 8-19

# Individual rule
game.exe:high:*p:*p:?8x*p@engine.dll:none:none:*p

# Group rule
browsers { chrome.exe: firefox.exe }:normal:*e:0:0:none:none
```

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/config.rs |
| **Signature** | `pub fn read_config<P: AsRef<Path>>(path: P) -> ConfigResult` |
| **Called by** | [main](../main.rs/main.md), [sort_and_group_config](sort_and_group_config.md) |
| **Calls** | [parse_constant](parse_constant.md), [parse_alias](parse_alias.md), [collect_members](collect_members.md), [collect_group_block](collect_group_block.md), [parse_and_insert_rules](parse_and_insert_rules.md) |