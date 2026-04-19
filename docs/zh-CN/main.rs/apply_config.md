# apply_config 函数 (main.rs)

组合入口点，为单个匹配的进程同时应用进程级和线程级配置。创建共享的线程缓存，使得每个进程每次迭代最多只执行一次线程枚举，然后委托给 [apply_process_level](apply_process_level.md) 和 [apply_thread_level](apply_thread_level.md)。跟踪每个级别已处理的 PID 并记录合并后的结果。

## 语法

```rust
fn apply_config(
    cli: &CliArgs,
    configs: &ConfigResult,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    process_level_applied: &mut SmallVec<[u32; PIDS]>,
    thread_level_applied: &mut SmallVec<[u32; PENDING]>,
    grade: &u32,
    pid: &u32,
    name: &&str,
    process_level_config: &ProcessLevelConfig,
    process: &ProcessEntry,
)
```

## 参数

`cli: &CliArgs`

已解析的 [命令行参数](../cli.rs/CliArgs.md)。用于读取 `cli.dry_run`，该值会被转发给两个 apply 函数。

`configs: &ConfigResult`

完整的 [ConfigResult](../config.rs/ConfigResult.md)。用于从 `configs.thread_level_configs` 中按 `grade` 和 `name` 查找匹配的 [ThreadLevelConfig](../config.rs/ThreadLevelConfig.md)。

`prime_core_scheduler: &mut PrimeThreadScheduler`

[PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) 实例，传递给 [apply_thread_level](apply_thread_level.md) 用于 Prime 线程跟踪和调度。

`process_level_applied: &mut SmallVec<[u32; PIDS]>`

已应用进程级设置的 PID 运行列表。在 [apply_process_level](apply_process_level.md) 完成后，当前 `pid` 会被无条件追加。在后续迭代中（除非设置了 `-continuous_process_level_apply`），此列表中的 PID 将跳过进程级工作。

`thread_level_applied: &mut SmallVec<[u32; PENDING]>`

在*当前*迭代中已应用线程级设置的 PID 运行列表。仅当存在 [ThreadLevelConfig](../config.rs/ThreadLevelConfig.md) 且调用了 [apply_thread_level](apply_thread_level.md) 时，才会追加当前 `pid`。这可以防止同一进程在单次循环迭代中被处理两次（否则会破坏调度器基于增量的周期时间跟踪）。

`grade: &u32`

找到匹配规则所在的配置等级（轮询频率乘数）。用于在 `configs.thread_level_configs` 中查找对应的线程级配置。

`pid: &u32`

目标进程的进程标识符。

`name: &&str`

目标进程的小写可执行文件名（例如 `"chrome.exe"`）。用作查找线程级配置的键。

`process_level_config: &ProcessLevelConfig`

匹配此进程的 [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md)。转发给 [apply_process_level](apply_process_level.md)。

`process: &ProcessEntry`

来自当前快照的 [ProcessEntry](../process.rs/ProcessEntry.md)，用于通过 `process.get_threads()` 枚举线程。

## 返回值

此函数不返回值。所有结果通过 `process_level_applied`、`thread_level_applied` 以及 [log_apply_results](log_apply_results.md) 生成的日志输出进行传递。

## 备注

### 线程缓存

该函数创建一个 `OnceCell<HashMap<u32, SYSTEM_THREAD_INFORMATION>>`，延迟求值 `process.get_threads()`。此 cell 通过闭包引用在 [apply_process_level](apply_process_level.md) 和 [apply_thread_level](apply_thread_level.md) 之间共享，确保涉及 `NtQuerySystemInformation` 的线程枚举在每次调用中最多只发生一次，无论有多少 apply 函数需要线程数据。

### 两级应用流程

1. 创建一个新的 [ApplyConfigResult](../apply.rs/ApplyConfigResult.md)。
2. 无条件调用 [apply_process_level](apply_process_level.md)，传入进程级配置。
3. 然后在 `configs.thread_level_configs[grade][name]` 中查找线程级配置。如果存在，则调用 [apply_thread_level](apply_thread_level.md) 并将 PID 添加到 `thread_level_applied`。
4. PID 始终会被添加到 `process_level_applied`。
5. 使用累积的结果调用 [log_apply_results](log_apply_results.md)。

### 等级不变量

该函数按约定断言用于查找进程级配置的等级与用于线程级配置查找的等级相同。这由主循环中的调用方保证，主循环按等级迭代配置。

## 要求

| | |
|---|---|
| **模块** | `src/main.rs` |
| **调用方** | [main](main.md) 循环 — ETW 挂起路径和完整匹配路径 |
| **被调用方** | [apply_process_level](apply_process_level.md)、[apply_thread_level](apply_thread_level.md)、[log_apply_results](log_apply_results.md)、`ProcessEntry::get_threads` |
| **Win32 API** | 无（委托给被调用方） |
| **权限** | 无（委托给被调用方） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 进程级应用包装器 | [apply_process_level](apply_process_level.md) |
| 线程级应用包装器 | [apply_thread_level](apply_thread_level.md) |
| 结果日志记录 | [log_apply_results](log_apply_results.md) |
| 结果累加器 | [ApplyConfigResult](../apply.rs/ApplyConfigResult.md) |
| 配置类型 | [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md)、[ThreadLevelConfig](../config.rs/ThreadLevelConfig.md) |
| 模块概述 | [main.rs](README.md) |

*为提交 [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf) 记录*