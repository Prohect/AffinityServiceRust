# get_thread_handle 函数 (winapi.rs)

为给定线程 ID (TID) 打开多个具有不同访问级别的线程句柄，返回一个 [`ThreadHandle`](ThreadHandle.md) RAII 包装器。`r_limited_handle`（受限查询）是必需的；如果无法获取，函数将返回 `None`。其余句柄（`r_handle`、`w_limited_handle`、`w_handle`）会尝试获取，但如果调用者权限不足，则可能是无效的。

## 语法

```rust
pub fn get_thread_handle(tid: u32, pid: u32, process_name: &str) -> Option<ThreadHandle>
```

## 参数

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `tid` | `u32` | 目标线程的线程标识符。 |
| `pid` | `u32` | 拥有该线程的进程标识符。用于通过 [`is_new_error`](../logging.rs/is_new_error.md) 进行错误跟踪。 |
| `process_name` | `&str` | 拥有该线程的进程名称。用于错误跟踪和诊断日志记录。 |

## 返回值

如果成功打开了必需的 `THREAD_QUERY_LIMITED_INFORMATION` 句柄，则返回 `Some(ThreadHandle)`。如果无法获取必需的句柄，则返回 `None`。

返回的 [`ThreadHandle`](ThreadHandle.md) 包含：

| 字段 | 访问权限 | 是否必需 |
|-------|-------------|----------|
| `r_limited_handle` | `THREAD_QUERY_LIMITED_INFORMATION` | **是** — 返回 `Some` 时始终有效。 |
| `r_handle` | `THREAD_QUERY_INFORMATION` | 否 — 失败时可能为 `HANDLE::default()`（无效）。 |
| `w_limited_handle` | `THREAD_SET_LIMITED_INFORMATION` | 否 — 失败时可能为 `HANDLE::default()`（无效）。 |
| `w_handle` | `THREAD_SET_INFORMATION` | 否 — 失败时可能为 `HANDLE::default()`（无效）。 |

当 `ThreadHandle` 被销毁时，所有有效句柄将自动关闭。

## 备注

该函数使用 Windows `OpenThread` API 逐步打开句柄：

1. **`r_limited_handle`** — 以 `THREAD_QUERY_LIMITED_INFORMATION` 权限打开。这是唯一必需的句柄。如果失败或返回无效句柄，函数将通过 [`log_to_find`](../logging.rs/log_to_find.md)（受 [`is_new_error`](../logging.rs/is_new_error.md) 去重控制）记录失败信息并返回 `None`。

2. **`r_handle`** — 通过 [`try_open_thread`](try_open_thread.md) 以 `THREAD_QUERY_INFORMATION` 权限打开（internal_op_code `1`）。失败时静默处理；存储无效句柄。

3. **`w_limited_handle`** — 通过 [`try_open_thread`](try_open_thread.md) 以 `THREAD_SET_LIMITED_INFORMATION` 权限打开（internal_op_code `2`）。失败时静默处理；存储无效句柄。

4. **`w_handle`** — 通过 [`try_open_thread`](try_open_thread.md) 以 `THREAD_SET_INFORMATION` 权限打开（internal_op_code `3`）。失败时静默处理；存储无效句柄。

### is_new_error 的错误代码映射

| `internal_op_code` | 含义 |
|--------------------|---------|
| `0` | `THREAD_QUERY_LIMITED_INFORMATION` 打开失败或无效句柄 |
| `1` | `THREAD_QUERY_INFORMATION` |
| `2` | `THREAD_SET_LIMITED_INFORMATION` |
| `3` | `THREAD_SET_INFORMATION` |

非必需句柄的失败（代码 1–3）在 [`try_open_thread`](try_open_thread.md) 的源代码中目前已被注释掉，不会产生日志输出。

### 平台说明

- **仅限 Windows。** 使用 `windows::Win32::System::Threading` 中的 `OpenThread`。
- 需要调用者具有适当的权限。以管理员身份运行并启用 [`SeDebugPrivilege`](enable_debug_privilege.md) 可最大程度地提高获取全部四个句柄的成功率。
- 受保护进程和系统线程可能会拒绝甚至 `THREAD_QUERY_LIMITED_INFORMATION` 的访问。

## 要求

| 要求 | 值 |
|-------------|-------|
| **模块** | `winapi.rs` |
| **调用者** | `apply.rs`、`scheduler.rs` |
| **被调用者** | [`try_open_thread`](try_open_thread.md)、[`is_new_error`](../logging.rs/is_new_error.md)、[`log_to_find`](../logging.rs/log_to_find.md)、[`error_from_code_win32`](../error_codes.rs/error_from_code_win32.md) |
| **Win32 API** | `OpenThread`、`GetLastError` |
| **权限** | 建议启用 `SeDebugPrivilege` |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| ThreadHandle 结构体 | [ThreadHandle](ThreadHandle.md) |
| try_open_thread 辅助函数 | [try_open_thread](try_open_thread.md) |
| get_process_handle | [get_process_handle](get_process_handle.md) |
| Operation 枚举 | [Operation](../logging.rs/Operation.md) |
| is_new_error | [is_new_error](../logging.rs/is_new_error.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
