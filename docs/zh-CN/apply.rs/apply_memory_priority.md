# apply_memory_priority 函数 (apply.rs)

设置目标进程的内存页面优先级。内存优先级影响操作系统在内存压力下回收进程页面的速度——优先级较低的页面会被优先回收。

## 语法

```rust
pub fn apply_memory_priority(
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

指向 [ProcessConfig](../config.rs/ProcessConfig.md) 的引用，包含所需的 `memory_priority` 设置。若 `config.memory_priority` 为 `None`，函数将立即返回，不执行任何操作。

`dry_run`

为 `true` 时，函数将在 `apply_config_result` 中记录*将要*进行的更改，但不会调用任何 Windows API 来修改进程。

`process_handle`

指向 [ProcessHandle](../winapi.rs/ProcessHandle.md) 的引用，提供对目标进程的读写访问。需要读句柄（用于查询）和写句柄（用于设置）；若任一句柄不可用，函数将立即返回。

`apply_config_result`

指向 [ApplyConfigResult](ApplyConfigResult.md) 的可变引用，用于收集更改描述和错误消息。

## 返回值

此函数无返回值。结果通过 `apply_config_result` 报告。

## 备注

该函数使用 `GetProcessInformation` 和 `SetProcessInformation` API，配合 `ProcessMemoryPriority` 信息类以及 `MEMORY_PRIORITY_INFORMATION` 结构体（在代码库中封装为 `MemoryPriorityInformation(u32)`）。

### 工作流程

1. 调用 [get_handles](get_handles.md) 从 `process_handle` 中提取读写 `HANDLE` 值。若任一句柄缺失则提前返回。
2. 检查 `config.memory_priority` 是否通过 `as_win_const()` 解析为有效的 Windows 常量。若为 `None`，函数不执行任何操作直接退出。
3. 使用 `GetProcessInformation(r_handle, ProcessMemoryPriority, ...)` 查询当前内存优先级。
   - 若查询失败，通过 [log_error_if_new](log_error_if_new.md) 使用 `Operation::GetProcessInformation2ProcessMemoryPriority` 记录错误并返回。
4. 将当前内存优先级值与目标值进行比较。若两者相等，则不执行任何操作。
5. 若 `dry_run` 为 `true`，记录预期更改并返回。
6. 否则，调用 `SetProcessInformation(w_handle, ProcessMemoryPriority, ...)` 设置目标优先级。
   - 若设置失败，通过 [log_error_if_new](log_error_if_new.md) 使用 `Operation::SetProcessInformation2ProcessMemoryPriority` 记录错误。
   - 若设置成功，记录更改。

### 更改日志格式

```
Memory Priority: {old} -> {new}
```

其中 `{old}` 和 `{new}` 为 `MemoryPriority` 枚举变体的可读名称（例如 `Normal`、`Low`、`VeryLow`）。

### 有效内存优先级级别

| 级别 | 说明 |
| --- | --- |
| **VeryLow** | 内存压力下最先被回收的页面 |
| **Low** | 低优先级页面 |
| **Medium** | 中等优先级页面 |
| **BelowNormal** | 低于默认优先级 |
| **Normal** | 默认内存优先级 |

完整枚举定义参见 [MemoryPriority](../priority.rs/MemoryPriority.md)。

### 错误处理

所有错误均通过 [log_error_if_new](log_error_if_new.md) 进行去重，该函数使用 `logging::is_new_error()` 确保每个唯一的 pid/操作/错误码组合仅记录一次。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | `src/apply.rs` |
| **行号** | L508–L595 |
| **调用者** | [apply_config](../main.rs/apply_config.md)（`main.rs`） |
| **调用** | [get_handles](get_handles.md)、[log_error_if_new](log_error_if_new.md) |
| **Windows API** | `GetProcessInformation` (`ProcessMemoryPriority`)、`SetProcessInformation` (`ProcessMemoryPriority`) |

## 另请参阅

- [ApplyConfigResult](ApplyConfigResult.md)
- [apply_priority](apply_priority.md)
- [apply_io_priority](apply_io_priority.md)
- [ProcessConfig](../config.rs/ProcessConfig.md)
- [MemoryPriority](../priority.rs/MemoryPriority.md)