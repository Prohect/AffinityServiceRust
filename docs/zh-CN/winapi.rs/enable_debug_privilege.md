# enable_debug_privilege 函数 (winapi.rs)

在当前进程令牌上启用 `SeDebugPrivilege` 权限。此权限允许进程打开其他进程（包括系统进程和提升权限的进程）的句柄，否则这些操作将被拒绝。这对于 AffinityServiceRust 管理所有正在运行的进程的 CPU 亲和性和优先级设置至关重要。

## 语法

```rust
pub fn enable_debug_privilege()
```

## 参数

此函数不接受参数。

## 返回值

此函数不返回值。成功或失败通过 [`log_message`](../logging.rs/log_message.md)（通过 `log!` 宏）报告。

## 备注

该函数执行以下步骤：

1. **打开进程令牌** — 以 `TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY` 访问权限对当前进程（`GetCurrentProcess()`）调用 `OpenProcessToken`。如果失败，函数记录错误并提前返回。

2. **查找权限 LUID** — 使用 `SE_DEBUG_NAME` 调用 `LookupPrivilegeValueW` 以获取 `SeDebugPrivilege` 的本地唯一标识符 (LUID)。如果查找失败，函数记录错误，关闭令牌句柄并返回。

3. **调整令牌权限** — 使用包含单个 `LUID_AND_ATTRIBUTES` 条目（带有 `SE_PRIVILEGE_ENABLED`）的 `TOKEN_PRIVILEGES` 结构体调用 `AdjustTokenPrivileges`。结果以成功或失败的形式记录。

4. **关闭令牌句柄** — 无论结果如何，通过 `CloseHandle` 关闭令牌句柄。

### 重要副作用

- 此函数**就地修改当前进程令牌**。一旦启用 `SeDebugPrivilege`，它将在进程的整个生命周期内保持启用状态（除非被显式禁用）。
- 该权限必须已由操作系统**分配**给进程令牌。通常，这意味着进程必须在管理员帐户下运行。启用权限只是将其激活——如果从未被分配，则无法授予。
- 如果进程未以管理员身份运行，`AdjustTokenPrivileges` 可能会失败，返回 `ERROR_NOT_ALL_ASSIGNED` (1300) 或 `PRIVILEGE_NOT_HELD` (1314)。

### 平台说明

- **仅限 Windows。** 依赖 Win32 安全 API：`OpenProcessToken`、`LookupPrivilegeValueW` 和 `AdjustTokenPrivileges`。
- `SE_DEBUG_NAME` 常量解析为字符串 `"SeDebugPrivilege"`。
- 此函数通常在进程启动时调用一次，在 [is_running_as_admin](is_running_as_admin.md) 确认进程具有管理员权限之后。

### 日志输出

| 条件 | 日志消息 |
|------|----------|
| `OpenProcessToken` 失败 | `enable_debug_privilege: self OpenProcessToken failed` |
| `LookupPrivilegeValueW` 失败 | `enable_debug_privilege: LookupPrivilegeValueW failed` |
| `AdjustTokenPrivileges` 失败 | `enable_debug_privilege: AdjustTokenPrivileges failed` |
| `AdjustTokenPrivileges` 成功 | `enable_debug_privilege: AdjustTokenPrivileges succeeded` |

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `winapi.rs` |
| **调用者** | `main.rs` — 在启动初始化期间调用 |
| **被调用者** | `OpenProcessToken`、`LookupPrivilegeValueW`、`AdjustTokenPrivileges`、`CloseHandle`（Win32 API）；`log!` 宏 |
| **Win32 API** | `OpenProcessToken`、`GetCurrentProcess`、`LookupPrivilegeValueW`、`AdjustTokenPrivileges`、`CloseHandle` |
| **权限** | 必须以管理员身份运行，权限才会存在于令牌中。 |
| **平台** | Windows |

## 另请参阅

| 主题 | 链接 |
|------|------|
| enable_inc_base_priority_privilege | [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md) |
| is_running_as_admin | [is_running_as_admin](is_running_as_admin.md) |
| request_uac_elevation | [request_uac_elevation](request_uac_elevation.md) |
| get_process_handle | [get_process_handle](get_process_handle.md) |
| get_thread_handle | [get_thread_handle](get_thread_handle.md) |

---
> Commit SHA: `7221ea0694670265d4eb4975582d8ed2ae02439d`
