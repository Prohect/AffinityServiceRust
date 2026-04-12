# try_open_thread 函数 (winapi.rs)

尝试以指定的访问权限打开线程句柄，失败时执行去重错误日志记录。

## 语法

```rust
pub fn try_open_thread(
    pid: u32,
    tid: u32,
    process_name: &str,
    access: THREAD_ACCESS_RIGHTS,
    internal_op_code: u32,
) -> HANDLE
```

## 参数

`pid`

拥有该线程的进程的进程标识符。仅用于错误日志记录上下文。

`tid`

要打开的线程标识符。

`process_name`

拥有该线程的进程的名称。仅用于错误日志记录上下文。

`access`

所需的线程句柄访问权限，以 `THREAD_ACCESS_RIGHTS` 标志值指定（例如 `THREAD_QUERY_LIMITED_INFORMATION`、`THREAD_SET_INFORMATION`）。

`internal_op_code`

内部操作代码，映射到 [`Operation`](../logging.rs/Operation.md) 变体用于错误去重。这允许调用方区分使用相同访问权限打开线程的不同逻辑操作。

## 返回值

成功时返回已打开线程的 `HANDLE`。失败时返回无效句柄（在记录错误之后）。

## 备注

这是一个较低层级的辅助函数，由 [`get_thread_handle`](get_thread_handle.md) 使用，以不同的访问级别打开各个线程句柄。它封装了 Windows `OpenThread` API 调用并集成了错误处理。

失败时，函数使用 pid、tid、进程名称、映射的操作和 Win32 错误代码调用 [`is_new_error`](../logging.rs/is_new_error.md)。仅当该组合之前未被记录时才会记录错误，防止同一线程反复打开失败（例如由于受保护进程的访问限制）时产生日志垃圾。

`internal_op_code` 参数被转换为 [`Operation`](../logging.rs/Operation.md) 枚举变体，以标识正在尝试的具体逻辑操作。这使得去重系统能够区分例如以查询访问打开线程和以设置访问打开同一线程这两种不同操作。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/winapi.rs |
| **源码行** | L273–L301 |
| **调用方** | [`get_thread_handle`](get_thread_handle.md) |
| **调用** | [`is_new_error`](../logging.rs/is_new_error.md)、[`error_from_code_win32`](../error_codes.rs/error_from_code_win32.md) |
| **Windows API** | [OpenThread](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openthread)、[GetLastError](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror) |

## 另请参阅

- [get_thread_handle](get_thread_handle.md)
- [ThreadHandle](ThreadHandle.md)
- [Operation 枚举](../logging.rs/Operation.md)