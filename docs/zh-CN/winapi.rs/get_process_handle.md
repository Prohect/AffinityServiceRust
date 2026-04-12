# get_process_handle 函数 (winapi.rs)

打开目标进程并返回包含受限和完全访问级别的读写 HANDLE 的 [`ProcessHandle`](ProcessHandle.md)。

## 语法

```rust
pub fn get_process_handle(pid: u32, process_name: &str) -> Option<ProcessHandle>
```

## 参数

`pid`

要打开的目标进程的进程标识符。

`process_name`

目标进程的名称，在句柄获取失败时用于错误日志记录。

## 返回值

如果至少能成功打开受限访问句柄，则返回 `Some(ProcessHandle)`。如果进程完全无法打开，则返回 `None`。

## 备注

此函数尝试以多种访问级别打开目标进程，根据调用方权限尽可能多地构建 [`ProcessHandle`](ProcessHandle.md) 的各项能力：

1. **读取受限** (`PROCESS_QUERY_LIMITED_INFORMATION`) — 始终必需；若失败则函数返回 `None`。
2. **读取完全** (`PROCESS_QUERY_INFORMATION`) — 可选；对于拒绝完全查询访问的受保护进程设为 `None`。
3. **写入受限** (`PROCESS_SET_LIMITED_INFORMATION`) — 始终必需；若失败则函数返回 `None`。
4. **写入完全** (`PROCESS_SET_INFORMATION`) — 可选；对于拒绝完全设置访问的受保护进程设为 `None`。

受限句柄对大多数操作（优先级、亲和性掩码）已经足够。完全句柄用于高级操作，如 CPU 集分配以及通过 `NtSetInformationProcess` 设置 I/O 和内存优先级。

句柄获取过程中的错误通过 [`is_new_error`](../logging.rs/is_new_error.md) 记录，并使用相应的 [`Operation`](../logging.rs/Operation.md) 变体进行去重。错误代码通过 [`error_from_code_win32`](../error_codes.rs/error_from_code_win32.md) 转换为可读字符串。

返回的 [`ProcessHandle`](ProcessHandle.md) 实现了 `Drop`，当值离开作用域时自动关闭所有已打开的句柄。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/winapi.rs |
| **源码行** | L96–L195 |
| **调用方** | [`apply_config`](../main.rs/apply_config.md)（main.rs）、[`is_affinity_unset`](is_affinity_unset.md) |
| **调用** | [`is_new_error`](../logging.rs/is_new_error.md)、[`error_from_code_win32`](../error_codes.rs/error_from_code_win32.md) |
| **Windows API** | [OpenProcess](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess)、[GetLastError](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror) |

## 另请参阅

- [ProcessHandle](ProcessHandle.md)
- [get_thread_handle](get_thread_handle.md)
- [winapi.rs 模块概述](README.md)