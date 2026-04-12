# mask_to_cpu_indices 函数 (config.rs)

将 64 位位掩码转换为按升序排列的 CPU 索引向量，每个置位比特对应一个 CPU。

## 语法

```rust
fn mask_to_cpu_indices(mask: u64) -> Vec<u32>
```

## 参数

`mask`

一个 64 位无符号整数位掩码，其中每个置位比特表示一个活跃的 CPU。比特 0 对应 CPU 0，比特 1 对应 CPU 1，以此类推，最高到比特 63 对应 CPU 63。

## 返回值

返回 `Vec<u32>`，包含掩码中所有置位比特的从零开始的索引，按升序排列。如果掩码为 `0`，则返回空向量。

## 备注

这是一个内部辅助函数，由 [parse_cpu_spec](parse_cpu_spec.md) 在处理旧版十六进制位掩码表示法（如 `0xFF`）时使用。当 `parse_cpu_spec` 遇到以 `0x` 或 `0X` 开头的字符串时，会将十六进制值解析为 `u64`，然后委托本函数提取各个 CPU 索引。

该函数遍历比特位 0 到 63，通过 `(mask >> i) & 1 == 1` 检查每个比特位。这意味着使用位掩码表示法时，最多只支持 64 个逻辑处理器的系统。对于超过 64 核的系统，应改用范围或分号分隔的语法（如 `0-7;64-71`）。

本函数是 [cpu_indices_to_mask](cpu_indices_to_mask.md) 的逆运算。

### 示例

| 输入掩码 | 输出 |
| --- | --- |
| `0x0F` (15) | `[0, 1, 2, 3]` |
| `0xFF` (255) | `[0, 1, 2, 3, 4, 5, 6, 7]` |
| `0x05` (5) | `[0, 2]` |
| `0x00` (0) | `[]` |

## 要求

| 要求 | 值 |
| --- | --- |
| **所属模块** | src/config.rs |
| **可见性** | 私有 (`fn`) |
| **调用方** | [parse_cpu_spec](parse_cpu_spec.md) |
| **另请参阅** | [cpu_indices_to_mask](cpu_indices_to_mask.md)、[format_cpu_indices](format_cpu_indices.md) |