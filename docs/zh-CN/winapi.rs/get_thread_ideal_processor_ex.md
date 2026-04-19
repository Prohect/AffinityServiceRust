# get_thread_ideal_processor_ex 函数 (winapi.rs)

获取线程当前的理想处理器分配。包装 Windows `GetThreadIdealProcessorEx` API，返回包含处理器组和组内编号的 `PROCESSOR_NUMBER` 结构。

## 语法

```rust
pub fn get_thread_ideal_processor_ex(thread_handle: HANDLE) -> Result<PROCESSOR_NUMBER, Error>
```

## 参数

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `thread_handle` | `HANDLE` | 有效的线程句柄，具有 `THREAD_QUERY_LIMITED_INFORMATION` 或 `THREAD_QUERY_INFORMATION` 访问权限。通常来自 [ThreadHandle](ThreadHandle.md)`.r_limited_handle` 或 `.r_handle`。 |

## 返回值

`Result<PROCESSOR_NUMBER, Error>` — 成功时返回包含以下字段的 `PROCESSOR_NUMBER` 结构：

| 字段 | 类型 | 描述 |
|-------|------|-------------|
| `Group` | `u16` | 线程理想处理器的处理器组。在单组系统（≤64 个逻辑处理器）上，此值始终为 `0`。 |
| `Number` | `u8` | 组内作为线程当前理想处理器的逻辑处理器编号。 |
| `Reserved` | `u8` | 保留字段；始终为 `0`。 |

失败时返回包含底层 Win32 错误代码的 `windows::core::Error`（例如，如果句柄无效或缺少所需的访问权限，则为 `ERROR_INVALID_HANDLE`）。

## 说明

- 函数在栈上分配一个默认的 `PROCESSOR_NUMBER`，并将其作为输出参数传递给 `GetThreadIdealProcessorEx`。成功时，填充的结构被返回。
- 理想处理器是一个调度*提示*——Windows 倾向于在指定的处理器上运行线程，但在负载下可能会在其他地方调度它。此函数读取当前提示，该提示可能由操作系统、应用程序本身或之前对 [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) 的调用设置。
- 此函数由 [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) 逻辑使用，用于在决定是否需要更新之前读取当前理想处理器。返回的组号和编号与 [IdealProcessorState](../scheduler.rs/IdealProcessorState.md) 进行比较，以避免冗余的 `SetThreadIdealProcessorEx` 调用。

### 与 set_thread_ideal_processor_ex 的关系

| 函数 | 方向 | API |
|----------|-----------|-----|
| **get_thread_ideal_processor_ex** | 读取 | `GetThreadIdealProcessorEx` |
| [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) | 写入 | `SetThreadIdealProcessorEx` |

两个函数都操作相同的每线程理想处理器属性。get 变体只需要读取访问权限；set 变体需要写入访问权限（`THREAD_SET_INFORMATION`）。

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用方** | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md)、[reset_thread_ideal_processors](../apply.rs/reset_thread_ideal_processors.md) |
| **被调用方** | 无（薄 Win32 封装） |
| **Win32 API** | [GetThreadIdealProcessorEx](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadidealprocessorex) |
| **特权** | 线程句柄上的 `THREAD_QUERY_LIMITED_INFORMATION` 或 `THREAD_QUERY_INFORMATION` 访问权限 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [winapi.rs](README.md) |
| 写入对应项 | [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) |
| 理想处理器跟踪状态 | [IdealProcessorState](../scheduler.rs/IdealProcessorState.md) |
| 线程句柄包装器 | [ThreadHandle](ThreadHandle.md) |
| 设置理想处理器的应用逻辑 | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |

*文档版本：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*