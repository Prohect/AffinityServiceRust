# read_config function (config.rs)

Reads and parses a configuration file into a fully resolved [ConfigResult](ConfigResult.md) containing process rules, CPU aliases, tuning constants, and any errors or warnings encountered during parsing. This is the primary entry point for loading the AffinityServiceRust configuration and is called at startup and during hot-reload.

## Syntax

```rust
pub fn read_config<P: AsRef<Path>>(path: P) -> ConfigResult
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `path` | `P: AsRef<Path>` | File system path to the configuration file. Accepts any type that implements `AsRef<Path>`, including `&str`, `String`, and `PathBuf`. The file is opened with `File::open` and read line-by-line via a buffered reader. |

## Return value

Returns a [ConfigResult](ConfigResult.md) containing:

- **`configs`** — A `HashMap<u32, HashMap<String, ProcessConfig>>` keyed by grade then by lowercased process name. Each leaf value is a fully resolved [ProcessConfig](ProcessConfig.md).
- **`constants`** — A [ConfigConstants](ConfigConstants.md) initialized to defaults and overwritten by any `@CONSTANT = value` lines in the file.
- **Statistical counters** — `constants_count`, `aliases_count`, `groups_count`, `group_members_count`, `process_rules_count`, `redundant_rules_count`.
- **`errors`** — Fatal parse errors. When non-empty, the configuration should not be applied.
- **`warnings`** — Non-fatal parse warnings (unknown priorities, redundant rules, etc.).

If the file cannot be opened, a `ConfigResult` with a single error in the `errors` vector is returned immediately.

## Remarks

### File format overview

The configuration file uses an INI-like, line-oriented format with five kinds of top-level lines:

| Line prefix | Meaning | Handled by |
|-------------|---------|------------|
| `#` | Comment — ignored entirely. | `read_config` (skipped) |
| `@NAME = value` | Constant definition — sets a scheduler tuning parameter. | [parse_constant](parse_constant.md) |
| `*name = cpu_spec` | CPU alias definition — creates a named CPU set for use in rule fields. | [parse_alias](parse_alias.md) |
| `name { ... }:rule` | Process group — defines multiple processes sharing a single rule. | [collect_group_block](collect_group_block.md), [parse_and_insert_rules](parse_and_insert_rules.md) |
| `name:priority:affinity:...` | Individual process rule — defines settings for a single process. | [parse_and_insert_rules](parse_and_insert_rules.md) |

Blank lines are ignored.

### Parse algorithm

1. **Open and buffer** — The file is opened and all lines are collected into a `Vec<String>` for random-access iteration (needed for multi-line group blocks).
2. **Initialize state** — A default [ConfigResult](ConfigResult.md) and an empty `cpu_aliases` map are created.
3. **Line-by-line dispatch** — An index `i` walks through the line vector. Each line is trimmed, and the first non-whitespace character determines the dispatch path:
   - Empty or `#` → skip, advance `i`.
   - `@` → split on `=`, delegate to [parse_constant](parse_constant.md), advance `i`.
   - `*` → split on `=`, delegate to [parse_alias](parse_alias.md), advance `i`.
   - Contains `{` → begin group parsing (see below).
   - Otherwise → split on `:`, extract the process name from `parts[0]`, delegate `parts[1..]` to [parse_and_insert_rules](parse_and_insert_rules.md), advance `i`.
4. **Return** — The completed `ConfigResult` is returned.

### Group block parsing

When a line contains `{`, the parser enters group mode:

1. **Group name** — Text before the `{` is used as a label for diagnostics. If empty, the label is `"anonymous@L{line_number}"`.
2. **Single-line group** — If `}` appears on the same line as `{`, the text between the braces is parsed inline via [collect_members](collect_members.md) and the text after `}:` is the rule suffix.
3. **Multi-line group** — If no `}` on the same line, [collect_group_block](collect_group_block.md) scans subsequent lines until the closing `}` is found, accumulating member names. The index `i` is advanced past the block.
4. **Unclosed group** — If the file ends without `}`, an error is recorded and the group is skipped.
5. **Empty group** — If no members are found between the braces, a warning is recorded and the group is skipped.
6. **Rule application** — If a rule suffix is present after the closing brace (i.e., `}:priority:affinity:...`), [parse_and_insert_rules](parse_and_insert_rules.md) is called with the collected member list. If no rule suffix is found, an error is recorded.

### Individual rule parsing

For non-group lines, the line is split on `:` and the parts are validated:

- **Fewer than 3 parts** — An error is recorded (`"Too few fields — expected name:priority:affinity,..."`).
- **Empty process name** — An error is recorded.
- **Valid line** — `parts[0]` (lowercased) becomes the member name and `parts[1..]` are passed to [parse_and_insert_rules](parse_and_insert_rules.md).

### Order dependencies

- **Aliases must precede rules** — CPU alias definitions (`*name = spec`) are processed top-to-bottom and stored in the `cpu_aliases` map. Rule lines that reference an alias via `*name` will fail with an "Undefined alias" error if the alias has not been defined on a prior line.
- **Constants can appear anywhere** — `@CONSTANT` lines update `result.constants` immediately and do not depend on parse order relative to rules.
- **Groups cannot nest** — Nested `{ { } }` structures are not supported; a `{` inside an open group block is treated as normal text.

### Error accumulation

`read_config` uses a **continue-on-error** strategy. When a parse error is encountered, a descriptive message (including the 1-based line number) is pushed to `result.errors`, and parsing continues to the next line. This allows the user to see all errors in a single parse pass rather than fixing them one at a time.

### Config file example

```text
# CPU aliases
*perf = 0-7
*eff = 8-15

# Scheduler tuning
@MIN_ACTIVE_STREAK = 3
@KEEP_THRESHOLD = 0.70

# Individual rule
game.exe:high:*perf:*perf:?8x*perf@engine.dll:none:none:*perf

# Group rule
background_apps {
    updater.exe: telemetry.exe
    cloud_sync.exe
}:below normal:*eff:0:0:none:none
```

### Thread safety

`read_config` is not designed for concurrent access. It reads from the file system and writes to a local `ConfigResult`. The caller ([main](../main.rs/main.md) or [hotreload_config](hotreload_config.md)) is responsible for synchronization when replacing the live configuration with the parsed result.

### Performance

All lines are read into memory before parsing begins. This is acceptable because configuration files are typically small (hundreds of lines at most). The two-pass architecture (collect lines, then parse) is necessary to support multi-line group blocks where the parser must look ahead from `{` to `}`.

## Requirements

| | |
|---|---|
| **Module** | `config` (`src/config.rs`) |
| **Visibility** | `pub` |
| **Callers** | [main](../main.rs/main.md), [hotreload_config](hotreload_config.md) |
| **Callees** | [parse_constant](parse_constant.md), [parse_alias](parse_alias.md), [collect_members](collect_members.md), [collect_group_block](collect_group_block.md), [parse_and_insert_rules](parse_and_insert_rules.md) |
| **API** | `std::fs::File::open`, `std::io::BufReader`, `std::io::BufRead::lines` |
| **Privileges** | Read access to the configuration file path |

## See Also

| Topic | Link |
|-------|------|
| Parsed config output | [ConfigResult](ConfigResult.md) |
| Per-process rule struct | [ProcessConfig](ProcessConfig.md) |
| Scheduler tuning constants | [ConfigConstants](ConfigConstants.md) |
| Rule field parser | [parse_and_insert_rules](parse_and_insert_rules.md) |
| Constant line parser | [parse_constant](parse_constant.md) |
| Alias line parser | [parse_alias](parse_alias.md) |
| Group block collector | [collect_group_block](collect_group_block.md) |
| Member name tokenizer | [collect_members](collect_members.md) |
| Hot-reload of config | [hotreload_config](hotreload_config.md) |
| Blacklist file reader | [read_list](read_list.md) |
| Config module overview | [README](README.md) |