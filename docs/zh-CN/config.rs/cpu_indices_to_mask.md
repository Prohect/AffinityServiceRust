# cpu_indices_to_mask 函数 (config.rs)

将 CPU 索引号数组转换为位掩码表示形式，适用于与 Windows 亲和性 API 一起使用。

## 语法

```rust
pub fn cpu_indices_to_mask(cpus: &[u32]) -> usize
```

## 参数

`cpus: &[u32]`

CPU 索引号数组（从零开始）。每个值代表一个逻辑处理器号。等于或大于 64 的索引会被静默忽略，因为 64 位 Windows 上的 `usize` 位掩码只能表示处理器 0-63。

## 返回值

`usize` — 位掩码，其中位 *N* 设置为 1，如果 CPU 索引 *N* 存在于输入数组中。

## 备注

此函数是 [mask_to_cpu_indices](mask_to_cpu_indices.md) 的反函数。它遍历输入数组，并使用左移操作（`1usize << cpu`）为每个 CPU 索引设置相应的位。

### 位掩码格式

返回值遵循 Windows `DWORD_PTR` 亲和性掩码约定：

| 输入 CPUs | 输出掩码 | 十六进制 |
|------------|------------|-----|
| `[0]` | `0b0001` | `0x1` |
| `[0, 1, 2, 3]` | `0b1111` | `0xF` |
| `[0, 2, 4]` | `0b10101` | `0x15` |
| `[]` | `0b0` | `0x0` |

### 64 核限制

CPU 索引 ≥ 64 会被静默跳过。这意味着该函数仅对具有单个处理器组（最多 64 个逻辑处理器）的系统产生有效结果。对于超过 64 个核心的系统，应使用 CPU 集合 API 而不是亲和性掩码——参见 [apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md)。

### 在 apply 管道中的使用

此函数由 [apply_affinity](../apply.rs/apply_affinity.md) 调用，用于将 [ProcessLevelConfig](ProcessLevelConfig.md) 中的 `affinity_cpus` 列表转换为 **SetProcessAffinityMask** 期望的位掩码格式。它也被 [parse_mask](parse_mask.md) 用作 [parse_cpu_spec](parse_cpu_spec.md) 之后的第二步。

## 需求

| | |
|---|---|
| **模块** | [`src/config.rs`](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/config.rs) |
| **调用方** | [apply_affinity](../apply.rs/apply_affinity.md), [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md), [parse_mask](parse_mask.md), [convert](convert.md) |
| **被调用方** | 无 |
| **权限** | 无 |

## 参见

| 主题 | 链接 |
|-------|------|
| 反向转换 | [mask_to_cpu_indices](mask_to_cpu_indices.md) |
| CPU 规格解析器 | [parse_cpu_spec](parse_cpu_spec.md) |
| CPU 列表的紧凑显示 | [format_cpu_indices](format_cpu_indices.md) |
| 亲和性应用 | [apply_affinity](../apply.rs/apply_affinity.md) |
| 模块概述 | [config.rs](README.md) |

*文档针对 Commit：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*