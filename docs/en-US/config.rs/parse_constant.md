# parse_constant function (config.rs)

Parses a `@CONSTANT = value` line from the configuration file and updates the corresponding field in [ConfigResult](ConfigResult.md). Recognized constants control the prime-thread scheduler's hysteresis behavior via [ConfigConstants](ConfigConstants.md). Unknown constant names produce a warning; invalid values produce an error.

## Syntax

```rust
fn parse_constant(name: &str, value: &str, line_number: usize, result: &mut ConfigResult)
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `name` | `&str` | The constant name **after** stripping the leading `@` and converting to uppercase. For example, if the config line is `@keep_threshold = 0.75`, the caller passes `"KEEP_THRESHOLD"`. |
| `value` | `&str` | The raw value string **after** the `=` sign, already trimmed of surrounding whitespace by the caller. |
| `line_number` | `usize` | 1-based line number in the config file, used for error and warning messages. |
| `result` | `&mut ConfigResult` | The mutable parse-result accumulator. On success, the matching field in `result.constants` is updated and `result.constants_count` is incremented. On failure, an entry is pushed to `result.errors` or `result.warnings`. |

## Return value

This function does not return a value. All output is written to `result` via side effects.

## Remarks

### Recognized constants

| Constant name | Type | Storage field | Description |
|---------------|------|---------------|-------------|
| `MIN_ACTIVE_STREAK` | `u8` | `result.constants.min_active_streak` | Minimum consecutive scheduling cycles a thread must appear in the top-N before promotion. Parsed with `u8::parse`; values outside 0–255 are rejected. |
| `KEEP_THRESHOLD` | `f64` | `result.constants.keep_threshold` | Fraction of the top thread's cycle delta that a promoted thread must sustain to avoid demotion. Parsed with `f64::parse`. |
| `ENTRY_THRESHOLD` | `f64` | `result.constants.entry_threshold` | Fraction of the top thread's cycle delta that an unpromoted thread must reach for promotion consideration. Parsed with `f64::parse`. |

### Success behavior

When a recognized constant name is matched and its value parses successfully:

1. The corresponding field in `result.constants` is updated.
2. `result.constants_count` is incremented by 1.
3. A log message is emitted via `log_message` (e.g., `"Config: KEEP_THRESHOLD = 0.75"`).

### Error behavior

- **Invalid value for a known constant** — An error is pushed to `result.errors` with the format:
  `"Line {N}: Invalid constant value '{value}' for '{name}' (expected u8)"` (for `MIN_ACTIVE_STREAK`) or
  `"Line {N}: Invalid constant value '{value}' for '{name}'"` (for threshold constants).
  The `constants_count` is **not** incremented.

- **Unknown constant name** — A warning is pushed to `result.warnings` with the format:
  `"Line {N}: Unknown constant '{name}' - will be ignored"`.
  This is non-fatal; the configuration remains valid.

### Calling context

`parse_constant` is called from the main parse loop in [read_config](read_config.md) when a line begins with `@`. The caller splits on `=`, extracts the name (trimmed, uppercased) and value (trimmed), and delegates to this function. Lines without an `=` sign are rejected by the caller before `parse_constant` is reached.

### Config file examples

```text
# Set a longer streak requirement for promotion stability
@MIN_ACTIVE_STREAK = 4

# Tighten the entry gate so only truly hot threads are promoted
@ENTRY_THRESHOLD = 0.55

# Relax the keep gate to reduce demotion churn
@KEEP_THRESHOLD = 0.60

# Unknown constants produce warnings but do not block parsing
@SOME_FUTURE_SETTING = 42
```

### Idempotency

If the same constant is defined multiple times in a config file, each occurrence overwrites the previous value and increments `constants_count` again. No warning is issued for duplicate constants — the last definition wins.

## Requirements

| | |
|---|---|
| **Module** | `config` (`src/config.rs`) |
| **Visibility** | Crate-private |
| **Called by** | [read_config](read_config.md) |
| **Modifies** | [ConfigResult](ConfigResult.md) (fields `constants`, `constants_count`, `errors`, `warnings`) |
| **Dependencies** | `log_message` for diagnostic output |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Hysteresis tuning constants struct | [ConfigConstants](ConfigConstants.md) |
| Parsed config output | [ConfigResult](ConfigResult.md) |
| Main config file reader | [read_config](read_config.md) |
| CPU alias line parser | [parse_alias](parse_alias.md) |
| Prime-thread scheduler | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| Config module overview | [README](README.md) |