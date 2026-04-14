# apply_prime_threads_promote 函数 (apply.rs)

将新选定的主力线程提升为专用性能核心，通过 `SetThreadSelectedCpuSets` 将其固定到专用性能核心 CPU 集合，并可选地提升其线程优先级。对于每个被 [apply_prime_threads_select](apply_prime_threads_select.md) 标记为主力的线程，此函数解析线程的启动模块，将其与配置的前缀进行匹配以确定使用哪些 CPU 和优先级，应用 CPU 集合固定，然后调整线程的优先级——要么调整到显式配置的级别，要么在当前值基础上提升一级。

## 语法

```AffinityServiceRust/src/apply.rs#L824-833
pub fn apply_prime_threads_promote(
    pid: u32,
    config: &ProcessConfig,
    current_mask: &mut usize,
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `pid` | `u32` | 目标进程的进程标识符。用作调度器每进程统计映射表的键，用于通过 [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md) 解析模块，以及在格式化的错误/变更消息中使用。 |
| `config` | `&`[ProcessConfig](../config.rs/ProcessConfig.md) | 此进程的已解析配置规则。`prime_threads_cpus` 字段提供用于固定的默认 CPU 索引集合；`prime_threads_prefixes` 字段提供每模块的覆盖（每个 [PrimePrefix](../config.rs/PrimePrefix.md) 可以指定备用 CPU 和线程优先级）。`name` 字段用于日志消息。 |
| `current_mask` | `&mut usize` | 进程当前的 CPU 亲和性掩码，由 [apply_affinity](apply_affinity.md) 填充。当非零时，主力 CPU 索引通过 [filter_indices_by_mask](../winapi.rs/filter_indices_by_mask.md) 进行过滤，以确保仅使用进程硬亲和性范围内的 CPU 进行固定。这可以防止将线程分配到进程不允许运行的 CPU 上。 |
| `tid_with_delta_cycles` | `&[(u32, u64, bool)]` | 由 [apply_prime_threads_select](apply_prime_threads_select.md) 生成的 `(thread_id, delta_cycles, is_prime)` 元组切片。此函数仅处理 `is_prime` 为 `true` 的条目。`delta_cycles` 值包含在变更消息中用于可观测性，但不影响提升逻辑。 |
| `prime_core_scheduler` | `&mut`[PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) | 持久化的调度器状态。此函数从 [ThreadStats](../scheduler.rs/ThreadStats.md) 中读取线程句柄和起始地址，并在成功提升后写入 `pinned_cpu_set_ids` 和 `original_priority`。 |
| `apply_config_result` | `&mut`[ApplyConfigResult](ApplyConfigResult.md) | 提升过程中产生的变更描述和错误消息的累加器。 |

## 返回值

无（`()`）。结果通过对 `prime_core_scheduler` 的修改（更新 `pinned_cpu_set_ids`、`original_priority`）和追加到 `apply_config_result` 的条目来传达。

## 备注

### 算法

对于 `tid_with_delta_cycles` 中 `is_prime` 为 `true` 的每个条目：

1. **跳过已固定的线程。** 如果 `thread_stats.pinned_cpu_set_ids` 非空，则该线程在之前的周期中已被提升且仍为主力——无需进一步操作。

2. **解析写入句柄。** 从线程缓存的 [ThreadHandle](../winapi.rs/ThreadHandle.md) 中选择最佳可用写入句柄（`w_handle`，回退到 `w_limited_handle`）。如果两者都无效，则通过 [log_error_if_new](log_error_if_new.md) 以 `Operation::OpenThread` 记录错误并跳过该线程。

3. **解析启动模块。** 线程的起始地址（由 [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) 预先填充）通过 [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md) 解析为模块名。模块名用于前缀匹配并包含在变更消息中。

4. **前缀匹配。** 如果 `config.prime_threads_prefixes` 非空，则将每个 [PrimePrefix](../config.rs/PrimePrefix.md) 与小写化的启动模块进行测试。第一个匹配的前缀决定：
   - 使用的 CPU 集合（如果前缀指定了 `prefix.cpus` 则使用它，否则使用 `config.prime_threads_cpus`）。
   - 要设置的线程优先级（如果 `prefix.thread_priority` 不为 `None`）。

   如果没有前缀匹配且前缀列表非空，则该线程被**跳过**——它不符合任何已配置模块前缀的提升条件。

5. **按亲和性掩码过滤 CPU。** 如果 `*current_mask` 非零，则通过 [filter_indices_by_mask](../winapi.rs/filter_indices_by_mask.md) 过滤已解析的 CPU 索引，以移除进程硬亲和性范围之外的 CPU。这可以防止 `SetThreadSelectedCpuSets` 指定进程被限制使用的 CPU。

6. **转换为 CPU 集合 ID。** 过滤后的 CPU 索引通过 [cpusetids_from_indices](../winapi.rs/cpusetids_from_indices.md) 转换为 Windows CPU 集合 ID。如果结果为空（没有解析出有效的 CPU 集合 ID），则跳过该线程。

7. **固定线程。** 使用已解析的 CPU 集合 ID 调用 `SetThreadSelectedCpuSets`。成功时，ID 被存储在 `thread_stats.pinned_cpu_set_ids` 中（以便在后续周期中将该线程识别为已固定）并记录一条变更消息。失败时，Win32 错误代码通过 [log_error_if_new](log_error_if_new.md) 以 `Operation::SetThreadSelectedCpuSets` 路由。

8. **读取当前优先级。** 调用 `GetThreadPriority` 读取线程的当前优先级级别。如果调用返回 `THREAD_PRIORITY_ERROR_RETURN`（`0x7FFFFFFF`），则完全跳过优先级提升步骤。

9. **保存原始优先级。** 当前优先级（作为 [ThreadPriority](../priority.rs/ThreadPriority.md) 枚举）被存储在 `thread_stats.original_priority` 中。此值稍后由 [apply_prime_threads_demote](apply_prime_threads_demote.md) 在线程被降级时用于恢复其优先级。

10. **提升优先级。** 新优先级通过以下两种路径之一确定：
    - 如果匹配的前缀指定了非 `None` 的 `thread_priority`，则使用该显式值（*优先级设定*）。
    - 否则，当前优先级通过 `ThreadPriority::boost_one()` 提升一级（*优先级提升*）。例如，`Normal` 变为 `AboveNormal`，`AboveNormal` 变为 `Highest`。

    如果新优先级等于当前优先级（例如线程已处于 `TimeCritical` 且无法进一步提升），则不进行设置调用。否则，调用 `SetThreadPriority`。成功时记录变更消息；失败时通过 [log_error_if_new](log_error_if_new.md) 以 `Operation::SetThreadPriority` 记录错误。

### 变更消息格式

每个成功提升的线程会生成两条变更消息：

**CPU 集合固定：**
```/dev/null/example.txt#L1
Thread 1234 -> (promoted, [4,5], cycles=98000, start=ntdll.dll!RtlUserThreadStart)
```

**优先级调整：**
```/dev/null/example.txt#L1-2
Thread 1234 -> (priority boosted: Normal -> AboveNormal)
Thread 1234 -> (priority set: Normal -> Highest)
```

当优先级是自动递增一级时使用"boosted"一词；当通过前缀规则配置了显式优先级时使用"set"。

### 前缀匹配细节

前缀匹配不区分大小写（对模块名和前缀均使用 `to_lowercase()`）。第一个匹配的前缀生效——配置文件中的顺序很重要。如果前缀指定了 `cpus: None`，则使用默认的 `config.prime_threads_cpus`。如果前缀指定了 `thread_priority: None`，则应用自动提升一级的逻辑。

当 `config.prime_threads_prefixes` 为空（无前缀规则）时，所有被选为主力的线程都使用 `config.prime_threads_cpus` 和自动提升优先级逻辑进行提升。

### CPU 亲和性掩码交互

`current_mask` 过滤步骤对正确性至关重要。如果进程通过 [apply_affinity](apply_affinity.md) 设置了排除某些已配置主力 CPU 的硬亲和性掩码，则这些 CPU 不得出现在 `SetThreadSelectedCpuSets` 调用中。没有此过滤器，API 调用会成功但线程实际上不会在指定的 CPU 上运行，可能导致令人困惑的行为。当 `*current_mask` 为 `0`（未查询亲和性或进程没有亲和性约束）时，过滤器被绕过，使用所有已配置的主力 CPU。

### 幂等性

该函数在单个周期内是幂等的：已固定的线程（`pinned_cpu_set_ids` 非空）会被跳过。跨周期来看，保持主力状态的线程不会被重新提升，因为其 `pinned_cpu_set_ids` 在 [ThreadStats](../scheduler.rs/ThreadStats.md) 中持续存在，直到被 [apply_prime_threads_demote](apply_prime_threads_demote.md) 清除。

## 要求

| 要求 | 值 |
|-------------|-------|
| 模块 | `apply` |
| 可见性 | `pub`（crate 公开） |
| 调用者 | [apply_prime_threads](apply_prime_threads.md) |
| 被调用者 | [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md), [filter_indices_by_mask](../winapi.rs/filter_indices_by_mask.md), [cpusetids_from_indices](../winapi.rs/cpusetids_from_indices.md), [indices_from_cpusetids](../winapi.rs/indices_from_cpusetids.md), [log_error_if_new](log_error_if_new.md) |
| Win32 API | [`SetThreadSelectedCpuSets`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadselectedcpusets), [`GetThreadPriority`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadpriority), [`SetThreadPriority`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadpriority) |
| 权限 | `THREAD_SET_LIMITED_INFORMATION`（用于 `SetThreadSelectedCpuSets`），`THREAD_SET_INFORMATION`（用于 `SetThreadPriority`），`THREAD_QUERY_LIMITED_INFORMATION`（用于 `GetThreadPriority`）。服务通常持有 `SeDebugPrivilege`，可授予所有这些权限。 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 主力线程编排 | [apply_prime_threads](apply_prime_threads.md) |
| 基于迟滞的选择 | [apply_prime_threads_select](apply_prime_threads_select.md) |
| 降级（逆向操作） | [apply_prime_threads_demote](apply_prime_threads_demote.md) |
| 周期时间预取（填充起始地址和句柄） | [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) |
| 前缀配置模型 | [PrimePrefix](../config.rs/PrimePrefix.md) |
| 线程优先级枚举和提升逻辑 | [ThreadPriority](../priority.rs/ThreadPriority.md) |
| CPU 集合 ID 转换 | [cpusetids_from_indices](../winapi.rs/cpusetids_from_indices.md) |
| CPU 亲和性掩码过滤 | [filter_indices_by_mask](../winapi.rs/filter_indices_by_mask.md) |
| 模块名解析 | [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md) |
| 调度器状态模型 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md), [ThreadStats](../scheduler.rs/ThreadStats.md) |
| apply 模块概述 | [apply](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd