# ThreadHandle 结构体 (winapi.rs)

用于一组具有不同访问级别的线程句柄的 RAII 包装器。`r_limited_handle` 字段始终有效（构造此结构体时为必需）。其他句柄（`r_handle`、`w_limited_handle`、`w_handle`）会尝试获取，但如果相应的 `OpenThread` 调用失败，则可能持有无效的 `HANDLE` 值。当结构体被销毁时，所有有效句柄会自动关闭。

## 语法

```rust
#[derive(Debug)]
pub struct ThreadHandle {
    pub r_limited_handle: HANDLE,
    pub r_handle: HANDLE,
    pub w_limited_handle: HANDLE,
    pub w_handle: HANDLE,
}
```

## 成员

| 字段 | 类型 | 描述 |
|-------|------|-------------|
| `r_limited_handle` | `HANDLE` | 以 `THREAD_QUERY_LIMITED_INFORMATION` 访问权限打开的线程句柄。始终有效。 |
| `r_handle` | `HANDLE` | 以 `THREAD_QUERY_INFORMATION` 访问权限打开的线程句柄。如果打开调用失败，可能是无效句柄。 |
| `w_limited_handle` | `HANDLE` | 以 `THREAD_SET_LIMITED_INFORMATION` 访问权限打开的线程句柄。如果打开调用失败，可能是无效句柄。 |
| `w_handle` | `HANDLE` | 以 `THREAD_SET_INFORMATION` 访问权限打开的线程句柄。如果打开调用失败，可能是无效句柄。 |

## 备注

与 [ProcessHandle](ProcessHandle.md) 使用 `Option<HANDLE>` 表示可选句柄不同，`ThreadHandle` 将所有句柄存储为原始 `HANDLE` 值。调用者在使用 `r_handle`、`w_limited_handle` 或 `w_handle` 之前必须检查 `HANDLE::is_invalid()`。

`Drop` 实现无条件关闭 `r_limited_handle`（因为它保证有效），并且仅在其他三个句柄不是无效时才有条件地关闭它们。这确保不会发生重复关闭或关闭无效句柄的情况。

`ThreadHandle` 派生了 `Debug` 以用于诊断输出。

### 句柄访问权限映射

| 字段 | Win32 访问权限 |
|-------|--------------------|
| `r_limited_handle` | `THREAD_QUERY_LIMITED_INFORMATION` |
| `r_handle` | `THREAD_QUERY_INFORMATION` |
| `w_limited_handle` | `THREAD_SET_LIMITED_INFORMATION` |
| `w_handle` | `THREAD_SET_INFORMATION` |

### Drop 行为

```text
Drop 顺序：
  1. CloseHandle(r_limited_handle)           — 始终关闭
  2. CloseHandle(r_handle)                   — 仅在非无效时关闭
  3. CloseHandle(w_limited_handle)           — 仅在非无效时关闭
  4. CloseHandle(w_handle)                   — 仅在非无效时关闭
```

## 要求

| 要求 | 值 |
|-------------|-------|
| 模块 | `winapi.rs` |
| 创建者 | [get_thread_handle](get_thread_handle.md) |
| 使用者 | [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md)、[get_thread_ideal_processor_ex](get_thread_ideal_processor_ex.md)、[get_thread_start_address](get_thread_start_address.md) |
| 平台 | Windows |
| 权限 | 访问其他用户会话中的线程需要 `SeDebugPrivilege` |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| get_thread_handle | [get_thread_handle](get_thread_handle.md) |
| ProcessHandle | [ProcessHandle](ProcessHandle.md) |
| try_open_thread | [try_open_thread](try_open_thread.md) |
| winapi 模块概述 | [README](README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
