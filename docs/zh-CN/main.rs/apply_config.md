# apply_config 函数 (main.rs)

编排所有配置设置到目标进程的应用。这是主循环中为每个匹配进程调用的顶层函数，按固定顺序依次调用各 `apply_*` 子函数。

## 语法

```rust
fn apply_config(
    pid: u32,
    config: &ProcessConfig,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    process: &mut ProcessEntry,
    dry_run: bool,
) -> ApplyConfigResult
```

## 参数

`pid`

目标进程的进程标识符。

`config`

指向 [ProcessConfig](../config.rs/ProcessConfig.md) 的引用，包含要应用的所有设置（优先级、亲和性、CPU 集、I/O 优先级、内存优先级、prime 线程规则、理想处理器规则等）。

`prime_core_scheduler`

指向 [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) 的可变引用，用于管理 prime 线程的调度状态、线程统计和核心分配。

`process`

指向 [ProcessEntry](../process.rs/ProcessEntry.md) 的可变引用，提供当前进程快照中的线程枚举信息。

`dry_run`

当为 `true` 时，所有子函数仅记录将要执行的更改，而不实际调用 Windows API。

## 返回值

返回 [ApplyConfigResult](../apply.rs/ApplyConfigResult.md)，包含所有更改描述（`changes`）和错误消息（`errors`）。调用者根据 `is_empty()` 决定是否记录日志。

## 备注

### 执行顺序

函数按以下固定顺序应用配置：

1. **获取进程句柄** — 调用 [get_process_handle](../winapi.rs/get_process_handle.md) 获取目标进程的读写句柄。若无法获取，函数立即返回空的 `ApplyConfigResult`，不执行任何后续操作。
2. **优先级** — [apply_priority](../apply.rs/apply_priority.md) 设置进程优先级类。
3. **亲和性** — [apply_affinity](../apply.rs/apply_affinity.md) 设置硬 CPU 亲和性掩码，并更新 `current_mask`。
4. **CPU 集** — [apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md) 设置软 CPU 偏好。
5. **I/O 优先级** — [apply_io_priority](../apply.rs/apply_io_priority.md) 设置 I/O 优先级。
6. **内存优先级** — [apply_memory_priority](../apply.rs/apply_memory_priority.md) 设置内存页面优先级。

### Prime 调度阶段

当配置中存在 `prime_threads_cpus`、`prime_threads_prefixes`、`ideal_processor_rules` 或 `track_top_x_threads != 0` 时，进入 prime 调度阶段：

7. **释放模块缓存** — [drop_module_cache](../winapi.rs/drop_module_cache.md) 清除进程的模块地址缓存，确保下次解析使用最新数据。
8. **设置存活** — `prime_core_scheduler.set_alive(pid)` 标记进程仍在运行。
9. **预取周期** — [prefetch_all_thread_cycles](../apply.rs/prefetch_all_thread_cycles.md) 采集线程周期基线数据。
10. **应用 prime** — [apply_prime_threads](../apply.rs/apply_prime_threads.md) 选择、提升和降级 prime 线程。
11. **应用理想处理器** — [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) 根据模块前缀规则分配理想处理器。
12. **更新统计** — [update_thread_stats](../apply.rs/update_thread_stats.md) 持久化缓存的周期和时间数据，供下次迭代使用。

### 句柄生命周期

`process_handle` 在所有操作完成后通过 `drop` 显式释放。`current_mask` 作为局部变量在亲和性设置和 prime 线程调度之间共享，用于追踪当前生效的亲和性掩码。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/main.rs |
| **源码行** | L46–L104 |
| **调用者** | [main](main.md) 主循环 |
| **调用** | [get_process_handle](../winapi.rs/get_process_handle.md)、[apply_priority](../apply.rs/apply_priority.md)、[apply_affinity](../apply.rs/apply_affinity.md)、[apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md)、[apply_io_priority](../apply.rs/apply_io_priority.md)、[apply_memory_priority](../apply.rs/apply_memory_priority.md)、[drop_module_cache](../winapi.rs/drop_module_cache.md)、[prefetch_all_thread_cycles](../apply.rs/prefetch_all_thread_cycles.md)、[apply_prime_threads](../apply.rs/apply_prime_threads.md)、[apply_ideal_processors](../apply.rs/apply_ideal_processors.md)、[update_thread_stats](../apply.rs/update_thread_stats.md) |

## 另请参阅

- [main.rs 模块概述](README.md)
- [main 函数](main.md) — 主循环中调用此函数
- [ApplyConfigResult](../apply.rs/ApplyConfigResult.md) — 返回值类型
- [apply.rs 模块概述](../apply.rs/README.md) — 各 `apply_*` 子函数的详细文档