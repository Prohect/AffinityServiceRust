# get_thread_ideal_processor_ex 函数 (winapi.rs)

获取线程当前的理想处理器（包括处理器组信息）。这是对旧版 `GetThreadIdealProcessorEx` Win32 API 的组感知封装，返回一个 `PROCESSOR_NUMBER`，同时标识处理器组和组内的处理器编号。

## 语法

```rust
pub fn get_thread_ideal_processor_ex(thread_handle: HANDLE) -> Result<PROCESSOR_NUMBER, Error>
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `thread_handle` | `HANDLE` | 一个有效的线程句柄，需具有 `THREAD_QUERY_INFORMATION` 或 `THREAD_QUERY_LIMITED_INFORMATION` 访问权限。通常从 [ThreadHandle](ThreadHandle.md) 的 `r_handle` 字段获取。 |

## 返回值

| 值 | 描述 |
|----|------|
| `Ok(PROCESSOR_NUMBER)` | 一个 `PROCESSOR_NUMBER` 结构体，包含线程当前的理想处理器分配信息。`Group` 字段标识处理器组（从 0 开始），`Number` 字段标识该组内的处理器编号（从 0 开始）。 |
| `Err(Error)` | 底层 `GetThreadIdealProcessorEx` Win32 调用失败。`Error` 包含 Win32 错误码。常见原因包括句柄无效或特权不足。 |

## 备注

### PROCESSOR_NUMBER 结构体

返回的 `PROCESSOR_NUMBER` 包含以下字段：

| 字段 | 类型 | 描述 |
|------|------|------|
| `Group` | `u16` | 处理器组索引（大多数单组系统上为 0）。 |
| `Number` | `u8` | 组内的处理器编号。 |
| `Reserved` | `u8` | 系统保留字段；对调用方无意义。 |

### 与 set_thread_ideal_processor_ex 的关系

此函数是 [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) 的读取对应函数。两者共同构成线程理想处理器分配的 get/set 操作对：

- **获取** — `get_thread_ideal_processor_ex` 读取当前理想处理器。
- **设置** — [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) 写入新的理想处理器并返回先前的值。

### 在调度器中的使用

[apply_ideal_processors](../apply.rs/apply_ideal_processors.md) 函数和 [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) 使用此函数在决定是否更改理想处理器之前读取线程当前的理想处理器分配。这避免了当线程已分配到所需处理器时进行冗余的 `SetThreadIdealProcessorEx` 调用。

### 线程句柄要求

该函数需要至少具有 `THREAD_QUERY_LIMITED_INFORMATION` 访问权限的句柄。在 [ThreadHandle](ThreadHandle.md) 结构体中，通常使用 `r_handle` 字段（以 `THREAD_QUERY_INFORMATION` 打开）。如果仅有 `r_limited_handle` 可用，调用是否成功取决于 Windows 版本和目标线程的保护级别。

### 多处理器组系统

在具有多个处理器组（超过 64 个逻辑处理器）的系统上，`Group` 字段用于区分属于不同组但共享相同 `Number` 值的处理器。在单组系统（桌面 PC 的常见情况）上，`Group` 始终为 `0`，`Number` 直接对应逻辑处理器索引。

### 错误处理

该函数将底层 Win32 调用产生的 `windows::core::Error` 直接传播给调用方。apply 模块中的调用方通常通过 `is_new_error` 去重系统记录错误，并继续处理其他线程，而非中止整个应用周期。

## 要求

| | |
|---|---|
| **模块** | `winapi` (`src/winapi.rs`) |
| **可见性** | `pub` |
| **调用方** | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md)、[PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| **被调用方** | `GetThreadIdealProcessorEx`（Win32 `kernel32.dll`） |
| **API** | [`GetThreadIdealProcessorEx`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadidealprocessorex) |
| **特权** | 线程句柄必须具有 `THREAD_QUERY_INFORMATION` 或 `THREAD_QUERY_LIMITED_INFORMATION` 访问权限 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 理想处理器设置器 | [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) |
| 线程句柄容器 | [ThreadHandle](ThreadHandle.md) |
| 理想处理器状态跟踪 | [IdealProcessorState](../scheduler.rs/IdealProcessorState.md) |
| 理想处理器分配逻辑 | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |
| 线程统计信息 | [ThreadStats](../scheduler.rs/ThreadStats.md) |
| GetThreadIdealProcessorEx (MSDN) | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadidealprocessorex) |