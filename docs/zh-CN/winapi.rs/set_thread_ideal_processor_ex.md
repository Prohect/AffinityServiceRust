# set_thread_ideal_processor_ex 函数 (winapi.rs)

将线程的理想处理器设置为指定处理器组中的特定处理器编号。这是 `SetThreadIdealProcessor` 的处理器组感知变体，AffinityServiceRust 使用它在具有一个或多个处理器组的系统上，根据配置规则将主线程固定到特定核心并分配理想处理器。

## 语法

```rust
pub fn set_thread_ideal_processor_ex(
    thread_handle: HANDLE,
    group: u16,
    number: u8,
) -> Result<PROCESSOR_NUMBER, Error>
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `thread_handle` | `HANDLE` | 具有 `THREAD_SET_INFORMATION` 访问权限的有效线程句柄。通常从 [ThreadHandle](ThreadHandle.md) 的 `w_handle` 字段获取。 |
| `group` | `u16` | 要分配的从零开始的处理器组编号。在单处理器组系统（≤ 64 个逻辑处理器）上，此值始终为 `0`。 |
| `number` | `u8` | 组内从零开始的处理器编号。例如，在组 0 中有 16 个核心的系统上，有效值为 `0` 到 `15`。 |

## 返回值

| 值 | 描述 |
|----|------|
| `Ok(PROCESSOR_NUMBER)` | 线程**先前**的理想处理器，由底层 `SetThreadIdealProcessorEx` 调用返回。调用方可以使用此值在需要时恢复原始的理想处理器。返回的 `PROCESSOR_NUMBER` 包含 `Group`、`Number` 和 `Reserved` 字段。 |
| `Err(Error)` | Win32 调用失败。`Error` 值包装了底层的 Windows 错误码。常见原因包括无效句柄、访问权限不足（未授予 `THREAD_SET_INFORMATION`）或线程已退出。 |

## 备注

### 实现

该函数使用指定的 `Group` 和 `Number`（`Reserved` 设为 `0`）构造一个 `PROCESSOR_NUMBER` 结构体，然后调用 Win32 `SetThreadIdealProcessorEx` 函数。一个可变的 `PROCESSOR_NUMBER` 作为 `lpPreviousIdealProcessor` 输出参数传入，用于捕获线程先前的理想处理器分配。

### 理想处理器语义

设置线程的理想处理器是给 Windows 调度器的一个**提示**，而非硬性约束。调度器会优先在理想处理器可用时将线程调度到该处理器上，但当理想处理器繁忙时会将线程调度到其他允许的处理器上。如需硬性 CPU 固定，请使用 `SetThreadSelectedCpuSets` 的 CPU 集合功能或 `SetProcessAffinityMask` 的进程亲和性功能。

### 与 get_thread_ideal_processor_ex 的关系

此函数是 [get_thread_ideal_processor_ex](get_thread_ideal_processor_ex.md) 的设置对应函数。两者共同构成管理线程理想处理器分配的读取/设置对：

1. 使用 [get_thread_ideal_processor_ex](get_thread_ideal_processor_ex.md) 读取当前理想处理器。
2. 使用 `set_thread_ideal_processor_ex` 设置新的理想处理器。
3. 先前的值会被返回，可存储以便后续恢复。

### 在 apply 模块中的使用

[apply_ideal_processors](../apply.rs/apply_ideal_processors.md) 和 [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md) 函数调用 `set_thread_ideal_processor_ex` 将热线程引导至首选核心（例如混合架构上的性能核心）。[reset_thread_ideal_processors](../apply.rs/reset_thread_ideal_processors.md) 函数使用它在 CPU 集合变更需要重新平衡时将理想处理器重置为轮转分配。

### IdealProcessorState 跟踪

调度器模块中的 [IdealProcessorState](../scheduler.rs/IdealProcessorState.md) 结构体跟踪每个线程的当前和先前理想处理器分配，使服务能够检测何时需要重新分配，并避免冗余的 Win32 调用。

### 句柄要求

线程句柄**必须**具有 `THREAD_SET_INFORMATION` 访问权限。这对应于 [ThreadHandle](ThreadHandle.md) 的 `w_handle` 字段。如果 `w_handle` 无效（在 [get_thread_handle](get_thread_handle.md) 期间未授予该访问权限），调用方应跳过调用而非传入无效句柄。

## 要求

| | |
|---|---|
| **模块** | `winapi` (`src/winapi.rs`) |
| **可见性** | `pub` |
| **调用方** | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md)、[apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md)、[reset_thread_ideal_processors](../apply.rs/reset_thread_ideal_processors.md) |
| **被调用方** | `SetThreadIdealProcessorEx`（Win32 `kernel32.dll`） |
| **API** | [`SetThreadIdealProcessorEx`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadidealprocessorex) |
| **特权** | 设置其他用户拥有的进程中的线程的理想处理器时可能需要 `SeDebugPrivilege` |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 理想处理器获取器 | [get_thread_ideal_processor_ex](get_thread_ideal_processor_ex.md) |
| 线程句柄 RAII 容器 | [ThreadHandle](ThreadHandle.md) |
| 线程句柄获取 | [get_thread_handle](get_thread_handle.md) |
| 理想处理器状态跟踪 | [IdealProcessorState](../scheduler.rs/IdealProcessorState.md) |
| 理想处理器应用逻辑 | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |
| 主线程提升 | [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md) |
| 理想处理器重置 | [reset_thread_ideal_processors](../apply.rs/reset_thread_ideal_processors.md) |
| SetThreadIdealProcessorEx (MSDN) | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadidealprocessorex) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd