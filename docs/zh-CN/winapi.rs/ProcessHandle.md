# ProcessHandle 结构体 (winapi.rs)

持有进程的读写 Windows `HANDLE`，提供受限和完全访问两种变体。实现 `Drop` 以在结构体离开作用域时自动关闭所有持有的句柄。

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

`r_limited_handle`

以 `PROCESS_QUERY_LIMITED_INFORMATION` 访问权限打开的只读句柄。此句柄始终存在且有效，因为即使对于受保护进程也可以获得受限查询访问。

`r_handle`

以 `PROCESS_QUERY_INFORMATION` 访问权限打开的只读句柄。对于拒绝完全查询访问的受保护进程，此值为 `None`。

`w_limited_handle`

以 `PROCESS_SET_LIMITED_INFORMATION` 访问权限打开的写入句柄。此句柄始终存在，足以执行 `SetPriorityClass` 等操作。

`w_handle`

以 `PROCESS_SET_INFORMATION` 访问权限打开的写入句柄。对于拒绝完全设置访问的受保护进程，此值为 `None`。执行 `SetProcessAffinityMask`、`SetProcessDefaultCpuSets` 和 `SetProcessInformation` 等操作时需要此句柄。

## 备注

`ProcessHandle` 由 [`get_process_handle`](get_process_handle.md) 创建，该函数尝试以全部四种访问级别打开进程。受限句柄（`r_limited_handle`、`w_limited_handle`）对可访问的进程始终成功，而完全句柄（`r_handle`、`w_handle`）可能因受保护或系统进程而失败，此时设置为 `None`。

该结构体实现了 `Drop`，在 `ProcessHandle` 离开作用域时调用 `CloseHandle` 关闭所有持有的句柄。这确保即使在配置应用过程中发生错误，内核句柄资源也不会泄漏。

调用方通常使用 apply 模块中的 [`get_handles`](../apply.rs/get_handles.md) 来提取适当的读写 `HANDLE` 值，根据需要从完全句柄降级到受限句柄。

### 访问级别用途

| 句柄 | 访问权限 | 用途 |
| --- | --- | --- |
| `r_limited_handle` | `PROCESS_QUERY_LIMITED_INFORMATION` | `GetPriorityClass`、`GetProcessAffinityMask`（降级） |
| `r_handle` | `PROCESS_QUERY_INFORMATION` | `GetProcessAffinityMask`、`GetProcessDefaultCpuSets`、`NtQueryInformationProcess` |
| `w_limited_handle` | `PROCESS_SET_LIMITED_INFORMATION` | `SetPriorityClass` |
| `w_handle` | `PROCESS_SET_INFORMATION` | `SetProcessAffinityMask`、`SetProcessDefaultCpuSets`、`NtSetInformationProcess`、`SetProcessInformation` |

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/winapi.rs |
| **行号** | L68–L75 |
| **创建方** | [`get_process_handle`](get_process_handle.md) |
| **使用方** | [`apply_config`](../main.rs/apply_config.md)、[`get_handles`](../apply.rs/get_handles.md)、[`is_affinity_unset`](is_affinity_unset.md) |

## 另请参阅

- [get_process_handle](get_process_handle.md)
- [ThreadHandle](ThreadHandle.md)
- [winapi.rs 模块概述](README.md)