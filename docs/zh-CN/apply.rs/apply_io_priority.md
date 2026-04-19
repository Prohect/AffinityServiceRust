# apply_io_priority 函数 (apply.rs)

使用未公开的 `NtQueryInformationProcess` 和 `NtSetInformationProcess` 原生 API 以及信息类 `ProcessInformationClassIOPriority` (33) 获取和设置进程的 I/O 优先级。

## 语法

```AffinityServiceRust/src/apply.rs#L403-410
pub fn apply_io_priority(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

`pid`

目标进程的进程标识符。

`config`

[ProcessLevelConfig](../config.rs/ProcessLevelConfig.md) 的引用，其中包含所需的 `io_priority` 设置。如果 `io_priority` 为 `IOPriority::None`，函数将立即返回而不执行任何操作。

`dry_run`

如果为 **true**，函数会将计划对 [ApplyConfigResult](ApplyConfigResult.md) 所做的更改记录在案，但不会调用任何 Windows API 来修改状态。

`process_handle`

目标进程的 [ProcessHandle](../winapi.rs/ProcessHandle.md) 的引用。通过 [get_handles](get_handles.md) 提取读句柄（用于查询）和写句柄（用于设置）。

`apply_config_result`

对 [ApplyConfigResult](ApplyConfigResult.md) 的可变引用，用于累积更改描述和错误消息。

## 返回值

此函数不返回值。结果通过 `apply_config_result` 累加器进行通信。

## 备注

此函数使用 NT 原生 API 而不是文档化的 Win32 API，因为没有公开的 Win32 函数可以获取或设置每个进程的 I/O 优先级。

信息类常量 `PROCESS_INFORMATION_IO_PRIORITY` (值 **33**) 在函数体内局部定义。

### 查询阶段

通过调用 `NtQueryInformationProcess` 读取当前 I/O 优先级，使用 `u32` 输出缓冲区。检查 NTSTATUS 返回值：

- 如果为负数（失败），则通过 [log_error_if_new](log_error_if_new.md) 记录错误，操作为 `NtQueryInformationProcess2ProcessInformationIOPriority`，函数返回而不尝试设置。
- 如果为零或正数（成功），将当前值与配置的目标值进行比较。

### 设置阶段

如果当前 I/O 优先级与配置值不同：

- 在 **dry_run** 模式下，记录更改消息。
- 否则，调用 `NtSetInformationProcess` 并传入目标 I/O 优先级值。失败时，通过 [log_error_if_new](log_error_if_new.md) 记录 NTSTATUS 错误。成功时，以格式 `"IO Priority: {current} -> {target}"` 记录更改消息。

### I/O 优先级值

[IOPriority](../priority.rs/IOPriority.md) 枚举映射到 Windows `IO_PRIORITY_HINT` 值，由 NT 内核调度器使用：

| IOPriority | 值 | 效果 |
|---|---|---|
| VeryLow | 0 | 后台 I/O，最低调度优先级 |
| Low | 1 | 低于正常的 I/O 调度 |
| Normal | 2 | 默认 I/O 调度优先级 |

### 错误处理

查询和设置操作的错误都通过 [log_error_if_new](log_error_if_new.md) 进行去重，使用 `(pid, operation, error_code)` 键。NTSTATUS 代码通过 `error_from_ntstatus` 格式化为人类可读的错误消息。

### 句柄要求

读句柄需要 `PROCESS_QUERY_INFORMATION` 或 `PROCESS_QUERY_LIMITED_INFORMATION` 访问权限。写句柄需要 `PROCESS_SET_INFORMATION` 访问权限。如果任一句柄不可用，函数将提前返回。

## 要求

| | |
|---|---|
| **模块** | `apply` |
| **调用方** | 主应用循环（进程级强制） |
| **被调用方** | [get_handles](get_handles.md), [log_error_if_new](log_error_if_new.md), `NtQueryInformationProcess`, `NtSetInformationProcess` |
| **API** | NT 原生 API (`ntdll.dll`) |
| **特权** | 受保护进程可能需要 `SeDebugPrivilege` |

## 另请参阅

| 主题 | 描述 |
|---|---|
| [ApplyConfigResult](ApplyConfigResult.md) | 更改和错误的累加器 |
| [apply_memory_priority](apply_memory_priority.md) | 内存优先级辅助函数（使用文档化的 Win32 API） |
| [apply_priority](apply_priority.md) | 设置进程调度优先级类 |
| [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md) | 包含 `io_priority` 字段的配置结构 |
| [IOPriority](../priority.rs/IOPriority.md) | I/O 优先级级别枚举 |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*