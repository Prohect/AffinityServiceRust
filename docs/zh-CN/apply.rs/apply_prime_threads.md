# apply_prime_threads 函数 (apply.rs)

主力线程调度管线的顶层编排器。此函数识别进程中 CPU 使用最密集的线程，使用基于滞后阈值的选择算法选出一部分线程进行提升，通过每线程 CPU 集合将被提升的线程固定到专用的高性能核心 CPU 上，并降级不再符合条件的线程。实际的选择、提升和降级步骤分别委托给 [apply_prime_threads_select](apply_prime_threads_select.md)、[apply_prime_threads_promote](apply_prime_threads_promote.md) 和 [apply_prime_threads_demote](apply_prime_threads_demote.md)。

## 语法

```AffinityServiceRust/src/apply.rs#L713-800
pub fn apply_prime_threads(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    current_mask: &mut usize,
    process: &mut ProcessEntry,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 目标进程的进程标识符。用作 [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) 状态映射的键，同时用于日志记录。 |
| `config` | `&`[ProcessConfig](../config.rs/ProcessConfig.md) | 此进程的已解析配置规则。字段 `prime_threads_cpus`、`prime_threads_prefixes` 和 `track_top_x_threads` 控制主力线程调度是否激活以及跟踪多少个线程。 |
| `dry_run` | `bool` | 当为 `true` 时，函数将已配置的主力 CPU 记录为变更消息并返回，不修改任何操作系统状态。 |
| `current_mask` | `&mut usize` | 进程的当前 CPU 亲和性掩码，之前由 [apply_affinity](apply_affinity.md) 填充。传递给 [apply_prime_threads_promote](apply_prime_threads_promote.md)，以便主力 CPU 索引可以根据有效亲和性进行过滤。 |
| `process` | `&mut`[ProcessEntry](../process.rs/ProcessEntry.md) | 目标进程的快照条目。提供线程列表以及来自最近一次 `NtQuerySystemInformation` 快照的每线程内核/用户时间。传递给 [apply_prime_threads_demote](apply_prime_threads_demote.md) 用于活动线程枚举。 |
| `prime_core_scheduler` | `&mut`[PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) | 持久化状态，跨轮询周期跟踪每线程的周期计数、活跃连续次数、固定的 CPU 集合 ID、原始优先级和已打开的线程句柄。 |
| `apply_config_result` | `&mut`[ApplyConfigResult](ApplyConfigResult.md) | 变更描述和错误消息的累加器。由本函数及三个委托函数填充。 |

## 返回值

无（`()`）。所有结果通过 `apply_config_result` 以及对 `prime_core_scheduler`、`process` 和 `current_mask` 的副作用进行传递。

## 备注

### 激活条件

函数仅在以下至少一个条件为真时执行工作：

- `config.prime_threads_cpus` 非空，或 `config.prime_threads_prefixes` 非空（存在可固定的主力 CPU）。
- `config.track_top_x_threads` 非零（线程跟踪已启用，用于调度器的诊断日志输出）。

`track_top_x_threads` 为负值时会禁用主力线程提升/降级管线，但仍允许线程跟踪。当两个条件都不满足时，函数立即返回。

### 算法概览

```/dev/null/pipeline.txt#L1-5
 ┌────────────────┐    ┌──────────────┐    ┌─────────────┐    ┌──────────────┐
 │ 收集并排序     │───>│    选择      │───>│    提升      │───>│    降级      │
 │  候选线程      │    │  (滞后算法)  │    │ (CPU 固定 + │    │ (取消固定 + │
 │                │    │              │    │  优先级提升) │    │  优先级恢复) │
 └────────────────┘    └──────────────┘    └─────────────┘    └──────────────┘
```

1. **收集线程时间增量** — 遍历进程快照中的所有线程。对于每个线程，计算存储在 [ThreadStats](../scheduler.rs/ThreadStats.md) 中的缓存总时间（`KernelTime + UserTime`）与上一周期 `last_total_time` 之间的增量。如果 `track_top_x_threads` 非零，当前的 `SYSTEM_THREAD_INFORMATION` 也会保存到 `last_system_thread_info` 中，供调度器的诊断输出使用。

2. **排序并选择候选者** — 线程按时间增量降序排列。候选池大小计算为 `max(prime_count × 4, cpu_count)`，并以总线程数为上限。乘以 4 的系数确保候选池足够大，使滞后算法能够考虑到 CPU 使用率正在上升但尚未达到主力状态的线程。之前被固定但掉出前几名候选者的线程会被追加到池中，以便在不再符合条件时能够被正确降级。

3. **构建周期增量元组** — 对于每个候选线程，构建一个 `(tid, delta_cycles, is_prime)` 元组。`delta_cycles` 是 `cached_cycles`（由 [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) 设置）与上一轮 `last_cycles` 之间的差值。`is_prime` 标志初始对所有条目为 `false`。

4. **选择** — [apply_prime_threads_select](apply_prime_threads_select.md) 使用 [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) 的滞后算法将前 *N* 个条目标记为主力线程，其中 *N* 等于已配置的主力 CPU 数量。

5. **提升** — [apply_prime_threads_promote](apply_prime_threads_promote.md) 将每个新提升的线程固定到其分配的主力 CPU 集合，并可选地提升其线程优先级。

6. **降级** — [apply_prime_threads_demote](apply_prime_threads_demote.md) 取消失去主力状态的线程的固定，并恢复其原始线程优先级。

7. **句柄清理** — 管线完成后，函数遍历 [ThreadStats](../scheduler.rs/ThreadStats.md) 条目，移除不再出现在活动线程集中的线程的句柄。[ThreadHandle](../winapi.rs/ThreadHandle.md) 的 `Drop` 实现在从 `Option` 中取出时会自动关闭操作系统句柄。

### 候选池大小调整

候选池公式 `max(prime_count × 4, cpu_count)` 是一种调优启发式方法：

- `prime_count × 4` 项确保评估的线程数至少是主力槽位的四倍，为滞后算法提供了跟踪上升中线程的空间。
- `cpu_count` 下限确保在 CPU 数量多但主力槽位少的系统上，仍有足够的线程被跟踪以检测有意义的 CPU 使用模式。
- 以 `thread_count` 为上限可防止线程数少于计算池大小的进程出现越界索引。

### 与 prefetch_all_thread_cycles 的关系

此函数假定在当前周期中已为此进程调用了 [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md)。预取会打开线程句柄、查询 `QueryThreadCycleTime`，并填充 [ThreadStats](../scheduler.rs/ThreadStats.md) 中的 `cached_cycles` 和 `cached_total_time`。`apply_prime_threads` 读取这些缓存值来计算增量。如果未调用 `prefetch_all_thread_cycles`（或所有线程都失败了），则所有周期增量将为零，不会有线程被选为主力线程。

### 模拟运行行为

在模拟运行模式下，函数记录一条变更消息：

`"Prime CPUs: -> [4,5,6,7]"`

它不会调用任何委托函数、查询线程周期时间或修改任何调度器状态。

## 要求

| 要求 | 值 |
|------|------|
| 模块 | `apply` |
| 可见性 | `pub`（crate 内公开） |
| 调用者 | [apply_config_thread_level](../main.rs/apply_config_thread_level.md) |
| 被调用者 | [apply_prime_threads_select](apply_prime_threads_select.md)、[apply_prime_threads_promote](apply_prime_threads_promote.md)、[apply_prime_threads_demote](apply_prime_threads_demote.md)、[get_cpu_set_information](../winapi.rs/get_cpu_set_information.md)、[format_cpu_indices](../config.rs/format_cpu_indices.md) |
| Win32 API | 无直接调用 — 所有 Win32 调用由委托函数执行 |
| 权限 | 继承 [apply_prime_threads_promote](apply_prime_threads_promote.md) 和 [apply_prime_threads_demote](apply_prime_threads_demote.md) 的权限要求 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 主力线程选择（滞后算法） | [apply_prime_threads_select](apply_prime_threads_select.md) |
| 主力线程提升（CPU 固定 + 优先级提升） | [apply_prime_threads_promote](apply_prime_threads_promote.md) |
| 主力线程降级（取消固定 + 优先级恢复） | [apply_prime_threads_demote](apply_prime_threads_demote.md) |
| 周期时间预取 | [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) |
| 调度器状态和滞后算法 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| 线程级应用编排 | [apply_config_thread_level](../main.rs/apply_config_thread_level.md) |
| 应用后的周期/时间提交 | [update_thread_stats](update_thread_stats.md) |
| 配置模型 | [ProcessConfig](../config.rs/ProcessConfig.md) |
| apply 模块概览 | [apply](README.md) |