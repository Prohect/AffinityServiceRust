# apply_thread_level 函数 (main.rs)

在每次轮询迭代中对单个进程应用线程级设置。包括主线程调度（选择活动最高的线程并将其固定到首选 CPU 核心）、理想处理器分配以及每线程 CPU 周期时间跟踪。该函数在多次迭代中反复调用，以便调度器能够随时间对工作负载变化做出响应。

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

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 目标进程的 Windows 进程标识符。 |
| `config` | `&ThreadLevelConfig` | 该进程的线程级配置块，包含主线程 CPU 列表、线程名称前缀过滤器、理想处理器规则以及 `track_top_x_threads` 调试设置。 |
| `prime_core_scheduler` | `&mut PrimeThreadScheduler` | 调度器的可变引用，用于跟踪每线程的周期统计信息，并在跨迭代中管理基于滞后的主线程选择。 |
| `process` | `&'a ProcessEntry` | 进程快照条目的共享引用（具有生命周期 `'a`），在应用主线程规则时用于解析线程信息。 |
| `threads` | `&impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>` | 返回线程映射引用的惰性闭包。该闭包由 `OnceCell` 支持，因此实际的 `get_threads()` 调用会延迟到首次使用时执行并缓存结果。这使得通过 `apply_config` 一起调用时，`apply_process_level` 和 `apply_thread_level` 可以共享单次线程枚举。 |
| `dry_run` | `bool` | 为 `true` 时，不会发出修改线程状态的 Win32 API 调用；变更仅记录在 `apply_configs` 中用于日志输出。 |
| `apply_configs` | `&mut ApplyConfigResult` | 应用操作期间产生的变更描述和错误消息的累加器。 |

## 返回值

此函数不返回值。所有结果（成功和错误）均记录在 `apply_configs` 累加器中。

## 备注

如果以下线程级功能均未为该进程配置，函数将立即短路返回：

- `prime_threads_cpus` — 用于固定主线程的 CPU 核心。
- `prime_threads_prefixes` — 用于过滤候选线程的线程起始地址模块前缀。
- `ideal_processor_rules` — 设置线程理想处理器的规则。
- `track_top_x_threads` — 非零值启用进程退出时顶部线程的诊断日志。

当线程级功能处于活动状态时，函数按以下顺序执行步骤：

1. **亲和性掩码查询** — 如果 `prime_threads_cpus` 非空，则通过 `GetProcessAffinityMask` 获取当前进程亲和性掩码，以便主线程 CPU 过滤尊重进程级掩码。与先前实现不同，此条件中不再检查 `affinity_cpus` 字段，因为亲和性现在是由 `apply_process_level` 独占处理的进程级关注点。
2. **模块缓存重置** — 调用 `drop_module_cache` 以确保线程起始地址解析使用最新数据。
3. **调度器存活标记** — `prime_core_scheduler.set_alive(pid)` 将进程在调度器中标记为存活，以便死进程清理跳过它。
4. **周期预取** — `prefetch_all_thread_cycles` 查询每线程周期时间并填充调度器的 `ThreadStats` 条目。`threads` 闭包被传递进去，以便线程枚举延迟到实际需要时执行。
5. **主线程应用** — `apply_prime_threads` 使用基于滞后的选择来选择顶部线程并将其固定到配置的 CPU 集合。`process` 和 `threads` 闭包都传递给此函数。
6. **理想处理器分配** — `apply_ideal_processors` 为匹配配置规则的线程设置理想处理器。此函数直接接收 `threads` 闭包（而非 `process` 引用）。
7. **统计更新** — `update_thread_stats` 缓存当前周期计数，以便下一次迭代可以计算增量。

由于此函数在每次轮询迭代中都被调用（而非每个进程仅调用一次），调度器会累积多次迭代的历史记录，用于滞后算法，防止线程提升/降级抖动。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `main.rs` |
| 调用者 | [apply_config](apply_config.md)，[main](main.md) 中的独立线程级应用循环 |
| 被调用者 | `apply::prefetch_all_thread_cycles`、`apply::apply_prime_threads`、`apply::apply_ideal_processors`、`apply::update_thread_stats`、`winapi::get_process_handle`、`winapi::drop_module_cache`、[PrimeThreadScheduler::set_alive](../scheduler.rs/PrimeThreadScheduler.md) |
| Win32 API | `GetProcessAffinityMask` |
| 权限 | `SeDebugPrivilege`（用于打开其他会话的线程/进程句柄） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| apply_process_level | [apply_process_level](apply_process_level.md) |
| apply_config | [apply_config](apply_config.md) |
| PrimeThreadScheduler | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| ThreadLevelConfig | [config 模块](../config.rs/README.md) |
| main | [main](main.md) |

---
*提交：[29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
