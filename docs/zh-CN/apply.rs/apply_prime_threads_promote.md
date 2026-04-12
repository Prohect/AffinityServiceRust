# apply_prime_threads_promote 函数 (apply.rs)

将选中的线程提升为 prime 状态，通过 CPU 集将其固定到专用 CPU，并可选地提升其线程优先级。

## 语法

```rust
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

`pid`

目标进程的进程标识符。

`config`

指向 [ProcessConfig](../config.rs/ProcessConfig.md) 的引用，包含 `prime_threads_cpus`、`prime_threads_prefixes` 及相关设置。

`current_mask`

指向进程当前 CPU 亲和性掩码的可变引用。用于过滤 prime CPU 索引，确保仅分配处于进程亲和性范围内的 CPU。

`tid_with_delta_cycles`

由 [apply_prime_threads_select](apply_prime_threads_select.md) 生成的 `(thread_id, delta_cycles, is_prime)` 元组切片。仅处理 `is_prime` 为 `true` 的条目。

`prime_core_scheduler`

指向 [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) 的可变引用，持有每线程状态，包括句柄、`pinned_cpu_set_ids`、`start_address` 和 `original_priority`。

`apply_config_result`

指向 [ApplyConfigResult](ApplyConfigResult.md) 的可变引用，用于收集更改和错误消息。

## 返回值

此函数不返回值。结果通过 `apply_config_result` 记录。

## 备注

### 算法

对每个标记为 prime（`is_prime == true`）且尚未固定（即 `pinned_cpu_set_ids` 为空）的线程：

1. **解析起始模块** — 通过 `resolve_address_to_module()` 将线程的起始地址解析为模块名称。

2. **匹配前缀规则** — 如果 `config.prime_threads_prefixes` 非空，则将起始模块与每个前缀条目进行不区分大小写的比较。匹配时：
   - 前缀专用的 CPU 列表覆盖 `config.prime_threads_cpus`。
   - 使用前缀显式指定的 `thread_priority` 代替自动提升。
   - 如果没有前缀匹配，该线程将被**跳过**（不提升）。

3. **按亲和性掩码过滤 CPU** — 如果 `current_mask` 非零，prime CPU 索引通过 `filter_indices_by_mask()` 过滤，确保仅分配进程亲和性范围内的 CPU。

4. **分配 CPU 集** — 调用 `SetThreadSelectedCpuSets`，传入从过滤后 CPU 索引转换得到的 CPU Set ID。成功后，将分配的 CPU Set ID 存储到 `thread_stats.pinned_cpu_set_ids`。

5. **提升线程优先级** — CPU 固定完成后：
   - 通过 `GetThreadPriority` 读取当前线程优先级。
   - 将其保存为 `thread_stats.original_priority`，供后续 [apply_prime_threads_demote](apply_prime_threads_demote.md) 恢复使用。
   - 如果前缀规则指定了显式 `thread_priority`，则直接设置该值（"priority set"）。
   - 否则，通过 `ThreadPriority::boost_one()` 提升一级（"priority boosted"）。
   - 仅在新值与当前值不同时才更改优先级。

### 更改日志消息

- `"Thread {tid} -> (promoted, [{cpus}], cycles={delta}, start={module})"` — CPU 集分配成功时。
- `"Thread {tid} -> (priority set: {old} -> {new})"` — 应用前缀显式优先级时。
- `"Thread {tid} -> (priority boosted: {old} -> {new})"` — 应用自动提升时。

### 错误处理

- 无效的线程句柄通过 [log_error_if_new](log_error_if_new.md) 以 `Operation::OpenThread` 记录。
- `SetThreadSelectedCpuSets` 失败以 `Operation::SetThreadSelectedCpuSets` 记录。
- `SetThreadPriority` 失败以 `Operation::SetThreadPriority` 记录。

### 与其他函数的交互

此函数由 [apply_prime_threads](apply_prime_threads.md) 在 [apply_prime_threads_select](apply_prime_threads_select.md) 标记 prime 线程之后调用。此处提升的线程后续将由 [apply_prime_threads_demote](apply_prime_threads_demote.md) 评估，当线程不再符合条件时清除固定并恢复原始优先级。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/apply.rs |
| **源码行** | L818–L960 |
| **调用者** | [apply_prime_threads](apply_prime_threads.md) |
| **调用** | [log_error_if_new](log_error_if_new.md)、`resolve_address_to_module`、`filter_indices_by_mask`、`cpusetids_from_indices`、`indices_from_cpusetids` |
| **Windows API** | `SetThreadSelectedCpuSets`、`GetThreadPriority`、`SetThreadPriority` |

## 另请参阅

- [apply_prime_threads](apply_prime_threads.md)
- [apply_prime_threads_select](apply_prime_threads_select.md)
- [apply_prime_threads_demote](apply_prime_threads_demote.md)
- [ApplyConfigResult](ApplyConfigResult.md)
- [ProcessConfig](../config.rs/ProcessConfig.md)
- [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md)