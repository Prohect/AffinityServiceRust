# config 模块 (AffinityServiceRust)

`config` 模块负责 AffinityServiceRust 配置文件的解析、验证和管理。它定义了进程级和线程级调度策略的规则结构，并实现了多段 INI 风格的配置解析器，支持 CPU 别名（`*name = spec`）、命名组（`name { members }`）、调优常量（`@NAME = value`）以及复杂的 CPU 规格（范围、十六进制掩码、分号分隔的索引）。该模块还提供了配置和黑名单文件的热重载功能、Process Lasso 格式转换器，以及自动分组实用工具，可将具有相同规则的进程合并。

## 结构体

| 名称 | 描述 |
|------|-------------|
| [PrimePrefix](PrimePrefix.md) | 将模块名前缀与 CPU 集合和可选的线程优先级提升关联，用于 Prime 线程匹配。 |
| [IdealProcessorRule](IdealProcessorRule.md) | 将线程起始模块前缀映射到理想的 CPU 分配。 |
| [ProcessLevelConfig](ProcessLevelConfig.md) | 每个进程应用一次的规则：优先级、亲和性、CPU 集合、IO 优先级、内存优先级。 |
| [ThreadLevelConfig](ThreadLevelConfig.md) | 每个进程的线程级规则，每个轮询迭代应用：Prime 线程、理想处理器、跟踪。 |
| [ConfigConstants](ConfigConstants.md) | Prime 线程选择中的调优常量（streak、阈值）。 |
| [ConfigResult](ConfigResult.md) | 配置解析的聚合结果：按等级分类的规则映射、计数器、错误和警告。 |

## 函数

| 名称 | 描述 |
|------|-------------|
| [parse_cpu_spec](parse_cpu_spec.md) | 将 CPU 规格字符串（范围、十六进制掩码、分号）解析为排序后的 CPU 索引列表。 |
| [mask_to_cpu_indices](mask_to_cpu_indices.md) | 将 64 位位掩码转换为排序后的 CPU 索引列表。 |
| [cpu_indices_to_mask](cpu_indices_to_mask.md) | 将 CPU 索引切片转换为 `usize` 位掩码。 |
| [format_cpu_indices](format_cpu_indices.md) | 将 CPU 索引切片格式化为紧凑的范围字符串，如 `"0-7,12-19"`。 |
| [parse_mask](parse_mask.md) | 便利包装器：直接将 CPU 规格字符串解析为位掩码。 |
| [resolve_cpu_spec](resolve_cpu_spec.md) | 解析可能是 `*alias` 引用或字面量规格的 CPU 规格。 |
| [collect_members](collect_members.md) | 将冒号分隔的进程名称拆分为成员列表。 |
| [parse_constant](parse_constant.md) | 解析 `@NAME = value` 常量定义（MIN_ACTIVE_STREAK、KEEP_THRESHOLD、ENTRY_THRESHOLD）。 |
| [parse_alias](parse_alias.md) | 解析 `*name = cpu_spec` 别名定义。 |
| [parse_ideal_processor_spec](parse_ideal_processor_spec.md) | 解析理想处理器规格，如 `*alias@prefix1;prefix2`。 |
| [collect_group_block](collect_group_block.md) | 在 `{` 和 `}` 之间收集多行组块成员。 |
| [parse_and_insert_rules](parse_and_insert_rules.md) | 主规则解析器：拆分字段、验证、创建配置、按等级插入到 [ConfigResult](ConfigResult.md)。 |
| [read_config](read_config.md) | 主配置文件读取器。处理常量、别名、组和单行规则。 |
| [read_bleack_list](read_bleack_list.md) | 读取黑名单文件（每行一个进程名，`#` 注释）。 |
| [read_utf16le_file](read_utf16le_file.md) | 读取 UTF-16LE 编码的文件并返回为 Rust `String`。 |
| [convert](convert.md) | 将 Process Lasso 配置格式转换为 AffinityServiceRust 格式。 |
| [sort_and_group_config](sort_and_group_config.md) | 将具有相同设置的规则自动分组到命名组块中。 |
| [hotreload_blacklist](hotreload_blacklist.md) | 如果黑名单文件在磁盘上已修改，则热重载黑名单文件。 |
| [hotreload_config](hotreload_config.md) | 如果配置文件已修改，则热重载配置，成功时重置调度器状态。 |

## 配置文件格式

配置文件使用行导向格式，包含多个部分类型：

```
# 注释以 # 开头

# 常量（调优参数）
@MIN_ACTIVE_STREAK = 3
@KEEP_THRESHOLD = 0.05
@ENTRY_THRESHOLD = 0.1

# CPU 别名
*pcore = 0-7
*ecore = 8-19

# 单行规则
process.exe:priority:affinity:cpuset:prime_cpus:io_priority:memory_priority:ideal_processor:grade

# 命名组规则
group_name { proc1.exe: proc2.exe: proc3.exe }:normal:*ecore:0:0:low:none:0:1

# 多行组
group_name {
    proc1.exe: proc2.exe
    proc3.exe
}:normal:*ecore:0:0:low:none:0:1
```

## 参见

| 主题 | 链接 |
|-------|------|
| 执行引擎 | [apply.rs](../apply.rs/README.md) |
| 优先级枚举 | [ProcessPriority](../priority.rs/ProcessPriority.md), [IOPriority](../priority.rs/IOPriority.md), [MemoryPriority](../priority.rs/MemoryPriority.md), [ThreadPriority](../priority.rs/ThreadPriority.md) |
| Prime 线程调度器 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| CLI 参数 | [CliArgs](../cli.rs/CliArgs.md) |
| 集合类型 | [List / HashMap](../collections.rs/README.md) |
| 主服务循环 | [main.rs](../main.rs/README.md) |

*文档为 Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*