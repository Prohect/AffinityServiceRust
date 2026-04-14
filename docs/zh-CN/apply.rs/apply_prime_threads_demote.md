# apply_prime_threads_demote 函数 (apply.rs)

对不再符合主力线程资格的线程进行降级，移除其每线程 CPU 集合绑定并恢复其原始线程优先级。这是主力线程调度管线的最后阶段，由 [apply_prime_threads](apply_prime_threads.md) 在 [apply_prime_threads_select](apply_prime_threads_select.md) 和 [apply_prime_threads_promote](apply_prime_threads_promote.md) 完成后调用。

## 语法

```AffinityServiceRust/src/apply.rs#L966-977
pub fn apply_prime_threads_demote(
    pid: u32,
    config: &ProcessConfig,
    process: &mut ProcessEntry,
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 目标进程的进程标识符。用作 [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) 状态映射的键以及错误日志记录。 |
| `config` | `&`[ProcessConfig](../config.rs/ProcessConfig.md) | 此进程的已解析配置规则。`name` 字段用于错误消息。 |
| `process` | `&mut`[ProcessEntry](../process.rs/ProcessEntry.md) | 目标进程的快照条目。提供活跃线程列表——只有在快照中仍然存在的线程才是降级候选。 |
| `tid_with_delta_cycles` | `&[(u32, u64, bool)]` | 由 [apply_prime_threads_select](apply_prime_threads_select.md) 生成的 `(thread_id, delta_cycles, is_prime)` 元组切片。`is_prime` 标志指示该线程在本周期是否被选为主力线程。`is_prime == true` 的线程将被跳过；`is_prime == false` 且其 [ThreadStats](../scheduler.rs/ThreadStats.md) 中 `pinned_cpu_set_ids` 非空的线程将被降级。 |
| `prime_core_scheduler` | `&mut`[PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) | 持久化的调度器状态。此函数读取并修改 [ThreadStats](../scheduler.rs/ThreadStats.md) 中每个线程的 `pinned_cpu_set_ids`、`original_priority` 和 `handle` 字段。 |
| `apply_config_result` | `&mut`[ApplyConfigResult](ApplyConfigResult.md) | 降级过程中产生的变更描述和错误消息的累加器。 |

## 返回值

无 (`()`)。结果通过 `apply_config_result` 和对 `prime_core_scheduler` 的副作用传达。

## 备注

### 算法

1. **构建主力线程集合** — 从 `tid_with_delta_cycles` 切片中构建一个包含被选为主力线程（`is_prime == true`）的线程 ID 的 `HashSet<u32>`，用于 O(1) 查找。

2. **枚举活跃线程** — 从 `process.get_threads()` 收集活跃线程 ID。只迭代当前快照中存在的线程，防止函数操作过期的线程 ID。

3. **识别降级候选** — 对于每个活跃线程，函数检查两个条件：
   - 线程**不在**主力线程集合中（本周期未被选中）。
   - 线程在 [ThreadStats](../scheduler.rs/ThreadStats.md) 中的 `pinned_cpu_set_ids` **非空**（之前已被提升并绑定）。

   如果两个条件都满足，则该线程为降级候选。

4. **取消 CPU 集合绑定** — 使用空切片 (`&[]`) 调用 `SetThreadSelectedCpuSets` 以清除每线程的 CPU 集合分配，使线程恢复到进程默认的调度行为。需要一个有效的写句柄（`w_handle`，回退到 `w_limited_handle`）。

5. **清除绑定状态** — **无论取消绑定是否成功**，都会调用 `thread_stats.pinned_cpu_set_ids.clear()`。这是一个有意的设计选择，用于防止无限重试循环：如果取消绑定调用失败（例如线程在枚举和 API 调用之间退出），否则该线程将在每个后续周期被重试，产生重复的错误日志条目。清除状态确保函数继续前进。

6. **恢复原始优先级** — 如果 [ThreadStats](../scheduler.rs/ThreadStats.md) 中线程的 `original_priority` 字段为 `Some`，则通过 `SetThreadPriority` 恢复原始 [ThreadPriority](../priority.rs/ThreadPriority.md)。该字段通过 `Option::take()` 消费，防止在后续周期重复恢复。如果优先级恢复失败，错误会被记录，但该线程仍被视为已降级。

7. **日志记录** — 成功取消绑定后，记录一条变更消息：

   `"Thread 5678 -> (demoted, start=ntdll.dll)"`

   启动模块名称通过 [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md) 从线程的 `start_address` 解析。

### 错误处理

CPU 集合清除和优先级恢复调用的失败都通过 [log_error_if_new](log_error_if_new.md) 路由：

| 操作 | `Operation` 变体 | 常见失败原因 |
|------|-------------------|------------|
| `SetThreadSelectedCpuSets`（清除） | `Operation::SetThreadSelectedCpuSets` | 线程已退出或句柄无效 |
| `SetThreadPriority`（恢复） | `Operation::SetThreadPriority` | 线程已退出或访问被拒绝 |

优先级恢复失败的错误以函数名 `apply_prime_threads_promote` 和操作标签 `[RESTORE_SET_THREAD_PRIORITY]` 记录。尽管消息中的函数名如此，此代码路径实际位于 `apply_prime_threads_demote` 中。这是有意为之——提升函数最初拥有优先级控制权，因此"恢复"对应部分保留此命名以便在日志中追踪。

### 与 apply_prime_threads_promote 的关系

提升和降级互为逆操作：

| 操作 | [apply_prime_threads_promote](apply_prime_threads_promote.md) | apply_prime_threads_demote |
|------|---------------------------------------------------------------|----------------------------|
| CPU 集合 | 将线程绑定到主力 CPU 集合 ID | 清除线程 CPU 集合（空切片） |
| 优先级 | 提升线程优先级（配置的值或自动 +1 级） | 恢复提升期间保存的 `original_priority` |
| 状态跟踪 | 设置 `pinned_cpu_set_ids`，存储 `original_priority` | 清除 `pinned_cpu_set_ids`，取走 `original_priority` |

### 线程句柄选择

函数按照与其他 apply 函数相同的顺序遍历句柄层级：

1. 优先使用 `w_handle`（完整 `THREAD_SET_INFORMATION` 访问权限）。
2. 回退到 `w_limited_handle`（`THREAD_SET_LIMITED_INFORMATION`）。
3. 如果两者都无效，通过 [log_error_if_new](log_error_if_new.md) 记录错误并跳过该线程。
4. 如果根本不存在句柄（`thread_stats.handle` 为 `None`），则静默跳过该线程。

### 边界情况

- **线程在选择与降级之间退出** — 函数仅迭代快照中的活跃线程，因此在周期中途退出的线程自然会被排除。但如果快照过期，且线程在快照时间和 `SetThreadSelectedCpuSets` 调用之间退出，API 调用可能失败；这由无条件的 `pinned_cpu_set_ids.clear()` 处理。
- **未记录原始优先级** — 如果 `original_priority` 为 `None`（例如线程在优先级跟踪功能添加之前被提升，或者提升期间 `GetThreadPriority` 失败），则不尝试优先级恢复。
- **线程已降级** — `pinned_cpu_set_ids` 为空的线程会立即被跳过，因此对同一线程多次调用此函数是无害的。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `apply` |
| 可见性 | `pub`（crate 公开） |
| 调用方 | [apply_prime_threads](apply_prime_threads.md) |
| 被调用方 | [log_error_if_new](log_error_if_new.md), [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md) |
| Win32 API | [`SetThreadSelectedCpuSets`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadselectedcpusets), [`SetThreadPriority`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadpriority) |
| 权限 | `THREAD_SET_LIMITED_INFORMATION`（`SetThreadSelectedCpuSets` 的最低要求）、`THREAD_SET_INFORMATION`（`SetThreadPriority` 所需）。服务通常持有 `SeDebugPrivilege`，可授予这两种权限。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 主力线程编排器 | [apply_prime_threads](apply_prime_threads.md) |
| 主力线程选择（滞后算法） | [apply_prime_threads_select](apply_prime_threads_select.md) |
| 主力线程提升（逆操作） | [apply_prime_threads_promote](apply_prime_threads_promote.md) |
| 周期时间预取 | [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) |
| 调度器状态模型 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| 每线程统计 | [ThreadStats](../scheduler.rs/ThreadStats.md) |
| 线程优先级枚举 | [ThreadPriority](../priority.rs/ThreadPriority.md) |
| 错误去重 | [log_error_if_new](log_error_if_new.md) |
| apply 模块概览 | [apply](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd