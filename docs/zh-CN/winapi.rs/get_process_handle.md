# get_process_handle 函数 (winapi.rs)

为给定的进程 ID 打开多个具有不同访问级别的 Windows 进程句柄。返回一个 [`ProcessHandle`](ProcessHandle.md) RAII 包装器，在被丢弃（drop）时自动关闭所有有效句柄。

## 语法

```rust
pub fn get_process_handle(pid: u32, process_name: &str) -> Option<ProcessHandle>
```

## 参数

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `pid` | `u32` | 目标进程的进程标识符。 |
| `process_name` | `&str` | 目标进程的名称，用于错误跟踪和日志消息。 |

## 返回值

如果至少成功打开了受限访问句柄（`r_limited_handle` 和 `w_limited_handle`），则返回 `Some(ProcessHandle)`。如果任一受限句柄无法获取，则返回 `None`。

返回的 [`ProcessHandle`](ProcessHandle.md) 包含：

| 字段 | 访问权限 | 必需 |
|-------|-------------|----------|
| `r_limited_handle` | `PROCESS_QUERY_LIMITED_INFORMATION` | 是 — 失败则返回 `None`。 |
| `w_limited_handle` | `PROCESS_SET_LIMITED_INFORMATION` | 是 — 失败则返回 `None`。 |
| `r_handle` | `PROCESS_QUERY_INFORMATION` | 否 — 失败时为 `None`。 |
| `w_handle` | `PROCESS_SET_INFORMATION` | 否 — 失败时为 `None`。 |

## 备注

该函数尝试打开四个具有逐步提高权限要求的独立句柄。两个受限句柄（`PROCESS_QUERY_LIMITED_INFORMATION` 和 `PROCESS_SET_LIMITED_INFORMATION`）是**必需的**——如果任一失败，函数将记录错误并返回 `None`。两个完整访问句柄（`PROCESS_QUERY_INFORMATION` 和 `PROCESS_SET_INFORMATION`）是**可选的**——失败会被静默容忍，相应字段设置为 `None`。

错误去重通过 [`is_new_error`](../logging.rs/is_new_error.md) 执行，使得相同 PID/进程/操作/错误码组合的重复失败仅记录一次。`is_new_error` 的内部错误码映射为：

| 代码 | 句柄 |
|------|--------|
| `0` | `PROCESS_QUERY_LIMITED_INFORMATION` |
| `1` | `PROCESS_SET_LIMITED_INFORMATION` |
| `2` | `PROCESS_QUERY_INFORMATION` |
| `3` | `PROCESS_SET_INFORMATION` |

如果获取的句柄通过 `HANDLE::is_invalid()` 报告为无效，则也会被视为失败，使用 `Operation::InvalidHandle` 变体。

所有成功打开的句柄归返回的 [`ProcessHandle`](ProcessHandle.md) 所有，并通过其 `Drop` 实现自动关闭。如果函数返回 `None`，任何已部分打开的句柄会在返回前关闭。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `winapi.rs` |
| **调用者** | `apply.rs` 规则应用逻辑 |
| **被调用者** | `OpenProcess`（Win32）、[`is_new_error`](../logging.rs/is_new_error.md)、[`log_to_find`](../logging.rs/log_to_find.md)、[`error_from_code_win32`](../error_codes.rs/error_from_code_win32.md) |
| **API** | Win32 `OpenProcess`、`GetLastError`、`CloseHandle` |
| **权限** | 建议使用 `SeDebugPrivilege` 以打开受保护/提升的进程。 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| ProcessHandle 结构体 | [ProcessHandle](ProcessHandle.md) |
| get_thread_handle 函数 | [get_thread_handle](get_thread_handle.md) |
| is_new_error 函数 | [is_new_error](../logging.rs/is_new_error.md) |
| Operation 枚举 | [Operation](../logging.rs/Operation.md) |
| error_from_code_win32 | [error_from_code_win32](../error_codes.rs/error_from_code_win32.md) |

---
*提交：[37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
