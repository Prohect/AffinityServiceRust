# enable_debug_privilege 函数 (winapi.rs)

在当前进程的访问令牌上启用 `SeDebugPrivilege` 特权。此特权允许服务打开其他用户拥有的进程以及受保护系统进程的句柄，这对于在整个系统范围内应用亲和性、优先级、CPU 集合和其他设置至关重要。

## 语法

```rust
pub fn enable_debug_privilege()
```

## 参数

无。

## 返回值

无。函数在内部记录成功或失败信息，不返回结果。无论结果如何，调用者都应继续执行——如果无法启用该特权，服务仍将以降低的能力继续运行。

## 备注

### 特权启用流程

该函数遵循标准的 Windows 特权调整模式：

1. **打开进程令牌** — 以 `TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY` 访问权限对 `GetCurrentProcess()` 调用 `OpenProcessToken`。如果失败，函数记录错误并立即返回。

2. **查找特权 LUID** — 使用 `SE_DEBUG_NAME` 调用 `LookupPrivilegeValueW`，获取 `SeDebugPrivilege` 的本地唯一标识符（LUID）。如果查找失败，关闭令牌句柄并返回。

3. **调整令牌** — 构造一个包含单个 `LUID_AND_ATTRIBUTES` 条目（`SE_PRIVILEGE_ENABLED`）的 `TOKEN_PRIVILEGES` 结构体，并调用 `AdjustTokenPrivileges`。记录成功或失败信息。

4. **清理** — 无论结果如何，均通过 `CloseHandle` 关闭令牌句柄。

### 对服务的影响

如果没有 `SeDebugPrivilege`，针对受保护进程（如 `csrss.exe`、`smss.exe`、反作弊服务）的 `OpenProcess` 调用将失败并返回 `ERROR_ACCESS_DENIED` (5)。启用该特权后，服务可以打开这些进程并应用已配置的规则。此特权通常仅对管理员可用，因此在以提权方式运行时效果最佳（参见 [is_running_as_admin](is_running_as_admin.md) 和 [request_uac_elevation](request_uac_elevation.md)）。

### CLI 集成

`--no_debug_priv` CLI 标志允许用户跳过此函数的调用。当该标志存在时，[`main.rs`](../main.rs/README.md) 中的主循环不会调用 `enable_debug_privilege`，这在测试或有意在非提权访问下运行时很有用。

### 日志记录

所有结果都会产生一条日志消息：

| 结果 | 日志消息 |
|------|----------|
| `OpenProcessToken` 失败 | `"enable_debug_privilege: self OpenProcessToken failed"` |
| `LookupPrivilegeValueW` 失败 | `"enable_debug_privilege: LookupPrivilegeValueW failed"` |
| `AdjustTokenPrivileges` 失败 | `"enable_debug_privilege: AdjustTokenPrivileges failed"` |
| 成功 | `"enable_debug_privilege: AdjustTokenPrivileges succeeded"` |

### 与 SeIncreaseBasePriorityPrivilege 的关系

此函数是 [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md) 的 `SeDebugPrivilege` 对应版本，后者启用 `SeIncreaseBasePriorityPrivilege`。两者遵循相同的三步模式（打开令牌 → 查找 LUID → 调整特权），通常在服务启动期间一起调用。

## 要求

| | |
|---|---|
| **模块** | `winapi` (`src/winapi.rs`) |
| **可见性** | `pub` |
| **调用者** | [`main`](../main.rs/README.md)（启动期间，除非设置了 `--no_debug_priv`） |
| **被调用者** | `OpenProcessToken`、`GetCurrentProcess`、`LookupPrivilegeValueW`、`AdjustTokenPrivileges`、`CloseHandle`（Win32 Security） |
| **API** | [AdjustTokenPrivileges (Microsoft Learn)](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges) |
| **特权** | 调用令牌的可用特权列表中必须已包含 `SeDebugPrivilege`（管理员账户的标准配置）。此函数*启用*一个已有的特权；它无法*授予*令牌中未分配的特权。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 配套特权启用 | [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md) |
| 管理员提权检查 | [is_running_as_admin](is_running_as_admin.md) |
| UAC 提权请求 | [request_uac_elevation](request_uac_elevation.md) |
| 进程句柄获取（受益于此特权） | [get_process_handle](get_process_handle.md) |
| 线程句柄获取（受益于此特权） | [get_thread_handle](get_thread_handle.md) |
| CLI 参数（--no_debug_priv 标志） | [cli 模块](../cli.rs/README.md) |
| AdjustTokenPrivileges (MSDN) | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges) |
| SeDebugPrivilege 概述 | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/secauthz/privilege-constants) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd