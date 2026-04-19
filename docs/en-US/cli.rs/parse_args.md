# parse_args function (cli.rs)

Parses command-line argument strings into a [CliArgs](CliArgs.md) struct, setting flags and values based on recognized switches.

## Syntax

```rust
pub fn parse_args(args: &[String], cli: &mut CliArgs) -> Result<()>
```

## Parameters

`args: &[String]`

The raw command-line arguments, typically obtained from `std::env::args()`. Element `[0]` is the executable path and is skipped; parsing begins at index 1.

`cli: &mut CliArgs`

**\[in, out\]** The [CliArgs](CliArgs.md) struct to populate. Should be initialized with `CliArgs::new()` so that default values (e.g., `interval_ms = 5000`, `config_file_name = "config.ini"`) are in place before overrides are applied.

## Return value

`Result<()>` — Always returns `Ok(())`. The `Result` wrapper is retained for API consistency with other initialization functions that may fail.

## Remarks

### Argument format

All switches use a single `-` prefix (e.g., `-console`, `-interval`). For a few common flags, double-dash (`--help`, `--helpall`, `--dry-run`) and Windows-style (`/?`, `/? `) variants are also accepted. Argument matching is **case-sensitive** except where explicit aliases are listed (e.g., `-noUAC` and `-nouac` are both accepted).

### Value arguments

Switches that consume a following value (`-interval`, `-loop`, `-resolution`, `-config`, `-blacklist`, `-in`, `-out`) check `i + 1 < args.len()` before reading the next element. If the guard fails (no value provided), the switch is silently ignored and parsing continues.

### Numeric clamping

| Argument | Type | Default | Clamp range |
|----------|------|---------|-------------|
| `-interval <ms>` | `u32` | `5000` | `[16, 86_400_000]` (16 ms to 24 hours) |
| `-loop <count>` | `u32` | `1` | `[1, u32::MAX]` |
| `-resolution <t>` | `u32` | `0` | No clamping; `0` means do not set |

If parsing a numeric value fails, the default for that field is used (via `unwrap_or`).

### Side effects

- **`-console`** and **`-validate`**: In addition to setting their respective `CliArgs` fields, these switches set the global `USE_CONSOLE` flag to `true` via `get_use_console!()`, redirecting log output to stdout for the remainder of the process lifetime.

### Unrecognized arguments

Unrecognized switches are silently ignored (matched by the `_ => {}` arm). No warning is emitted.

### Recognized switches

| Switch | Field set | Notes |
|--------|-----------|-------|
| `-help`, `--help`, `-?`, `/?`, `?` | `help_mode = true` | |
| `-helpall`, `--helpall` | `help_all_mode = true` | |
| `-console` | Global `USE_CONSOLE` | Not stored in `CliArgs` |
| `-noUAC`, `-nouac` | `no_uac = true` | |
| `-convert` | `convert_mode = true` | |
| `-autogroup` | `autogroup_mode = true` | |
| `-find` | `find_mode = true` | |
| `-validate` | `validate_mode = true` | Also sets `USE_CONSOLE` |
| `-processlogs` | `process_logs_mode = true` | |
| `-dryrun`, `-dry-run`, `--dry-run` | `dry_run = true` | |
| `-interval <ms>` | `interval_ms` | Clamped to `[16, 86_400_000]` |
| `-loop <count>` | `loop_count = Some(n)` | Minimum 1 |
| `-resolution <t>` | `time_resolution` | |
| `-logloop` | `log_loop = true` | |
| `-config <file>` | `config_file_name` | |
| `-blacklist <file>` | `blacklist_file_name = Some(…)` | |
| `-in <file>` | `in_file_name = Some(…)` | |
| `-out <file>` | `out_file_name = Some(…)` | |
| `-skip_log_before_elevation` | `skip_log_before_elevation = true` | |
| `-noDebugPriv`, `-nodebugpriv` | `no_debug_priv = true` | |
| `-noIncBasePriority`, `-noincbasepriority` | `no_inc_base_priority = true` | |
| `-no_etw`, `-noetw` | `no_etw = true` | |
| `-continuous_process_level_apply` | `continuous_process_level_apply = true` | |

## Requirements

| | |
|---|---|
| **Module** | `src/cli.rs` |
| **Callers** | `main()` during startup |
| **Callees** | `get_use_console!()` macro |
| **Win32 API** | None |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Argument container | [CliArgs](CliArgs.md) |
| Module overview | [cli.rs](README.md) |
| Basic help output | [print_help](print_help.md) |
| Full help output | [print_help_all](print_help_all.md) |
| Logging system | [logging.rs](../logging.rs/README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*