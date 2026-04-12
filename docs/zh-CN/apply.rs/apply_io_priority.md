# apply_io_priority 函数 (apply.rs)

设置目标进程的 I/O 优先级。使用未公开的 `NtQueryInformationProcess` 和 `NtSetInformationProcess` 原生 API 调用，信息类为 33（`ProcessInformationIoPriority`）。

## 语法

```rust
pub fn apply_io_priority(
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

指向 [ProcessConfig](../config.rs/ProcessConfig.md) 的引用，包含期望的 `io_priority` 设置。若 `config.io_priority` 为 `None`，函数立即返回，不执行任何操作。

`dry_run`

为 `true` 时，函数记录将会执行的更改，但不调用任何 Windows API 来修改进程。

`process_handle`

指向 [ProcessHandle](../winapi.rs/ProcessHandle.md) 的引用，包含目标进程的读写句柄。查询当前值需要读句柄，设置新值需要写句柄。

`apply_config_result`

指向 [ApplyConfigResult](ApplyConfigResult.md) 的可变引用，用于收集执行过程中产生的更改消息和错误消息。

## 返回值

此函数不返回值。结果通过 `apply_config_result` 传递。

## 备注

由于 Win32 API 没有公开的函数用于获取或设置每进程 I/O 优先级，因此本函数使用 NT 原生 API。

信息类常量 `PROCESS_INFORMATION_IO_PRIORITY`（33）同时用于查询和设置操作。当前 I/O 优先级通过 `NtQueryInformationProcess` 以原始 `u32` 形式读取，与配置中的目标值比较，若不同则通过 `NtSetInformationProcess` 设置。

**特权要求：** 将 I/O 优先级设置为 `High` 需要 `SeIncreaseBasePriorityPrivilege` 特权，且进程必须以管理员身份运行。缺少此特权时，`NtSetInformationProcess` 将返回 NTSTATUS 错误。

**错误处理：**

- 若 `NtQueryInformationProcess` 失败（返回负的 NTSTATUS），错误通过 [log_error_if_new](log_error_if_new.md) 记录，函数不会尝试设置新值。
- 若 `NtSetInformationProcess` 失败，错误通过 [log_error_if_new](log_error_if_new.md) 记录，NTSTATUS 码由 `error_from_ntstatus` 转换为可读消息。
- 每个 pid/操作组合的错误去重处理，相同进程的重复失败不会刷屏日志。

**更改日志：** `"IO Priority: {old} -> {new}"`，其中 old 和 new 为 [IOPriority](../priority.rs/IOPriority.md) 枚举的可读名称（如 `VeryLow`、`Low`、`Normal`、`High`）。

**Dry-run 行为：** 当 `dry_run` 为 `true` 且当前 I/O 优先级与目标不同时，记录更改消息但不调用 `NtSetInformationProcess`。

### I/O 优先级值

| IOPriority 变体 | Win32 值 |
| --- | --- |
| VeryLow | 0 |
| Low | 1 |
| Normal | 2 |
| High | 3 |

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/apply.rs |
| **源码行** | L420–L506 |
| **调用者** | [apply_config](../main.rs/apply_config.md) |
| **调用** | [get_handles](get_handles.md)、[log_error_if_new](log_error_if_new.md) |
| **Windows API** | `NtQueryInformationProcess`、`NtSetInformationProcess`（class 33） |
| **特权** | `SeIncreaseBasePriorityPrivilege`（设置 High I/O 优先级时需要） |

## 另请参阅

- [apply_memory_priority](apply_memory_priority.md)
- [apply_priority](apply_priority.md)
- [ProcessConfig](../config.rs/ProcessConfig.md)
- [IOPriority](../priority.rs/IOPriority.md)