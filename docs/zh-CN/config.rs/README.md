# config 模块 (AffinityServiceRust)

`config` 模块是 AffinityServiceRust 的配置引擎。它负责解析、验证和热重载 INI 风格的配置文件，该文件定义了 Windows 进程的 CPU 亲和性、进程优先级、CPU 集合、I/O 优先级、内存优先级、主线程调度以及理想处理器规则。该模块还提供了转换 Process Lasso 配置和自动分组重复规则的实用工具。

## 函数

| 函数 | 描述 |
|------|------|
| [parse_cpu_spec](parse_cpu_spec.md) | 将 CPU 规格字符串（范围、十六进制掩码、单独索引）解析为排序后的 CPU 索引列表。 |
| [mask_to_cpu_indices](mask_to_cpu_indices.md) | 将 64 位位掩码转换为位被设置的 CPU 索引位置列表。 |
| [cpu_indices_to_mask](cpu_indices_to_mask.md) | 将 CPU 索引切片转换为 `usize` 位掩码（≤64 核）。 |
| [format_cpu_indices](format_cpu_indices.md) | 将 CPU 索引切片格式化为人类可读的紧凑范围字符串。 |
| [resolve_cpu_spec](resolve_cpu_spec.md) | 解析可能引用 `*alias` 或字面规格的 CPU 规格，并转换为 CPU 索引。 |
| [collect_members](collect_members.md) | 将以冒号分隔的进程名称字符串拆分为小写成员名称列表。 |
| [parse_constant](parse_constant.md) | 解析 `@NAME = value` 常量定义并将其应用到配置结果。 |
| [parse_alias](parse_alias.md) | 解析并注册 `*name = cpu_spec` 别名定义。 |
| [parse_ideal_processor_spec](parse_ideal_processor_spec.md) | 解析带有可选模块前缀过滤的理想处理器规格字符串，生成规则列表。 |
| [collect_group_block](collect_group_block.md) | 从多行 `{ … }` 分组块中收集进程名称，直到遇到右花括号。 |
| [parse_and_insert_rules](parse_and_insert_rules.md) | 从配置行中解析规则字段，并为所有分组成员插入进程级和线程级配置条目。 |
| [read_config](read_config.md) | 读取并解析整个配置文件，返回完整填充的 `ConfigResult`。 |
| [read_list](read_list.md) | 读取文本文件，将非空、非注释行作为小写字符串列表返回。 |
| [read_utf16le_file](read_utf16le_file.md) | 读取 UTF-16 LE 编码的文件，并将其内容作为 Rust `String` 返回。 |
| [parse_mask](parse_mask.md) | 便捷函数，解析 CPU 规格字符串并返回对应的 `usize` 位掩码。 |
| [convert](convert.md) | 将 Process Lasso 配置文件转换为 AffinityServiceRust 的原生格式。 |
| [sort_and_group_config](sort_and_group_config.md) | 自动将具有相同规则设置的进程分组，以减少配置重复。 |
| [hotreload_blacklist](hotreload_blacklist.md) | 监视黑名单文件的修改，并在文件变更时重新加载。 |
| [hotreload_config](hotreload_config.md) | 监视配置文件的修改，并在文件变更时热重载。 |

## 结构体

| 结构体 | 描述 |
|--------|------|
| [PrimePrefix](PrimePrefix.md) | 将线程启动模块前缀与可选的 CPU 亲和性和线程优先级关联，用于主线程调度。 |
| [IdealProcessorRule](IdealProcessorRule.md) | 将一组 CPU 映射到可选的模块名称前缀，用于理想处理器分配。 |
| [ProcessLevelConfig](ProcessLevelConfig.md) | 保存单个进程规则的所有进程级设置：优先级、亲和性、CPU 集合、I/O 优先级和内存优先级。 |
| [ThreadLevelConfig](ThreadLevelConfig.md) | 保存单个进程规则的所有线程级设置：主线程 CPU、前缀、跟踪数量和理想处理器规则。 |
| [ConfigConstants](ConfigConstants.md) | 控制主线程调度器行为的可调数值常量。 |
| [ConfigResult](ConfigResult.md) | 配置解析器的聚合输出，包含所有已解析的规则、常量、统计信息、错误和警告。 |

## 另请参阅

| 资源 | 链接 |
|------|------|
| cli 模块 | [cli.rs 概述](../cli.rs/README.md) |
| scheduler 模块 | [scheduler.rs 概述](../scheduler.rs/README.md) |
| priority 模块 | [priority.rs 概述](../priority.rs/README.md) |
| apply 模块 | [apply.rs 概述](../apply.rs/README.md) |

---
*Commit: 7221ea0694670265d4eb4975582d8ed2ae02439d*