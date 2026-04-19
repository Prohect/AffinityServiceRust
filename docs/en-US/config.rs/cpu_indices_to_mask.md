# cpu_indices_to_mask function (config.rs)

Converts an array of CPU index numbers into a bitmask representation suitable for use with Windows affinity APIs.

## Syntax

```rust
pub fn cpu_indices_to_mask(cpus: &[u32]) -> usize
```

## Parameters

`cpus: &[u32]`

A slice of CPU index numbers (zero-based). Each value represents a logical processor number. Indices equal to or greater than 64 are silently ignored because a `usize` bitmask on 64-bit Windows can only represent processors 0–63.

## Return value

`usize` — A bitmask where bit *N* is set to 1 if CPU index *N* is present in the input slice.

## Remarks

This function is the inverse of [mask_to_cpu_indices](mask_to_cpu_indices.md). It iterates through the input slice and sets the corresponding bit for each CPU index using a left-shift operation (`1usize << cpu`).

### Bitmask format

The returned value follows the Windows `DWORD_PTR` affinity mask convention:

| Input CPUs | Output mask | Hex |
|------------|------------|-----|
| `[0]` | `0b0001` | `0x1` |
| `[0, 1, 2, 3]` | `0b1111` | `0xF` |
| `[0, 2, 4]` | `0b10101` | `0x15` |
| `[]` | `0b0` | `0x0` |

### 64-core limit

CPU indices ≥ 64 are silently skipped. This means the function only produces valid results for systems with a single processor group (up to 64 logical processors). For systems with more than 64 cores, the CPU Sets API should be used instead of affinity masks — see [apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md).

### Usage in the apply pipeline

This function is called by [apply_affinity](../apply.rs/apply_affinity.md) to convert the `affinity_cpus` list from a [ProcessLevelConfig](ProcessLevelConfig.md) into the bitmask format expected by **SetProcessAffinityMask**. It is also used by [parse_mask](parse_mask.md) as the second step after [parse_cpu_spec](parse_cpu_spec.md).

## Requirements

| | |
|---|---|
| **Module** | [`src/config.rs`](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/config.rs) |
| **Callers** | [apply_affinity](../apply.rs/apply_affinity.md), [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md), [parse_mask](parse_mask.md), [convert](convert.md) |
| **Callees** | None |
| **Privileges** | None |

## See Also

| Topic | Link |
|-------|------|
| Inverse conversion | [mask_to_cpu_indices](mask_to_cpu_indices.md) |
| CPU spec parser | [parse_cpu_spec](parse_cpu_spec.md) |
| Compact display of CPU lists | [format_cpu_indices](format_cpu_indices.md) |
| Affinity application | [apply_affinity](../apply.rs/apply_affinity.md) |
| Module overview | [config.rs](README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*