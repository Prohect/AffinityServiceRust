# apply_config_thread_level 函数 (main.rs)

在每次轮询迭代中为受管理进程应用线程级设置。与一次性的 [apply_config_process_level](apply_config_process_level.md) 不同，此函数会被反复调用，以重新评估主线程调度、理想处理器分配和每线程周期时间跟踪。它是服务动态线程管理能力的核心。

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

`pid`

目标进程的进程标识符 (PID)。

`config`

对 [ProcessConfig](../config.rs/ProcessConfig.md) 的引用，包含此进程的线程级规则。相关字段为 `prime_threads_cpus`、`prime_threads_prefixes`、`ideal_processor_rules`、`affinity_cpus` 和 `track_top_x_threads`。

`prime_core_scheduler`

对 [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) 的可变引用，该调度器维护每进程、每线程的统计信息（周期计数、活跃连续次数、升级/降级状态）。此调度器在所有受管理进程间共享，并在多次轮询迭代间累积数据。

`process`

对目标进程 [ProcessEntry](../process.rs/ProcessEntry.md) 的可变引用。从此条目中读取上次快照的线程枚举数据，并更新缓存状态（如固定的 CPU 集 ID 和理想处理器分配）。

`dry_run`

当为 `true` 时，函数模拟变更并将预期操作记录在 `apply_config_result` 中，而不进行 Win32 API 调用。当为 `false` 时，线程级设置将应用到实际进程。

`apply_config_result`

对 [ApplyConfigResult](../apply.rs/ApplyConfigResult.md) 累加器的可变引用。子函数将其变更和错误追加到此结构中，供调用方记录日志。

## 返回值

此函数不返回值。

## 备注

如果没有任何线程级配置字段处于活跃状态，则此函数为空操作。具体来说，除非以下条件中至少有一个为真，否则函数会立即返回：

- `config.prime_threads_cpus` 非空（已配置主线程 CPU 绑定）。
- `config.prime_threads_prefixes` 非空（存在基于模块前缀的主线程规则）。
- `config.ideal_processor_rules` 非空（已定义理想处理器分配规则）。
- `config.track_top_x_threads` 非零（已启用 Top-N 线程跟踪）。

当函数继续执行时，它按以下顺序执行步骤：

1. **查询当前亲和性掩码** — 如果配置了主线程 CPU 或亲和性 CPU，函数会打开进程句柄并调用 `GetProcessAffinityMask` 获取当前掩码。此掩码用于过滤哪些 CPU 是主线程绑定的有效目标。

2. **丢弃模块缓存** — 调用 [drop_module_cache](../winapi.rs/drop_module_cache.md) 清除该 PID 的缓存，强制刷新用于基于前缀的线程识别的模块到地址映射。

3. **标记进程存活** — 调用 `prime_core_scheduler.set_alive(pid)` 以表明此进程在当前轮询迭代中被发现。未标记为存活的进程是清理候选对象。

4. **预取线程周期时间** — 调用 [prefetch_all_thread_cycles](../apply.rs/prefetch_all_thread_cycles.md) 为进程中的每个线程查询 `QueryThreadCycleTime` 并将结果存储在调度器中。迭代间的周期计数差值用于识别最活跃的（主）线程。

5. **应用主线程** — 调用 [apply_prime_threads](../apply.rs/apply_prime_threads.md) 将最高活跃度的线程提升到首选 CPU 核心（通过 `SetThreadSelectedCpuSets`），并可选地设置其线程优先级。活跃度低于阈值的线程会被降级回默认调度。

6. **应用理想处理器** — 调用 [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) 根据前缀匹配规则将线程分配到特定逻辑处理器（例如，起始地址解析为 `render.dll` 的线程通过 `SetThreadIdealProcessorEx` 被绑定到特定核心）。

7. **更新线程统计** — 调用 [update_thread_stats](../apply.rs/update_thread_stats.md) 将当前迭代的周期计数提交为下次迭代差值计算的基线。

### 与进程级设置的交互

线程级设置建立在进程级设置之上。例如，如果 `apply_config_process_level` 设置了 CPU 亲和性掩码将进程限制在核心 0–7，则主线程调度只会考虑该掩码内的核心。`current_mask` 变量桥接了这一关系。

### 基于等级的调度

此函数遵循主循环中的基于等级的调度系统。一个 `grade=5` 的进程规则只在每第 5 次轮询迭代时评估其线程级设置，从而减少不需要频繁线程重平衡的进程的开销。

### ETW 集成

当 ETW 进程监视器处于活跃状态时，已终止的进程通过进程退出事件被反应式清理，而非在每次轮询循环结束时清理。这意味着已退出进程的 `prime_core_scheduler` 状态会被及时移除，释放资源。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `main` |
| 调用方 | [main](main.md)（轮询循环） |
| 被调用方 | [get_process_handle](../winapi.rs/get_process_handle.md)、[drop_module_cache](../winapi.rs/drop_module_cache.md)、[prefetch_all_thread_cycles](../apply.rs/prefetch_all_thread_cycles.md)、[apply_prime_threads](../apply.rs/apply_prime_threads.md)、[apply_ideal_processors](../apply.rs/apply_ideal_processors.md)、[update_thread_stats](../apply.rs/update_thread_stats.md) |
| API | `GetProcessAffinityMask`、`QueryThreadCycleTime`、`SetThreadSelectedCpuSets`、`SetThreadIdealProcessorEx`、`SetThreadPriority` |
| 权限 | `SeDebugPrivilege`（建议，用于打开受保护进程的线程句柄） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 进程级设置（一次性） | [apply_config_process_level](apply_config_process_level.md) |
| 主线程调度器与滞后机制 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| 进程配置结构体 | [ProcessConfig](../config.rs/ProcessConfig.md) |
| 主入口点和轮询循环 | [main](main.md) |