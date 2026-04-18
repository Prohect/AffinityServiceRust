# apply_prime_threads_demote 函数 (apply.rs)

`apply_prime_threads_demote` 函数为不再符合 prime 条件的线程移除 CPU Set 固定并恢复其原始线程优先级。它遍历所有之前被固定（即 `pinned_cpu_set_ids` 非空）但不在当前 prime 选择集中的存活线程，通过使用空切片调用 `SetThreadSelectedCpuSets` 来清除其 CPU Set 分配，并将线程优先级恢复为提升阶段保存在 `original_priority` 中的值。这是 prime 线程调度算法的降级阶段。

## 语法

```AffinityServiceRust/src/apply.rs#L952-965
pub fn apply_prime_threads_demote<'a>(
    pid: u32,
    config: &ThreadLevelConfig,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 目标进程的进程 ID。用于线程统计信息查找、错误去重和日志消息。 |
| `config` | `&ThreadLevelConfig` | 线程级配置。`name` 字段用于传递给 `log_error_if_new` 和 `get_thread_handle` 的错误日志消息。 |
| `threads` | `&impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>` | 一个延迟闭包，返回线程 ID 到最近一次系统进程信息查询中对应 `SYSTEM_THREAD_INFORMATION` 快照的映射引用。闭包被调用一次以获取线程映射，其键定义了要遍历的存活线程集合。 |
| `tid_with_delta_cycles` | `&[(u32, u64, bool)]` | 包含线程 ID、增量周期计数和布尔值（指示线程是否被选为 prime（`true`）或未被选为（`false`））的元组切片。函数从 `is_prime == true` 的条目构建一个 `HashSet` 的 prime 线程 ID，用于确定哪些线程**不应**被降级。 |
| `prime_core_scheduler` | `&mut PrimeThreadScheduler` | 可变的 prime 线程调度器状态。函数读取并更新每个线程的统计信息，包括 `handle`、`pinned_cpu_set_ids` 和 `original_priority`。 |
| `apply_config_result` | `&mut ApplyConfigResult` | 执行过程中产生的变更描述和错误消息的累加器。 |

## 返回值

此函数不返回值。所有结果通过对 `prime_core_scheduler` 的修改和追加到 `apply_config_result` 的条目来传达。

## 备注

### 算法

1. **构建 prime 集合**：从 `tid_with_delta_cycles` 中 `is_prime == true` 的条目构建一个 `HashSet<u32>` 的线程 ID。这些线程当前被选为 prime，不应被降级。

2. **收集存活线程 ID**：调用闭包 `threads()` 获取线程映射，其键被收集到一个 `List<[u32; TIDS_CAPED]>` 中，表示进程中所有存活线程。

3. **遍历存活线程**：对于每个存活线程 ID，函数从调度器中检索其 `thread_stats`。在以下情况下跳过线程（不降级）：
   - 线程在 `prime_set` 中（仍被选为 prime），或
   - 线程的 `pinned_cpu_set_ids` 为空（从未被提升过）。

4. **句柄解析**：函数从 `thread_stats.handle` 获取线程的写句柄。优先使用 `w_handle`，其次使用 `w_limited_handle`。如果两者都无效，则通过 [`log_error_if_new`](log_error_if_new.md) 以 `Operation::OpenThread` 记录错误并跳过该线程。如果根本不存在句柄，则静默跳过该线程。

5. **移除 CPU Set 固定**：使用空切片（`&[]`）调用 `SetThreadSelectedCpuSets` 以清除线程的 CPU Set 分配，使其恢复到进程的默认调度行为。成功时记录变更消息：
   `"Thread <tid> -> (demoted, start=<module>)"`
   其中 `<module>` 通过 `resolve_address_to_module` 从线程的起始地址解析。失败时通过 [`log_error_if_new`](log_error_if_new.md) 以 `Operation::SetThreadSelectedCpuSets` 记录错误。

6. **无条件清除固定状态**：无论 `SetThreadSelectedCpuSets` 成功还是失败，`thread_stats.pinned_cpu_set_ids` 都会被清除。这是一个有意的设计决策，旨在防止无限重试循环——对于无法清除 CPU Set 的线程（例如由于访问权限不足或线程已退出），避免在每个应用周期中持续产生错误日志。

7. **恢复原始优先级**：如果 `thread_stats.original_priority` 包含保存的优先级值（在 [`apply_prime_threads_promote`](apply_prime_threads_promote.md) 提升阶段设置），则调用 `SetThreadPriority` 恢复该值。`original_priority` 字段通过 `.take()` 消耗，因此只恢复一次。失败时通过 [`log_error_if_new`](log_error_if_new.md) 以 `Operation::SetThreadPriority` 记录错误。错误消息引用 `"RESTORE_SET_THREAD_PRIORITY"` 以区别于提升阶段的优先级设置错误。

### 边界情况

- 如果 `SetThreadSelectedCpuSets` 失败（例如线程在快照和 API 调用之间已退出），`pinned_cpu_set_ids` 仍会被清除以避免在后续每个应用周期中重试失败的调用。优先级恢复仍会尝试。
- 如果 `original_priority` 为 `None`（例如提升阶段 `GetThreadPriority` 失败且未保存优先级），则不尝试优先级恢复，线程仅清除其 CPU Set。
- 存在于调度器状态中但不在 `threads()` 返回的映射中的线程（即已退出的线程）不会被此函数遍历，因为遍历是基于映射的键。过期条目由 [`apply_prime_threads`](apply_prime_threads.md) 中的句柄清理逻辑单独处理。
- 已降级但 `SetThreadPriority` 调用失败的线程，其 `original_priority` 已通过 `.take()` 被消耗（设为 `None`），因此在下一个周期中不会重新尝试恢复。

### 与提升阶段的交互

此函数撤销 [`apply_prime_threads_promote`](apply_prime_threads_promote.md) 的工作。`pinned_cpu_set_ids` 字段用作标志：非空表示已提升，空表示未提升。`original_priority` 字段连接两个阶段：在提升阶段设置，在降级阶段消耗。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `apply.rs` |
| Crate | `AffinityServiceRust` |
| 可见性 | `pub` |
| Windows API | `SetThreadSelectedCpuSets`、`SetThreadPriority`、`GetLastError` |
| 调用方 | [`apply_prime_threads`](apply_prime_threads.md) |
| 被调用方 | [`log_error_if_new`](log_error_if_new.md)、`winapi::resolve_address_to_module`、`error_codes::error_from_code_win32`、`ThreadPriority::to_thread_priority_struct` |
| 权限 | 需要具有 `THREAD_SET_INFORMATION` 或 `THREAD_SET_LIMITED_INFORMATION`（写入）访问权限的线程句柄。 |

## 另请参阅

| 参考 | 链接 |
|------|------|
| apply 模块概述 | [`README`](README.md) |
| ApplyConfigResult | [`ApplyConfigResult`](ApplyConfigResult.md) |
| apply_prime_threads | [`apply_prime_threads`](apply_prime_threads.md) |
| apply_prime_threads_select | [`apply_prime_threads_select`](apply_prime_threads_select.md) |
| apply_prime_threads_promote | [`apply_prime_threads_promote`](apply_prime_threads_promote.md) |
| prefetch_all_thread_cycles | [`prefetch_all_thread_cycles`](prefetch_all_thread_cycles.md) |
| log_error_if_new | [`log_error_if_new`](log_error_if_new.md) |
| ThreadLevelConfig | [`config.rs/ThreadLevelConfig`](../config.rs/ThreadLevelConfig.md) |
| PrimeThreadScheduler | [`scheduler.rs/PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md) |
| ThreadPriority | [`priority.rs/ThreadPriority`](../priority.rs/ThreadPriority.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*