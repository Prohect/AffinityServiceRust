# mask_to_cpu_indices 函数 (config.rs)

将 64 位位掩码转换为位被设置的 CPU 索引位置的已排序列表。这是一个私有辅助函数，在 [`parse_cpu_spec`](parse_cpu_spec.md) 解析十六进制 CPU 掩码值时内部使用。

## 语法

```AffinityServiceRust/src/config.rs#L115-117
fn mask_to_cpu_indices(mask: u64) -> List<[u32; CONSUMER_CPUS]>
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `mask` | `u64` | 一个 64 位无符号整数，每个被设置的位代表一个逻辑处理器。位 0 对应 CPU 0，位 1 对应 CPU 1，以此类推直到位 63 对应 CPU 63。 |

## 返回值

类型：`List<[u32; CONSUMER_CPUS]>`

与 `mask` 中被设置的位对应的 `u32` CPU 索引列表，按升序排列。如果 `mask` 为 `0`，则返回空列表。

### 示例

| 输入 (`mask`) | 输出 |
|---------------|------|
| `0x0F` | `[0, 1, 2, 3]` |
| `0x8001` | `[0, 15]` |
| `0x00` | `[]` |
| `0xFFFFFFFFFFFFFFFF` | `[0, 1, 2, ..., 63]` |

## 备注

- 该函数遍历位位置 0 到 63，收集满足 `(mask >> i) & 1 == 1` 的位置。它使用 Rust 的 `Iterator::filter` 和 `Iterator::collect` 实现简洁的实现。

- 由于它从位 0 到位 63 按顺序遍历，因此结果列表天然按升序排列，无需额外的排序步骤。

- 此函数仅支持最多 64 个逻辑处理器（单个 Windows 处理器组）。对于拥有超过 64 个逻辑处理器的系统，应使用基于范围的 CPU 规格格式（`0-7;64-71`）而非十六进制位掩码。十六进制掩码格式被视为旧式格式，保留是为了向后兼容。

- 此函数是模块私有的（`fn`，非 `pub fn`），在 `config.rs` 外部不可访问。

### 算法

```/dev/null/pseudocode.txt#L1-3
for i in 0..64:
    if bit i of mask is set:
        append i to result list
```

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `config.rs` |
| 可见性 | 私有（crate 内部） |
| 调用方 | [`parse_cpu_spec`](parse_cpu_spec.md) |
| 被调用方 | 标准迭代器方法（`filter`、`collect`） |
| 依赖 | [`collections.rs`](../collections.rs/README.md) 中的 `List` 和 `CONSUMER_CPUS` |
| 权限 | 无 |

## 另请参阅

| 资源 | 链接 |
|------|------|
| parse_cpu_spec | [parse_cpu_spec](parse_cpu_spec.md) |
| cpu_indices_to_mask | [cpu_indices_to_mask](cpu_indices_to_mask.md) |
| format_cpu_indices | [format_cpu_indices](format_cpu_indices.md) |
| config 模块概述 | [README](README.md) |

---
*提交：[37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*