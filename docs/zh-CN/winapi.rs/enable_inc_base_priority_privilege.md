# enable_inc_base_priority_privilege 函数 (winapi.rs)

为当前进程令牌启用 `SeIncreaseBasePriorityPrivilege`，使应用程序能够将进程优先级类设置为 `Normal` 以上，包括 `High` 和 `Realtime`。

## 语法

```rust
pub fn enable_inc_base_priority_privilege()
```

## 参数

此函数不接受参数。

## 返回值

此函数不返回值。成功或失败在内部记录日志。

## 备注

`SeIncreaseBasePriorityPrivilege` 是一项 Windows 特权，允许进程提高其他进程的基础优先级。没有此特权时，尝试通过 `SetPriorityClass` 将进程优先级类设置为 `High` 或 `Realtime` 会因非当前用户拥有的进程而失败并返回 `ERROR_ACCESS_DENIED`，或者可能被静默限制。

该函数执行以下步骤：

1. 通过 `OpenProcessToken` 以 `TOKEN_ADJUST_PRIVILEGES` 访问权限打开当前进程令牌。
2. 通过 `LookupPrivilegeValueW` 查找 `SeIncreaseBasePriorityPrivilege` 特权名称的 LUID（本地唯一标识符）。
3. 调用 `AdjustTokenPrivileges` 在令牌上启用该特权。

如果任何步骤失败，错误将被记录但应用程序继续运行。需要提升优先级类的进程将无法设置其优先级，这些错误将通过正常的 [`is_new_error`](../logging.rs/is_new_error.md) 去重路径记录。

此函数在启动期间由 [`main`](../main.rs/main.md) 调用一次，除非在 [`CliArgs`](../cli.rs/CliArgs.md) 中设置了 `--no-inc-base-priority` CLI 标志（此时将完全跳过调用）。

### 特权要求

与 [`enable_debug_privilege`](enable_debug_privilege.md) 类似，此函数需要进程在提升的（管理员）上下文中运行，`AdjustTokenPrivileges` 才能成功。如果进程未提升，特权调整将静默失败。这在 UAC 提升前阶段是预期行为。

### 与 apply_priority 的关系

[`apply.rs`](../apply.rs/README.md) 模块中的 [`apply_priority`](../apply.rs/apply_priority.md) 函数依赖此特权已启用，才能成功为目标进程设置 `High` 和 `Realtime` 优先级类。没有此特权时，仅能可靠地设置 `Idle`、`BelowNormal`、`Normal` 和 `AboveNormal`。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/winapi.rs |
| **源码行** | L543–L581 |
| **调用方** | [`main`](../main.rs/main.md) |
| **Windows API** | [OpenProcessToken](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocesstoken)、[LookupPrivilegeValueW](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lookupprivilegevaluew)、[AdjustTokenPrivileges](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges) |

## 另请参阅

- [enable_debug_privilege](enable_debug_privilege.md)
- [apply_priority](../apply.rs/apply_priority.md)
- [is_running_as_admin](is_running_as_admin.md)
- [CliArgs](../cli.rs/CliArgs.md)（`no_inc_base_priority` 标志）
- [winapi.rs 模块概述](README.md)