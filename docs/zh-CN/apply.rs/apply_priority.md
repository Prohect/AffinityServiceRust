# apply_priority 函数 (apply.rs)

设置目标进程的优先级类。

## 语法

```rust
pub fn apply_priority(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

`pid`

目标进程的进程标识符。

`config`

指向 [ProcessConfig](../config.rs/ProcessConfig.md) 的引用，其中 `config.priority` 包含期望的优先级类。

`dry_run`

当为 `true` 时，函数仅记录将要执行的更改，而不调用任何 Windows API 来实际应用。

`process_handle`

指向 [ProcessHandle](../winapi.rs/ProcessHandle.md) 的引用，提供对目标进程的读写访问。需要读句柄（用于查询当前优先级）和写句柄（用于设置新优先级）。

`apply_config_result`

指向 [ApplyConfigResult](ApplyConfigResult.md) 的可变引用，用于收集操作过程中产生的更改描述和错误消息。

## 返回值

此函数不返回值。结果通过 `apply_config_result` 传递。

## 备注

函数首先通过 [get_handles](get_handles.md) 提取读写句柄。如果任一句柄不可用，函数立即返回，不执行任何操作。

若 `config.priority` 为 `ProcessPriority::None`，则不执行任何操作，因为 `as_win_const()` 返回 `None`。

函数使用 `GetPriorityClass` 查询当前优先级类，并与配置的目标值进行比较。如果两者已经匹配，则不进行任何更改。

**试运行模式：** 当 `dry_run` 为 `true` 时，更改消息会被记录，但不会调用 `SetPriorityClass`。

**更改日志：** `"Priority: {old} -> {new}"`，其中两个值均为人类可读的优先级名称（例如 `Normal`、`High`、`AboveNormal`）。

**错误处理：** 如果 `SetPriorityClass` 失败，Win32 错误码通过 `GetLastError` 获取，并传递给 [log_error_if_new](log_error_if_new.md) 进行去重错误记录。每个 pid/操作组合的错误仅记录一次。

### 优先级值

支持的优先级类定义在 [ProcessPriority](../priority.rs/ProcessPriority.md) 中：`Idle`、`BelowNormal`、`Normal`、`AboveNormal`、`High` 和 `Realtime`。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/apply.rs |
| **源码行** | L83–L129 |
| **调用者** | [apply_config](../main.rs/apply_config.md) |
| **调用** | [get_handles](get_handles.md)、[log_error_if_new](log_error_if_new.md) |
| **Windows API** | [GetPriorityClass](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getpriorityclass)、[SetPriorityClass](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setpriorityclass)、[GetLastError](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror) |

## 另请参阅

- [apply_affinity](apply_affinity.md)
- [apply_io_priority](apply_io_priority.md)
- [apply_memory_priority](apply_memory_priority.md)
- [ProcessPriority 枚举](../priority.rs/ProcessPriority.md)