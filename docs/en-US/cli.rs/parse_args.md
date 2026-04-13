# parse_args function (cli.rs)

Parses a raw command-line argument slice into a [CliArgs](CliArgs.md) structure. The parser performs a single linear scan over the argument vector, matching each element against known flag strings and consuming an additional element for value-bearing flags. Unknown flags are silently ignored, making the parser forward-compatible with future additions.

## Syntax

```rust
pub fn parse_args(args: &[String], cli: &mut CliArgs) -> Result<()>
```

## Parameters

`args`

A slice of command-line argument strings, typically obtained from `std::env::args().collect()`. The first element (`args[0]`) is the executable path and is skipped — parsing begins at index 1.

`cli`

A mutable reference to a [CliArgs](CliArgs.md) structure that has already been initialized with defaults via `CliArgs::new()`. The parser overwrites individual fields as it encounters matching flags. Fields for which no flag is present retain their default values.

## Return value

Returns `Ok(())` unconditionally. The `Result<()>` return type (where `Result` is `windows::core::Result`) exists to allow future error reporting for malformed arguments, but the current implementation never returns an error. Unrecognized flags and missing values for value-bearing flags (when the flag is the last argument) are silently ignored.

## Remarks

### Flag format

All flags use a single-dash prefix (e.g., `-help`, `-config`). A small number of flags also accept a double-dash variant (e.g., `--help`, `--helpall`, `--dry-run`), and the legacy `?` and `/?` forms are accepted for help. Flag matching is case-sensitive, with explicit case-variant aliases provided where needed:

| Canonical flag | Aliases |
|----------------|---------|
| `-help` | `--help`, `-?`, `/?`, `?` |
| `-helpall` | `--helpall` |
| `-noUAC` | `-nouac` |
| `-dryrun` | `-dry-run`, `--dry-run` |
| `-noDebugPriv` | `-nodebugpriv` |
| `-noIncBasePriority` | `-noincbasepriority` |
| `-no_etw` | `-noetw` |

### Value-bearing flags

Flags that require a value consume the next element in the argument vector (`args[i + 1]`). The parser guards against out-of-bounds access with an `if i + 1 < args.len()` condition on each match arm. If the flag appears as the last argument without a following value, the match arm does not fire and the flag is treated as an unknown token (silently ignored).

| Flag | Value type | Default | Constraints |
|------|-----------|---------|-------------|
| `-interval` | `u64` (milliseconds) | 5000 | Clamped to minimum of 16 via `.max(16)` |
| `-loop` | `u32` (iteration count) | `None` (infinite) | Clamped to minimum of 1 via `.max(1)` |
| `-resolution` | `u32` (100-ns units) | 0 | No clamping; 0 means do not set |
| `-config` | `String` (file path) | `"config.ini"` | No validation |
| `-blacklist` | `String` (file path) | `None` | Wrapped in `Some` |
| `-in` | `String` (file path) | `None` | Wrapped in `Some` |
| `-out` | `String` (file path) | `None` | Wrapped in `Some` |

For numeric flags (`-interval`, `-loop`, `-resolution`), the value is parsed with `str::parse()`. If parsing fails, `.unwrap_or()` supplies the default (5000 for interval, 1 for loop, 0 for resolution).

### Side effects

Two flags modify global state directly rather than (or in addition to) setting a field on `CliArgs`:

- **`-console`** — Sets the global `USE_CONSOLE` static to `true` via the `get_use_console!()` macro. There is no corresponding field in `CliArgs` because console mode is consumed by the logging infrastructure, not the main loop.
- **`-validate`** — Sets `cli.validate_mode = true` **and** sets `USE_CONSOLE` to `true`, because validation output is always intended for interactive review.

### Unknown flag handling

Any argument that does not match a known flag is silently skipped. This means:

- Typoed flags (e.g., `-consle`) are ignored without warning.
- Positional arguments or bare values without a preceding flag are ignored.
- Future flags added to the parser will not break existing scripts that pass unknown flags.

### Parsing order

The parser processes arguments left to right. If a flag appears multiple times, the last occurrence wins for value-bearing flags (because each occurrence overwrites the field). For boolean flags, they can only be set to `true` — there is no mechanism to unset a flag once it has been set.

### Typical usage

```rust
let args: Vec<String> = env::args().collect();
let mut cli = CliArgs::new();
parse_args(&args, &mut cli)?;
```

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `cli` |
| Callers | [main](../main.rs/main.md) |
| Callees | `get_use_console!()` macro (global logging state) |
| API | None (pure argument parsing with one global side effect) |
| Privileges | N/A |

## See Also

| Topic | Link |
|-------|------|
| Argument container structure | [CliArgs](CliArgs.md) |
| Basic help output | [print_help](print_help.md) |
| Detailed help output | [print_cli_help](print_cli_help.md) |
| Combined help output | [print_help_all](print_help_all.md) |
| Entry point that calls parse_args | [main](../main.rs/main.md) |
