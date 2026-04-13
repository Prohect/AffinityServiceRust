# config.rs 模块

配置文件解析、CPU 规格处理和配置操作工具。

本模块负责读取和解释配置文件格式、解析 CPU 别名、解析进程规则（包括分组），以及提供转换/自动分组实用工具。

## 数据结构

| 结构体 | 说明 |
| --- | --- |
| [ProcessConfig](ProcessConfig.md) | 单个进程规则的完整配置 |
| [PrimePrefix](PrimePrefix.md) | 模块特定前缀规则，用于主线程调度 |
| [IdealProcessorRule](IdealProcessorRule.md) | 理想处理器分配规则，按模块前缀将线程分配到指定 CPU |
| [ConfigConstants](ConfigConstants.md) | 调度器行为调优常量 |
| [ConfigResult](ConfigResult.md) | 配置文件解析的聚合结果，含统计信息和诊断 |

## CPU 规格函数

| 函数 | 说明 |
| --- | --- |
| [parse_cpu_spec](parse_cpu_spec.md) | 将 CPU 规格字符串解析为排序后的 CPU 索引列表 |
| [mask_to_cpu_indices](mask_to_cpu_indices.md) | 将 64 位掩码转换为 CPU 索引向量 |
| [cpu_indices_to_mask](cpu_indices_to_mask.md) | 将 CPU 索引转换为位掩码（≤64 核） |
| [format_cpu_indices](format_cpu_indices.md) | 将 CPU 索引格式化为紧凑的可读字符串 |
| [parse_mask](parse_mask.md) | 便捷封装：将 CPU 规格字符串直接解析为位掩码 |

## 配置解析函数

| 函数 | 说明 |
| --- | --- |
| [read_config](read_config.md) | 主入口 - 读取并解析配置文件 |
| [resolve_cpu_spec](resolve_cpu_spec.md) | 解析可能包含别名引用的 CPU 规格 |
| [collect_members](collect_members.md) | 从冒号分隔的文本中提取组成员名称 |
| [parse_constant](parse_constant.md) | 解析并应用 `@CONSTANT = value` 行 |
| [parse_alias](parse_alias.md) | 解析并注册 `*alias = cpu_spec` 行 |
| [parse_ideal_processor_spec](parse_ideal_processor_spec.md) | 解析带模块前缀过滤的理想处理器规格 |
| [collect_group_block](collect_group_block.md) | 从多行 `{ ... }` 组块中收集成员 |
| [parse_and_insert_rules](parse_and_insert_rules.md) | 解析规则字段并为所有成员插入 [ProcessConfig](ProcessConfig.md) 条目 |

## 文件 I/O 函数

| 函数 | 说明 |
| --- | --- |
| [read_list](read_list.md) | 读取简单的逐行列表文件（如黑名单） |
| [read_utf16le_file](read_utf16le_file.md) | 读取 UTF-16 LE 编码文件为 Rust `String` |

## 转换与分组工具

| 函数 | 说明 |
| --- | --- |
| [convert](convert.md) | 将 Process Lasso INI 配置转换为本项目格式 |
| [sort_and_group_config](sort_and_group_config.md) | 自动分组具有相同规则的进程以减少重复 |

## 热重载函数

| 函数 | 说明 |
| --- | --- |
| [hotreload_blacklist](hotreload_blacklist.md) | 检查黑名单文件是否变更并按需重载 |
| [hotreload_config](hotreload_config.md) | 检查配置文件是否变更并按需重载 |

## 另请参阅

- [priority.rs](../priority.rs/) — `ProcessPriority`、`IOPriority`、`MemoryPriority`、`ThreadPriority` 枚举
- [apply.rs](../apply.rs/) — 使用 [ProcessConfig](ProcessConfig.md) 应用设置的函数
- [scheduler.rs](../scheduler.rs/) — 使用 [ConfigConstants](ConfigConstants.md) 的主线程调度器
- [cli.rs](../cli.rs/) — 调用 [read_config](read_config.md)、[convert](convert.md) 和 [sort_and_group_config](sort_and_group_config.md) 的命令行接口
