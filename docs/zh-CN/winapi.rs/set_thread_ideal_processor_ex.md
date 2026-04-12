# set_thread_ideal_processor_ex 函数 (winapi.rs)

设置线程的理想处理器，指定处理器组和组内的处理器编号。

## 语法

```rust
pub fn set_thread_ideal_processor_ex(
    thread_handle: HANDLE,
    group: u16,
    number: u8,
) -> Result<PROCESSOR_NUMBER, Error>
```

## 参数

`thread_handle`

目标线程的 `HANDLE`，需以 `THREAD_SET_INFORMATION` 访问权限打开。通常为 [`ThreadHandle`](ThreadHandle.md) 的 `w_handle` 字段。

`group`

要分配的处理器组编号。在只有单个处理器组的系统（大多数逻辑处理器少于 64 个的桌面系统）上，此值为 `0`。

`number`

指定组内从零开始的处理器编号。用于标识要设置为线程理想处理器的具体逻辑处理器。

## 返回值

成功时返回 `Ok(PROCESSOR_NUMBER)`，包含线程**之前**的理想处理器分配（组和编号）。这允许调用方在需要时恢复原始分配。

失败时返回 `Err(Error)`，包含底层 Windows 错误。常见的失败原因包括无效句柄、访问被拒绝或无效的处理器编号。

## 备注

此函数封装了 Windows API `SetThreadIdealProcessorEx`，用于设置线程调度的首选处理器。理想处理器是一个调度提示——Windows 调度器会在指定处理器可用时尝试将线程调度到该处理器上，但在负载较高时可能将其调度到其他处理器。

该函数根据提供的 `group` 和 `number` 参数构造一个 `PROCESSOR_NUMBER` 结构体，调用 `SetThreadIdealProcessorEx`，并返回 API 作为输出参数提供的先前理想处理器设置。

此函数被应用程序中的多个子系统使用：

- **理想处理器分配** — [`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md) 使用此函数根据线程起始模块前缀将线程分配到特定 CPU（例如将渲染线程分配到性能核心）。
- **主线程调度** — [`apply_prime_threads_promote`](../apply.rs/apply_prime_threads_promote.md) 使用此函数将最高活跃度的线程固定到指定的快速核心。
- **理想处理器重置** — [`reset_thread_ideal_processors`](../apply.rs/reset_thread_ideal_processors.md) 在亲和性或 CPU 集更改后重新分配理想处理器。

函数返回的先前理想处理器存储在 [`ThreadStats`](../scheduler.rs/ThreadStats.md) 中的 [`IdealProcessorState`](../scheduler.rs/IdealProcessorState.md) 里，使应用程序能够检测操作系统或其他工具所做的更改，并在降级线程时恢复原始分配。

### 处理器组

在拥有超过 64 个逻辑处理器的系统上，Windows 将 CPU 组织为处理器组。`group` 参数选择组，`number` 参数选择组内的处理器。大多数消费级系统只有一个组（组 0），但服务器和 HEDT 系统可能有多个组。

配套函数 [`get_thread_ideal_processor_ex`](get_thread_ideal_processor_ex.md) 用于查询当前的理想处理器分配。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/winapi.rs |
| **源码行** | L658–L669 |
| **调用方** | [`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md)、[`apply_prime_threads_promote`](../apply.rs/apply_prime_threads_promote.md)、[`reset_thread_ideal_processors`](../apply.rs/reset_thread_ideal_processors.md) |
| **Windows API** | [SetThreadIdealProcessorEx](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadidealprocessorex) |

## 另请参阅

- [get_thread_ideal_processor_ex](get_thread_ideal_processor_ex.md)
- [ThreadHandle](ThreadHandle.md)
- [IdealProcessorState](../scheduler.rs/IdealProcessorState.md)
- [apply_ideal_processors](../apply.rs/apply_ideal_processors.md)
- [winapi.rs 模块概述](README.md)