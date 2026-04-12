# ConfigResult 结构体 (config.rs)

由 [read_config](read_config.md) 返回的结果容器，包含所有已解析的配置数据、统计计数器，以及解析过程中遇到的错误和警告。

## 语法

```rust
pub struct ConfigResult {
    pub configs: HashMap<u32, HashMap<String, ProcessConfig>>,
    pub constants: ConfigConstants,
    pub constants_count: usize,
    pub aliases_count: usize,
    pub groups_count: usize,
    pub group_members_count: usize,
    pub process_rules_count: usize,
    pub redundant_rules_count: usize,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}
```

## 字段

`configs`

已解析的进程配置的两级映射。外层键为等级级别（`u32`，从 1 开始），内层键为小写的进程名称。每个值为一个 [ProcessConfig](ProcessConfig.md) 实例。等级级别允许分层规则应用——等级 1 的规则优先应用，更高等级用于后续的二级匹配。

`constants`

一个 [ConfigConstants](ConfigConstants.md) 实例，包含从配置文件中 `@CONSTANT = value` 行解析出的调度器调优参数。如果未指定常量，则使用默认值。

`constants_count`

从 `@CONSTANT` 指令中成功解析的常量数量。

`aliases_count`

从 `*alias = cpu_spec` 指令中成功解析的 CPU 别名数量。

`groups_count`

从配置文件中解析到的进程组（花括号块）数量。

`group_members_count`

所有组中各个进程名称的总数。

`process_rules_count`

插入到 `configs` 中的进程规则总数，包括单独规则和展开的组成员。

`redundant_rules_count`

覆盖先前定义的相同进程名称规则的数量。每条冗余规则同时会生成一条警告。

`errors`

解析过程中遇到的错误消息列表。如果此列表非空，则配置被视为无效，不应用于进程管理。

`warnings`

非致命性问题的警告消息列表，例如未知的优先级名称、空组或冗余规则。

## 备注

`ConfigResult` 实现了 `Default` trait，将 `configs` 初始化为空映射，`constants` 使用 [ConfigConstants](ConfigConstants.md) 的默认值，所有计数器初始化为零，`errors` 和 `warnings` 均为空向量。

该结构体提供三个便捷方法：

- **`is_valid()`** — 当 `errors` 为空时返回 `true`，表示配置可以安全使用。
- **`total_rules()`** — 返回所有等级中规则的总和。
- **`print_report()`** — 记录解析结果摘要。成功时打印组和规则计数；失败时打印所有错误和警告。

`configs` 映射按等级作为键，以支持分层规则匹配。典型配置使用等级 1（默认值）。等级字段是规则行中最后一个可选字段，由 [parse_and_insert_rules](parse_and_insert_rules.md) 解析。

### 验证流程

```text
read_config() → ConfigResult
    if result.is_valid()
        result.print_report()   // 记录成功摘要
        // 继续使用 configs
    else
        result.print_report()   // 记录错误
        // 中止
```

### 错误与警告示例

| 条件 | 严重级别 |
| --- | --- |
| 无法打开配置文件 | 错误 |
| 引用了未定义的 CPU 别名 | 错误 |
| 规则行字段过少 | 错误 |
| 未关闭的组块（缺少 `}`） | 错误 |
| 未知的优先级字符串 | 警告 |
| 空组（无成员） | 警告 |
| 冗余/重复的进程规则 | 警告 |

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/config.rs |
| **派生宏** | `Debug`, `Default` |
| **使用者** | [read_config](read_config.md)、[parse_constant](parse_constant.md)、[parse_alias](parse_alias.md)、[parse_and_insert_rules](parse_and_insert_rules.md)、[main](../main.rs/main.md) |

## 另请参阅

- [read_config](read_config.md)
- [ProcessConfig](ProcessConfig.md)
- [ConfigConstants](ConfigConstants.md)
- [parse_and_insert_rules](parse_and_insert_rules.md)