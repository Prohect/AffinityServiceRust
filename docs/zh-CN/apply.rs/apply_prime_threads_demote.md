# apply_prime_threads_demote 函数 (apply.rs)

降级不再符合 prime 条件的线程，移除其 CPU 集绑定并恢复原始线程优先级。

## 语法

```rust
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

`pid`

目标进程的进程标识符。

`config`

指向 [ProcessConfig](../config.rs/ProcessConfig.md) 的引用，包含进程规则配置，用于错误日志上下文（`config.name`）。

`process`

指向目标进程 [ProcessEntry](../process.rs/ProcessEntry.md) 的可变引用。用于枚举当前存活的线程 ID 集合。

`tid_with_delta_cycles`

由 [apply_prime_threads_select](apply_prime_threads_select.md) 生成的 `(tid, delta_cycles, is_prime)` 元组切片。`is_prime` 标志指示哪些线程被选为 prime 状态；**不在**此集合中但仍携带已绑定 CPU 集 ID 的线程是降级候选者。

`prime_core_scheduler`

指向 [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) 的可变引用。线程统计信息将被更新以清除 `pinned_cpu_set_ids`，并消费已保存的 `original_priority` 用于恢复。

`apply_config_result`

指向 [ApplyConfigResult](ApplyConfigResult.md) 的可变引用，用于收集更改消息和错误信息。

## 返回值

此函数无返回值。结果通过 `apply_config_result` 记录。

## 备注

此函数是 [apply_prime_threads_promote](apply_prime_threads_promote.md) 的对应操作，作为 [apply_prime_threads](apply_prime_threads.md) 编排流程的最终步骤被调用。

### 降级算法

1. 从 `tid_with_delta_cycles` 中构建一个当前标记为 prime 的线程 ID `HashSet`。
2. 从 `process.get_threads()` 收集所有存活线程 ID。
3. 对于每个**不在** prime 集合中**且** `pinned_cpu_set_ids` 非空的存活线程：
   - 通过传入空切片调用 `SetThreadSelectedCpuSets` 清除线程的 CPU 集分配。
   - 成功时记录更改消息：`"Thread {tid} -> (demoted, start={module})"`。
   - 失败时通过 [log_error_if_new](log_error_if_new.md) 记录错误。
   - **始终**清除 `pinned_cpu_set_ids`，无论成功与否，以防止无限重试循环导致日志泛滥。
4. 如果线程保存了 `original_priority`（在提升期间设置），则通过 `SetThreadPriority` 恢复。恢复失败时记录错误。

### 防御性清除

在尝试降级后，`pinned_cpu_set_ids` 向量会被**无条件**清除——即使 `SetThreadSelectedCpuSets` 调用失败。这是一个刻意的设计选择，旨在避免持续性 API 错误导致同一线程在每次循环迭代中被反复重试，从而淹没错误日志。

### 更改消息

| 事件 | 格式 |
| --- | --- |
| CPU 集已清除 | `Thread {tid} -> (demoted, start={module})` |
| 优先级恢复失败 | 通过 [log_error_if_new](log_error_if_new.md) 记录，操作为 `SetThreadPriority` |
| CPU 集清除失败 | 通过 [log_error_if_new](log_error_if_new.md) 记录，操作为 `SetThreadSelectedCpuSets` |

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | `src/apply.rs` |
| **行号** | L962–L1053 |
| **调用者** | [apply_prime_threads](apply_prime_threads.md) |
| **调用** | [log_error_if_new](log_error_if_new.md)、[resolve_address_to_module](../winapi.rs/resolve_address_to_module.md) |
| **Windows API** | `SetThreadSelectedCpuSets`、`SetThreadPriority` |

## 另请参阅

- [apply_prime_threads](apply_prime_threads.md) — 调用此函数的编排器
- [apply_prime_threads_promote](apply_prime_threads_promote.md) — 将线程提升为 prime 状态
- [apply_prime_threads_select](apply_prime_threads_select.md) — 选择符合 prime 条件的线程
- [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) — 管理每线程跟踪状态