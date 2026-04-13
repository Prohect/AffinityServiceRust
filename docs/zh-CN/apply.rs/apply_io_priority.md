# apply_io_priority 函数 (apply.rs)

通过原生 `NtSetInformationProcess` API 并使用信息类 `ProcessIoPriority` (33) 设置进程的 IO 优先级。与使用已文档化 Win32 函数的进程优先级类和 CPU 亲和性不同，IO 优先级仅通过 NT 原生 API 暴露。该函数使用 `NtQueryInformationProcess` 查询当前 IO 优先级，将其与配置的目标值进行比较，并在两者不同时应用更改。

## 语法

```AffinityServiceRust/src/apply.rs#L420-428
pub fn apply_io_priority(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 目标进程的进程标识符。用于 [log_error_if_new](log_error_if_new.md) 中的错误去重和日志消息。 |
| `config` | `&`[ProcessConfig](../config.rs/ProcessConfig.md) | 此进程的已解析配置。`io_priority` 字段（一个 [IOPriority](../priority.rs/IOPriority.md) 枚举）决定期望的 IO 优先级。当 `io_priority` 为 `IOPriority::None` 时，`as_win_const()` 返回 `None`，函数立即退出，不进行任何查询或修改。 |
| `dry_run` | `bool` | 为 `true` 时，函数将预期的更改记录在 `apply_config_result` 中，但不调用 `NtSetInformationProcess`。 |
| `process_handle` | `&`[ProcessHandle](../winapi.rs/ProcessHandle.md) | 目标进程的 OS 句柄。通过 [get_handles](get_handles.md) 提取读取句柄（用于 `NtQueryInformationProcess`）和写入句柄（用于 `NtSetInformationProcess`）。 |
| `apply_config_result` | `&mut`[ApplyConfigResult](ApplyConfigResult.md) | 此操作期间产生的更改描述和错误消息的累加器。 |

## 返回值

无 (`()`)。结果通过 `apply_config_result` 传达。

## 备注

### 原生 API 用法

Windows 没有暴露已文档化的 Win32 函数来设置每进程 IO 优先级。因此，本函数直接调用 NT 原生 API：

- **`NtQueryInformationProcess`**，使用 `ProcessInformationClass = 33`（`PROCESS_INFORMATION_IO_PRIORITY`），将当前 IO 优先级作为原始 `u32` 读取。
- **`NtSetInformationProcess`**，使用相同的信息类写入新值。

两个函数都返回 `NTSTATUS` 值。负返回值表示失败，由 [error_from_ntstatus](../error_codes.rs/error_from_ntstatus.md) 格式化。原始 `NTSTATUS` `i32` 通过 `i32::cast_unsigned` 转换为 `u32`，以存储在错误去重映射中。

### 控制流

1. [get_handles](get_handles.md) 提取最佳可用的读取和写入 `HANDLE`。如果任一为 `None`，函数立即返回。
2. `config.io_priority.as_win_const()` 将 [IOPriority](../priority.rs/IOPriority.md) 枚举转换为其原始 `u32` 值。如果配置的优先级为 `None`，函数返回——不查询、不更改。
3. `NtQueryInformationProcess` 将当前 IO 优先级读入一个本地 `u32`。如果失败（负 `NTSTATUS`），错误通过 [log_error_if_new](log_error_if_new.md) 以 `Operation::NtQueryInformationProcess2ProcessInformationIOPriority` 记录，函数返回且不尝试设置。
4. 如果当前值已等于目标值，函数静默返回。
5. 在模拟运行模式下，记录更改消息后函数返回。
6. 否则，调用 `NtSetInformationProcess`。成功时记录更改；失败时 `NTSTATUS` 通过 [log_error_if_new](log_error_if_new.md) 以 `Operation::NtSetInformationProcess2ProcessInformationIOPriority` 记录。

### IO 优先级值

传递给原生 API 的原始 `u32` 值对应于 [IOPriority](../priority.rs/IOPriority.md) 枚举：

| IOPriority 变体 | 原始值 | 描述 |
|-----------------|--------|------|
| `VeryLow` | 0 | 后台 I/O——最低带宽分配。 |
| `Low` | 1 | 低于正常的 I/O 吞吐量。 |
| `Normal` | 2 | 大多数进程的默认 IO 优先级。 |
| `High` | 3 | 提升的 I/O 吞吐量。需要 `SeIncreaseBasePriorityPrivilege`。 |

### 更改消息格式

```/dev/null/example.txt#L1
IO Priority: Normal -> VeryLow
```

消息显示旧优先级和新优先级的可读名称，分别通过 `IOPriority::from_win_const()` 和 `IOPriority::as_str()` 获取。更改消息字符串在设置调用之前预先格式化，以捕获"之前"的状态；仅在设置成功时才将其推入 `apply_config_result`。

### 错误处理

错误使用 NTSTATUS 格式化而非 Win32 错误代码，因为原生 API 直接返回 NTSTATUS，而不是设置线程本地的 Win32 错误。常见失败包括：

| NTSTATUS | 典型原因 |
|----------|----------|
| `STATUS_ACCESS_DENIED` (0xC0000022) | 句柄缺少目标进程所需的访问权限。 |
| `STATUS_INVALID_HANDLE` (0xC0000008) | 进程已退出或句柄已过期。 |

### 幂等性

函数是幂等的：当当前 IO 优先级已与目标匹配时，不会执行原生 API 设置调用，也不会记录更改。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `apply` |
| 可见性 | `pub`（crate 公开） |
| 调用者 | [apply_config_process_level](../main.rs/apply_config_process_level.md) |
| 被调用者 | [get_handles](get_handles.md)、[log_error_if_new](log_error_if_new.md)、[error_from_ntstatus](../error_codes.rs/error_from_ntstatus.md) |
| 原生 API | `NtQueryInformationProcess`（`ProcessIoPriority`，类 33）、`NtSetInformationProcess`（`ProcessIoPriority`，类 33） |
| 权限 | `PROCESS_QUERY_INFORMATION`（读取）、`PROCESS_SET_INFORMATION`（写入）。设置 `High` IO 优先级可能需要 `SeIncreaseBasePriorityPrivilege`。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| IO 优先级枚举和值映射 | [IOPriority](../priority.rs/IOPriority.md) |
| 内存优先级（类似模式，已文档化的 Win32 API） | [apply_memory_priority](apply_memory_priority.md) |
| 进程优先级类 | [apply_priority](apply_priority.md) |
| NTSTATUS 错误格式化 | [error_from_ntstatus](../error_codes.rs/error_from_ntstatus.md) |
| 配置模型 | [ProcessConfig](../config.rs/ProcessConfig.md) |
| 句柄提取辅助函数 | [get_handles](get_handles.md) |
| apply 模块概览 | [apply](README.md) |