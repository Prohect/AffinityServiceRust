# Operation 枚举 (logging.rs)

枚举由 [`is_new_error`](is_new_error.md) 去重系统跟踪其失败情况的 Windows API 操作。每个变体对应一个特定的 Win32 或 NT 原生 API 调用，当 AffinityServiceRust 尝试管理进程/线程的亲和性、优先级或 I/O 设置时，这些调用可能会失败。该枚举作为 [`ApplyFailEntry`](ApplyFailEntry.md) 中复合键的一部分使用，以确保同一进程/线程的不同操作失败被独立跟踪。

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
|------|------|
| `OpenProcess2processQueryLimitedInformation` | 以 `PROCESS_QUERY_LIMITED_INFORMATION` 访问权限调用 `OpenProcess`。 |
| `OpenProcess2processSetLimitedInformation` | 以 `PROCESS_SET_LIMITED_INFORMATION` 访问权限调用 `OpenProcess`。 |
| `OpenProcess2processQueryInformation` | 以 `PROCESS_QUERY_INFORMATION` 访问权限调用 `OpenProcess`。 |
| `OpenProcess2processSetInformation` | 以 `PROCESS_SET_INFORMATION` 访问权限调用 `OpenProcess`。 |
| `OpenThread` | 以任意线程访问权限调用 `OpenThread`。 |
| `SetPriorityClass` | `SetPriorityClass` — 设置进程优先级类。 |
| `GetProcessAffinityMask` | `GetProcessAffinityMask` — 查询进程亲和性位掩码。 |
| `SetProcessAffinityMask` | `SetProcessAffinityMask` — 设置进程亲和性位掩码。 |
| `GetProcessDefaultCpuSets` | `GetProcessDefaultCpuSets` — 查询进程的默认 CPU 集合 ID。 |
| `SetProcessDefaultCpuSets` | `SetProcessDefaultCpuSets` — 为进程分配默认 CPU 集合 ID。 |
| `QueryThreadCycleTime` | `QueryThreadCycleTime` — 读取线程的累计周期计数。 |
| `SetThreadSelectedCpuSets` | `SetThreadSelectedCpuSets` — 为特定线程分配 CPU 集合 ID。 |
| `SetThreadPriority` | `SetThreadPriority` — 设置线程的调度优先级。 |
| `NtQueryInformationProcess2ProcessInformationIOPriority` | 使用 `ProcessIoPriority` 信息类调用 `NtQueryInformationProcess`。 |
| `NtSetInformationProcess2ProcessInformationIOPriority` | 使用 `ProcessIoPriority` 信息类调用 `NtSetInformationProcess`。 |
| `GetProcessInformation2ProcessMemoryPriority` | 使用 `ProcessMemoryPriority` 类调用 `GetProcessInformation`。 |
| `SetProcessInformation2ProcessMemoryPriority` | 使用 `ProcessMemoryPriority` 类调用 `SetProcessInformation`。 |
| `SetThreadIdealProcessorEx` | `SetThreadIdealProcessorEx` — 设置线程的理想处理器提示。 |
| `GetThreadIdealProcessorEx` | `GetThreadIdealProcessorEx` — 查询线程的理想处理器。 |
| `InvalidHandle` | 哨兵变体，表示已获取句柄但发现其无效的操作。由 [`get_process_handle`](../winapi.rs/get_process_handle.md) 和 [`get_thread_handle`](../winapi.rs/get_thread_handle.md) 用于内部错误代码区分。 |

## 备注

- 该枚举派生了 `PartialEq`、`Eq` 和 `Hash`，使其适合用作 `HashMap` 和 `HashSet` 集合中的键组件。具体来说，它是 [`is_new_error`](is_new_error.md) 使用的 [`ApplyFailEntry`](ApplyFailEntry.md) 复合键的一部分。

- 变体命名约定遵循 `ApiName` 或 `ApiName2ParameterDescription` 的模式，其中 `2` 分隔符表示底层 Win32 API 的特定重载或参数组合。例如，`OpenProcess2processQueryLimitedInformation` 表示"以 `PROCESS_QUERY_LIMITED_INFORMATION` 访问标志调用 `OpenProcess`"。

- 该枚举在源代码中标记为 `#[allow(dead_code)]`，表示并非所有变体当前在代码库中被引用。部分变体保留供将来使用，或仅在被注释掉的错误报告路径中使用。

- `InvalidHandle` 变体是一个特殊情况——它不对应特定的 API 调用，而是对应 API 调用成功但返回无效句柄值的场景。它与辅助 `error_code` 判别值（0、1、2、3）配合使用，以区分多句柄打开操作中哪个特定句柄无效。

### 按类别分组

| 类别 | 变体 |
|------|------|
| **进程句柄获取** | `OpenProcess2processQueryLimitedInformation`、`OpenProcess2processSetLimitedInformation`、`OpenProcess2processQueryInformation`、`OpenProcess2processSetInformation` |
| **线程句柄获取** | `OpenThread` |
| **亲和性 / CPU 集合** | `GetProcessAffinityMask`、`SetProcessAffinityMask`、`GetProcessDefaultCpuSets`、`SetProcessDefaultCpuSets`、`SetThreadSelectedCpuSets` |
| **优先级** | `SetPriorityClass`、`SetThreadPriority` |
| **I/O 优先级** | `NtQueryInformationProcess2ProcessInformationIOPriority`、`NtSetInformationProcess2ProcessInformationIOPriority` |
| **内存优先级** | `GetProcessInformation2ProcessMemoryPriority`、`SetProcessInformation2ProcessMemoryPriority` |
| **理想处理器** | `SetThreadIdealProcessorEx`、`GetThreadIdealProcessorEx` |
| **诊断** | `QueryThreadCycleTime` |
| **哨兵** | `InvalidHandle` |

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `logging.rs` |
| **使用者** | [`is_new_error`](is_new_error.md)、[`ApplyFailEntry`](ApplyFailEntry.md)、[`get_process_handle`](../winapi.rs/get_process_handle.md)、[`get_thread_handle`](../winapi.rs/get_thread_handle.md)、`apply.rs` |
| **Trait** | `PartialEq`、`Eq`、`Hash` |
| **平台** | 枚举本身与平台无关；其命名的操作是 Windows 特定的。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| ApplyFailEntry 结构体 | [ApplyFailEntry](ApplyFailEntry.md) |
| is_new_error 函数 | [is_new_error](is_new_error.md) |
| purge_fail_map 函数 | [purge_fail_map](purge_fail_map.md) |
| get_process_handle 函数 | [get_process_handle](../winapi.rs/get_process_handle.md) |
| get_thread_handle 函数 | [get_thread_handle](../winapi.rs/get_thread_handle.md) |
| logging 模块概述 | [README](README.md) |

---
*提交：[29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
