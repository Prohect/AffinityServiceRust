# enable_debug_privilege 函数 (winapi.rs)

为当前进程令牌启用 `SeDebugPrivilege`，使应用程序能够打开其他用户拥有的进程和系统进程的句柄。

## 语法

```rust
pub fn enable_debug_privilege()
```

## 参数

此函数不接受参数。

## 返回值

此函数不返回值。成功或失败在内部记录日志。

## 备注

`SeDebugPrivilege` 是一项强大的 Windows 特权，授予持有者打开系统上任何进程的能力，无论其安全描述符如何。这对 AffinityService 至关重要，因为它需要查询和修改在不同用户帐户下运行的进程以及系统进程的属性（亲和性、优先级、CPU 集等）。

该函数执行以下步骤：

1. 通过 `OpenProcessToken` 以 `TOKEN_ADJUST_PRIVILEGES` 访问权限打开当前进程令牌。
2. 通过 `LookupPrivilegeValueW` 查找 `SeDebugPrivilege` 特权名称的 LUID（本地唯一标识符）。
3. 调用 `AdjustTokenPrivileges` 在令牌上启用该特权。

如果任何步骤失败，错误将被记录但应用程序继续运行——没有该特权时某些进程将无法访问。函数不会 panic 或返回错误。

此函数在启动期间由 [`main`](../main.rs/main.md) 调用一次，除非设置了 `--no-debug-priv` CLI 标志（此时将完全跳过调用）。

### 特权要求

调用进程必须在提升的（管理员）上下文中运行，`AdjustTokenPrivileges` 才能成功。如果进程未提升，特权调整将静默失败——这在 UAC 提升前阶段是预期行为，也是应用程序通过 [`request_uac_elevation`](request_uac_elevation.md) 请求提升的原因之一。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/winapi.rs |
| **源码行** | L503–L541 |
| **调用方** | [`main`](../main.rs/main.md) |
| **Windows API** | [OpenProcessToken](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocesstoken)、[LookupPrivilegeValueW](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-lookupprivilegevaluew)、[AdjustTokenPrivileges](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges) |

## 另请参阅

- [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md)
- [is_running_as_admin](is_running_as_admin.md)
- [request_uac_elevation](request_uac_elevation.md)
- [winapi.rs 模块概述](README.md)