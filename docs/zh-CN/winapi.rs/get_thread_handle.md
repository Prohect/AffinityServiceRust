# get_thread_handle 函数 (winapi.rs)

以多种访问级别打开线程，并返回一个 [ThreadHandle](ThreadHandle.md) RAII 容器。该函数要求 `THREAD_QUERY_LIMITED_INFORMATION` 作为最低访问权限；如果此权限获取失败，函数返回 `None`。其余三个访问级别（`THREAD_QUERY_INFORMATION`、`THREAD_SET_LIMITED_INFORMATION`、`THREAD_SET_INFORMATION`）会尝试获取，但失败不致命——对应的句柄字段将被设置为无效句柄。

## 语法

```rust
pub fn get_thread_handle(tid: u32, pid: u32, process_name: &str) -> Option<ThreadHandle>
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `tid` | `u32` | 要打开的线程的线程标识符。 |
| `pid` | `u32` | 拥有该线程的进程标识符。仅用于通过 `is_new_error` 进行错误日志记录和去重。 |
| `process_name` | `&str` | 拥有该线程的进程的映像名称（例如 `"explorer.exe"`）。仅用于通过 `is_new_error` 进行错误日志记录和去重。 |

## 返回值

| 值 | 描述 |
|----|------|
| `Some(ThreadHandle)` | 一个 [ThreadHandle](ThreadHandle.md)，其 `r_limited_handle` 保证有效。`r_handle`、`w_limited_handle` 和 `w_handle` 字段在对应的 `OpenThread` 调用失败时可能为无效句柄。 |
| `None` | `THREAD_QUERY_LIMITED_INFORMATION` 打开失败或返回了无效句柄。在首次出现该 PID/TID/操作组合时，会通过 `log_to_find` 记录错误。 |

## 备注

### 句柄获取策略

该函数按顺序打开四个句柄，每个请求不同的访问权限：

| 顺序 | 访问权限 | 字段 | 是否必需 |
|------|---------|------|---------|
| 1 | `THREAD_QUERY_LIMITED_INFORMATION` | `r_limited_handle` | **是** — 失败时返回 `None` |
| 2 | `THREAD_QUERY_INFORMATION` | `r_handle` | 否 — 失败时为无效句柄 |
| 3 | `THREAD_SET_LIMITED_INFORMATION` | `w_limited_handle` | 否 — 失败时为无效句柄 |
| 4 | `THREAD_SET_INFORMATION` | `w_handle` | 否 — 失败时为无效句柄 |

第一个句柄通过 `OpenThread` 直接打开。其余三个通过辅助函数 [try_open_thread](try_open_thread.md) 打开，该函数在失败时返回 `HANDLE::default()`（无效句柄）而非传播错误。

### 错误日志记录

每个失败的 `OpenThread` 调用都会与每进程/线程错误去重系统（`is_new_error`）进行检查。只有给定 `(pid, tid, operation, error_code)` 元组的首次失败会被记录到 find 日志中。`internal_op_code` 映射如下：

| 代码 | 含义 |
|------|------|
| `0` | `THREAD_QUERY_LIMITED_INFORMATION`（致命） |
| `1` | `THREAD_QUERY_INFORMATION` |
| `2` | `THREAD_SET_LIMITED_INFORMATION` |
| `3` | `THREAD_SET_INFORMATION` |

### RAII 清理

返回的 [ThreadHandle](ThreadHandle.md) 实现了 `Drop`。当被销毁时，它会无条件关闭 `r_limited_handle`，并仅在其他三个句柄非无效时才有条件地关闭它们。

### 调用方预期

需要设置线程属性（理想处理器、CPU 集合、线程优先级）的调用方应在尝试写操作前检查 `w_handle` 或 `w_limited_handle` 是否有效。[apply 模块](../apply.rs/README.md)和 [scheduler 模块](../scheduler.rs/README.md)常规地处理仅有受限访问句柄可用的情况。

## 要求

| | |
|---|---|
| **模块** | `winapi` (`src/winapi.rs`) |
| **调用方** | [scheduler 模块](../scheduler.rs/README.md)（`ThreadStats` 句柄获取）、[apply 模块](../apply.rs/README.md)（线程级别操作） |
| **被调用方** | `OpenThread`（Windows API）、[try_open_thread](try_open_thread.md)、`is_new_error`、`log_to_find`、`error_from_code_win32` |
| **API** | `Win32::System::Threading::OpenThread` |
| **特权** | 建议启用 `SeDebugPrivilege` 以对受保护进程进行跨进程线程访问 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 线程句柄容器 | [ThreadHandle](ThreadHandle.md) |
| 非必需句柄打开辅助函数 | [try_open_thread](try_open_thread.md) |
| 进程句柄获取 | [get_process_handle](get_process_handle.md) |
| 进程句柄容器 | [ProcessHandle](ProcessHandle.md) |
| 错误去重系统 | [logging 模块](../logging.rs/README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd