# cpu_indices_to_mask 函数 (config.rs)

将一组 CPU 索引切片转换为 `usize` 位掩码，其中每个置位的比特对应输入中存在的一个 CPU 索引。大于或等于 64 的索引将被静默忽略，因为它们无法用单个 64 位掩码表示。

## 语法

```rust
pub fn cpu_indices_to_mask(cpus: &[u32]) -> usize
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `cpus` | `&[u32]` | 要编码的逻辑 CPU 索引切片。重复值是无害的（比特位设置是幂等操作）。切片不需要排序。 |

## 返回值

返回一个 `usize` 位掩码。如果值 *N* 出现在 `cpus` 中且 *N* < 64，则第 *N* 位被置位。如果 `cpus` 为空或所有索引 ≥ 64，则返回 `0`。

## 备注

### 比特布局

最低有效位（第 0 位）代表 CPU 0。例如，输入 `[0, 2, 4]` 产生掩码 `0b10101`（`0x15`）。

### 64 核限制

由于返回类型为 `usize`（在 x86-64 Windows 上为 64 位），索引 ≥ 64 的值会被静默丢弃。这与 Windows 的 `DWORD_PTR` 亲和性掩码一致，后者每个处理器组也最多支持 64 个处理器。对于拥有超过 64 个逻辑处理器的系统，应使用 CPU 集合 API（[cpu_set_cpus](ProcessConfig.md) 字段）而非亲和性掩码。

### 逆操作

[mask_to_cpu_indices](mask_to_cpu_indices.md) 执行反向转换，将位掩码展开为排序后的 CPU 索引向量。

### 在代码库中的使用

此函数被以下调用方使用：

- [parse_mask](parse_mask.md)，它将 [parse_cpu_spec](parse_cpu_spec.md) → `cpu_indices_to_mask` 串联为一个便捷包装。
- apply 模块，在根据 [ProcessConfig](ProcessConfig.md)`.affinity_cpus` 构建亲和性掩码以供 `SetProcessAffinityMask` 使用时调用。
- winapi 模块，用于 CPU 集合和掩码转换。

### 示例

| 输入 | 输出 | 备注 |
|------|------|------|
| `&[]` | `0` | 空输入产生零掩码。 |
| `&[0]` | `1` | 单个 CPU 0。 |
| `&[0, 1, 2, 3]` | `0xF` | 连续范围。 |
| `&[0, 2, 4, 63]` | `0x8000000000000015` | 稀疏索引，包含最后一个可表示的比特位。 |
| `&[64, 128]` | `0` | 所有索引超出范围，被静默忽略。 |
| `&[0, 0, 0]` | `1` | 重复值是无害的。 |

## 要求

| | |
|---|---|
| **模块** | `config` (`src/config.rs`) |
| **调用方** | [parse_mask](parse_mask.md)、apply 模块（`apply_affinity`、`apply_prime_threads_promote`）、winapi 模块（`cpusetids_from_mask`） |
| **被调用方** | 无 |
| **逆操作** | [mask_to_cpu_indices](mask_to_cpu_indices.md) |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 位掩码转索引列表 | [mask_to_cpu_indices](mask_to_cpu_indices.md) |
| CPU 规格字符串解析器 | [parse_cpu_spec](parse_cpu_spec.md) |
| 便捷规格转掩码 | [parse_mask](parse_mask.md) |
| 紧凑范围格式化器 | [format_cpu_indices](format_cpu_indices.md) |
| 进程级配置（affinity_cpus 字段） | [ProcessConfig](ProcessConfig.md) |
| 模块概述 | [config 模块](README.md) |