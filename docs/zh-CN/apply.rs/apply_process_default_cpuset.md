# apply_process_default_cpuset 函数 (apply.rs)

通过 Windows CPU 集 API 设置进程的默认 CPU 集，提供软 CPU 偏好。与硬亲和性掩码不同，CPU 集允许调度器在竞争情况下使用其他 CPU。

## 语法

```rust
pub fn apply_process_default_cpuset(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    process: &mut ProcessEntry,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

`pid`

目标进程的进程标识符。

`config`

指向 [ProcessConfig](../config.rs/ProcessConfig.md) 的引用，包含所需的 `cpu_set_cpus` 索引列表和 `cpu_set_reset_ideal` 标志。

`dry_run`

为 `true` 时，记录预期的更改但不调用任何 Windows API。

`process_handle`

指向 [ProcessHandle](../winapi.rs/ProcessHandle.md) 的引用，提供对目标进程的读写访问权限。

`process`

指向目标进程 [ProcessEntry](../process.rs/ProcessEntry.md) 的可变引用。当 `cpu_set_reset_ideal` 启用时，传递给 [reset_thread_ideal_processors](reset_thread_ideal_processors.md)。

`apply_config_result`

指向 [ApplyConfigResult](ApplyConfigResult.md) 的可变引用，用于收集更改消息和错误消息。

## 返回值

此函数不返回值。结果通过 `apply_config_result` 记录。

## 备注

函数执行流程如下：

1. **前置检查** — 如果 `config.cpu_set_cpus` 为空或系统 CPU 集信息不可用，立即返回。
2. **Dry run** — 如果 `dry_run` 为 `true`，记录目标 CPU 集的更改消息后返回。
3. **将 CPU 索引转换为 CPU Set ID** — 使用 `cpusetids_from_indices` 将 `config.cpu_set_cpus` 中的逻辑 CPU 索引转换为 Windows CPU Set ID。
4. **查询当前 CPU 集** — 首先以 `None` 缓冲区调用 `GetProcessDefaultCpuSets`：
   - 如果调用**成功**，说明进程尚未分配默认 CPU 集，需要执行设置操作。
   - 如果调用**失败且错误码为 122**（`ERROR_INSUFFICIENT_BUFFER`），这是预期路径——表示进程已分配了 CPU 集。返回的 `requiredidcount` 指示所需的缓冲区大小。第二次调用获取当前 CPU Set ID。然后将当前集合与目标集合进行比较；如果一致，则不执行任何操作。
   - 如果调用**以其他错误码失败**，通过 [log_error_if_new](log_error_if_new.md) 记录错误，函数继续执行但不设置。
5. **重置理想处理器** — 如果 `config.cpu_set_reset_ideal` 为 `true` 且需要执行设置操作，则在应用新 CPU 集**之前**调用 [reset_thread_ideal_processors](reset_thread_ideal_processors.md)，传入 `&config.cpu_set_cpus`。这会在新 CPU 集范围内重新分配线程理想处理器，避免聚集。
6. **应用新 CPU 集** — 调用 `SetProcessDefaultCpuSets` 设置目标 CPU Set ID。成功时记录显示新旧 CPU 索引的更改消息。失败时记录错误。

### 错误码 122 (ERROR_INSUFFICIENT_BUFFER)

首次以 `None` 缓冲区调用 `GetProcessDefaultCpuSets` 时，如果进程已分配默认 CPU 集，预期返回错误码 122。这是 Windows API 中返回可变长度数据的标准两次调用查询模式。此错误会被有意抑制，不会记录到日志中。

### 更改日志格式

```
CPU Set: [旧索引] -> [新索引]
```

例如：`CPU Set: [0,1,2,3] -> [4,5,6,7]`

首次设置时（无先前 CPU 集）：`CPU Set: [] -> [4,5,6,7]`

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/apply.rs |
| **行号** | L315–L418 |
| **调用者** | [apply_config](../main.rs/apply_config.md)（main.rs） |
| **调用** | [get_handles](get_handles.md)、[reset_thread_ideal_processors](reset_thread_ideal_processors.md)、[log_error_if_new](log_error_if_new.md) |
| **Windows API** | [GetProcessDefaultCpuSets](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessdefaultcpusets)、[SetProcessDefaultCpuSets](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessdefaultcpusets) |

## 另请参阅

- [apply_affinity](apply_affinity.md) — 通过硬亲和性掩码设置 CPU 绑定
- [reset_thread_ideal_processors](reset_thread_ideal_processors.md) — CPU 集更改后重新分配理想处理器
- [ProcessConfig](../config.rs/ProcessConfig.md)
- [ApplyConfigResult](ApplyConfigResult.md)