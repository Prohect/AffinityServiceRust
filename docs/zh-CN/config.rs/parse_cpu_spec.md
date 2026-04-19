# parse_cpu_spec 函数 (config.rs)

将 CPU 规格字符串解析为已排序的 CPU 索引列表。这是配置系统中使用的中心 CPU 集合解析器——每个 CPU 别名定义、亲和性字段、CPU 集合字段和 Prime 线程字段最终都会通过这个函数。

## 语法

```rust
pub fn parse_cpu_spec(s: &str) -> List<[u32; CONSUMER_CPUS]>
```

## 参数

`s: &str`

描述一组逻辑处理器索引的字符串。接受以下格式：

| 格式 | 示例 | 结果 |
|--------|---------|--------|
| 空或 `"0"` | `""`, `"0"` | 空列表（无 CPU/无更改） |
| 十六进制位掩码 | `"0xFF"`, `"0x15"` | 从设置位派生的 CPU 索引（旧版，≤64 核） |
| 包含范围 | `"0-7"` | `[0, 1, 2, 3, 4, 5, 6, 7]` |
| 分号分隔的单个值 | `"0;4;8"` | `[0, 4, 8]` |
| 混合范围和单个值 | `"0-3;8;12-15"` | `[0, 1, 2, 3, 8, 12, 13, 14, 15]` |

## 返回值

`List<[u32; CONSUMER_CPUS]>` — 已排序、去重的 [SmallVec](../collections.rs/README.md) 支持的 `u32` CPU 索引列表。如果输入为空、`"0"` 或无法解析，则返回空列表。

## 备注

### 解析算法

1. **修剪** 输入字符串。
2. **空/零检查：** 如果修剪后的字符串为空或恰好是 `"0"`，立即返回空列表。值 `"0"` 是配置约定，表示"无 CPU 限制"。
3. **十六进制前缀检测：** 如果字符串以 `"0x"` 或 `"0X"` 开头，将剩余部分解析为十六进制 `u64`，并委托给 [mask_to_cpu_indices](mask_to_cpu_indices.md) 提取设置位位置。解析失败时，返回空列表。
4. **分号分割：** 在 `';'` 上分割字符串。每个段独立处理：
   - 如果段包含 `'-'`，解析破折号前后的部分为 `u32` 起始值和结束值，并插入 `[start, end]` 范围内每个整数。
   - 否则，将段解析为单个 `u32` CPU 索引。
   - 插入时跳过重复项（`contains` 检查）。
5. 在返回之前，将结果列表按升序**排序**。

### 设计选择

- **分号作为分隔符：** 冒号 (`:`) 保留用于配置规则格式中的字段分隔符，因此 CPU 规格使用分号来分隔单个值或范围。
- **`"0"` 表示无更改：** 这是故意的哨兵值。配置字段为 `0` 表示服务不应修改相应设置。这与十六进制掩码 `0x0`（也产生空列表）不同。
- **十六进制掩码限制在 64 核：** 十六进制位掩码格式是作为 Process Lasso 兼容性的旧版便利功能继承而来的。对于拥有超过 64 个逻辑处理器的系统，请使用基于范围的语法（`"0-7;64-71"`）。
- **无错误报告：** 分号分隔规格内的无效令牌会被静默跳过（`unwrap_or` 到 `0` 或解析失败被忽略）。验证错误在更高层级通过检查结果列表是否意外为空来捕获。

### 栈分配

返回类型 `List<[u32; CONSUMER_CPUS]>` 是 `SmallVec`，可在栈上存储多达 `CONSUMER_CPUS`（32）个 CPU 索引。典型的消费者系统（≤32 核）永远不会堆分配。

## 需求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **调用方** | [parse_alias](parse_alias.md), [resolve_cpu_spec](resolve_cpu_spec.md), [parse_mask](parse_mask.md), [convert](convert.md) |
| **被调用方** | [mask_to_cpu_indices](mask_to_cpu_indices.md) |
| **权限** | 无 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [config.rs](README.md) |
| 位掩码到 CPU 索引 | [mask_to_cpu_indices](mask_to_cpu_indices.md) |
| CPU 索引到位掩码 | [cpu_indices_to_mask](cpu_indices_to_mask.md) |
| 格式化 CPU 索引用于显示 | [format_cpu_indices](format_cpu_indices.md) |
| 别名解析包装器 | [resolve_cpu_spec](resolve_cpu_spec.md) |
| 集合类型 | [List / CONSUMER_CPUS](../collections.rs/README.md) |

*文档针对 Commit：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*