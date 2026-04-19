# apply_prime_threads_promote 函数 (apply.rs)

将通过迟滞选择的线程提升为专用的高性能 CPU，通过 CPU 集合固定并可选择提升线程优先级。此函数是 Prime 线程调度算法的"奖励"阶段——展示了持续高 CPU 使用率的线程可以获得特定处理器内核的优先访问权限，从而改善缓存局部性并减少竞争。

## 语法

```AffinityServiceRust/src/apply.rs#L811-819
pub fn apply_prime_threads_promote(
    pid: u32,
    config: &ThreadLevelConfig,
    current_mask: &mut usize,
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

`pid: u32`

目标进程的进程标识符。

`config: &ThreadLevelConfig`

线程级别配置，包含：
- `prime_threads_cpus` —— 用于固定 Prime 线程的默认 CPU 索引集。
- `prime_threads_prefixes` —— 覆盖默认 CPU 集合和线程优先级的 [`PrimePrefix`](../config.rs/PrimePrefix.md) 规则列表，基于线程入口点的模块名称。
- `name` —— 用于日志记录的配置规则名称。

`current_mask: &mut usize`

进程的当前亲和性掩码，用于过滤 Prime CPU 索引。如果掩码非零，则仅使用掩码中存在的 CPU 进行固定（通过 `filter_indices_by_mask`）。这可以防止将线程分配给超出进程允许亲和性的 CPU。

`tid_with_delta_cycles: &[(u32, u64, bool)]`

由 [`apply_prime_threads_select`](apply_prime_threads_select.md) 产生的 `(线程 id, delta 周期，is_prime)` 元组列表。此函数仅处理 `is_prime == true` 的条目。

`prime_core_scheduler: &mut PrimeThreadScheduler`

跟踪每线程状态的 [`PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md) 实例，包括缓存的句柄、起始地址、固定的 CPU 集合 ID 和原始线程优先级。

`apply_config_result: &mut ApplyConfigResult`

变更消息和错误的累加器。参见 [`ApplyConfigResult`](ApplyConfigResult.md)。

## 返回值

此函数不返回值。结果累加在 `apply_config_result` 中。

## 备注

### 每个线程的提升流程

对于每个标记为 Prime 的线程（`is_prime == true`）：

1. **如果已固定则跳过** —— 如果 `thread_stats.pinned_cpu_set_ids` 非空，则该线程已被提升，无需操作。

2. **解析写句柄** —— 从缓存的 `ThreadHandle` 获取可写的线程句柄，优先使用完全访问句柄而非受限句柄。如果没有可用的有效句柄，则跳过该线程并通过 [`log_error_if_new`](log_error_if_new.md) 记录错误。

3. **解析起始模块** —— 调用 `resolve_address_to_module`，传入线程缓存的起始地址，以确定线程入口点属于哪个模块（DLL/EXE）。

4. **与前缀规则匹配** —— 遍历 `config.prime_threads_prefixes` 并对起始模块名称进行不区分大小写的前缀匹配。第一个匹配的 [`PrimePrefix`](../config.rs/PrimePrefix.md) 规则可以覆盖：
   - 用于固定的 CPU 集合（通过 `prefix.cpus`）。
   - 设置的线程优先级（通过 `prefix.thread_priority`）。
   
   如果配置了前缀但未匹配到任何前缀，则**完全跳过**该线程（不提升）。如果未配置前缀，则使用默认的 `config.prime_threads_cpus`。

5. **通过亲和性掩码过滤 CPU** —— 如果 `current_mask` 非零，则过滤目标 CPU 索引，仅保留进程亲和性掩码允许的 CPU。

6. **通过 `SetThreadSelectedCpuSets` 固定** —— 将目标 CPU 索引转换为 CPU 集合 ID 并调用 Windows API 固定线程。成功后，在 `thread_stats.pinned_cpu_set_ids` 中记录固定的 CPU 集合 ID，并记录变更消息，包含线程 ID、提升的 CPU、周期计数和起始模块。

7. **提升线程优先级** —— 固定后，通过 `GetThreadPriority` 读取当前线程优先级并将其保存在 `thread_stats.original_priority` 中，以便在降级的后期恢复。新优先级确定如下：
   - 如果匹配的前缀指定了 `thread_priority`，则直接使用该值（记录为"优先级设置"）。
   - 否则，通过 `ThreadPriority::boost_one()` 将当前优先级提升一级（记录为"优先级提升"）。
   
   仅当新值与当前值不同时才更改优先级。优先级提升适用于 CPU 固定成功或失败的情况。

### 前缀匹配详情

- 匹配**不区分大小写**——在比较前将模块名称和前缀都转换为小写。
- 仅使用**第一个**匹配的前缀；不评估后续前缀。
- 当配置了前缀但线程的模块未匹配任何前缀时，该线程**被排除在提升之外**。这允许针对特定子系统的提升（如渲染线程、音频线程）。

### 错误处理

所有 Windows API 错误都通过 [`log_error_if_new`](log_error_if_new.md) 报告，支持去重，防止持久性错误的日志刷屏。跟踪的操作包括：
- `Operation::OpenThread` —— 无效的线程句柄。
- `Operation::SetThreadSelectedCpuSets` —— CPU 集合固定失败。
- `Operation::SetThreadPriority` —— 优先级提升失败。

### 变更消息

提升成功时，可能记录两条变更消息：
- `"Thread {tid} -> (promoted, [{cpus}], cycles={delta}, start={module})"` —— CPU 集合固定。
- `"Thread {tid} -> (priority boosted: {old} -> {new})"` 或 `"Thread {tid} -> (priority set: {old} -> {new})"` —— 优先级更改。

## 要求

| | |
|---|---|
| **模块** | `src/apply.rs` |
| **调用方** | [`apply_prime_threads`](apply_prime_threads.md) |
| **被调用方** | [`log_error_if_new`](log_error_if_new.md), `resolve_address_to_module`, `filter_indices_by_mask`, `cpusetids_from_indices`, `indices_from_cpusetids`, `format_cpu_indices` |
| **Win32 API** | [`SetThreadSelectedCpuSets`](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadselectedcpusets), [`GetThreadPriority`](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadpriority), [`SetThreadPriority`](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadpriority), [`GetLastError`](https://learn.microsoft.com/zh-cn/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror) |
| **权限** | 需要在目标线程上具有 `THREAD_SET_INFORMATION` 和 `THREAD_QUERY_INFORMATION`（或受限变体）。 |

## 另请参阅

| | |
|---|---|
| [`apply_prime_threads`](apply_prime_threads.md) | 调用 select → promote → demote 的编排器。 |
| [`apply_prime_threads_select`](apply_prime_threads_select.md) | 基于迟滞选择要提升的线程。 |
| [`apply_prime_threads_demote`](apply_prime_threads_demote.md) | 逆转提升：取消固定 CPU 并恢复优先级。 |
| [`PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md) | 管理每线程调度状态和迟滞逻辑。 |
| [`ThreadLevelConfig`](../config.rs/ThreadLevelConfig.md) | 包含 Prime 前缀的线程级别设置配置。 |
| [`PrimePrefix`](../config.rs/PrimePrefix.md) | 可选 CPU 集合和线程优先级覆盖的每模块前缀规则。 |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*