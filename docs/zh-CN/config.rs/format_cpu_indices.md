# format_cpu_indices 函数 (config.rs)

将 CPU 索引切片格式化为紧凑的、人类可读的范围字符串。连续的索引被折叠成连字符分隔的范围，非连续的索引用逗号分隔。

## 语法

```rust
pub fn format_cpu_indices(cpus: &[u32]) -> String
```

## 参数

`cpus: &[u32]`

要格式化的 CPU 索引切片。切片不需要预先排序；函数会在内部对副本进行排序。重复值在排序时保留，但不影响输出范围。

## 返回值

`String` — CPU 索引的紧凑表示。如果输入切片为空，则返回 `"0"`。否则，返回逗号分隔的字符串，其中连续的运行被折叠为范围。

**输出示例：**

| 输入 | 输出 |
|-------|--------|
| `[]` | `"0"` |
| `[0, 1, 2, 3]` | `"0-3"` |
| `[0, 2, 4]` | `"0,2,4"` |
| `[0, 1, 2, 8, 9, 10, 16]` | `"0-2,8-10,16"` |
| `[5]` | `"5"` |

## 备注

函数将输入复制到栈分配的 `List<[u32; CONSUMER_CPUS]>`（`SmallVec`），并在格式化之前对其进行排序。这确保了无论输入顺序如何，都能正确检测范围。

### 范围折叠算法

函数迭代排序后的列表。对于每个起始索引，它扩展连续运行，只要下一个值等于当前值加一。当运行结束时：

- 如果运行的起始和结束相等，则附加单个值。
- 如果起始和结束不同，则附加 `"start-end"` 范围字符串。

范围和单个值用逗号连接。

### 空值返回零约定

对于空输入，返回字符串 `"0"`，因为 AffinityServiceRust 配置格式中的 `0` 意味着"无更改"或"未配置"。此约定允许 `format_cpu_indices` 的输出直接插入配置行中，而无需特殊处理空的 CPU 集合。

### 与 parse_cpu_spec 的关系

`format_cpu_indices` 是 [parse_cpu_spec](parse_cpu_spec.md) 的反向函数，适用于范围和单独格式。也就是说，`parse_cpu_spec(format_cpu_indices(cpus))` 会产生原始输入的已排序、去重版本。但是，即使输入最初是从十六进制字符串解析的，`format_cpu_indices` 也永远不会发射十六进制掩码格式输出。

### 用法

此函数在整个代码库中使用，用于：

- 生成人类可读的日志消息，显示 CPU 分配（亲和性更改、CPU 集合更改、Prime 线程绑定）。
- 在 [convert](convert.md) 和 [sort_and_group_config](sort_and_group_config.md) 实用工具中生成配置文件输出。
- 在诊断消息中调试显示 CPU 集合。

## 需求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **调用方** | `apply_affinity`, `apply_process_default_cpuset`, `apply_prime_threads_promote`, `apply_prime_threads_demote`, `reset_thread_ideal_processors`, `apply_ideal_processors`, 日志函数 |
| **被调用方** | 无 |
| **权限** | 无 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [config.rs](README.md) |
| 反向解析器 | [parse_cpu_spec](parse_cpu_spec.md) |
| 位掩码到索引 | [mask_to_cpu_indices](mask_to_cpu_indices.md) |
| 索引到位掩码 | [cpu_indices_to_mask](cpu_indices_to_mask.md) |
| 集合类型 | [List / CONSUMER_CPUS](../collections.rs/README.md) |

*文档针对 Commit：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*