# parse_constant function (config.rs)

Parses and applies a `@NAME = value` constant definition from the configuration file to the `ConfigResult`. This function validates the constant name, parses the value into the appropriate type, updates the corresponding field in `ConfigResult.constants`, and records errors for invalid names or unparseable values.

## Syntax

```AffinityServiceRust/src/config.rs#L251-291
fn parse_constant(name: &str, value: &str, line_number: usize, result: &mut ConfigResult)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `name` | `&str` | The constant name (already stripped of the leading `@` and converted to uppercase by the caller). Must match one of the recognized constant names: `MIN_ACTIVE_STREAK`, `KEEP_THRESHOLD`, or `ENTRY_THRESHOLD`. |
| `value` | `&str` | The string value to parse, trimmed of surrounding whitespace by the caller. The expected type depends on the constant name (`u8` for `MIN_ACTIVE_STREAK`, `f64` for the threshold constants). |
| `line_number` | `usize` | The 1-based line number in the configuration file where this constant definition appears. Used in error and warning messages. |
| `result` | `&mut ConfigResult` | A mutable reference to the configuration result being built. On success, the corresponding field in `result.constants` is updated and `result.constants_count` is incremented. On failure, an error is pushed to `result.errors`. |

## Return value

This function does not return a value. Results are communicated through mutations to the `result` parameter.

## Remarks

### Recognized constants

| Constant Name | Type | Target Field | Description |
|---------------|------|--------------|-------------|
| `MIN_ACTIVE_STREAK` | `u8` | `result.constants.min_active_streak` | Minimum consecutive active polling intervals before a thread is eligible for prime-thread promotion. |
| `KEEP_THRESHOLD` | `f64` | `result.constants.keep_threshold` | CPU utilization fraction a prime thread must maintain to keep its status. |
| `ENTRY_THRESHOLD` | `f64` | `result.constants.entry_threshold` | CPU utilization fraction a non-prime thread must exceed for promotion consideration. |

### Error handling

- If the `value` string cannot be parsed into the expected numeric type (`u8` for `MIN_ACTIVE_STREAK`, `f64` for thresholds), an error message is pushed to `result.errors` indicating the line number, the invalid value, and the expected type.
- If `name` does not match any recognized constant, a **warning** (not an error) is pushed to `result.warnings` indicating that the constant is unknown and will be ignored. This allows forward compatibility with future constant names.

### Logging

On successful parse, the function calls `log_message` to emit a diagnostic log entry of the form `Config: NAME = value`. This provides visibility into which constants were loaded during startup or hot-reload.

### Counting

Each successful parse increments `result.constants_count` by 1. This counter is used in diagnostic reporting to show how many constants were loaded from the configuration file.

### Configuration file syntax

Constants are defined using the `@` prefix in the config file:

```/dev/null/example.ini#L1-3
@MIN_ACTIVE_STREAK = 3
@KEEP_THRESHOLD = 0.75
@ENTRY_THRESHOLD = 0.50
```

The `@` prefix and `=` sign are stripped by the caller ([`read_config`](read_config.md)) before invoking `parse_constant`. The name is uppercased and the value is trimmed by the caller.

## Requirements

| Requirement | Value |
|-------------|-------|
| Module | `config.rs` |
| Visibility | Private (`fn`, not `pub fn`) |
| Callers | [`read_config`](read_config.md) |
| Callees | `log_message` (from [`logging.rs`](../logging.rs/README.md)), `str::parse` |
| API | Standard library parsing (`u8::parse`, `f64::parse`) |
| Privileges | None |

## See Also

| Resource | Link |
|----------|------|
| ConfigConstants | [ConfigConstants](ConfigConstants.md) |
| ConfigResult | [ConfigResult](ConfigResult.md) |
| read_config | [read_config](read_config.md) |
| parse_alias | [parse_alias](parse_alias.md) |
| config module overview | [README](README.md) |

---
*Commit: 7221ea0694670265d4eb4975582d8ed2ae02439d*