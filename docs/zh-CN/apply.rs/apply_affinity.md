# apply_affinity 函数 (apply.rs)

设置进程的硬 CPU 亲和性掩码。副作用：将进程当前的亲和性掩码填充到调用者的 `current_mask` 中，该值供下游 prime 线程逻辑使用。

## 语法

```rust
pub fn apply_affinity(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    current_mask: &mut usize,
    process_handle: &ProcessHandle,
    process: &mut ProcessEntry,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

`pid`

目标进程的进程标识符。

`config`

指向 [ProcessConfig](../config.rs/ProcessConfig.md) 的引用，包含所需的 `affinity_cpus` 和 `prime_threads_cpus` 设置。

`dry_run`

为 `true` 时，仅记录将要执行的更改，不调用任何 Windows API。

`current_mask`

指向 `usize` 的可变引用，用于接收进程当前的亲和性掩码。该值将被后续函数（如 [apply_prime_threads](apply_prime_threads.md)）读取，以便将 CPU 集与实际亲和性进行过滤。成功设置操作后，掩码会被更新为新值。

`process_handle`

指向 [ProcessHandle](../winapi.rs/ProcessHandle.md) 的引用，通过 [get_handles](get_handles.md) 从中提取读写 `HANDLE`。

`process`

指向目标进程 [ProcessEntry](../process.rs/ProcessEntry.md) 的可变引用。在亲和性掩码更改时，传递给 [reset_thread_ideal_processors](reset_thread_ideal_processors.md)。

`apply_config_result`

指向 [ApplyConfigResult](ApplyConfigResult.md) 的可变引用，用于累积更改描述和错误信息。

## 返回值

此函数没有返回值。结果通过 `apply_config_result` 记录，`current_mask` 作为副作用被更新。

## 备注

仅当 `config.affinity_cpus` 或 `config.prime_threads_cpus` 非空时，函数才会执行操作，因为这两个功能都需要当前的亲和性掩码。

亲和性掩码通过 `cpu_indices_to_mask()` 从 CPU 索引列表转换而来。仅当计算得到的掩码与当前掩码不同且掩码非零时，才执行设置操作。

**后续操作：** 成功更改亲和性掩码后，函数会立即调用 [reset_thread_ideal_processors](reset_thread_ideal_processors.md)，传入 `&config.affinity_cpus`，将线程的理想处理器重新分配到新的 CPU 集合上。这可以防止线程滞留在不再属于亲和性掩码的 CPU 上。

**更改日志：** `"Affinity: {old:#X} -> {new:#X}"`

**错误去重：** 所有错误均通过 [log_error_if_new](log_error_if_new.md) 报告，因此同一 PID 和操作的相同错误只会记录一次。

### 执行流程

1. 通过 [get_handles](get_handles.md) 提取读写句柄；如果任一句柄为 `None` 则提前返回。
2. 检查是否配置了 `affinity_cpus` 或 `prime_threads_cpus`。
3. 调用 `GetProcessAffinityMask` 将当前掩码读入 `current_mask`。
4. 如果目标掩码与当前掩码不同：
   - **Dry run 模式：** 记录更改信息。
   - **实际运行：** 调用 `SetProcessAffinityMask`。成功后更新 `current_mask` 并调用 [reset_thread_ideal_processors](reset_thread_ideal_processors.md)。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/apply.rs |
| **行号** | L131–L208 |
| **调用者** | [apply_config](../main.rs/apply_config.md) |
| **调用** | [get_handles](get_handles.md)、[log_error_if_new](log_error_if_new.md)、[reset_thread_ideal_processors](reset_thread_ideal_processors.md) |
| **Windows API** | `GetProcessAffinityMask`、`SetProcessAffinityMask` |
| **配置字段** | `affinity_cpus`、`prime_threads_cpus` |

## 另请参阅

- [apply_process_default_cpuset](apply_process_default_cpuset.md) — 通过 CPU 集实现的软 CPU 偏好
- [apply_prime_threads](apply_prime_threads.md) — 使用本函数填充的 `current_mask`
- [reset_thread_ideal_processors](reset_thread_ideal_processors.md) — 亲和性更改后重新分配理想处理器