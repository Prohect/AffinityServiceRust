# apply_prime_threads_demote 函数 (apply.rs)

`apply_prime_threads_demote` 函数为不再具备主线程资格的线程移除 CPU 集合固定并恢复其原始线程优先级。该函数遍历所有之前被固定（即 `pinned_cpu_set_ids` 非空）但不在当前主线程选择集合中的存活线程，通过使用空切片调用 `SetThreadSelectedCpuSets` 清除其 CPU 集合分配，并将线程优先级恢复为提升阶段保存在 `original_priority` 中的值。这是主线程调度算法的降级阶段。

## 语法

```AffinityServiceRust/src/apply.rs#L952-964
pub fn apply_prime_threads_demote(
    pid: u32,
    config: &ThreadLevelConfig,
    threads: &HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 目标进程的进程 ID。用于线程统计查找、错误去重和日志消息。 |
| `config` | `&ThreadLevelConfig` | 线程级配置。`name` 字段用于传递给 `log_error_if_new` 和 `get_thread_handle` 的错误日志消息。 |
| `threads` | `&HashMap<u32, SYSTEM_THREAD_INFORMATION>` | 线程 ID 到其最近系统进程信息查询所得 `SYSTEM_THREAD_INFORMATION` 快照的映射。此映射的键定义了要遍历的存活线程集合。 |
| `tid_with_delta_cycles` | `&[(u32, u64, bool)]` | 包含线程 ID、增量周期计数和布尔值（指示线程是否被选为主线程，`true` 为是，`false` 为否）的元组切片。函数从 `is_prime == true` 的条目构建主线程 ID 的 `HashSet`，用于确定哪些线程**不应**被降级。 |
| `prime_core_scheduler` | `&mut PrimeThreadScheduler` | 可变的主线程调度器状态。函数读取和更新每线程统计信息，包括 `handle`、`pinned_cpu_set_ids` 和 `original_priority`。 |
| `apply_config_result` | `&mut ApplyConfigResult` | 执行期间产生的变更描述和错误消息的累加器。 |

## 返回值

此函数不返回值。所有结果通过对 `prime_core_scheduler` 的修改和 `apply_config_result` 中追加的条目来传达。

## 备注

### 算法

1. **构建主线程集合**：从 `tid_with_delta_cycles` 中 `is_prime == true` 的条目构建 `HashSet<u32>` 线程 ID 集合。这些线程当前被选为主线程，不应被降级。

2. **收集存活线程 ID**：将 `threads` 映射的键收集到 `List<[u32; TIDS_CAPED]>` 中，表示进程中所有存活线程。

3. **遍历存活线程**：对于每个存活线程 ID，函数从调度器获取其 `thread_stats`。在以下情况下跳过（不降级）线程：
   - 在 `prime_set` 中（仍被选为主线程），或
   - 其 `pinned_cpu_set_ids` 为空（从未被提升过）。

4. **句柄解析**：函数从 `thread_stats.handle` 获取线程的写句柄。优先使用 `w_handle`，其次使用 `w_limited_handle`。如果两者都无效，则通过 [`log_error_if_new`](log_error_if_new.md) 以 `Operation::OpenThread` 记录错误并跳过该线程。如果完全没有句柄，则静默跳过该线程。

5. **移除 CPU 集合固定**：使用空切片（`&[]`）调用 `SetThreadSelectedCpuSets` 以清除线程的 CPU 集合分配，使其恢复到进程的默认调度行为。成功时记录变更消息：
   `"Thread <tid> -> (demoted, start=<module>)"`
   其中 `<module>` 是通过 `resolve_address_to_module` 从线程的起始地址解析得到的。失败时通过 [`log_error_if_new`](log_error_if_new.md) 以 `Operation::SetThreadSelectedCpuSets` 记录错误。

6. **无条件清除固定状态**：无论 `SetThreadSelectedCpuSets` 是否成功，`thread_stats.pinned_cpu_set_ids` 都会被清除。这是一个有意的设计决策，旨在防止无限重试循环——如果线程的 CPU 集合无法清除（例如由于访问权限不足或线程已退出），每个应用周期都会重试并向错误日志发送垃圾信息。

7. **恢复原始优先级**：如果 `thread_stats.original_priority` 包含保存的优先级值（在 [`apply_prime_threads_promote`](apply_prime_threads_promote.md) 提升期间设置），则调用 `SetThreadPriority` 恢复该值。`original_priority` 字段通过 `.take()` 消耗，因此只会恢复一次。失败时通过 [`log_error_if_new`](log_error_if_new.md) 以 `Operation::SetThreadPriority` 记录错误。错误消息引用 `"RESTORE_SET_THREAD_PRIORITY"` 以区分提升期间的优先级设置错误。

### 边界情况

- 如果 `SetThreadSelectedCpuSets` 失败（例如线程在快照和 API 调用之间退出），`pinned_cpu_set_ids` 仍会被清除，以避免在每个后续应用周期重试失败的调用。优先级恢复仍会尝试。
- 如果 `original_priority` 为 `None`（例如提升期间 `GetThreadPriority` 失败且未保存优先级），则不尝试优先级恢复，线程仅清除其 CPU 集合。
- 存在于调度器状态中但不在 `threads` 映射中的线程（即已退出的线程）不会被此函数遍历，因为遍历基于 `threads.keys()`。过时条目由 [`apply_prime_threads`](apply_prime_threads.md) 中的句柄清理逻辑单独清理。
- 被降级但 `SetThreadPriority` 调用失败的线程，其 `original_priority` 已被消耗（通过 `.take()` 设为 `None`），因此恢复不会在下一个周期重新尝试。

### 与提升的交互

此函数撤销 [`apply_prime_threads_promote`](apply_prime_threads_promote.md) 的工作。`pinned_cpu_set_ids` 字段作为标志：非空表示已提升，空表示未提升。`original_priority` 字段桥接两个阶段：在提升期间设置，在降级期间消耗。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `apply.rs` |
| Crate | `AffinityServiceRust` |
| 可见性 | `pub` |
| Windows API | `SetThreadSelectedCpuSets`、`SetThreadPriority`、`GetLastError` |
| 调用者 | [`apply_prime_threads`](apply_prime_threads.md) |
| 被调用者 | [`log_error_if_new`](log_error_if_new.md)、`winapi::resolve_address_to_module`、`error_codes::error_from_code_win32`、`ThreadPriority::to_thread_priority_struct` |
| 权限 | 需要具有 `THREAD_SET_INFORMATION` 或 `THREAD_SET_LIMITED_INFORMATION`（写入）的线程句柄。 |

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
*Commit: 7221ea0694670265d4eb4975582d8ed2ae02439d*