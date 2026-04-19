# try_open_thread 函数 (winapi.rs)

底层辅助函数，尝试以特定访问级别打开单个线程句柄。成功时返回有效的 `HANDLE`，失败时返回无效的 `HANDLE`，允许调用方在不完全失败的情况下继续句柄获取。

## 语法

```rust
#[inline(always)]
fn try_open_thread(
    pid: u32,
    tid: u32,
    process_name: &str,
    access: THREAD_ACCESS_RIGHTS,
    internal_op_code: u32,
) -> HANDLE
```

## 参数

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `pid` | `u32` | 拥有线程的进程标识符。用于诊断消息中的错误上下文（当前已注释）。 |
| `tid` | `u32` | 要打开的线程标识符。传递给 `OpenThread`。 |
| `process_name` | `&str` | 拥有进程的名称。用于诊断消息中的错误上下文（当前已注释）。 |
| `access` | `THREAD_ACCESS_RIGHTS` | 句柄所需的访问权限。通常是 `THREAD_QUERY_INFORMATION`、`THREAD_SET_LIMITED_INFORMATION` 或 `THREAD_SET_INFORMATION` 之一。 |
| `internal_op_code` | `u32` | 正在尝试的访问级别的数字标识符，用于将错误映射为 `error_detail` 中的人类可读句柄名称。值：`1` → `"r_handle"`、`2` → `"w_limited_handle"`、`3` → `"w_handle"`。 |

## 返回值

`HANDLE` — 成功时返回有效的线程句柄，失败时返回 `HANDLE::default()`（无效句柄）。调用方在使用返回的句柄前必须检查 `is_invalid()`。

## 说明

此函数是 [get_thread_handle](get_thread_handle.md) 用来打开非必需句柄级别（`r_handle`、`w_limited_handle`、`w_handle`）的构建块。与必需的 `r_limited_handle`（其失败导致 `get_thread_handle` 返回 `None`）不同，`try_open_thread` 中的失败是非致命的 — 返回的无效句柄存储在 [ThreadHandle](ThreadHandle.md) 中，调用方只是避免使用该访问级别。

### 错误处理

该函数包含对 `is_new_error` 和 `log_to_find` 的注释调用，用于 `OpenThread` 失败路径和无效句柄路径。这些在生产中被故意禁用，因为在这些非必需访问级别的失败是预期且频繁的（例如，即使有 SeDebugPrivilege，`THREAD_SET_INFORMATION` 也可能因受保护进程而被拒绝）。`error_detail` 内部函数将 `internal_op_code` 映射到人类可读的字符串，以便在启用日志记录代码时用于诊断目的。

### 内部函数：error_detail

```rust
fn error_detail(internal_op_code: &u32) -> String
```

将数字 `internal_op_code` 映射到句柄字段名字符串：

| 代码 | 返回 |
|------|---------|
| `1` | `"r_handle"` |
| `2` | `"w_limited_handle"` |
| `3` | `"w_handle"` |
| 其他 | `"UNKNOWN_OP_CODE"` |

### 可见性

此函数是模块私有的（`fn`，而非 `pub fn`），仅由 [get_thread_handle](get_thread_handle.md) 调用。它标记为 `#[inline(always)]`，因为每个线程句柄获取时调用三次，且函数体很小。

### 失败语义

失败时，函数返回 `HANDLE::default()`，这是一个零化/无效的句柄。[ThreadHandle](ThreadHandle.md) 结构体的 `Drop` 实现在调用 `CloseHandle` 之前检查 `is_invalid()`，因此存储无效句柄是安全的，不会在清理期间导致双重关闭或错误。

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用方** | [get_thread_handle](get_thread_handle.md) |
| **被调用方** | `OpenThread` (Win32) |
| **Win32 API** | [OpenThread](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-openthread) |
| **特权** | 取决于 `access` — `THREAD_QUERY_INFORMATION` 需要进程查询权限；`THREAD_SET_INFORMATION` 对于受保护进程需要 [SeDebugPrivilege](enable_debug_privilege.md) |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [winapi.rs](README.md) |
| 主要调用方 | [get_thread_handle](get_thread_handle.md) |
| 线程句柄 RAII 封装 | [ThreadHandle](ThreadHandle.md) |
| 进程句柄等效项 | [get_process_handle](get_process_handle.md) |

*文档版本：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*