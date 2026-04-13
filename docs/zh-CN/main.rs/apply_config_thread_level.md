# apply_config_thread_level 函数 (main.rs)

每次轮询迭代应用线程级设置到目标进程。包括 Prime 线程调度、理想处理器分配和周期时间跟踪。

## 语法

```rust
fn apply_config_thread_level(
    pid: u32,
    config: &ProcessConfig,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    process: &mut ProcessEntry,
    dry_run: bool,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

`pid` — 目标进程 ID。

`config` — 指向 [`ProcessConfig`](../config.rs/ProcessConfig.md) 的引用。

`prime_core_scheduler` — 指向 [`PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md) 的可变引用。

`process` — 指向 [`ProcessEntry`](../process.rs/ProcessEntry.md) 的可变引用。

`dry_run` — 为 `true` 时仅记录更改，不实际应用。

`apply_config_result` — 指向 [`ApplyConfigResult`](../apply.rs/ApplyConfigResult.md) 的可变引用。

## 备注

仅在配置中包含 prime 线程、理想处理器或线程跟踪设置时执行。步骤：

1. 通过 `GetProcessAffinityMask` 查询当前亲和性掩码，用于 prime 线程 CPU 过滤
2. 通过 [`drop_module_cache`](../winapi.rs/drop_module_cache.md) 清除模块缓存
3. 在调度器中标记进程存活
4. [`prefetch_all_thread_cycles`](../apply.rs/prefetch_all_thread_cycles.md) — 采集周期基线
5. [`apply_prime_threads`](../apply.rs/apply_prime_threads.md) — 选择、提升和降级 prime 线程
6. [`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md) — 分配理想处理器
7. [`update_thread_stats`](../apply.rs/update_thread_stats.md) — 持久化缓存数据

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/main.rs |
| **源码行** | L76–L115 |
| **调用方** | [`main`](main.md) 主循环 |

## 另请参阅

- [apply_config_process_level](apply_config_process_level.md)
- [scheduler.rs 模块概述](../scheduler.rs/README.md)
- [main.rs 模块概述](README.md)