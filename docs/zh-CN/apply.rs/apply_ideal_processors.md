# apply_ideal_processors 函数 (apply.rs)

根据线程的起始模块，使用按规则的前缀匹配和基于滞后的选择，为线程分配理想处理器。

## 语法

```rust
pub fn apply_ideal_processors(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process: &mut ProcessEntry,
    prime_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

`pid`

目标进程的进程标识符。

`config`

指向 [ProcessConfig](../config.rs/ProcessConfig.md) 的引用，其中包含 `ideal_processor_rules`，每条规则带有一组 CPU 列表和可选的模块前缀。

`dry_run`

当为 `true` 时，记录将要执行的更改但不调用任何 Windows API。

`process`

指向 [ProcessEntry](../process.rs/ProcessEntry.md) 的可变引用，用于枚举存活线程 ID。

`prime_scheduler`

指向 [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) 的可变引用，持有每线程的缓存周期计数、句柄、起始地址和 [IdealProcessorState](../scheduler.rs/IdealProcessorState.md) 跟踪状态。

`apply_config_result`

指向 [ApplyConfigResult](ApplyConfigResult.md) 的可变引用，用于收集更改消息和错误消息。

## 返回值

此函数无返回值。结果通过 `apply_config_result` 记录。

## 备注

此函数独立处理 `config.ideal_processor_rules` 中的每条 `IdealProcessorRule`。每条规则指定一组 CPU 和可选的模块前缀，用于匹配线程起始地址。

### 每条规则的算法

1. **按模块前缀过滤线程** — 对于每个具有非零缓存周期的线程，通过 `resolve_address_to_module` 将起始地址解析为模块名称。如果规则包含前缀列表，则仅纳入模块名称（不区分大小写）以其中某个前缀开头的线程。如果未指定前缀，则所有线程匹配。

2. **使用滞后选择前 N 个线程** — 调用 `select_top_threads_with_hysteresis`，N 等于规则中的 CPU 数量。滞后判定谓词检查 `thread_stats.ideal_processor.is_assigned` 以确定线程当前是否已被提升。这防止了分配/未分配状态之间的快速翻转。

3. **声明已分配的 CPU** — 已标记为 `is_assigned` 的线程将其 `current_number` 添加到已声明集合中。新选中但尚未分配的线程通过 `GetThreadIdealProcessorEx` 查询当前理想处理器；如果当前理想处理器恰好已在规则的 CPU 列表中，则直接声明（延迟设置优化——无需系统调用）。

4. **从空闲池分配** — 尚未被声明的 CPU 构成空闲池。每个仍需分配的新选中线程从空闲池中获取下一个 CPU，通过 `SetThreadIdealProcessorEx` 设置。成功后，线程的 `IdealProcessorState` 被更新，`is_assigned` 设为 `true`。

5. **恢复降级线程** — 先前已分配（`is_assigned == true`）但不再在选中集合中的线程，其理想处理器通过 `SetThreadIdealProcessorEx` 恢复为先前的值（`previous_group`、`previous_number`）。恢复后，`is_assigned` 清除为 `false`。

### 延迟设置优化

如果线程的当前理想处理器已在规则的 CPU 池内，函数将跳过 `SetThreadIdealProcessorEx` 系统调用，直接将线程标记为已分配。这避免了不必要的内核态切换。

### Dry run 行为

当 `dry_run` 为 `true` 时，为每条规则记录一条摘要消息，显示目标 CPU、数量和匹配的前缀，然后返回而不执行任何系统调用。

### 更改日志

- `"Thread {tid} -> ideal CPU {cpu} (group 0) start={module}"` — 成功分配时。
- `"Thread {tid} -> restored ideal CPU {number} (group {group}) start={module}"` — 成功降级/恢复时。

### 错误处理

来自 `GetThreadIdealProcessorEx` 和 `SetThreadIdealProcessorEx` 的错误通过 [log_error_if_new](log_error_if_new.md) 去重后记录到 `apply_config_result`。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | `src/apply.rs` |
| **行号** | L1055–L1325 |
| **调用者** | [apply_config](../main.rs/apply_config.md)（`main.rs`） |
| **依赖** | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md)、[ProcessConfig](../config.rs/ProcessConfig.md)、[ProcessEntry](../process.rs/ProcessEntry.md)、[ApplyConfigResult](ApplyConfigResult.md)、[log_error_if_new](log_error_if_new.md) |
| **Windows API** | [SetThreadIdealProcessorEx](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadidealprocessorex)、[GetThreadIdealProcessorEx](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadidealprocessorex) |

## 另请参阅

- [apply_affinity](apply_affinity.md) — 硬亲和性掩码分配
- [reset_thread_ideal_processors](reset_thread_ideal_processors.md) — 亲和性/CPU 集更改后的轮询理想处理器重分配
- [apply_prime_threads](apply_prime_threads.md) — 基于 CPU 集的线程固定（与理想处理器分配互补）