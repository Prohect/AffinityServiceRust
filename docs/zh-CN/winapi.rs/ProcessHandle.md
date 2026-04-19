# ProcessHandle 结构体 (winapi.rs)

封装多个 Windows `HANDLE` 值的 RAII 包装器，这些句柄以不同的访问级别为单个进程打开。提供受限和完全访问层级的读/写句柄。所有有效的句柄在结构体被析构时都通过 `CloseHandle` 自动关闭。

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
|--------|------|-------------|
| `r_limited_handle` | `HANDLE` | 以 `PROCESS_QUERY_LIMITED_INFORMATION` 访问级别打开的进程句柄。当结构体存在时始终有效 — 如果无法获取此句柄，构造将失败。用于轻量级查询，如 `QueryFullProcessImageNameW` 和周期时间查询，这些查询不需要完全查询权限。 |
| `r_handle` | `Option<HANDLE>` | 以 `PROCESS_QUERY_INFORMATION` 访问级别打开的进程句柄。如果访问权限被拒绝（受保护/系统进程常见），则为 `None`。用于如 `GetProcessAffinityMask` 和 `NtQueryInformationProcess` 等操作。 |
| `w_limited_handle` | `HANDLE` | 以 `PROCESS_SET_LIMITED_INFORMATION` 访问级别打开的进程句柄。当结构体存在时始终有效。用于通过 `SetProcessDefaultCpuSets` 进行 CPU 集合分配。 |
| `w_handle` | `Option<HANDLE>` | 以 `PROCESS_SET_INFORMATION` 访问级别打开的进程句柄。如果访问权限被拒绝，则为 `None`。用于如 `SetProcessAffinityMask`、`SetPriorityClass` 和 `NtSetInformationProcess` 等操作。 |

## Drop

```rust
impl Drop for ProcessHandle {
    fn drop(&mut self);
}
```

通过 `CloseHandle` 关闭所有持有的句柄：

- `r_limited_handle` 和 `w_limited_handle` 始终关闭（它们始终有效）。
- 仅当 `r_handle` 和 `w_handle` 为 `Some(_)` 时才关闭它们。

每个 `CloseHandle` 调用的结果被有意丢弃（`let _ = CloseHandle(...)`），因为清理期间的句柄关闭失败是不可恢复的，不应引发恐慌。

## 备注

### 访问级别层级

`ProcessHandle` 结构体捕获两个访问层级的四个句柄：

| 层级 | 读 | 写 |
|------|------|-------|
| **受限** | `PROCESS_QUERY_LIMITED_INFORMATION` | `PROCESS_SET_LIMITED_INFORMATION` |
| **完全** | `PROCESS_QUERY_INFORMATION` | `PROCESS_SET_INFORMATION` |

受限访问句柄对大多数进程成功，包括其他会话中的进程。完全访问句柄可能对受保护进程、系统进程或安全描述符拒绝请求账户的进程失败。[get_process_handle](get_process_handle.md) 工厂函数通过将这些句柄包装在 `Option` 中来优雅地处理这种情况。

### 调用方中的句柄选择

[get_handles](../apply.rs/get_handles.md) 辅助函数从 `ProcessHandle` 中提取 `(read_handle, write_handle)` 对，优先使用完全访问句柄（如果可用），否则回退到受限访问句柄。这允许应用函数使用最佳可用访问级别，而无需自行检查 `Option`。

### 有效性保证

- **构造：** `ProcessHandle` 仅由 [get_process_handle](get_process_handle.md) 创建，如果任一受限访问句柄打开失败，它返回 `None`。如果结构体存在，则 `r_limited_handle` 和 `w_limited_handle` 保证有效。
- **生命周期：** 调用方（通常是主轮询循环）拥有 `ProcessHandle`，并在不再跟踪该进程时丢弃它。
- **线程安全：** `ProcessHandle` 默认不是 `Send` 或 `Sync`，因为原始 `HANDLE` 值。它在主轮询循环的单线程上下文中使用。

### 错误去重

构造期间句柄打开失败时，错误通过 [is_new_error](../logging.rs/is_new_error.md) 记录，以便对相同进程/操作/错误码组合的重复失败在首次出现后被抑制。内部操作代码映射到特定句柄：

| 操作码 | 句柄 |
|---------|--------|
| `0` | `r_limited_handle` (`PROCESS_QUERY_LIMITED_INFORMATION`) |
| `1` | `w_limited_handle` (`PROCESS_SET_LIMITED_INFORMATION`) |
| `2` | `r_handle` (`PROCESS_QUERY_INFORMATION`) |
| `3` | `w_handle` (`PROCESS_SET_INFORMATION`) |

## 要求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用方** | 主轮询循环，[get_handles](../apply.rs/get_handles.md)，所有 `apply_*` 函数 |
| **工厂** | [get_process_handle](get_process_handle.md) |
| **Win32 API** | [OpenProcess](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess), [CloseHandle](https://learn.microsoft.com/zh-cn/windows/win32/api/handleapi/nf-handleapi-closehandle) |
| **特权** | [SeDebugPrivilege](enable_debug_privilege.md) 扩展对受保护进程的访问 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 模块概述 | [winapi.rs](README.md) |
| 工厂函数 | [get_process_handle](get_process_handle.md) |
| 句柄提取辅助函数 | [get_handles](../apply.rs/get_handles.md) |
| 线程句柄对应项 | [ThreadHandle](ThreadHandle.md) |
| 错误去重 | [is_new_error](../logging.rs/is_new_error.md) |

*为 Commit 记录：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*