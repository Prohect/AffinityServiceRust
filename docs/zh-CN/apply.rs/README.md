# apply 模块 (AffinityServiceRust)

`apply` 模块负责将进程级和线程级配置策略应用到正在运行的 Windows 进程上。它提供了一系列函数，用于读取当前进程/线程的属性（优先级、亲和性、CPU 集合、I/O 优先级、内存优先级），将其与期望的配置进行比较，并调用相应的 Windows API 使进程达到合规状态。该模块还实现了"主力线程"调度算法，该算法识别进程中 CPU 占用最高的线程，并使用 CPU Sets 和理想处理器分配将它们固定到指定的高性能核心上，同时采用基于滞后的提升和降级机制以避免快速振荡。

## 函数

| 函数 | 描述 |
|------|------|
| [`get_handles`](get_handles.md) | 从 `ProcessHandle` 中提取读取和写入 `HANDLE` 值，优先使用完全访问句柄而非受限句柄。 |
| [`log_error_if_new`](log_error_if_new.md) | 仅当相同的 pid/操作/错误码组合尚未记录过时，才将错误记录到 `ApplyConfigResult` 中。 |
| [`apply_priority`](apply_priority.md) | 读取当前进程优先级类，如果与配置值不同则进行设置。 |
| [`apply_affinity`](apply_affinity.md) | 读取当前进程亲和性掩码并将其设置为配置的 CPU 掩码，在更改时重新分配线程理想处理器。 |
| [`reset_thread_ideal_processors`](reset_thread_ideal_processors.md) | 在亲和性或 CPU 集合更改后，按 CPU 时间排序并以随机偏移进行轮转分配，将线程理想处理器重新分配到一组 CPU 上。 |
| [`apply_process_default_cpuset`](apply_process_default_cpuset.md) | 查询并设置进程的默认 CPU Set ID，之后可选择性地重置线程理想处理器。 |
| [`apply_io_priority`](apply_io_priority.md) | 通过 `NtQueryInformationProcess` 读取当前进程 I/O 优先级，并通过 `NtSetInformationProcess` 将其设置为配置值。 |
| [`apply_memory_priority`](apply_memory_priority.md) | 通过 `GetProcessInformation` 读取当前进程内存优先级，并通过 `SetProcessInformation` 将其设置为配置值。 |
| [`prefetch_all_thread_cycles`](prefetch_all_thread_cycles.md) | 打开 CPU 消耗最高的线程句柄并查询其周期计数器，为主力线程选择建立基线测量值。 |
| [`apply_prime_threads`](apply_prime_threads.md) | 编排主力线程调度管线：按 CPU 增量排序线程、选择候选者、提升获胜者、降级落选者。 |
| [`apply_prime_threads_select`](apply_prime_threads_select.md) | 使用滞后阈值选择排名靠前的线程作为主力线程，以防止快速翻转。 |
| [`apply_prime_threads_promote`](apply_prime_threads_promote.md) | 通过 `SetThreadSelectedCpuSets` 将新选定的主力线程固定到指定 CPU，并可选择性地提升其优先级。 |
| [`apply_prime_threads_demote`](apply_prime_threads_demote.md) | 移除不再符合主力条件的线程的 CPU 集合固定，并恢复其原始线程优先级。 |
| [`apply_ideal_processors`](apply_ideal_processors.md) | 根据模块前缀匹配规则为线程分配理想处理器，按每条规则的周期计数选择前 N 个线程。 |
| [`update_thread_stats`](update_thread_stats.md) | 将缓存的周期和时间测量值提交到 `last_cycles`/`last_total_time` 中，并将缓存值重置为零。 |

## 结构体

| 结构体 | 描述 |
|--------|------|
| [`ApplyConfigResult`](ApplyConfigResult.md) | 在单次配置应用过程中累积人类可读的变更描述和错误消息。 |

## 另请参阅

| 参考 | 链接 |
|------|------|
| config 模块 | [`config.rs`](../config.rs/README.md) |
| priority 模块 | [`priority.rs`](../priority.rs/README.md) |
| process 模块 | [`process.rs`](../process.rs/README.md) |
| scheduler 模块 | [`scheduler.rs`](../scheduler.rs/README.md) |
| winapi 模块 | [`winapi.rs`](../winapi.rs/README.md) |
| logging 模块 | [`logging.rs`](../logging.rs/README.md) |
| error_codes 模块 | [`error_codes.rs`](../error_codes.rs/README.md) |
| collections 模块 | [`collections.rs`](../collections.rs/README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*