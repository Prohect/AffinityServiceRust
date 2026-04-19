# enable_inc_base_priority_privilege 函数 (winapi.rs)

在进程令牌中启用 `SeIncreaseBasePriorityPrivilege` 特权，允许服务将线程和进程的优先级类提升到 `Normal` 之上。没有此特权时，设置 `High` 或 `Realtime` 优先级类的尝试将因 `ERROR_PRIVILEGE_NOT_HELD` 而失败。

## 语法

```rust
pub fn enable_inc_base_priority_privilege(no_inc_base_priority: bool)
```

## 参数

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `no_inc_base_priority` | `bool` | 如果为 `true`，函数记录一条消息，表明特权已被 `-noIncBasePriority` CLI 标志禁用，并立即返回，不修改进程令牌。如果为 `false`，则函数继续执行特权启用操作。 |

## 返回值

此函数不返回值。成功或失败通过日志消息传达。

## 说明

函数遵循与 [enable_debug_privilege](enable_debug_privilege.md) 相同的三步特权启用模式：

1. **OpenProcessToken** — 使用 `TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY` 访问权限打开当前进程令牌。
2. **LookupPrivilegeValueW** — 将 `SE_INC_BASE_PRIORITY_NAME` 特权名称解析为 `LUID`。
3. **AdjustTokenPrivileges** — 通过构建具有 `SE_PRIVILEGE_ENABLED` 的 `TOKEN_PRIVILEGES` 结构并将其传递给 API 来启用特权。

每个操作后，令牌句柄都通过 `CloseHandle` 关闭，包括错误路径。

### 提前退出

当 `no_inc_base_priority` 为 `true` 时，函数记录 `"SeIncreaseBasePriorityPrivilege disabled by -noIncBasePriority flag"` 并返回，不打开令牌。这允许用户在受限上下文中运行服务时选择不进行优先级提升。

### 错误处理

每个步骤在失败时记录描述性消息并提前返回：

| 失败点 | 日志消息 |
|---|---|
| `OpenProcessToken` | `"enable_inc_base_priority_privilege: self OpenProcessToken failed"` |
| `LookupPrivilegeValueW` | `"enable_inc_base_priority_privilege: LookupPrivilegeValueW failed"` |
| `AdjustTokenPrivileges` | `"enable_inc_base_priority_privilege: AdjustTokenPrivileges failed"` |
| 成功 | `"enable_inc_base_priority_privilege: AdjustTokenPrivileges succeeded"` |

当 `LookupPrivilegeValueW` 失败时，在前一步打开的令牌句柄在返回前被关闭。

### 何时需要此特权

Windows 需要 `SeIncreaseBasePriorityPrivilege` 来将进程优先级类设置为 `HIGH_PRIORITY_CLASS` 或 `REALTIME_PRIORITY_CLASS`，或在某些情况下将线程优先级提升到 `THREAD_PRIORITY_NORMAL` 之上。服务在启动时调用此函数，以便后续的 [apply_priority](../apply.rs/apply_priority.md) 调用可以根据配置设置提升的优先级类。

### 与 SeDebugPrivilege 的关系

此权限独立于 [SeDebugPrivilege](enable_debug_privilege.md)。`SeDebugPrivilege` 控制打开由其他用户拥有的进程或受保护进程的句柄的能力，而 `SeIncreaseBasePriorityPrivilege` 控制提升调度优先级的能力。两者通常在启动时启用，但可以通过 CLI 标志独立禁用。

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用方** | `src/main.rs` 中的主启动逻辑 |
| **被调用方** | 无（直接调用 Win32 API 的叶函数） |
| **Win32 API** | [OpenProcessToken](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocesstoken), [LookupPrivilegeValueW](https://learn.microsoft.com/zh-cn/windows/win32/api/winbase/nf-winbase-lookupprivilegevaluew), [AdjustTokenPrivileges](https://learn.microsoft.com/zh-cn/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges), [GetCurrentProcess](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocess), [CloseHandle](https://learn.microsoft.com/zh-cn/windows/win32/api/handleapi/nf-handleapi-closehandle) |
| **特权** | 要求进程在持有 `SeIncreaseBasePriorityPrivilege` 的账户下运行（通常是管理员）。该函数*启用*特权；如果账户不具备该特权，它不能*授予*它。 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [winapi.rs](README.md) |
| 调试特权启用 | [enable_debug_privilege](enable_debug_privilege.md) |
| 提升检查 | [is_running_as_admin](is_running_as_admin.md) |
| UAC 提升请求 | [request_uac_elevation](request_uac_elevation.md) |
| 优先级应用 | [apply_priority](../apply.rs/apply_priority.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*