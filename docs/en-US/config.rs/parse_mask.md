# parse_mask function (config.rs)

Convenience wrapper that parses a CPU specification string directly into a `usize` bitmask. Equivalent to calling [parse_cpu_spec](parse_cpu_spec.md) followed by [cpu_indices_to_mask](cpu_indices_to_mask.md) in a single step.

## Syntax

```rust
pub fn parse_mask(s: &str) -> usize
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `s` | `&str` | A CPU specification string in any format accepted by [parse_cpu_spec](parse_cpu_spec.md): hex bitmask (`"0xFF"`), inclusive range (`"0-7"`), semicolon-separated indices (`"0;4;8"`), or mixed (`"0-3;8;12-15"`). The string `"0"` and empty strings both produce a mask of `0`. |

## Return value

Returns a `usize` bitmask where bit *N* is set if CPU index *N* was present in the parsed specification. CPU indices ≥ 64 are silently dropped because they cannot be represented in a `usize` on 64-bit platforms.

Returns `0` when:

- `s` is empty, whitespace-only, or the literal `"0"`.
- `s` is a hex prefix with an unparseable value.
- All parsed CPU indices are ≥ 64.

## Remarks

### Implementation

The function is a two-line composition of existing utilities:

```rust
pub fn parse_mask(s: &str) -> usize {
    let cpus = parse_cpu_spec(s);
    cpu_indices_to_mask(&cpus)
}
```

No additional validation, error reporting, or alias resolution is performed. For alias-aware parsing, use [resolve_cpu_spec](resolve_cpu_spec.md) instead.

### Dead-code annotation

The function carries an `#[allow(dead_code)]` attribute in the source, indicating it may not be called in all build configurations. It is provided as a public utility for external consumers or future use.

### 64-core limitation

Because the output is a `usize` bitmask (64 bits on x86-64 Windows), this function cannot represent more than 64 CPUs. For systems with more than 64 logical processors, callers should work with `Vec<u32>` indices directly (via [parse_cpu_spec](parse_cpu_spec.md)) or use the CPU set API path ([ProcessConfig](ProcessConfig.md)`.cpu_set_cpus`).

### Relationship to format_cpu_indices

While [format_cpu_indices](format_cpu_indices.md) converts a CPU index slice to a display string and [parse_cpu_spec](parse_cpu_spec.md) converts a string to an index list, `parse_mask` offers the shortcut path from string directly to a Windows-compatible affinity bitmask.

### Examples

| Input | Intermediate indices | Output mask |
|-------|---------------------|-------------|
| `"0-3"` | `[0, 1, 2, 3]` | `0x0F` |
| `"0;4;8"` | `[0, 4, 8]` | `0x111` |
| `"0xFF"` | `[0, 1, 2, 3, 4, 5, 6, 7]` | `0xFF` |
| `"0"` | `[]` | `0` |
| `""` | `[]` | `0` |
| `"0-3;63"` | `[0, 1, 2, 3, 63]` | `0x800000000000000F` |

## Requirements

| | |
|---|---|
| **Module** | `config` (`src/config.rs`) |
| **Visibility** | `pub` (with `#[allow(dead_code)]`) |
| **Callers** | Reserved for external consumers and future use |
| **Callees** | [parse_cpu_spec](parse_cpu_spec.md), [cpu_indices_to_mask](cpu_indices_to_mask.md) |
| **API** | Pure function — no I/O, no Windows API calls |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| CPU spec string parser | [parse_cpu_spec](parse_cpu_spec.md) |
| CPU indices to bitmask | [cpu_indices_to_mask](cpu_indices_to_mask.md) |
| Bitmask to CPU indices (inverse) | [mask_to_cpu_indices](mask_to_cpu_indices.md) |
| Compact range display formatter | [format_cpu_indices](format_cpu_indices.md) |
| Per-process config (affinity mask usage) | [ProcessConfig](ProcessConfig.md) |
| Config module overview | [README](README.md) |