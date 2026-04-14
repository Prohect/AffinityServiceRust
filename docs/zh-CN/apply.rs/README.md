# apply 模块 (AffinityServiceRust)

`apply` 模块包含所有通过 Windows API 调用直接修改进程和线程属性的函数。它是 AffinityServiceRust 的执行层：一旦配置解析完成并完成进程枚举，所有对进程优先级类、CPU 亲和性掩码、CPU 集合、IO 优先级、内存优先级、理想处理器分配以及主力线程调度的更改，都通过此处定义的函数来执行。

每个公开函数都遵循一个通用模式：接受进程标识符、已解析的 [ProcessConfig](../config.rs/ProcessConfig.md) 引用、`dry_run` 标志、所需的操作系统句柄，以及一个 [ApplyConfigResult](#applyconfigresult) 累加器。函数从操作系统读取当前值，将其与期望值进行比较，并在两者不同且 `dry_run` 为 `false` 时调用相应的 Windows API 来应用更改。成功的操作被记录为*变更*；失败的操作被记录为*错误*（通过 [log_error_if_new](log_error_if_new.md) 辅助函数进行去重，避免对同一操作的重复失败刷屏日志）。

## 结构体

| 名称 | 描述 |
|------|------|
| [ApplyConfigResult](ApplyConfigResult.md) | 在单次应用过程中累积人类可读的变更描述和错误消息。 |

## 函数

| 名称 | 描述 |
|------|------|
| [get_handles](get_handles.md) | 从 [ProcessHandle](../winapi.rs/ProcessHandle.md) 中提取最佳可用的读写 `HANDLE`，优先使用完全访问权限的句柄。 |
| [log_error_if_new](log_error_if_new.md) | 仅在相同的 pid / 操作 / 错误码组合未被记录过时才记录错误消息。 |
| [apply_priority](apply_priority.md) | 通过 `SetPriorityClass` 设置进程优先级类（从 Idle 到 Realtime）。 |
| [apply_affinity](apply_affinity.md) | 通过 `SetProcessAffinityMask` 在进程上设置硬 CPU 亲和性掩码。 |
| [reset_thread_ideal_processors](reset_thread_ideal_processors.md) | 在亲和性或 CPU 集合变更后，重新分配线程的理想处理器。 |
| [apply_process_default_cpuset](apply_process_default_cpuset.md) | 通过 `SetProcessDefaultCpuSets` 为进程应用软 CPU 集合偏好。 |
| [apply_io_priority](apply_io_priority.md) | 通过 `NtSetInformationProcess` 设置进程的 IO 优先级。 |
| [apply_memory_priority](apply_memory_priority.md) | 通过 `SetProcessInformation(ProcessMemoryPriority)` 设置进程的内存优先级。 |
| [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) | 查询占用 CPU 最多的线程的周期计数，为主力线程选择建立基线。 |
| [apply_prime_threads](apply_prime_threads.md) | 主力线程调度管线的顶层编排器（选择 → 提升 → 降级）。 |
| [apply_prime_threads_select](apply_prime_threads_select.md) | 使用滞后阈值按 CPU 周期数选出前 *N* 个线程。 |
| [apply_prime_threads_promote](apply_prime_threads_promote.md) | 将新选中的主力线程固定到性能核心 CPU 集合，并可选地提升其优先级。 |
| [apply_prime_threads_demote](apply_prime_threads_demote.md) | 取消失去主力状态的线程的固定，并恢复其原始优先级。 |
| [apply_ideal_processors](apply_ideal_processors.md) | 根据可配置的前缀规则，为起始模块匹配的线程分配理想处理器。 |
| [update_thread_stats](update_thread_stats.md) | 提交缓存的周期和时间计数器，以便下一次迭代计算正确的增量。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 进程级应用编排 | [apply_config_process_level](../main.rs/apply_config_process_level.md) |
| 线程级应用编排 | [apply_config_thread_level](../main.rs/apply_config_thread_level.md) |
| 配置模型 | [ProcessConfig](../config.rs/ProcessConfig.md) |
| 主力线程调度器状态 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| 操作系统句柄封装 | [ProcessHandle](../winapi.rs/ProcessHandle.md)、[ThreadHandle](../winapi.rs/ThreadHandle.md) |
| 优先级枚举 | [ProcessPriority](../priority.rs/ProcessPriority.md)、[IOPriority](../priority.rs/IOPriority.md)、[MemoryPriority](../priority.rs/MemoryPriority.md)、[ThreadPriority](../priority.rs/ThreadPriority.md) |
| 错误格式化辅助函数 | [error_from_code_win32](../error_codes.rs/error_from_code_win32.md)、[error_from_ntstatus](../error_codes.rs/error_from_ntstatus.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd