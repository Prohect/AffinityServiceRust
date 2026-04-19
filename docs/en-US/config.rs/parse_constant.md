# parse_constant function (config.rs)

Parses a `@NAME = value` constant definition from the configuration file and updates the corresponding field in the [ConfigResult](ConfigResult.md)'s [ConfigConstants](ConfigConstants.md). Recognized constants control the hysteresis behavior of prime thread scheduling.

## Syntax

```rust
fn parse_constant(name: &str, value: &str, line_number: usize, result: &mut ConfigResult)
```

## Parameters

`name: &str`

The uppercase constant name extracted from the portion between `@` and `=` in the config line. The caller is responsible for trimming whitespace and converting to uppercase before passing. Recognized names are:

| Name | Expected type | Description |
|------|---------------|-------------|
| `MIN_ACTIVE_STREAK` | `u8` | Minimum consecutive iterations a thread must exceed the entry threshold before promotion. |
| `KEEP_THRESHOLD` | `f64` | CPU cycle share fraction below which a prime thread is demoted. |
| `ENTRY_THRESHOLD` | `f64` | CPU cycle share fraction a non-prime thread must exceed to begin accumulating an active streak. |

`value: &str`

The string value to the right of the `=` sign, trimmed of whitespace by the caller. Parsed as the type appropriate for the constant name (`u8` for `MIN_ACTIVE_STREAK`, `f64` for thresholds).

`line_number: usize`

The 1-based line number in the config file where the constant definition appears. Included in error and log messages for diagnostics.

`result: &mut ConfigResult`

**\[in, out\]** The parse result accumulator. On success, the appropriate field in `result.constants` is updated and `result.constants_count` is incremented. On failure, an error message is pushed to `result.errors`.

## Return value

This function does not return a value. Results are communicated through mutation of the `result` parameter.

## Remarks

### Parsing behavior

The function uses a `match` on the `name` parameter to determine which constant is being set:

1. **`MIN_ACTIVE_STREAK`:** The value is parsed as `u8`. On success, `result.constants.min_active_streak` is set and a log message is emitted. On failure, an error is pushed indicating an invalid `u8` value.

2. **`KEEP_THRESHOLD` / `ENTRY_THRESHOLD`:** The value is parsed as `f64`. On success, the corresponding field (`keep_threshold` or `entry_threshold`) is set and logged. On failure, an error is pushed.

3. **Unknown names:** Any unrecognized constant name produces a warning (not an error) indicating it will be ignored. This allows forward compatibility with future constants without breaking existing configurations.

### Logging

Each successfully parsed constant produces a log message in the format `"Config: NAME = value"` via `log_message`. This provides immediate feedback during startup and hot-reload.

### Error messages

Error messages follow the format `"Line {N}: Invalid constant value '{value}' for '{name}' (expected type)"` for type mismatches, and `"Line {N}: Unknown constant '{name}' - will be ignored"` for unrecognized names.

### Relationship with the config file

Constants appear at the top of the config file using the `@` prefix:

```
@MIN_ACTIVE_STREAK = 3
@KEEP_THRESHOLD = 0.05
@ENTRY_THRESHOLD = 0.1
```

The [read_config](read_config.md) function detects lines starting with `@`, extracts the name and value around the `=` sign, uppercases the name, trims the value, and delegates to `parse_constant`.

### Idempotency

If the same constant is defined multiple times in the config file, the last definition wins. Each definition increments `constants_count`, so the count may exceed 3 (the number of recognized constants) if duplicates exist.

## Requirements

| | |
|---|---|
| **Module** | `src/config.rs` |
| **Visibility** | Private (`fn`) |
| **Callers** | [read_config](read_config.md) |
| **Callees** | `log_message` (logging) |
| **Dependencies** | [ConfigResult](ConfigResult.md), [ConfigConstants](ConfigConstants.md) |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Module overview | [config.rs](README.md) |
| Constants struct | [ConfigConstants](ConfigConstants.md) |
| Config file reader | [read_config](read_config.md) |
| Alias definition parser | [parse_alias](parse_alias.md) |
| Hot-reload that propagates constants | [hotreload_config](hotreload_config.md) |
| Parse result container | [ConfigResult](ConfigResult.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*