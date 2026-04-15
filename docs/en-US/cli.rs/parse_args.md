# parse_args function (cli.rs)

Parses a slice of command-line argument strings into a mutable `CliArgs` instance, setting flags, modes, and values according to recognized argument tokens. Unrecognized arguments are silently ignored.

## Syntax

```AffinityServiceRust/src/cli.rs#L42-43
pub fn parse_args(args: &[String], cli: &mut CliArgs) -> Result<()>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `args` | `&[String]` | The full command-line argument list. Element `[0]` (the executable path) is skipped; parsing begins at index 1. |
| `cli` | `&mut CliArgs` | A mutable reference to the `CliArgs` struct that will be populated with the parsed values. Should be initialized via `CliArgs::new()` before calling. |

## Return value

Returns `windows::core::Result<()>`. Currently always returns `Ok(())` — argument parsing errors are handled by falling back to default values (e.g., `unwrap_or`) rather than propagating errors.

## Remarks

### Recognized arguments

| Argument | Effect | Notes |
|----------|--------|-------|
| `-help`, `--help`, `-?`, `/?`, `?` | Sets `help_mode = true` | |
| `-helpall`, `--helpall` | Sets `help_all_mode = true` | |
| `-console` | Enables console output via `get_use_console!()` | Overrides log-file output |
| `-noUAC`, `-nouac` | Sets `no_uac = true` | Case-insensitive variant |
| `-convert` | Sets `convert_mode = true` | Requires `-in` and `-out` |
| `-autogroup` | Sets `autogroup_mode = true` | Requires `-in` and `-out` |
| `-find` | Sets `find_mode = true` | |
| `-validate` | Sets `validate_mode = true` | Also forces console output |
| `-processlogs` | Sets `process_logs_mode = true` | |
| `-dryrun`, `-dry-run`, `--dry-run` | Sets `dry_run = true` | |
| `-interval <ms>` | Sets `interval_ms` | Clamped to `[16, 86400000]`; defaults to `5000` on parse failure |
| `-loop <count>` | Sets `loop_count = Some(n)` | Minimum value is `1` |
| `-resolution <t>` | Sets `time_resolution` | `0` means do not change system timer resolution |
| `-logloop` | Sets `log_loop = true` | |
| `-config <file>` | Sets `config_file_name` | |
| `-blacklist <file>` | Sets `blacklist_file_name = Some(...)` | |
| `-in <file>` | Sets `in_file_name = Some(...)` | |
| `-out <file>` | Sets `out_file_name = Some(...)` | |
| `-skip_log_before_elevation` | Sets `skip_log_before_elevation = true` | |
| `-noDebugPriv`, `-nodebugpriv` | Sets `no_debug_priv = true` | |
| `-noIncBasePriority`, `-noincbasepriority` | Sets `no_inc_base_priority = true` | |
| `-no_etw`, `-noetw` | Sets `no_etw = true` | |
| `-continuous_process_level_apply` | Sets `continuous_process_level_apply = true` | |

### Argument consumption

Arguments that require a value (e.g., `-interval`, `-config`) consume the *next* element in `args`. A bounds check (`i + 1 < args.len()`) prevents out-of-bounds access; if the value argument is missing, the flag is silently skipped.

### Unrecognized arguments

Any argument string that does not match a known token is ignored without emitting a warning or error.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `cli.rs` |
| Callers | `main` (entry point) |
| Callees | `CliArgs` fields, `get_use_console!()` macro |
| API | `windows::core::Result` |
| Privileges | None |

## See Also

| Resource | Link |
|----------|------|
| CliArgs struct | [CliArgs](CliArgs.md) |
| print_help | [print_help](print_help.md) |
| print_help_all | [print_help_all](print_help_all.md) |
| config module | [config.rs overview](../config.rs/README.md) |

---
*Commit: 7221ea0694670265d4eb4975582d8ed2ae02439d*