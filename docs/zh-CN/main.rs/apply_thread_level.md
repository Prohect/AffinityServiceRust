# apply_thread_level 函数 (main.rs)

在每个轮询周期中对单个进程应用线程级设置。包括预取线程周期时间以进行增量计算、运行 Prime 线程调度算法以及分配理想处理器提示。此函数仅在进程的 [ThreadLevelConfig](../config.rs/ThreadLevelConfig.md) 包含至少一个线程级设置（Prime 线程 CPU、Prime 线程前缀、理想处理器规则或 top-X 线程跟踪）时才会执行。

## 语法

```rust
fn apply_thread_level<'a>(
    pid: u32,
    config: &ThreadLevelConfig,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    process: &'a ProcessEntry,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    dry_run: bool,
    apply_configs: &mut ApplyConfigResult,
)
```

## 参数

`pid: u32`

目标进程的进程标识符。

`config: &ThreadLevelConfig`

匹配进程的 [`ThreadLevelConfig`](../config.rs/ThreadLevelConfig.md)。包含 Prime 线程 CPU 分配、模块前缀匹配规则、理想处理器规则以及 top-X 线程跟踪计数。如果所有这些字段都为空/零，函数将立即返回。

`prime_core_scheduler: &mut PrimeThreadScheduler`

[`PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md)，用于跨迭代跟踪每线程的周期时间增量、活跃连续计数以及 prime/non-prime 状态。调度器会为此 PID 标记为活跃，并使用当前周期数据进行更新。

`process: &'a ProcessEntry`

目标进程的 [`ProcessEntry`](../process.rs/ProcessEntry.md)，当线程缓存尚未填充时用于枚举线程。

`threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>`

延迟求值的闭包，返回进程的线程映射。由 `OnceCell` 支撑，因此线程枚举在每个应用周期最多发生一次，通过 [`apply_config`](apply_config.md) 调用时与进程级应用路径共享。

`dry_run: bool`

当为 **true** 时，所有子函数仅记录*将要*进行的更改而不调用 Windows API。当为 **false** 时，更改将应用到实际线程。

`apply_configs: &mut ApplyConfigResult`

更改描述和错误消息的累加器。参见 [`ApplyConfigResult`](../apply.rs/ApplyConfigResult.md)。

## 返回值

此函数不返回值。结果通过 `apply_configs` 传达。

## 备注

此函数按以下顺序执行操作：

1. **守卫检查** — 如果 `prime_threads_cpus`、`prime_threads_prefixes`、`ideal_processor_rules` 均为空且 `track_top_x_threads` 为零，则立即返回。
2. **查询亲和性掩码** — 如果 `prime_threads_cpus` 非空，则打开进程句柄并调用 `GetProcessAffinityMask` 以确定进程允许使用哪些 CPU。此掩码约束 Prime 线程调度器可以分配的核心范围。
3. **丢弃模块缓存** — 调用 [`drop_module_cache`](../winapi.rs/drop_module_cache.md) 以刷新该 PID 的线程到模块的查找。
4. **标记活跃** — 调用 `prime_core_scheduler.set_alive(pid)`，使调度器知道此进程仍在运行（死亡进程将在主循环中稍后被清理）。
5. **预取周期时间** — 调用 [`prefetch_all_thread_cycles`](../apply.rs/prefetch_all_thread_cycles.md) 查询当前线程周期计数，并计算与上一次迭代的增量，为调度器的排名算法提供数据。
6. **应用 Prime 线程** — 调用 [`apply_prime_threads`](../apply.rs/apply_prime_threads.md) 根据周期时间排名和迟滞阈值选择、提升和降级线程。
7. **应用理想处理器** — 调用 [`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md) 为匹配模块前缀规则的线程分配理想处理器提示。
8. **更新统计信息** — 调用 [`update_thread_stats`](../apply.rs/update_thread_stats.md) 将当前周期/时间测量值缓存为下一次迭代的基线值。

### 与 apply_process_level 的区别

[`apply_process_level`](apply_process_level.md) 对每个进程运行一次（或在未设置 `continuous_process_level_apply` 时每次配置重载运行一次），设置进程级属性。`apply_thread_level` 在**每个**轮询周期中运行，因为线程周期排名会持续变化，Prime 线程选择必须重新评估。

### 线程缓存共享

通过 [`apply_config`](apply_config.md) 调用时，`threads` 闭包与进程级路径共享同一个 `OnceCell`，避免了对线程枚举的冗余 `NtQuerySystemInformation` 调用。

## 要求

| | |
|---|---|
| **模块** | `src/main.rs` |
| **调用方** | [`apply_config`](apply_config.md)、主循环线程级路径 |
| **被调用方** | [`get_process_handle`](../winapi.rs/get_process_handle.md)、[`drop_module_cache`](../winapi.rs/drop_module_cache.md)、[`prefetch_all_thread_cycles`](../apply.rs/prefetch_all_thread_cycles.md)、[`apply_prime_threads`](../apply.rs/apply_prime_threads.md)、[`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md)、[`update_thread_stats`](../apply.rs/update_thread_stats.md) |
| **Win32 API** | [`GetProcessAffinityMask`](https://learn.microsoft.com/zh-cn/windows/win32/api/winbase/nf-winbase-getprocessaffinitymask) |
| **权限** | `PROCESS_QUERY_LIMITED_INFORMATION`（亲和性查询），线程级访问权限委托给被调用方 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 进程级对应函数 | [apply_process_level](apply_process_level.md) |
| 组合协调器 | [apply_config](apply_config.md) |
| 线程级配置类型 | [ThreadLevelConfig](../config.rs/ThreadLevelConfig.md) |
| Prime 线程调度器 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| 应用引擎概述 | [apply.rs](../apply.rs/README.md) |
| 结果累加器 | [ApplyConfigResult](../apply.rs/ApplyConfigResult.md) |

*为提交 [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf) 记录*