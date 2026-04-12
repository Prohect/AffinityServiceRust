# get_thread_ideal_processor_ex 函数 (winapi.rs)

查询线程当前的理想处理器分配，返回处理器组和编号。

## 语法

```rust
pub fn get_thread_ideal_processor_ex(thread_handle: HANDLE) -> Result<PROCESSOR_NUMBER, Error>
```

## 参数

`thread_handle`

目标线程的 `HANDLE`，需至少以 `THREAD_QUERY_INFORMATION` 访问权限打开。通常为 [`ThreadHandle`](ThreadHandle.md) 的 `r_handle` 字段。

## 返回值

成功时返回 `Ok(PROCESSOR_NUMBER)`，包含线程当前的理想处理器组和编号。失败时返回 `Err(Error)`（底层 Windows API 调用失败时）。

`PROCESSOR_NUMBER` 结构包含：

- `Group` — 处理器组（在逻辑处理器少于 64 个的系统上通常为 0）。
- `Number` — 组内从零开始的处理器编号。

## 备注

此函数封装了 Windows API `GetThreadIdealProcessorEx`。它获取 Windows 调度器在调度指定线程时将优先选择的处理器，该值可能由 [`set_thread_ideal_processor_ex`](set_thread_ideal_processor_ex.md) 或系统的默认分配所设置。

理想处理器是一个调度提示——它不保证线程始终在该处理器上运行。调度器利用它在指定处理器可用时优先调度以改善缓存局部性。

此函数被 [`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md) 函数用于在决定是否需要更改之前读取当前的理想处理器分配。通过将当前分配与期望值进行比较，应用程序可以在线程已分配到正确处理器时避免不必要的 API 调用。

**错误处理：** 如果调用失败（例如由于无效句柄或访问权限不足），函数返回封装在 `windows::core::Error` 中的 Windows 错误。调用方负责处理或记录该错误，通常通过 [`log_error_if_new`](../apply.rs/log_error_if_new.md) 进行。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/winapi.rs |
| **源码行** | L671–L677 |
| **调用方** | [`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md) |
| **Windows API** | [GetThreadIdealProcessorEx](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadidealprocessorex) |

## 另请参阅

- [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md)
- [ThreadHandle](ThreadHandle.md)
- [apply_ideal_processors](../apply.rs/apply_ideal_processors.md)
- [winapi.rs 模块概述](README.md)