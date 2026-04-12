# cpu_indices_to_mask 函数 (config.rs)

将 CPU 索引切片转换为位掩码表示，适用于 64 核及以下系统的旧版 Windows 亲和性 API。

## 语法

```rust
pub fn cpu_indices_to_mask(cpus: &[u32]) -> usize
```

## 参数

`cpus`

待编码为位掩码的 CPU 索引切片。每个值表示一个从零开始的逻辑处理器编号。大于或等于 64 的值会被静默忽略，因为单个 `usize` 位掩码无法表示这些值。

## 返回值

返回一个 `usize` 位掩码，其中第 N 位被置位表示 CPU 索引 N 存在于输入切片中。例如，索引 `[0, 2, 3]` 生成位掩码 `0b1101`（十进制 13）。

## 备注

此函数是 [mask_to_cpu_indices](mask_to_cpu_indices.md) 的逆操作。它在与需要位掩码而非处理器索引列表的 Windows API（如 `SetProcessAffinityMask`）交互时使用。

CPU 索引 >= 64 会被跳过，因为位掩码存储在 `usize` 中，在 64 位 Windows 上为 64 位。对于超过 64 个逻辑处理器的系统，应改用 CPU 集合 API（参见 [ProcessConfig](ProcessConfig.md) 中的 `cpu_set_cpus`）。

此函数不要求输入已排序或去重。由于使用按位或运算，重复设置同一位不会产生任何影响。

### 算法

对于输入切片中的每个 CPU 索引：
1. 如果索引小于 64，通过 `mask |= 1usize << cpu` 设置掩码中对应的位。
2. 否则，跳过该索引。

### 示例

给定 CPU `[0, 1, 2, 3]`，结果为 `0x0F`（二进制 `00001111`）。

给定 CPU `[4, 8, 12]`，结果为 `0x1110`（二进制 `0001_0001_0001_0000`）。

### 相关函数

| 函数 | 用途 |
| --- | --- |
| [mask_to_cpu_indices](mask_to_cpu_indices.md) | 逆操作 - 将位掩码转换为 CPU 索引列表 |
| [parse_cpu_spec](parse_cpu_spec.md) | 将 CPU 规格字符串解析为 CPU 索引 |
| [format_cpu_indices](format_cpu_indices.md) | 将 CPU 索引格式化为人类可读的紧凑字符串 |
| [parse_mask](parse_mask.md) | 便捷封装：将规格字符串直接解析为掩码 |

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/config.rs |
| **可见性** | `pub` |
| **调用方** | [parse_mask](parse_mask.md)、[apply_affinity](../apply.rs/apply_affinity.md)、[apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md) |
| **依赖** | 无 |