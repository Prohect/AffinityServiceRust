# apply_config 函数 (main.rs)

协调对单个已匹配进程的进程级和线程级配置的完整应用。该函数一次性获取进程的线程映射，应用进程级设置，查找并应用任何对应的线程级设置，记录已应用的 PID，并将日志输出委托给 `log_apply_results`。

## 语法

```rust
fn apply_config(
    cli: &CliArgs,
    configs: &ConfigResult,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    process_level_applied: &mut smallvec::SmallVec<[u32; PIDS]>,
    thread_level_applied: &mut smallvec::SmallVec<[u32; PENDING]>,
    grade: &u32,
    pid: &u32,
    name: &&str,
    process_level_config: &ProcessLevelConfig,
    process: &ProcessEntry,
)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `cli` | `&CliArgs` | 已解析的命令行参数。`dry_run` 字段控制是否实际调用 Win32 API。 |
| `configs` | `&ConfigResult` | 完整加载的配置，包括按等级和进程名称索引的 `process_level_configs` 和 `thread_level_configs`。 |
| `prime_core_scheduler` | `&mut PrimeThreadScheduler` | 线程调度器实例，在存在线程级规则时用于主线程（prime thread）跟踪。 |
| `process_level_applied` | `&mut smallvec::SmallVec<[u32; PIDS]>` | 已应用进程级设置的 PID 累积列表。函数返回时，当前 PID 会被推入此列表。 |
| `thread_level_applied` | `&mut smallvec::SmallVec<[u32; PENDING]>` | 本次迭代中已应用线程级设置的 PID 累积列表。用于防止在循环后续阶段重复应用线程级设置。 |
| `grade` | `&u32` | 配置等级（轮询间隔倍数）。用于查找同一等级下匹配的线程级配置。 |
| `pid` | `&u32` | 目标进程的 Windows 进程 ID。 |
| `name` | `&&str` | 小写的可执行文件名（例如 `"game.exe"`），用作配置查找键。 |
| `process_level_config` | `&ProcessLevelConfig` | 已为此进程解析好的进程级配置条目。 |
| `process` | `&ProcessEntry` | 进程快照条目的引用，通过 `get_threads()` 获取线程映射。 |

## 返回值

此函数不返回值。

## 备注

- 该函数调用 `process.get_threads()` 一次，并将生成的 `HashMap<u32, SYSTEM_THREAD_INFORMATION>` 同时用于 `apply_process_level` 和 `apply_thread_level`，避免重复的线程枚举。
- 函数内部创建一个 `ApplyConfigResult` 来收集两个级别产生的变更和错误。两个级别都应用完成后，合并的结果将被转发给 `log_apply_results`。
- 线程级配置通过相同的 `grade` 和 `name` 在 `configs.thread_level_configs` 中查找。如果不存在该进程的线程级条目，则仅应用进程级设置。
- 源代码中记录的契约（`assert(grade for process_level_config == grade for thread_level_config)`）意味着调用方必须确保用于查找进程级配置的等级与用于查找线程级配置的等级相同。
- 无论是否实际进行了任何变更，PID 都会被无条件推入 `process_level_applied`。这可以防止在后续循环迭代中重复应用（除非在主循环中设置了 `cli.continuous_process_level_apply`）。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `main.rs` |
| 调用方 | [main](main.md)（主轮询循环，包括等级迭代和 ETW 待处理路径） |
| 被调用方 | [apply_process_level](apply_process_level.md)、[apply_thread_level](apply_thread_level.md)、[log_apply_results](log_apply_results.md) |
| API | `ProcessEntry::get_threads`（process 模块） |
| 权限 | 继承 `apply_process_level` 和 `apply_thread_level` 的权限要求 |

## 另请参阅

| 资源 | 链接 |
|------|------|
| apply_process_level | [apply_process_level](apply_process_level.md) |
| apply_thread_level | [apply_thread_level](apply_thread_level.md) |
| log_apply_results | [log_apply_results](log_apply_results.md) |
| main | [main](main.md) |
| PrimeThreadScheduler | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |

---
*提交：[37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
