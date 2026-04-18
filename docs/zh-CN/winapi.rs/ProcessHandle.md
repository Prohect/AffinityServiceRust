# ProcessHandle 结构体 (winapi.rs)

`ProcessHandle` 结构体是一个 RAII 包装器，持有一组具有不同访问级别的 Windows 进程句柄。它保证在结构体被丢弃（drop）时自动关闭所有包含的句柄。该结构体为每个进程维护最多四个句柄——用于读取（查询）和写入（设置）操作的受限版本和完整版本——允许调用者为每个操作使用适当的访问级别。

## 语法

```rust
pub struct ProcessHandle {
    pub r_limited_handle: HANDLE,
    pub r_handle: Option<HANDLE>,
    pub w_limited_handle: HANDLE,
    pub w_handle: Option<HANDLE>,
}
```

## 成员

| 成员 | 类型 | 描述 |
|------|------|------|
| `r_limited_handle` | `HANDLE` | 以 `PROCESS_QUERY_LIMITED_INFORMATION` 权限打开的进程句柄。在结构体构造时始终有效。 |
| `r_handle` | `Option<HANDLE>` | 以 `PROCESS_QUERY_INFORMATION` 权限打开的进程句柄。如果更高权限的打开操作失败（例如受保护进程），则可能为 `None`。 |
| `w_limited_handle` | `HANDLE` | 以 `PROCESS_SET_LIMITED_INFORMATION` 权限打开的进程句柄。在结构体构造时始终有效。 |
| `w_handle` | `Option<HANDLE>` | 以 `PROCESS_SET_INFORMATION` 权限打开的进程句柄。如果更高权限的打开操作失败，则可能为 `None`。 |

## 备注

- **RAII 句柄生命周期。** `Drop` 实现会自动关闭所有有效句柄。`Option<HANDLE>` 变体仅在为 `Some` 时才被关闭。受限句柄（`r_limited_handle`、`w_limited_handle`）始终会被关闭，因为它们在构造时保证有效。

- **访问级别回退模式。** 该结构体支持两级访问模型。受限句柄（`PROCESS_QUERY_LIMITED_INFORMATION`、`PROCESS_SET_LIMITED_INFORMATION`）对大多数进程都能成功获取，而完整句柄（`PROCESS_QUERY_INFORMATION`、`PROCESS_SET_INFORMATION`）对于受保护或系统进程可能会失败。调用者应在使用 `r_handle` 或 `w_handle` 之前检查其是否为 `Some`，并在失败时回退到受限变体。

- **构造。** 实例仅通过 [get_process_handle](get_process_handle.md) 创建，该函数打开所有四个句柄，如果任一必需的受限句柄无法获取则返回 `None`。完整句柄是尽力而为的。

- **线程安全性。** 该结构体未实现 `Send` 或 `Sync`。句柄不应在线程间共享；需要访问的每个线程应获取新的 `ProcessHandle`。

- **平台。** 仅限 Windows。依赖 `windows::Win32::Foundation::HANDLE` 和 `CloseHandle`。

## 要求

| 要求 | 值 |
|------|------|
| 模块 | `winapi.rs` |
| 创建者 | [get_process_handle](get_process_handle.md) |
| 依赖 | `windows::Win32::Foundation::{CloseHandle, HANDLE}` |
| 权限 | 建议使用 `SeDebugPrivilege` 以获取受保护进程的完整访问句柄 |
| 平台 | Windows |

## 另请参阅

| 主题 | 链接 |
|------|------|
| get_process_handle | [get_process_handle](get_process_handle.md) |
| ThreadHandle 结构体 | [ThreadHandle](ThreadHandle.md) |
| process 模块 | [process.rs](../process.rs/README.md) |

---
*提交：[29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
