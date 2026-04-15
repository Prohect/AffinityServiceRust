# enable_inc_base_priority_privilege 函数 (winapi.rs)

在当前进程令牌上启用 `SeIncreaseBasePriorityPrivilege` 权限。此权限是将进程或线程的调度优先级类提升到 `NORMAL_PRIORITY_CLASS` 以上（例如提升到 `HIGH_PRIORITY_CLASS` 或 `REALTIME_PRIORITY_CLASS`）所必需的。如果没有此权限，使用提升的优先级值调用 `SetPriorityClass` 可能会失败，返回 `ERROR_PRIVILEGE_NOT_HELD`。

## 语法

```rust
pub fn enable_inc_base_priority_privilege()
```

## 参数

此函数不接受参数。

## 返回值

此函数不返回值。成功或失败通过 [`log_message`](../logging.rs/log_message.md)（通过 `log!` 宏）报告。

## 备注

该函数遵循标准的 Windows 权限启用模式：

1. **打开进程令牌** — 对当前进程（`GetCurrentProcess()`）调用 `OpenProcessToken`，请求 `TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY` 访问权限。如果失败，函数记录消息并提前返回。

2. **查找权限 LUID** — 使用 `SE_INC_BASE_PRIORITY_NAME` 调用 `LookupPrivilegeValueW`，以获取 `SeIncreaseBasePriorityPrivilege` 的本地唯一标识符 (LUID)。如果查找失败，关闭令牌句柄并返回。

3. **调整令牌权限** — 构造一个 `TOKEN_PRIVILEGES` 结构体，包含一个设置了 `SE_PRIVILEGE_ENABLED` 的 `LUID_AND_ATTRIBUTES` 条目，然后调用 `AdjustTokenPrivileges`。结果（成功或失败）会被记录。

4. **关闭令牌句柄** — 无论成功或失败，令牌句柄在函数返回前都会被无条件关闭。

### 与 enable_debug_privilege 的关系

此函数在结构上与 [`enable_debug_privilege`](enable_debug_privilege.md) 完全相同，但针对不同的权限常量（`SE_INC_BASE_PRIORITY_NAME` 与 `SE_DEBUG_NAME`）。两者通常在应用程序启动时调用。

### 何时需要此权限

- 将进程设置为 `HIGH_PRIORITY_CLASS` 或更高。
- 将线程优先级设置为 `THREAD_PRIORITY_TIME_CRITICAL`。
- 为延迟敏感的工作负载配置 `REALTIME_PRIORITY_CLASS`。

如果没有启用此权限，操作系统会静默限制有效优先级或返回错误，具体取决于所使用的 API。

### 平台说明

- **仅限 Windows。** `SeIncreaseBasePriorityPrivilege` 是 Windows 安全权限。
- 该权限必须已经**分配**给运行进程的用户或组（通常通过本地安全策略或组策略）。此函数只能**启用**已分配的权限；它无法授予未被分配的权限。
- 以管理员身份运行通常默认包含此权限。

### 日志输出

| 条件 | 日志消息 |
|------|----------|
| `OpenProcessToken` 失败 | `enable_inc_base_priority_privilege: self OpenProcessToken failed` |
| `LookupPrivilegeValueW` 失败 | `enable_inc_base_priority_privilege: LookupPrivilegeValueW failed` |
| `AdjustTokenPrivileges` 失败 | `enable_inc_base_priority_privilege: AdjustTokenPrivileges failed` |
| `AdjustTokenPrivileges` 成功 | `enable_inc_base_priority_privilege: AdjustTokenPrivileges succeeded` |

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `winapi.rs` |
| **调用者** | 应用程序启动 (`main.rs`) |
| **被调用者** | `OpenProcessToken`、`GetCurrentProcess`、`LookupPrivilegeValueW`、`AdjustTokenPrivileges`、`CloseHandle`（Win32 API）；`log!` 宏 |
| **Win32 API** | `advapi32.dll` — `OpenProcessToken`、`LookupPrivilegeValueW`、`AdjustTokenPrivileges` |
| **权限** | `SeIncreaseBasePriorityPrivilege` 必须已分配给当前用户/组。 |
| **平台** | Windows |

## 另请参阅

| 主题 | 链接 |
|------|------|
| enable_debug_privilege | [enable_debug_privilege](enable_debug_privilege.md) |
| is_running_as_admin | [is_running_as_admin](is_running_as_admin.md) |
| request_uac_elevation | [request_uac_elevation](request_uac_elevation.md) |
| logging 模块 | [logging.rs](../logging.rs/README.md) |
| winapi 模块概述 | [README](README.md) |

---
> Commit SHA: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
