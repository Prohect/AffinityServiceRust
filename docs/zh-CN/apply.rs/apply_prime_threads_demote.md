# apply_prime_threads_demote 函数 (apply.rs)

通过移除线程的 CPU 集合绑定并恢复其原始线程优先级，来降级不再符合 prime 状态的线程。

## 语法

```AffinityServiceRust/src/apply.rs#L953-961
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

`pid: u32`

要降级其线程的目标进程的进程标识符。

`config: &ThreadLevelConfig`

包含用于日志记录的进程名称的线程级别配置。参见 [ThreadLevelConfig](../config.rs/ThreadLevelConfig.md)。

`threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>`

返回进程所有活动线程 ID 及其系统线程信息的惰性访问器。用于枚举可能需要降级的线程。

`tid_with_delta_cycles: &[(u32, u64, bool)]`

由之前流水线阶段产生的候选线程列表。每个元组为 `(线程 ID, delta_cycles, is_prime)`。`is_prime` 标志指示应**保持**prime 状态的线程；所有其他具有非空 `pinned_cpu_set_ids` 的线程将被降级。

`prime_core_scheduler: &mut PrimeThreadScheduler`

持有每线程状态的 Prime 线程调度器，包括缓存的句柄、绑定的 CPU 集合 ID 和原始线程优先级。参见 [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md)。

`apply_config_result: &mut ApplyConfigResult`

更改和错误消息的累加器。参见 [ApplyConfigResult](ApplyConfigResult.md)。

## 返回值

此函数不返回值。结果记录在 `apply_config_result` 中。

## 备注

此函数是 Prime 线程调度流水线的第三阶段也是最后阶段，由 [`apply_prime_threads`](apply_prime_threads.md) 在调用 [`apply_prime_threads_select`](apply_prime_threads_select.md) 和 [`apply_prime_threads_promote`](apply_prime_threads_promote.md) 后调用。

### 降级逻辑

1. **识别 prime 集** — 在 `tid_with_delta_cycles` 中构建当前标记为 `is_prime == true` 的线程 ID 的 `HashSet`。
2. **枚举活动线程** — 从 `threads()` 访问器收集所有活动线程 ID。
3. **过滤候选者** — 对于每个活动线程，跳过仍然是 prime 的线程或具有空 `pinned_cpu_set_ids`（从未提升过）的线程。
4. **移除 CPU 集合绑定** — 调用 `SetThreadSelectedCpuSets` 并传入空切片以清除任何 CPU 集合分配，允许线程在任何处理器上运行。
5. **清除绑定状态** — 无论 `SetThreadSelectedCpuSets` 调用成功与否，都清除 `pinned_cpu_set_ids`。这可以防止无限重试循环导致错误日志泛滥。
6. **恢复线程优先级** — 如果在提升期间保存了 `original_priority`，则通过 `SetThreadPriority` 将线程恢复为其之前的优先级。

### 错误弹性

该函数即使在 `SetThreadSelectedCpuSets` 失败时也故意清除 `pinned_cpu_set_ids`。这种设计选择优先考虑避免日志垃圾，而非保证清理。如果 API 调用失败（例如，由于线程句柄已失效），线程在其退出或重新创建时将自然失去绑定。

错误通过 [`log_error_if_new`](log_error_if_new.md) 去重，因此每个唯一的 `(pid, tid, operation, error_code)` 组合仅报告一次。

### 线程句柄选择

优先使用写句柄而非受限写句柄。如果两者都无效，则跳过该线程并记录错误。

### 优先级恢复

该函数使用 `thread_stats.original_priority.take()` 来消耗存储的优先级。这确保优先级仅恢复一次，防止在后续周期中重复恢复。

### 示例更改消息

成功降级时：
- `Thread 1234 -> (demoted, start=ntdll.dll)`

当优先级恢复失败时，通过 `log_error_if_new` 记录错误。

## 要求

| | |
|---|---|
| **模块** | `src/apply.rs` |
| **调用方** | [`apply_prime_threads`](apply_prime_threads.md) |
| **被调用方** | `SetThreadSelectedCpuSets`, `SetThreadPriority`, `resolve_address_to_module`, [`log_error_if_new`](log_error_if_new.md) |
| **Win32 API** | [SetThreadSelectedCpuSets](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadselectedcpusets), [SetThreadPriority](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadpriority) |
| **权限** | 目标线程上的 `THREAD_SET_INFORMATION` 或 `THREAD_SET_LIMITED_INFORMATION` |

## 参见

| 主题 | 描述 |
|---|---|
| [`apply_prime_threads`](apply_prime_threads.md) | Prime 线程调度流水线的编排函数 |
| [`apply_prime_threads_select`](apply_prime_threads_select.md) | 选择哪些线程符合 Prime 状态 |
| [`apply_prime_threads_promote`](apply_prime_threads_promote.md) | 通过 CPU 绑定和优先级提升来提升选定线程 |
| [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) | 管理每线程调度状态和迟滞逻辑 |
| [ApplyConfigResult](ApplyConfigResult.md) | 更改和错误报告的累加器 |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*