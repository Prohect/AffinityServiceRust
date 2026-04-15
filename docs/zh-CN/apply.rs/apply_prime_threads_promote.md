# apply_prime_threads_promote 函数 (apply.rs)

`apply_prime_threads_promote` 函数通过 `SetThreadSelectedCpuSets` 将新选定的主线程固定到指定的高性能 CPU 上，并可选择性地提升其线程优先级。对于选择结果中标记为主线程的每个线程，该函数会将线程的起始地址解析为模块名，根据配置的前缀规则匹配以确定应用哪个 CPU 集和优先级，然后发出相应的 Windows API 调用。这是主线程调度算法的提升阶段。

## 语法

```AffinityServiceRust/src/apply.rs#L810-822
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

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 目标进程的进程 ID。用于线程统计查找、错误去重和日志消息。 |
| `config` | `&ThreadLevelConfig` | 线程级配置，包含 `prime_threads_cpus`（主线程的默认 CPU 索引集）、`prime_threads_prefixes`（前缀匹配规则列表，可按模块覆盖 CPU 集和线程优先级）以及 `name`（日志消息中使用的配置规则名称）。 |
| `current_mask` | `&mut usize` | 当前进程亲和性掩码。当非零时，主 CPU 索引会通过 `filter_indices_by_mask` 进行过滤，确保只使用进程亲和性范围内的 CPU。这可以防止将线程分配到进程无法执行的 CPU 上。 |
| `tid_with_delta_cycles` | `&[(u32, u64, bool)]` | 包含线程 ID、自上次测量以来的增量周期计数以及表示线程是否被选为主线程（`true`）或未选中（`false`）的布尔值的元组切片。本函数仅处理 `is_prime == true` 的条目。 |
| `prime_core_scheduler` | `&mut PrimeThreadScheduler` | 可变的主线程调度器状态。该函数读取并更新每个线程的统计信息，包括 `handle`、`pinned_cpu_set_ids`、`start_address` 和 `original_priority`。 |
| `apply_config_result` | `&mut ApplyConfigResult` | 执行过程中产生的变更描述和错误消息的累加器。 |

## 返回值

此函数不返回值。所有结果通过对 `prime_core_scheduler` 的修改和 `apply_config_result` 中追加的条目来传达。

## 备注

### 算法

对于 `tid_with_delta_cycles` 中每个 `is_prime` 为 `true` 的 `(tid, delta_cycles, is_prime)` 元组：

1. **跳过已固定的线程**：如果 `thread_stats.pinned_cpu_set_ids` 非空，说明线程已经被提升，将被跳过。这可以防止在每个应用周期重复应用 CPU 集和优先级提升。

2. **句柄解析**：该函数从 `thread_stats.handle` 获取线程的写句柄。优先使用 `w_handle`，其次使用 `w_limited_handle`。如果两者都无效，则通过 [`log_error_if_new`](log_error_if_new.md) 以 `Operation::OpenThread` 记录错误并跳过该线程。

3. **模块前缀匹配**：通过 `resolve_address_to_module` 将线程的起始地址解析为模块名。如果 `config.prime_threads_prefixes` 非空，则将模块名（不区分大小写）与每个前缀规则的 `prefix` 字段进行比较。第一个匹配的前缀决定：
   - 使用替代 CPU 集（`prefix.cpus`）代替 `config.prime_threads_cpus`。
   - 设置特定的线程优先级（`prefix.thread_priority`）代替默认的单级提升。
   如果没有前缀匹配且 `prime_threads_prefixes` 非空，则完全跳过该线程（只有匹配前缀规则的线程才会被提升）。

4. **亲和性过滤**：如果 `*current_mask` 非零，则通过 `filter_indices_by_mask` 将选定的主 CPU 索引与进程亲和性掩码进行过滤。这确保只包含进程被允许使用的 CPU。

5. **CPU 集应用**：过滤后的 CPU 索引通过 `cpusetids_from_indices` 转换为 CPU 集 ID。如果结果列表非空，则调用 `SetThreadSelectedCpuSets`。成功时，`thread_stats.pinned_cpu_set_ids` 被更新并记录变更消息：
   `"Thread <tid> -> (promoted, [<cpus>], cycles=<delta>, start=<module>)"`
   失败时，通过 [`log_error_if_new`](log_error_if_new.md) 以 `Operation::SetThreadSelectedCpuSets` 记录错误。

6. **优先级提升**：在 CPU 集固定之后（无论成功与否），该函数通过 `GetThreadPriority` 读取当前线程优先级。如果读取成功（返回值不是 `0x7FFFFFFF`）：
   - 当前优先级保存在 `thread_stats.original_priority` 中，以便在降级时恢复。
   - 新优先级的确定方式为：(a) 如果前缀规则的 `thread_priority` 已明确设置且不是 `ThreadPriority::None`，则使用该值；或 (b) 使用 `current_priority.boost_one()` 将优先级提升一级。
   - 如果新优先级与当前不同，则调用 `SetThreadPriority`。成功时记录变更消息：
     `"Thread <tid> -> (priority set: <old> -> <new>)"` 或 `"Thread <tid> -> (priority boosted: <old> -> <new>)"`。
   - 失败时，通过 [`log_error_if_new`](log_error_if_new.md) 以 `Operation::SetThreadPriority` 记录错误。

### 边界情况

- 如果 `prime_threads_prefixes` 为空，所有被选为主线程的线程都使用 `config.prime_threads_cpus` 作为 CPU 集并进行单级优先级提升。
- 如果 `prime_threads_prefixes` 非空但没有前缀匹配给定线程的起始模块，即使该线程被选为主线程也**不会**被提升。这允许对哪些模块接受主线程处理进行细粒度控制。
- 如果 `current_mask` 为 `0`（例如，从未查询过亲和性），则不进行过滤，使用完整的 `prime_threads_cpus` 列表。
- 如果 `cpusetids_from_indices` 返回空列表（例如，所有主 CPU 都被亲和性掩码过滤掉了），则不应用 CPU 集，该线程的优先级提升也会被跳过。
- `GetThreadPriority` 返回值 `0x7FFFFFFF`（`THREAD_PRIORITY_ERROR_RETURN`）表示失败；优先级提升会被静默跳过而不记录错误。

### 与降级的交互

由此函数提升的线程，如果在后续应用周期中退出主线程选择，将随后被 [`apply_prime_threads_demote`](apply_prime_threads_demote.md) 降级。`pinned_cpu_set_ids` 字段充当提升标记：非空表示已提升。`original_priority` 字段用于降级时将线程优先级恢复到提升前的水平。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `apply.rs` |
| Crate | `AffinityServiceRust` |
| 可见性 | `pub` |
| Windows API | `SetThreadSelectedCpuSets`、`GetThreadPriority`、`SetThreadPriority`、`GetLastError` |
| 调用者 | [`apply_prime_threads`](apply_prime_threads.md) |
| 被调用者 | [`log_error_if_new`](log_error_if_new.md)、`winapi::resolve_address_to_module`、`winapi::filter_indices_by_mask`、`winapi::cpusetids_from_indices`、`winapi::indices_from_cpusetids`、`config::format_cpu_indices`、`error_codes::error_from_code_win32`、`ThreadPriority::from_win_const`、`ThreadPriority::boost_one`、`ThreadPriority::to_thread_priority_struct` |
| 权限 | 需要具有 `THREAD_SET_INFORMATION` 或 `THREAD_SET_LIMITED_INFORMATION`（写入）和 `THREAD_QUERY_INFORMATION` 或 `THREAD_QUERY_LIMITED_INFORMATION`（读取，用于 `GetThreadPriority`）的线程句柄。 |

## 另请参阅

| 参考 | 链接 |
|------|------|
| apply 模块概述 | [`README`](README.md) |
| ApplyConfigResult | [`ApplyConfigResult`](ApplyConfigResult.md) |
| apply_prime_threads | [`apply_prime_threads`](apply_prime_threads.md) |
| apply_prime_threads_select | [`apply_prime_threads_select`](apply_prime_threads_select.md) |
| apply_prime_threads_demote | [`apply_prime_threads_demote`](apply_prime_threads_demote.md) |
| prefetch_all_thread_cycles | [`prefetch_all_thread_cycles`](prefetch_all_thread_cycles.md) |
| log_error_if_new | [`log_error_if_new`](log_error_if_new.md) |
| ThreadLevelConfig | [`config.rs/ThreadLevelConfig`](../config.rs/ThreadLevelConfig.md) |
| PrimeThreadScheduler | [`scheduler.rs/PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md) |
| ThreadPriority | [`priority.rs/ThreadPriority`](../priority.rs/ThreadPriority.md) |

---
*Commit: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)*