# Operation 枚举 (logging.rs)

枚举所有在配置应用期间可能产生错误的 Windows API 操作，用于错误去重跟踪。每个变体对应一个特定的系统调用或句柄操作，使 [`is_new_error`](is_new_error.md) 能够区分同一进程上不同操作产生的不同错误。

## 语法

```rust
enum Operation {
    OpenProcess2processQueryLimitedInformation,
    OpenProcess2processSetLimitedInformation,
    OpenProcess2processQueryInformation,
    OpenProcess2processSetInformation,
    OpenThread,
    SetPriorityClass,
    GetProcessAffinityMask,
    SetProcessAffinityMask,
    GetProcessDefaultCpuSets,
    SetProcessDefaultCpuSets,
    QueryThreadCycleTime,
    SetThreadSelectedCpuSets,
    SetThreadPriority,
    NtQueryInformationProcess2ProcessInformationIOPriority,
    NtSetInformationProcess2ProcessInformationIOPriority,
    GetProcessInformation2ProcessMemoryPriority,
    SetProcessInformation2ProcessMemoryPriority,
    SetThreadIdealProcessorEx,
    GetThreadIdealProcessorEx,
    InvalidHandle,
}
```

## 成员

| 变体 | 说明 |
| --- | --- |
| `OpenProcess2processQueryLimitedInformation` | 以 `PROCESS_QUERY_LIMITED_INFORMATION` 权限打开进程句柄。 |
| `OpenProcess2processSetLimitedInformation` | 以 `PROCESS_SET_LIMITED_INFORMATION` 权限打开进程句柄。 |
| `OpenProcess2processQueryInformation` | 以 `PROCESS_QUERY_INFORMATION` 权限打开进程句柄。 |
| `OpenProcess2processSetInformation` | 以 `PROCESS_SET_INFORMATION` 权限打开进程句柄。 |
| `OpenThread` | 打开线程句柄。 |
| `SetPriorityClass` | 设置进程优先级类别。 |
| `GetProcessAffinityMask` | 查询进程亲和性掩码。 |
| `SetProcessAffinityMask` | 设置进程亲和性掩码。 |
| `GetProcessDefaultCpuSets` | 查询进程默认 CPU 集合。 |
| `SetProcessDefaultCpuSets` | 设置进程默认 CPU 集合。 |
| `QueryThreadCycleTime` | 查询线程周期时间。 |
| `SetThreadSelectedCpuSets` | 设置线程选定的 CPU 集合。 |
| `SetThreadPriority` | 设置线程优先级。 |
| `NtQueryInformationProcess2ProcessInformationIOPriority` | 通过 `NtQueryInformationProcess` 查询 I/O 优先级。 |
| `NtSetInformationProcess2ProcessInformationIOPriority` | 通过 `NtSetInformationProcess` 设置 I/O 优先级。 |
| `GetProcessInformation2ProcessMemoryPriority` | 查询进程内存优先级。 |
| `SetProcessInformation2ProcessMemoryPriority` | 设置进程内存优先级。 |
| `SetThreadIdealProcessorEx` | 设置线程理想处理器。 |
| `GetThreadIdealProcessorEx` | 查询线程理想处理器。 |
| `InvalidHandle` | 表示无效句柄情况（例如句柄获取失败后的后续操作）。 |

## 备注

`Operation` 枚举共定义了 20 种操作变体，覆盖了 [`apply.rs`](../apply.rs/README.md) 模块在应用配置期间可能调用的所有 Windows API 操作。它作为 [`ApplyFailEntry`](ApplyFailEntry.md) 结构体的一个字段，与 `pid`、`tid`、`process_name` 和 `error_code` 一起构成错误去重的复合键。

### 命名约定

变体名称遵循 `操作名2参数` 的命名模式。例如 `OpenProcess2processQueryLimitedInformation` 表示调用 `OpenProcess` 时传入 `PROCESS_QUERY_LIMITED_INFORMATION` 访问权限。这种命名方式使得在日志输出中能够精确定位失败的具体 API 调用及其参数。

### 在错误去重中的作用

不同的操作可能对同一进程产生不同的错误。例如，一个进程可能允许查询亲和性掩码（`GetProcessAffinityMask`）但拒绝设置（`SetProcessAffinityMask`）。通过在去重键中包含 `Operation`，系统能够独立跟踪每种操作的错误状态，确保每种独特的失败情况都至少被记录一次。

### InvalidHandle

`InvalidHandle` 变体用于标记无效句柄的情况。当 [`get_process_handle`](../winapi.rs/get_process_handle.md) 或 [`get_thread_handle`](../winapi.rs/get_thread_handle.md) 返回无效句柄时，后续操作使用此变体记录错误。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/logging.rs |
| **源码行** | L74–L95 |
| **派生 trait** | `Eq`、`PartialEq`、`Hash`（用于作为 `HashMap` 键） |
| **使用方** | [`ApplyFailEntry`](ApplyFailEntry.md)、[`is_new_error`](is_new_error.md)、[`log_error_if_new`](../apply.rs/log_error_if_new.md) |

## 另请参阅

- [ApplyFailEntry 结构体](ApplyFailEntry.md)
- [is_new_error 函数](is_new_error.md)
- [PID_MAP_FAIL_ENTRY_SET 静态变量](PID_MAP_FAIL_ENTRY_SET.md)
- [logging.rs 模块概述](README.md)