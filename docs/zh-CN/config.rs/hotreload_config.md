# hotreload_config 函数 (config.rs)

检查配置文件的最后修改时间戳，如果自上次检查以来发生了变化，则通过 [read_config](read_config.md) 重新解析该文件。当解析成功（无错误）时，实时规则映射、调优常量和相关调度器状态将被原子性地替换。当解析失败时，保留先前的配置并记录错误。

## 语法

```rust
pub fn hotreload_config(
    cli: &CliArgs,
    configs: &mut HashMap<u32, HashMap<String, ProcessConfig>>,
    last_config_mod_time: &mut Option<std::time::SystemTime>,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    process_level_applied: &mut HashSet<u32>,
)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `cli` | `&CliArgs` | 已解析的命令行参数的引用。`config_file_name` 字段提供了被监控修改时间的配置文件路径。 |
| `configs` | `&mut HashMap<u32, HashMap<String, ProcessConfig>>` | 实时规则映射的可变引用（按等级索引，再按小写进程名索引）。热重载成功时，该映射将被 [ConfigResult](ConfigResult.md)`.configs` 中新解析的规则完全替换。 |
| `last_config_mod_time` | `&mut Option<std::time::SystemTime>` | 上次检查时缓存的修改时间戳的可变引用。每次成功读取元数据后更新为文件当前的 `modified()` 时间。当为 `None` 时，任何可读的时间戳都会触发重新加载。 |
| `prime_core_scheduler` | `&mut PrimeThreadScheduler` | 实时主线程调度器的可变引用。热重载成功时，调度器的 `constants` 字段将被新解析的 [ConfigConstants](ConfigConstants.md) 替换，使阈值变更在下一个调度周期生效，无需重启服务。 |
| `process_level_applied` | `&mut HashSet<u32>` | 已应用进程级别设置（进程优先级、CPU 亲和性、CPU 集合、I/O 优先级、内存优先级）的进程 ID 集合的可变引用。热重载成功时清空该集合，以便在下一次服务循环迭代中将新规则重新应用到所有运行中的进程。 |

## 返回值

此函数没有返回值。所有结果通过对参数的修改和日志输出来传达。

## 备注

### 重新加载算法

1. **元数据检查** — 对 `cli.config_file_name` 调用 `std::fs::metadata`。如果调用失败（例如文件被删除或不可访问），函数直接返回，不做任何操作，保留当前配置。
2. **时间戳比较** — 将文件的 `modified()` 时间与 `*last_config_mod_time` 进行比较。如果时间戳相同（或两者都是具有相同值的 `Some`），则无需重新加载，函数立即返回。
3. **时间戳更新** — 一旦观察到新的修改时间，`*last_config_mod_time` 将无条件设置为 `Some(mod_time)`。这可以防止在同一文件版本的每次循环迭代中重复尝试重新加载。
4. **重新解析** — 调用 [read_config](read_config.md) 并传入 `cli.config_file_name`，生成新的 [ConfigResult](ConfigResult.md)。
5. **验证关卡** — 如果 `new_config_result.errors` 非空，则中止重新加载：
   - 记录一条消息，说明文件存在错误，将保留先前的配置。
   - 逐条记录每个错误。
   - `configs`、`prime_core_scheduler` 和 `process_level_applied` 保持不变。
6. **成功时替换** — 如果没有错误：
   - 调用 `new_config_result.print_report()` 记录组/规则统计信息和任何警告。
   - 通过 `new_config_result.total_rules()` 获取总规则数。
   - 将 `*configs` 替换为 `new_config_result.configs`。
   - 将 `prime_core_scheduler.constants` 替换为 `new_config_result.constants`。
   - 记录包含规则数量的完成消息。
   - 调用 `process_level_applied.clear()` 强制重新应用进程级别设置。

### 清空 process_level_applied

`process_level_applied` 集合跟踪哪些 PID 已经应用了进程级别规则。热重载成功后清空该集合，确保变更后的进程优先级、CPU 亲和性、CPU 集合、I/O 优先级和内存优先级设置在下一次服务循环迭代中重新应用到所有当前运行的进程。如果不进行此重置，在旧规则下已经配置过的进程将不会收到更新的设置，直到它们被重新启动。

### 错误容错

该函数使用 **故障安全** 策略：实时配置永远不会被无效的解析结果替换。如果新文件包含语法错误、未定义的别名或其他致命问题，先前的有效配置将继续使用。这对于长期运行的 Windows 服务至关重要，因为临时的配置文件编辑（例如用户仍在输入中）不应中断正在进行的进程管理。

### 与 hotreload_blacklist 的交互

[hotreload_blacklist](hotreload_blacklist.md) 独立监控黑名单文件，使用单独的时间戳。两个函数在服务主循环的每次迭代中都会被调用，因此配置和黑名单的变更会在一个循环间隔内被检测并应用（由 `-interval` CLI 参数控制，默认为几秒）。

### let-chain 模式匹配

该函数使用 Rust 的 `let`-chain 语法（`if let Ok(...) = ... && let Ok(...) = ... && ...`）将元数据读取、修改时间提取和时间戳比较组合成一个条件块。如果链中的任何步骤失败，整个块将被跳过，函数静默返回——在检查阶段不会为临时的文件系统错误产生日志输出。

### 首次调用行为

首次调用时，`*last_config_mod_time` 为 `None`。由于 `Some(mod_time) != None` 始终为真，首次成功的元数据读取总是会触发重新加载。这是有意设计的——它允许服务检测在启动时（配置最初由 [main](../main.rs/main.md) 加载时）和首次热重载检查之间发生的配置变更。

### 线程安全

此函数不是为并发访问设计的。它从单线程服务主循环中调用。调用者负责确保对 `configs`、`prime_core_scheduler` 和 `process_level_applied` 参数的独占访问。

### 日志记录

| 条件 | 日志输出 |
|------|----------|
| 检测到文件修改 | `"Configuration file '{path}' changed, reloading..."` |
| 重新加载成功 | 解析报告（通过 `print_report()`），然后 `"Configuration reload complete: {N} rules loaded."` |
| 重新加载失败（有错误） | `"Configuration file '{path}' has errors, keeping previous configuration."`，随后是每条错误前缀 `"  - "`。 |
| 文件不可访问 / 未变更 | *（无输出）* |

## 要求

| | |
|---|---|
| **模块** | `config` (`src/config.rs`) |
| **可见性** | `pub` |
| **调用者** | [main](../main.rs/main.md)（服务循环） |
| **被调用者** | [read_config](read_config.md)、[ConfigResult::print_report](ConfigResult.md)、[ConfigResult::total_rules](ConfigResult.md) |
| **API** | `std::fs::metadata`、`std::fs::Metadata::modified` |
| **权限** | 配置文件路径的读取权限 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 主配置解析器 | [read_config](read_config.md) |
| 解析后的配置聚合结果 | [ConfigResult](ConfigResult.md) |
| 滞后调优常量 | [ConfigConstants](ConfigConstants.md) |
| 每进程规则结构体 | [ProcessConfig](ProcessConfig.md) |
| 黑名单热重载 | [hotreload_blacklist](hotreload_blacklist.md) |
| 主线程调度器 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| CLI 参数结构体 | [CliArgs](../cli.rs/CliArgs.md) |
| 模块概述 | [config 模块](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd