# Operation 枚举 (logging.rs)

标识在对运行中的进程应用规则时可能失败的每个不同的 Windows API 操作。`Operation` 变体作为 [ApplyFailEntry](ApplyFailEntry.md) 复合键中的键，使 [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) 中的错误去重系统能够区分同一进程上来自不同 API 调用的失败。每个变体对应一个特定的 Win32 或 NT 原生 API 调用（或其特定访问权限变体），使服务能够记录每个唯一失败的首次出现，同时抑制后续相同的错误。

## 语法

```logging.rs
#[derive(PartialEq, Eq, Hash)]
#[allow(dead_code)]
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

| 变体 | Win32 / NT API | 描述 |
|---------|---------------|-------------|
| `OpenProcess2processQueryLimitedInformation` | `OpenProcess`（使用 `PROCESS_QUERY_LIMITED_INFORMATION`） | 无法以有限查询访问权限打开进程句柄。 |
| `OpenProcess2processSetLimitedInformation` | `OpenProcess`（使用 `PROCESS_SET_LIMITED_INFORMATION`） | 无法以有限设置访问权限打开进程句柄。 |
| `OpenProcess2processQueryInformation` | `OpenProcess`（使用 `PROCESS_QUERY_INFORMATION`） | 无法以完整查询访问权限打开进程句柄。 |
| `OpenProcess2processSetInformation` | `OpenProcess`（使用 `PROCESS_SET_INFORMATION`） | 无法以完整设置访问权限打开进程句柄。 |
| `OpenThread` | `OpenThread` | 无法打开线程句柄。 |
| `SetPriorityClass` | `SetPriorityClass` | 无法设置进程优先级类。 |
| `GetProcessAffinityMask` | `GetProcessAffinityMask` | 无法查询当前进程亲和性掩码。 |
| `SetProcessAffinityMask` | `SetProcessAffinityMask` | 无法设置进程亲和性掩码。 |
| `GetProcessDefaultCpuSets` | `GetProcessDefaultCpuSets` | 无法查询进程的默认 CPU 集合分配。 |
| `SetProcessDefaultCpuSets` | `SetProcessDefaultCpuSets` | 无法设置进程的默认 CPU 集合分配。 |
| `QueryThreadCycleTime` | `QueryThreadCycleTime` | 无法读取线程的周期时间计数器（由主线程调度器使用）。 |
| `SetThreadSelectedCpuSets` | `SetThreadSelectedCpuSets` | 无法为线程分配选定的 CPU 集合。 |
| `SetThreadPriority` | `SetThreadPriority` | 无法设置线程的调度优先级。 |
| `NtQueryInformationProcess2ProcessInformationIOPriority` | `NtQueryInformationProcess`（I/O 优先级类） | 无法通过 NT 原生 API 查询进程的 I/O 优先级。 |
| `NtSetInformationProcess2ProcessInformationIOPriority` | `NtSetInformationProcess`（I/O 优先级类） | 无法通过 NT 原生 API 设置进程的 I/O 优先级。 |
| `GetProcessInformation2ProcessMemoryPriority` | `GetProcessInformation`（`ProcessMemoryPriority`） | 无法查询进程的内存优先级。 |
| `SetProcessInformation2ProcessMemoryPriority` | `SetProcessInformation`（`ProcessMemoryPriority`） | 无法设置进程的内存优先级。 |
| `SetThreadIdealProcessorEx` | `SetThreadIdealProcessorEx` | 无法设置线程的理想处理器。 |
| `GetThreadIdealProcessorEx` | `GetThreadIdealProcessorEx` | 无法查询线程的理想处理器。 |
| `InvalidHandle` | *（无）* | 哨兵变体，表示在尝试 API 调用之前所需的句柄已无效或为空。 |

## 备注

- 该枚举派生了 `PartialEq`、`Eq` 和 `Hash`，以便可以作为 `HashMap` 和 `HashSet` 数据结构中 [ApplyFailEntry](ApplyFailEntry.md) 复合键的一部分使用。它**未**派生 `Debug` 或 `Clone`。
- `#[allow(dead_code)]` 属性抑制了未使用变体的警告。当前代码库中并非每个变体都被主动使用——部分变体为将来使用或为覆盖完整的 API 接口而保留。
- 命名约定使用 `2` 作为分隔符来编码 "使用" 或 "用于" 的关系。例如，`OpenProcess2processQueryLimitedInformation` 读作 "OpenProcess **用于** PROCESS_QUERY_LIMITED_INFORMATION 访问权限"。类似地，`NtSetInformationProcess2ProcessInformationIOPriority` 读作 "NtSetInformationProcess **使用** ProcessInformation 类 I/O 优先级"。此约定避免了当同一 Win32 函数以不同的访问权限或信息类被调用且可能独立失败时产生的歧义。
- `OpenProcess` 调用被拆分为四个变体，因为服务针对不同操作以不同的访问权限打开句柄（有限读取、有限写入、完整读取、完整写入）。以 `PROCESS_SET_INFORMATION` 打开失败与以 `PROCESS_QUERY_LIMITED_INFORMATION` 打开失败是不同的错误，且两者可能在同一 PID 上发生。
- `InvalidHandle` 变体表示调用前失败——即将传递给 API 的句柄已知为无效（例如，由先前失败的 `OpenProcess` 调用返回的 `NULL`）。这使去重系统能够抑制因同一根本原因（句柄获取失败）而产生的级联失败的重复日志消息。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `logging` |
| Trait | `PartialEq`、`Eq`、`Hash` |
| 使用位置 | [ApplyFailEntry](ApplyFailEntry.md)（作为字段）、[is_new_error](is_new_error.md)（作为参数） |
| 调用者 | [log_error_if_new](../apply.rs/log_error_if_new.md)、[get_process_handle](../winapi.rs/get_process_handle.md)、[get_thread_handle](../winapi.rs/get_thread_handle.md)、[apply_priority](../apply.rs/apply_priority.md)、[apply_affinity](../apply.rs/apply_affinity.md)、[apply_io_priority](../apply.rs/apply_io_priority.md)、[apply_memory_priority](../apply.rs/apply_memory_priority.md)、[apply_prime_threads](../apply.rs/apply_prime_threads.md)、[apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 使用此枚举的复合失败键 | [ApplyFailEntry](ApplyFailEntry.md) |
| 错误去重逻辑 | [is_new_error](is_new_error.md) |
| 每 PID 失败跟踪映射 | [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) |
| 过期条目清理 | [purge_fail_map](purge_fail_map.md) |
| Win32 错误代码翻译 | [error_from_code_win32](../error_codes.rs/error_from_code_win32.md) |
| NTSTATUS 错误代码翻译 | [error_from_ntstatus](../error_codes.rs/error_from_ntstatus.md) |
| logging 模块概述 | [logging 模块](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd