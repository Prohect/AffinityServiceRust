# apply_config 函数 (main.rs)

编排对单个匹配进程的进程级和线程级配置的完整应用。它一次性获取进程的线程映射，应用进程级设置，查找并应用对应的线程级设置，记录已应用的 PID，并将日志记录委托给 `log_apply_results`。

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
| `cli` | `&CliArgs` | 已解析的命令行参数。`dry_run` 字段控制是否实际发出 Win32 API 调用。 |
| `configs` | `&ConfigResult` | 完整的已加载配置，包括按等级和进程名索引的 `process_level_configs` 和 `thread_level_configs`。 |
| `prime_core_scheduler` | `&mut PrimeThreadScheduler` | 线程调度器实例，在存在线程级规则时用于主线程跟踪。 |
| `process_level_applied` | `&mut smallvec::SmallVec<[u32; PIDS]>` | 已应用进程级设置的 PID 累加器。函数返回时，当前 PID 会被追加到此列表中。 |
| `thread_level_applied` | `&mut smallvec::SmallVec<[u32; PENDING]>` | 在本次迭代中已应用线程级设置的 PID 累加器。防止后续循环中重复应用线程级配置。 |
| `grade` | `&u32` | 配置等级（轮询间隔倍数）。用于查找相同等级下的匹配线程级配置。 |
| `pid` | `&u32` | 目标进程的 Windows 进程 ID。 |
| `name` | `&&str` | 小写的可执行文件名（例如 `"game.exe"`），用作配置查找键。 |
| `process_level_config` | `&ProcessLevelConfig` | 该进程已解析的进程级配置条目。 |
| `process` | `&ProcessEntry` | 进程快照条目的引用，通过 `get_threads()` 从中获取线程映射。 |

## 返回值

此函数没有返回值。

## 备注

- 该函数调用 `process.get_threads()` 一次，并将返回的 `HashMap<u32, SYSTEM_THREAD_INFORMATION>` 同时用于 `apply_process_level` 和 `apply_thread_level`，避免重复的线程枚举。
- 内部创建一个 `ApplyConfigResult` 来收集两个级别的变更和错误。两个级别都应用完成后，将合并的结果转发给 `log_apply_results`。
- 线程级配置通过相同的 `grade` 和 `name` 在 `configs.thread_level_configs` 中查找。如果该进程不存在线程级条目，则仅应用进程级设置。
- 源代码中记录的契约（`assert(grade for process_level_config == grade for thread_level_config)`）意味着调用方必须确保用于查找进程级配置的 grade 与用于查找线程级配置的 grade 相同。
- 无论是否实际进行了任何更改，PID 都会无条件地追加到 `process_level_applied` 中。这可以防止在后续循环迭代中重复应用（除非在主循环中设置了 `cli.continuous_process_level_apply`）。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `main.rs` |
| 调用方 | [main](main.md)（主轮询循环，包括等级迭代和 ETW 待处理路径） |
| 被调用方 | [apply_process_level](apply_process_level.md)、[apply_thread_level](apply_thread_level.md)、[log_apply_results](log_apply_results.md) |
| API | `ProcessEntry::get_threads`（process 模块） |
| 权限 | 继承 `apply_process_level` 和 `apply_thread_level` 的权限要求 |

## 另请参阅

| 参考 | 链接 |
|------|------|
| apply_process_level | [apply_process_level](apply_process_level.md) |
| apply_thread_level | [apply_thread_level](apply_thread_level.md) |
| log_apply_results | [log_apply_results](log_apply_results.md) |
| main | [main](main.md) |
| PrimeThreadScheduler | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |

---
> Commit SHA: `7221ea0694670265d4eb4975582d8ed2ae02439d`
