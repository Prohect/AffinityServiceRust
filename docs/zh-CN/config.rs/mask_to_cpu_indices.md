# mask_to_cpu_indices 函数 (config.rs)

将 64 位 CPU 位掩码转换为单个 CPU 索引的排序列表。掩码中的每个设置位对应输出列表中的一个逻辑处理器编号。

## 语法

```rust
fn mask_to_cpu_indices(mask: u64) -> List<[u32; CONSUMER_CPUS]>
```

## 参数

`mask: u64`

64 位位掩码，其中位*N*被设置表示逻辑处理器*N*应包含在输出列表中。例如，`0x15`（二进制 `10101`）表示处理器 0、2 和 4。

## 返回值

`List<[u32; CONSUMER_CPUS]>` — 按升序排序的栈分配的小型向量，包含 CPU 索引。如果 `mask` 为 `0`，则返回的空列表。

## 备注

函数迭代位位置 0 到 63，使用 `(mask >> i) & 1 == 1` 测试每个位。匹配的位位置通过 `Iterator::collect()` 特性收集到输出列表中。由于位从 LSB 到 MSB 测试，结果列表自然按升序排序。

### 限制

- 函数支持最多 64 个逻辑处理器，这是单个 Windows 处理器组的限制。具有超过 64 个处理器的系统需要使用基于范围的 CPU 规格（`"0-7;64-71"`），由 [parse_cpu_spec](parse_cpu_spec.md) 处理，而非位掩码表示法。
- 返回类型使用 `CONSUMER_CPUS`（32）作为内联容量。如果掩码中设置了超过 32 个 CPU，列表会溢出到堆分配，这对于配置解析热路径是可以接受的。

### 可见性

此函数是模块私有的（`fn`，而非 `pub fn`）。仅由 [parse_cpu_spec](parse_cpu_spec.md) 调用，用于解析十六进制 CPU 掩码字符串（例如，`"0xFF"`）。

### 逆操作

此函数的逆操作是 [cpu_indices_to_mask](cpu_indices_to_mask.md)，它将 CPU 索引切片转换回 `usize` 位掩码。

## 需求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **调用方** | [parse_cpu_spec](parse_cpu_spec.md)（十六进制掩码分支） |
| **被调用方** | 无 |
| **权限** | 无 |

## 参见

| 主题 | 链接 |
|-------|------|
| CPU 规格解析器 | [parse_cpu_spec](parse_cpu_spec.md) |
| 逆转换 | [cpu_indices_to_mask](cpu_indices_to_mask.md) |
| CPU 索引格式化工具 | [format_cpu_indices](format_cpu_indices.md) |
| 十六进制掩码解析器 | [parse_mask](parse_mask.md) |
| 集合类型 | [List](../collections.rs/README.md) |

*文档针对 Commit：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*