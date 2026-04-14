# config 模块 (AffinityServiceRust)

`config` 模块是 AffinityServiceRust 的配置引擎。它将类 INI 格式的配置文件解析为强类型的规则结构体，供服务循环用于管理进程优先级、CPU 亲和性、CPU 集合、I/O 优先级、内存优先级、主线程调度以及理想处理器分配。该模块支持 CPU 别名、命名进程组、调优常量、多字段规则（含按进程分级）以及在运行时对配置文件和黑名单的热重载。它还提供了用于转换 Process Lasso 配置和自动合并冗余规则的实用工具。

## 结构体

| 结构体 | 描述 |
|--------|------|
| [PrimePrefix](PrimePrefix.md) | 将模块名前缀过滤器与可选的 CPU 集合和线程优先级关联，用于主线程调度。 |
| [IdealProcessorRule](IdealProcessorRule.md) | 将一组 CPU 映射到可选的模块名前缀过滤器，用于理想处理器分配。 |
| [ProcessConfig](ProcessConfig.md) | 完整的按进程配置记录，包含从单个配置条目解析出的所有规则字段。 |
| [ConfigConstants](ConfigConstants.md) | 可调节的滞后常量，控制主线程的提升和降级行为。 |
| [ConfigResult](ConfigResult.md) | 配置解析过程的聚合输出，包含规则映射、常量、统计信息、错误和警告。 |

## 函数

| 函数 | 描述 |
|------|------|
| [parse_cpu_spec](parse_cpu_spec.md) | 将 CPU 规格字符串（范围、十六进制掩码、分号分隔的索引）解析为排序后的 `Vec<u32>`。 |
| [mask_to_cpu_indices](mask_to_cpu_indices.md) | 将 64 位位掩码转换为排序后的 CPU 索引向量。 |
| [cpu_indices_to_mask](cpu_indices_to_mask.md) | 将 CPU 索引切片转换为 `usize` 位掩码。 |
| [format_cpu_indices](format_cpu_indices.md) | 将 CPU 索引切片格式化为紧凑的、人类可读的范围字符串。 |
| [resolve_cpu_spec](resolve_cpu_spec.md) | 解析可能引用 `*alias` 的 CPU 规格字符串，回退到 [parse_cpu_spec](parse_cpu_spec.md)。 |
| [collect_members](collect_members.md) | 将冒号分隔的文本行拆分为经过修剪和小写化的成员名称。 |
| [parse_constant](parse_constant.md) | 解析 `@CONSTANT = value` 行并更新 [ConfigResult](ConfigResult.md) 中的常量。 |
| [parse_alias](parse_alias.md) | 解析 `*alias = cpu_spec` 行并将别名插入 CPU 别名映射。 |
| [parse_ideal_processor_spec](parse_ideal_processor_spec.md) | 将理想处理器规格字符串解析为 [IdealProcessorRule](IdealProcessorRule.md) 向量。 |
| [collect_group_block](collect_group_block.md) | 从多行 `{ ... }` 组块中收集进程名称，直到遇到右花括号。 |
| [parse_and_insert_rules](parse_and_insert_rules.md) | 解析冒号分隔的规则字段，并为每个成员插入一个 [ProcessConfig](ProcessConfig.md)。 |
| [read_config](read_config.md) | 主入口点：读取并解析配置文件为 [ConfigResult](ConfigResult.md)。 |
| [read_list](read_list.md) | 读取简单的逐行列表文件（用于黑名单）。 |
| [read_utf16le_file](read_utf16le_file.md) | 读取 UTF-16 LE 编码的文件并将其内容作为 Rust `String` 返回。 |
| [parse_mask](parse_mask.md) | 便捷封装函数，将 CPU 规格字符串直接解析为 `usize` 位掩码。 |
| [convert](convert.md) | 将 Process Lasso 配置文件转换为 AffinityServiceRust 原生格式。 |
| [sort_and_group_config](sort_and_group_config.md) | 自动合并具有相同规则设置的进程，以减少配置重复。 |
| [hotreload_blacklist](hotreload_blacklist.md) | 检查黑名单文件的修改时间，如有更改则重新加载。 |
| [hotreload_config](hotreload_config.md) | 检查配置文件的修改时间，如有更改则重新加载，解析失败时保留原配置。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 进程优先级 / IO 优先级 / 内存优先级枚举 | [priority 模块](../priority.rs/README.md) |
| 规则应用到运行中的进程 | [apply 模块](../apply.rs/README.md) |
| 主线程调度器状态 | [scheduler 模块](../scheduler.rs/README.md) |
| CLI 参数解析与帮助文本 | [cli 模块](../cli.rs/README.md) |
| Windows API 封装（CPU 集合、句柄） | [winapi 模块](../winapi.rs/README.md) |
| 服务主循环 | [main 模块](../main.rs/README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd