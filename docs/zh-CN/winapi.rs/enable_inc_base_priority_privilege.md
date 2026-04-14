# enable_inc_base_priority_privilege 函数 (winapi.rs)

在当前进程令牌上启用 `SeIncreaseBasePriorityPrivilege` 特权，允许 AffinityServiceRust 将进程优先级类提升到 `Normal` 以上（包括 `High` 和 `Realtime`）。如果没有此特权，使用提升的优先级类调用 `SetPriorityClass` 将会失败并返回 `ERROR_PRIVILEGE_NOT_HELD`。

## 语法

```rust
pub fn enable_inc_base_priority_privilege()
```

## 参数

无。

## 返回值

无。该函数将成功或失败信息记录到应用程序日志中，并在所有情况下返回。它不会将错误传播给调用者。

## 备注

### 特权用途

Windows 要求 `SeIncreaseBasePriorityPrivilege` 才能将进程的优先级类设置为 `HIGH_PRIORITY_CLASS` 或 `REALTIME_PRIORITY_CLASS`。默认情况下，此特权存在于管理员令牌中但处于禁用状态。此函数启用该特权，以便 [apply_priority](../apply.rs/apply_priority.md) 函数可以按照配置中指定的方式提升进程优先级类。

### 实现

该函数遵循标准的 Windows 特权调整模式，其结构与 [enable_debug_privilege](enable_debug_privilege.md) 完全相同：

1. **打开进程令牌** — 使用 `TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY` 访问权限对当前进程（`GetCurrentProcess()`）调用 `OpenProcessToken`。如果失败，记录 `"enable_inc_base_priority_privilege: self OpenProcessToken failed"` 并返回。

2. **查找特权 LUID** — 使用 `SE_INC_BASE_PRIORITY_NAME` 调用 `LookupPrivilegeValueW` 以获取该特权的本地唯一标识符。如果失败，记录 `"enable_inc_base_priority_privilege: LookupPrivilegeValueW failed"`，关闭令牌句柄并返回。

3. **调整令牌** — 构造一个包含单个 `LUID_AND_ATTRIBUTES` 条目（查找到的 LUID 及 `SE_PRIVILEGE_ENABLED` 属性）的 `TOKEN_PRIVILEGES` 结构，并调用 `AdjustTokenPrivileges`。成功时记录 `"enable_inc_base_priority_privilege: AdjustTokenPrivileges succeeded"`；失败时记录 `"enable_inc_base_priority_privilege: AdjustTokenPrivileges failed"`。

4. **清理** — 无论结果如何，通过 `CloseHandle` 关闭令牌句柄。

### 与 enable_debug_privilege 的关系

[enable_debug_privilege](enable_debug_privilege.md) 和 `enable_inc_base_priority_privilege` 遵循相同的三步模式，仅在特权名称常量上有所不同：

| 函数 | 特权常量 | SE_* 名称 |
|------|---------|-----------|
| [enable_debug_privilege](enable_debug_privilege.md) | `SE_DEBUG_NAME` | `SeDebugPrivilege` |
| **enable_inc_base_priority_privilege** | `SE_INC_BASE_PRIORITY_NAME` | `SeIncreaseBasePriorityPrivilege` |

### CLI 退出选项

`--no_inc_base_priority` CLI 标志可跳过调用此函数。当设置该标志时，服务不会尝试启用该特权，超出 `Normal` 的优先级提升可能会根据令牌的默认特权状态静默失败。

### 前置条件

该特权必须已经存在于进程令牌中（但处于禁用状态）。这通常适用于以 `Administrators` 组成员身份运行且已通过 UAC 提权的进程。非管理员令牌根本不包含此特权，`AdjustTokenPrivileges` 将静默失败（该函数不区分"特权未持有"和其他失败情况）。

### 错误处理

所有错误均被记录但不传播。服务将继续使用可用的特权运行。如果无法启用该特权，任何后续请求 `High` 或 `Realtime` 优先级的 `SetPriorityClass` 调用将失败，并由 [apply_priority](../apply.rs/apply_priority.md) 函数记录。

## 要求

| | |
|---|---|
| **模块** | `winapi` (`src/winapi.rs`) |
| **可见性** | `pub` |
| **调用者** | [`main`](../main.rs/README.md)（启动期间，除非设置了 `--no_inc_base_priority`） |
| **被调用者** | `OpenProcessToken`、`GetCurrentProcess`、`LookupPrivilegeValueW`、`AdjustTokenPrivileges`、`CloseHandle`（Win32 Security / Threading） |
| **API** | [`AdjustTokenPrivileges`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges)、[`LookupPrivilegeValueW`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lookupprivilegevaluew) |
| **特权** | 令牌必须已包含 `SeIncreaseBasePriorityPrivilege`（已禁用）。需要管理员提权。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 调试特权启用（相同模式） | [enable_debug_privilege](enable_debug_privilege.md) |
| UAC 提权请求 | [request_uac_elevation](request_uac_elevation.md) |
| 管理员检查 | [is_running_as_admin](is_running_as_admin.md) |
| 依赖此特权的优先级应用 | [apply_priority](../apply.rs/apply_priority.md) |
| 进程优先级枚举 | [ProcessPriority](../priority.rs/README.md) |
| CLI 参数 | [cli 模块](../cli.rs/README.md) |
| AdjustTokenPrivileges (MSDN) | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd