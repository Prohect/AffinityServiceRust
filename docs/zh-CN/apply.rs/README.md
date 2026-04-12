# apply.rs 模块 (apply.rs)

`apply` 模块实现了将进程配置设置应用到目标进程的核心逻辑。它是每次循环迭代中由 `main.rs` 中的 [`apply_config`](../main.rs/apply_config.md) 调用的主要执行引擎。

## 概述

本模块提供以下进程和线程属性的 Windows API 配置函数：

- **进程优先级类** — [`apply_priority`](apply_priority.md)
- **CPU 亲和性掩码**（硬绑定） — [`apply_affinity`](apply_affinity.md)
- **CPU 集**（软偏好） — [`apply_process_default_cpuset`](apply_process_default_cpuset.md)
- **I/O 优先级** — [`apply_io_priority`](apply_io_priority.md)
- **内存优先级** — [`apply_memory_priority`](apply_memory_priority.md)
- **Prime 线程调度**（将高负载线程钉到快速核心） — [`apply_prime_threads`](apply_prime_threads.md)
- **理想处理器分配**（按线程、基于模块） — [`apply_ideal_processors`](apply_ideal_processors.md)

所有函数将结果收集到 [`ApplyConfigResult`](ApplyConfigResult.md) 结构体中，该结构体累积人类可读的更改描述和错误消息，供调用者记录日志。

## 项目

### 结构体

| 名称 | 描述 |
| --- | --- |
| [ApplyConfigResult](ApplyConfigResult.md) | 收集配置应用过程中的更改和错误。 |

### 函数

| 名称 | 描述 |
| --- | --- |
| [get_handles](get_handles.md) | 从 [`ProcessHandle`](../winapi.rs/ProcessHandle.md) 中提取读写 `HANDLE`。 |
| [log_error_if_new](log_error_if_new.md) | 仅当对此 pid/tid/操作组合是新的错误时才记录。 |
| [apply_priority](apply_priority.md) | 设置进程优先级类。 |
| [apply_affinity](apply_affinity.md) | 设置进程的硬 CPU 亲和性掩码。 |
| [reset_thread_ideal_processors](reset_thread_ideal_processors.md) | 在亲和性或 CPU 集更改后，重新分配线程理想处理器到指定 CPU。 |
| [apply_process_default_cpuset](apply_process_default_cpuset.md) | 设置进程的默认 CPU 集（软 CPU 偏好）。 |
| [apply_io_priority](apply_io_priority.md) | 设置进程的 I/O 优先级。 |
| [apply_memory_priority](apply_memory_priority.md) | 设置进程的内存页面优先级。 |
| [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) | 为 prime 线程选择预取线程周期计数。 |
| [apply_prime_threads](apply_prime_threads.md) | Prime 线程调度的主编排函数。 |
| [apply_prime_threads_select](apply_prime_threads_select.md) | 使用滞后机制选择顶部线程获得 prime 状态。 |
| [apply_prime_threads_promote](apply_prime_threads_promote.md) | 将选中线程提升到 prime CPU 并提升优先级。 |
| [apply_prime_threads_demote](apply_prime_threads_demote.md) | 降级不再符合 prime 条件的线程。 |
| [apply_ideal_processors](apply_ideal_processors.md) | 根据起始模块前缀规则为线程分配理想处理器。 |
| [update_thread_stats](update_thread_stats.md) | 将缓存的周期和时间数据持久化，供下一次迭代的增量计算使用。 |

## 执行流程

单次循环迭代中的典型调用顺序（由 [`apply_config`](../main.rs/apply_config.md) 驱动）：

1. `apply_priority` — 设置进程优先级类
2. `apply_affinity` — 设置硬亲和性掩码（可能触发 `reset_thread_ideal_processors`）
3. `apply_process_default_cpuset` — 设置软 CPU 集偏好（当启用 `cpu_set_reset_ideal` 时可能触发 `reset_thread_ideal_processors`）
4. `apply_io_priority` — 设置 I/O 优先级
5. `apply_memory_priority` — 设置内存优先级
6. `prefetch_all_thread_cycles` — 采集线程周期基线
7. `apply_prime_threads` → `apply_prime_threads_select` → `apply_prime_threads_promote` → `apply_prime_threads_demote`
8. `apply_ideal_processors` — 基于模块的理想处理器分配
9. `update_thread_stats` — 持久化缓存数据供下次迭代使用

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | `src/apply.rs` |
| **调用者** | `src/main.rs` 中的 [`apply_config`](../main.rs/apply_config.md) |
| **关键依赖** | [`ProcessConfig`](../config.rs/ProcessConfig.md)、[`PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md)、[`ProcessEntry`](../process.rs/ProcessEntry.md)、[`ProcessHandle`](../winapi.rs/ProcessHandle.md) |