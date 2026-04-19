# reset_thread_ideal_processors 函数 (apply.rs)

在亲和性或 CPU 集合更改后，重置进程中所有线程的理想处理器分配。通过按 CPU 时间（降序）对线程排序并使用随机偏移进行轮询分配，将线程分发到新的一组 CPU 上。

## 语法

```AffinityServiceRust/src/apply.rs#L219-226
pub fn reset_thread_ideal_processors<'a>(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    cpus: &[u32],
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

`pid: u32`

目标进程的进程标识符。

`config: &ProcessLevelConfig`

匹配进程的 [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md)。用于日志消息中的 `name` 字段，并用于打开线程句柄。

`dry_run: bool`

当为 `true` 时，将预期更改记录到 `apply_config_result` 中，但不调用任何 Windows API。

`cpus: &[u32]`

要分配线程理想处理器的 CPU 索引集。调用方在亲和性更改后传递 `&config.affinity_cpus`，或在 CPU 集合更改后传递 `&config.cpu_set_cpus`（当设置 `cpu_set_reset_ideal` 时）。

`threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>`

返回进程线程 ID 到 `SYSTEM_THREAD_INFORMATION` 映射的惰性访问器。

`apply_config_result: &mut ApplyConfigResult`

记录更改和错误的累加器。见 [ApplyConfigResult](ApplyConfigResult.md)。

## 返回值

此函数不返回值。结果累积在 `apply_config_result` 中。

## 说明

当 Windows 更改进程的亲和性掩码时，线程理想处理器分配可能变得过时——线程可能保留指向不再在亲和性集中的 CPU 的理想处理器提示。此函数通过将理想处理器重新分配到新的 CPU 集合来纠正此问题。

### 算法

1. **提前退出** — 如果 `cpus` 为空，立即返回。在试运行模式下，记录总结性更改消息并返回。
2. **收集线程时间** — 遍历所有线程，将每个 TID 与其总 CPU 时间（`KernelTime + UserTime`）配对。
3. **降序排序** — 按总 CPU 时间降序对线程排序，以便首先分配最繁忙的线程。
4. **随机偏移** — 生成随机 `u8` 移位值，以随机化 CPU 列表中的起始位置。这防止相同的 CPU 在连续调用中始终接收活动最高的线程。
5. **轮询分配** — 对于每个线程（按排序顺序）：
   - 通过 [`get_thread_handle`](../winapi.rs/get_thread_handle.md) 打开线程句柄。
   - 选择写句柄，优先使用 `w_handle` 而非 `w_limited_handle`。
   - 计算目标 CPU 为 `cpus[(success_count + random_shift) % cpu_count]`。
   - 调用 `SetThreadIdealProcessorEx`，处理器组为 0，目标 CPU 编号。
   - 失败时通过 [`log_error_if_new`](log_error_if_new.md) 记录日志。成功时，递增成功计数器。
6. **总结** — 记录单个更改条目：`"reset ideal processor for {N} threads"`。

### 平台说明

- `SetThreadIdealProcessorEx` 是对 Windows 调度程序的提示，而非硬约束。操作系统仍可能在其他 CPU 上调度线程。
- 所有分配使用处理器组 0。拥有超过 64 个逻辑处理器并跨越多个组的系统无法完全处理。
- 此函数打开的线程句柄在使用后立即丢弃（不缓存在 `PrimeThreadScheduler` 中）。

### 边界情况

- 如果无法打开任何线程（例如所有句柄都失败），总结消息报告 `"reset ideal processor for 0 threads"`。
- 随机偏移通过模运算包装，因此对于任何 `random::<u8>()` 值都是安全的。

## 要求

| | |
|---|---|
| **模块** | `src/apply.rs` |
| **调用方** | [apply_affinity](apply_affinity.md)（成功 `SetProcessAffinityMask` 后），[apply_process_default_cpuset](apply_process_default_cpuset.md)（当 `cpu_set_reset_ideal` 为 `true` 时） |
| **被调用方** | [`get_thread_handle`](../winapi.rs/get_thread_handle.md)，[`set_thread_ideal_processor_ex`](../winapi.rs/set_thread_ideal_processor_ex.md)，[`log_error_if_new`](log_error_if_new.md) |
| **Win32 API** | [SetThreadIdealProcessorEx](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadidealprocessorex) |
| **权限** | 每个线程上的 `THREAD_SET_INFORMATION`（或 `THREAD_SET_LIMITED_INFORMATION`） |

## 另请参阅

| | |
|---|---|
| [apply_affinity](apply_affinity.md) | 设置进程亲和性掩码；成功时调用此函数 |
| [apply_process_default_cpuset](apply_process_default_cpuset.md) | 设置进程默认 CPU 集合；可选择调用此函数 |
| [apply_ideal_processors](apply_ideal_processors.md) | 基于规则的线程级配置的理想处理器分配 |
| [ApplyConfigResult](ApplyConfigResult.md) | 更改和错误的累加器 |
| [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md) | 进程级配置结构 |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*