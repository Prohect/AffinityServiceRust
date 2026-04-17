# get_thread_ideal_processor_ex 函数 (winapi.rs)

获取线程的理想处理器分配，返回处理器组和编号。此函数用于在应用配置规则之前或之后读取当前的理想处理器设置。

## 语法

```AffinityServiceRust/src/winapi.rs#L673-679
pub fn get_thread_ideal_processor_ex(thread_handle: HANDLE) -> Result<PROCESSOR_NUMBER, Error>
```

## 参数

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `thread_handle` | `HANDLE` | 一个以至少 `THREAD_QUERY_LIMITED_INFORMATION` 访问权限打开的有效线程句柄。通常为 [`ThreadHandle`](ThreadHandle.md) 的 `r_limited_handle` 或 `r_handle` 字段。 |

## 返回值

返回 `Result<PROCESSOR_NUMBER, Error>`。

| 结果 | 描述 |
|---------|-------------|
| `Ok(PROCESSOR_NUMBER)` | 一个 `PROCESSOR_NUMBER` 结构体，包含 `Group`（处理器组编号）、`Number`（组内处理器编号）和 `Reserved` 字段，表示线程当前的理想处理器。 |
| `Err(Error)` | 一个 `windows::core::Error`，描述底层 `GetThreadIdealProcessorEx` Win32 调用失败的原因。 |

## 备注

- 此函数是对 Win32 [`GetThreadIdealProcessorEx`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadidealprocessorex) API 的轻量安全封装。它初始化一个默认的 `PROCESSOR_NUMBER` 结构体并将其传递给 API 以被填充。

- 理想处理器是给 Windows 调度程序的一个**调度提示**，指示线程应优先在哪个逻辑处理器上运行。它不保证在该处理器上执行。

- 如果线程从未被显式设置过理想处理器，操作系统将返回调度程序在线程创建时分配的默认理想处理器。

- 此函数**不会**在内部记录错误日志。调用者负责处理 `Err` 变体并决定是否记录或传播错误。

- 配套函数 [`set_thread_ideal_processor_ex`](set_thread_ideal_processor_ex.md) 用于分配理想处理器，并在其成功返回值中包含**先前**的理想处理器。

### PROCESSOR_NUMBER 布局

| 字段 | 类型 | 描述 |
|-------|------|-------------|
| `Group` | `u16` | 处理器组编号（在具有 ≤ 64 个逻辑处理器的系统上为 0）。 |
| `Number` | `u8` | 组内的处理器编号。 |
| `Reserved` | `u8` | 保留字段；应忽略。 |

### 平台说明

- **仅限 Windows。** 使用 `windows::Win32::System::Kernel::PROCESSOR_NUMBER` 类型和来自 `processthreadsapi.h` 的 `GetThreadIdealProcessorEx`。
- 在单处理器组系统（≤ 64 个逻辑处理器）上，`Group` 字段始终为 `0`。
- 在多处理器组系统（> 64 个逻辑处理器）上，`Group` 字段标识理想处理器所属的处理器组。

## 要求

| 要求 | 值 |
|-------------|-------|
| **模块** | `winapi.rs` |
| **调用者** | `apply.rs` — 在设置理想处理器前后读取其值，用于比较和日志记录。 |
| **被调用者** | [`GetThreadIdealProcessorEx`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadidealprocessorex)（Win32 API） |
| **Win32 API** | `kernel32.dll` — `GetThreadIdealProcessorEx` |
| **权限** | 需要具有查询访问权限的有效线程句柄。 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| set_thread_ideal_processor_ex | [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) |
| get_thread_start_address | [get_thread_start_address](get_thread_start_address.md) |
| ThreadHandle 结构体 | [ThreadHandle](ThreadHandle.md) |
| get_thread_handle | [get_thread_handle](get_thread_handle.md) |
| resolve_address_to_module | [resolve_address_to_module](resolve_address_to_module.md) |
| winapi 模块概述 | [README](README.md) |

---
*提交：[37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
