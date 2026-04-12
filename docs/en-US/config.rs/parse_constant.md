# parse_constant function (config.rs)

Parses a scheduler constant definition from the configuration file and stores the result in a [ConfigResult](ConfigResult.md).

## Syntax

```rust
fn parse_constant(
    name: &str,
    value: &str,
    line_number: usize,
    result: &mut ConfigResult,
)
```

## Parameters

`name`

The uppercase name of the constant, with the leading `@` already stripped. Recognized names are `MIN_ACTIVE_STREAK`, `KEEP_THRESHOLD`, and `ENTRY_THRESHOLD`. Unrecognized names produce a warning.

`value`

The string value to the right of the `=` sign, already trimmed. Parsed as `u8` for `MIN_ACTIVE_STREAK` and as `f64` for threshold constants.

`line_number`

The 1-based line number in the config file where the constant was defined. Used for error and warning messages.

`result`

A mutable reference to the [ConfigResult](ConfigResult.md) being built. On success the corresponding field in `result.constants` is updated and `result.constants_count` is incremented. On failure an error is pushed to `result.errors`.

## Return value

This function does not return a value. Results are recorded by mutating `result`.

## Remarks

The function handles three recognized constants that control the prime-thread scheduler's behavior (see [ConfigConstants](ConfigConstants.md)):

| Constant | Type | Default | Purpose |
| --- | --- | --- | --- |
| `MIN_ACTIVE_STREAK` | `u8` | 2 | Minimum consecutive active ticks before a thread is eligible for prime promotion |
| `KEEP_THRESHOLD` | `f64` | 0.69 | Fraction of top-thread cycle share required to keep prime status |
| `ENTRY_THRESHOLD` | `f64` | 0.42 | Fraction of top-thread cycle share required to enter prime status |

Constants are defined in the configuration file using the syntax:

```
@MIN_ACTIVE_STREAK = 3
@KEEP_THRESHOLD = 0.75
@ENTRY_THRESHOLD = 0.50
```

If the value cannot be parsed into the expected type, an error is pushed to `result.errors`. If the constant name is not one of the three recognized names, a warning is pushed to `result.warnings` and the value is ignored.

Each successful parse logs the new value via `log_message`.

## Requirements

| Requirement | Value |
| --- | --- |
| **Module** | src/config.rs |
| **Visibility** | Private (`fn`) |
| **Called by** | [read_config](read_config.md) |
| **Modifies** | [ConfigResult](ConfigResult.md), [ConfigConstants](ConfigConstants.md) |