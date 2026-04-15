# cpu_indices_to_mask 函数 (config.rs)

将 CPU 索引切片转换为适用于 Windows 亲和性 API（如 `SetProcessAffinityMask`）的 `usize` 位掩码。输入中的每个 CPU 索引会设置输出掩码中对应的位位置。

## 语法

```AffinityServiceRust/src/config.rs#L119-127
pub fn cpu_indices_to_mask(cpus: &[u32]) -> usize {
    let mut mask: usize = 0;
    for &cpu in cpus {
        if cpu < 64 {
            mask |= 1usize << cpu;
        }
    }
    mask
}
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `cpus` | `&[u32]` | CPU 索引值切片。每个值表示一个从零开始的逻辑处理器编号。大于等于 64 的值会被静默忽略，因为 64 位 Windows 上的 `usize` 位掩码只能表示处理器 0–63。 |

## 返回值

类型：`usize`

一个位掩码，如果值 `N` 出现在 `cpus` 切片中，则位 *N* 被设置为 `1`，否则为 `0`。如果 `cpus` 为空或所有索引都 ≥ 64，则返回 `0`。

### 示例

| 输入 | 输出 | 说明 |
|------|------|------|
| `&[0, 1, 2, 3]` | `0x0F` | 位 0–3 被设置 |
| `&[0, 2, 4]` | `0x15` | 位 0、2、4 被设置 |
| `&[]` | `0` | 无位被设置 |
| `&[64, 65]` | `0` | 所有索引 ≥ 64，静默跳过 |
| `&[0, 63]` | `0x8000000000000001` | 第一个和最后一个可表示的位 |

## 备注

- **64 核限制**：此函数仅限于表示前 64 个逻辑处理器。在拥有超过 64 个逻辑处理器（多处理器组）的系统上，≥ 64 的索引会被静默丢弃。对于超过 64 核的系统，应优先使用 CPU 集合 API 而非基于位掩码的亲和性。

- **`mask_to_cpu_indices` 的逆操作**：此函数是 [`mask_to_cpu_indices`](mask_to_cpu_indices.md) 的逻辑逆操作。将掩码转换为索引再转换回来会产生原始掩码：对于任何 `u64` 值 `m`（当转换为 `usize` 时），`cpu_indices_to_mask(&mask_to_cpu_indices(m)) == m`。

- **重复值处理**：输入切片中的重复值是无害的——通过按位 OR 两次设置同一位是幂等的。

- **无需排序**：输入不需要排序。函数无论顺序如何都会遍历所有元素。

- **平台说明**：在 64 位 Windows 上，`usize` 为 64 位宽，因此所有索引 0–63 都是可表示的。在假设的 32 位构建中，索引 32–63 也会由于 `usize` 的宽度而被静默忽略，但 AffinityServiceRust 仅面向 64 位 Windows。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `config.rs` |
| 可见性 | `pub` |
| 调用方 | [`parse_mask`](parse_mask.md)、`apply.rs`（亲和性应用逻辑） |
| 被调用方 | 无 |
| API | 无（纯计算） |
| 权限 | 无 |

## 另请参阅

| 资源 | 链接 |
|------|------|
| mask_to_cpu_indices | [mask_to_cpu_indices](mask_to_cpu_indices.md) |
| parse_cpu_spec | [parse_cpu_spec](parse_cpu_spec.md) |
| format_cpu_indices | [format_cpu_indices](format_cpu_indices.md) |
| parse_mask | [parse_mask](parse_mask.md) |
| config 模块概述 | [README](README.md) |

---
*Commit: 7221ea0694670265d4eb4975582d8ed2ae02439d*