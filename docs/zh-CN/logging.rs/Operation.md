# Operation 枚举 (logging.rs)

列出 AffinityServiceRust 可能对目标进程或线程调用的每个 Windows API 操作。用作 [ApplyFailEntry](ApplyFailEntry.md) 去重键的组成部分，以便独立跟踪同一进程上的不同失败。

## 语法

```rust
#[derive(PartialEq, Eq, Hash)]
pub enum Operation {
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

| 变体 | 描述 |
|---------|-------------|
| `OpenProcess2processQueryLimitedInformation` | 使用 `PROCESS_QUERY_LIMITED_INFORMATION` 访问权限调用 `OpenProcess`。 |
| `OpenProcess2processSetLimitedInformation` | 使用 `PROCESS_SET_LIMITED_INFORMATION` 访问权限调用 `OpenProcess`。 |
| `OpenProcess2processQueryInformation` | 使用 `PROCESS_QUERY_INFORMATION` 访问权限调用 `OpenProcess`。 |
| `OpenProcess2processSetInformation` | 使用 `PROCESS_SET_INFORMATION` 访问权限调用 `OpenProcess`。 |
| `OpenThread` | 用于线程级别操作的 `OpenThread`。 |
| `SetPriorityClass` | [SetPriorityClass](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-setpriorityclass) — 设置进程优先级类。 |
| `GetProcessAffinityMask` | [GetProcessAffinityMask](https://learn.microsoft.com/zh-cn/windows/win32/api/winbase/nf-winbase-getprocessaffinitymask) — 查询进程 CPU 亲和性。 |
| `SetProcessAffinityMask` | [SetProcessAffinityMask](https://learn.microsoft.com/zh-cn/windows/win32/api/winbase/nf-winbase-setprocessaffinitymask) — 设置进程 CPU 亲和性。 |
| `GetProcessDefaultCpuSets` | [GetProcessDefaultCpuSets](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessdefaultcpusets) — 查询进程默认 CPU 集合。 |
| `SetProcessDefaultCpuSets` | [SetProcessDefaultCpuSets](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessdefaultcpusets) — 设置进程默认 CPU 集合。 |
| `QueryThreadCycleTime` | [QueryThreadCycleTime](https://learn.microsoft.com/zh-cn/windows/win32/api/realtimeapiset/nf-realtimeapiset-querythreadcycletime) — 读取线程周期计数器以选择 Prime 线程。 |
| `SetThreadSelectedCpuSets` | [SetThreadSelectedCpuSets](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadselectedcpusets) — 将线程绑定到特定的 CPU 集合。 |
| `SetThreadPriority` | [SetThreadPriority](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadpriority) — 设置线程优先级级别。 |
| `NtQueryInformationProcess2ProcessInformationIOPriority` | `NtQueryInformationProcess` 带 `ProcessIoPriority` 信息类 — 读取 IO 优先级。 |
| `NtSetInformationProcess2ProcessInformationIOPriority` | `NtSetInformationProcess` 带 `ProcessIoPriority` 信息类 — 设置 IO 优先级。 |
| `GetProcessInformation2ProcessMemoryPriority` | [GetProcessInformation](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessinformation) 带 `ProcessMemoryPriority` 类。 |
| `SetProcessInformation2ProcessMemoryPriority` | [SetProcessInformation](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessinformation) 带 `ProcessMemoryPriority` 类。 |
| `SetThreadIdealProcessorEx` | [SetThreadIdealProcessorEx](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadidealprocessorex) — 为线程设置理想处理器提示。 |
| `GetThreadIdealProcessorEx` | [GetThreadIdealProcessorEx](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadidealprocessorex) — 查询线程的理想处理器提示。 |
| `InvalidHandle` | 指示所需句柄不可用的哨兵值。 |

## 备注

- 命名约定 `Verb2context`（例如，`OpenProcess2processQueryLimitedInformation`）编码了 Win32 函数名以及请求的访问权限或信息类。这允许单个枚举区分使用不同参数调用同一 API 的情况。
- 该枚举派生 `PartialEq`、`Eq` 和 `Hash`，以便可以在 [ApplyFailEntry](ApplyFailEntry.md) 内部用作键，并存储在 `HashMap`/`HashSet` 集合中。
- `InvalidHandle` 用于在 API 调用之前就发生失败的情况 — 例如，当 [ProcessHandle](../winapi.rs/ProcessHandle.md) 不携带所需的访问级别时。

## 需求

| | |
|---|---|
| **模块** | `src/logging.rs` |
| **调用方** | [log_error_if_new](../apply.rs/log_error_if_new.md)，[apply.rs](../apply.rs/README.md) 中的所有 `apply_*` 函数 |
| **依赖项** | 无（无字段枚举） |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 模块概览 | [logging.rs](README.md) |
| 错误去重键 | [ApplyFailEntry](ApplyFailEntry.md) |
| 首次出现检查 | [is_new_error](is_new_error.md) |
| Apply 模块 | [apply.rs](../apply.rs/README.md) |

*记录于提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*