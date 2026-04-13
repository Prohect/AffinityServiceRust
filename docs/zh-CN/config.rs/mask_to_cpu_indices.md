# mask_to_cpu_indices 函数 (config.rs)

将 64 位位掩码转换为已排序的 CPU 索引向量。掩码中每个被置位的比特对应返回向量中的一个逻辑 CPU 索引。此函数是 [cpu_indices_to_mask](cpu_indices_to_mask.md) 的逆操作（适用于 64 位以内的值）。

## 语法

```rust
fn mask_to_cpu_indices(mask: u64) -> Vec<u32>
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `mask` | `u64` | 一个位掩码，其中第 *N* 位被置位表示逻辑 CPU *N* 应包含在输出中。检查范围为第 0–63 位；更高编号的 CPU 无法在此格式中表示。 |

## 返回值

返回一个 `Vec<u32>`，包含 `mask` 中所有被置位比特的从零开始的索引，按升序排列。

如果 `mask` 为 `0`，则返回空向量。

## 备注

此函数是 [parse_cpu_spec](parse_cpu_spec.md) 中十六进制掩码分支的底层核心。当 CPU 规格字符串以 `0x` 或 `0X` 开头时，`parse_cpu_spec` 将十六进制值解析为 `u64`，然后委托给 `mask_to_cpu_indices` 进行位提取。

### 实现

该函数对范围 `0..64` 进行过滤，收集每个满足 `(mask >> i) & 1 == 1` 的索引 `i`。由于范围按升序迭代，结果自然是排序的。

```rust
fn mask_to_cpu_indices(mask: u64) -> Vec<u32> {
    (0..64).filter(|i| (mask >> i) & 1 == 1).collect()
}
```

### 可见性

此函数具有 **crate 内部**可见性（`fn`，而非 `pub fn`）。它仅在 `config` 模块内部被 [parse_cpu_spec](parse_cpu_spec.md) 调用。

### 64 核限制

由于输入类型为 `u64`，此函数只能表示 CPU 0–63。拥有超过 64 个逻辑处理器的系统应在其 CPU 规格中使用范围/分号语法（`0-7;64-71`）而非十六进制掩码。[parse_cpu_spec](parse_cpu_spec.md) 中的范围语法没有上限限制。

### 示例

| 输入掩码 | 输出 |
|----------|------|
| `0x00` | `[]` |
| `0x01` | `[0]` |
| `0x0F` | `[0, 1, 2, 3]` |
| `0xFF00` | `[8, 9, 10, 11, 12, 13, 14, 15]` |
| `0x8000_0000_0000_0001` | `[0, 63]` |

## 要求

| | |
|---|---|
| **模块** | `config` (`src/config.rs`) |
| **可见性** | Crate 内部 |
| **调用者** | [parse_cpu_spec](parse_cpu_spec.md) |
| **逆操作** | [cpu_indices_to_mask](cpu_indices_to_mask.md)（截断为 `usize`） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| CPU 规格字符串解析器 | [parse_cpu_spec](parse_cpu_spec.md) |
| CPU 索引 → 位掩码 | [cpu_indices_to_mask](cpu_indices_to_mask.md) |
| CPU 索引 → 显示字符串 | [format_cpu_indices](format_cpu_indices.md) |
| 便捷的解析到掩码函数 | [parse_mask](parse_mask.md) |
| 模块概述 | [config 模块](README.md) |