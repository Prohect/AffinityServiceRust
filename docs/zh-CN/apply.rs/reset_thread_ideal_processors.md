# reset_thread_ideal_processors 函数 (apply.rs)

在亲和性或 CPU 集更改后，将线程的理想处理器重新分配到指定的 CPU 集合上，避免线程聚集在少数核心上。

## 语法

```rust
pub fn reset_thread_ideal_processors(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    cpus: &[u32],
    process: &mut ProcessEntry,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

`pid`

目标进程的进程标识符。

`config`

[ProcessConfig](../config.rs/ProcessConfig.md) 的引用，包含进程规则配置。用于错误消息中的进程名称以及打开线程句柄。

`dry_run`

为 `true` 时，记录预期的更改但不调用任何 Windows API。日志消息包含目标 CPU 数量。

`cpus`

要分配理想处理器的 CPU 索引切片。调用者在亲和性更改后传入 `&config.affinity_cpus`，或在 CPU 集更改后（当 `cpu_set_reset_ideal` 启用时）传入 `&config.cpu_set_cpus`。如果为空，函数立即返回。

`process`

目标进程的 [ProcessEntry](../process.rs/ProcessEntry.md) 可变引用。提供包含每线程计时信息的线程列表，用于排序。

`apply_config_result`

[ApplyConfigResult](ApplyConfigResult.md) 的可变引用，用于收集更改消息和错误消息。

## 返回值

此函数没有返回值。结果通过 `apply_config_result` 记录。

## 备注

### 算法

1. **收集线程** — 遍历进程快照中的所有线程，将每个线程 ID 与其总 CPU 时间（内核 + 用户）配对。
2. **按 CPU 时间排序** — 按总 CPU 时间降序排列线程，使最活跃的线程最先被分配。
3. **打开句柄** — 通过 [get_thread_handle](../winapi.rs/get_thread_handle.md) 为每个线程打开写句柄。优先使用完整访问写句柄（`w_handle`）；回退到受限写句柄（`w_limited_handle`）。
4. **轮询分配 + 随机偏移** — 使用以下公式从 `cpus` 切片中为每个线程分配理想处理器：

   ```
   target_cpu = cpus[(thread_index + random_shift) % cpus.len()]
   ```

   `random_shift` 是每次调用生成一次的随机 `u8` 值。这避免了始终将第一个（CPU 时间最高的）线程分配到同一核心，从而在多次调用间更均匀地分配负载。
5. **不使用延迟设置** — 与 [apply_ideal_processors](apply_ideal_processors.md) 不同，此函数*不会*在线程的当前理想处理器已在目标 CPU 上时跳过系统调用。每个线程都会被无条件重新分配。
6. **变更日志** — 完成后记录 `"reset ideal processor for {N} threads"`，其中 N 为成功分配的线程数。

### 句柄清理

线程句柄包装在 `ThreadHandle` 结构体中，其 `Drop` 实现会在函数结束时 `tid_handles` 超出作用域时自动关闭底层 OS 句柄。

### 调用者

- [apply_affinity](apply_affinity.md) — 在成功调用 `SetProcessAffinityMask` 后立即调用，传入 `&config.affinity_cpus`。
- [apply_process_default_cpuset](apply_process_default_cpuset.md) — 当 `config.cpu_set_reset_ideal` 为 `true` 时，在调用 `SetProcessDefaultCpuSets` 之前调用，传入 `&config.cpu_set_cpus`。
- [apply_config](../main.rs/apply_config.md)（main.rs） — 当设置了 `cpu_set_reset_ideal` 时，可能在 CPU 集更改后调用。

### 错误处理

如果线程句柄无效（`w_handle` 和 `w_limited_handle` 均无效），错误通过 [log_error_if_new](log_error_if_new.md) 以 `Operation::OpenThread` 记录，该线程被跳过。如果 `SetThreadIdealProcessorEx` 失败，错误以 `Operation::SetThreadIdealProcessorEx` 记录，该线程被跳过；剩余线程继续处理。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | `src/apply.rs`（L210–L313） |
| **调用者** | [apply_affinity](apply_affinity.md)、[apply_process_default_cpuset](apply_process_default_cpuset.md)、[apply_config](../main.rs/apply_config.md) |
| **Windows API** | `SetThreadIdealProcessorEx`（通过 [set_thread_ideal_processor_ex](../winapi.rs/set_thread_ideal_processor_ex.md)） |
| **相关** | [apply_ideal_processors](apply_ideal_processors.md)（基于规则的理想处理器分配） |

## 另请参阅

- [apply_affinity](apply_affinity.md)
- [apply_process_default_cpuset](apply_process_default_cpuset.md)
- [apply_ideal_processors](apply_ideal_processors.md)
- [ApplyConfigResult](ApplyConfigResult.md)