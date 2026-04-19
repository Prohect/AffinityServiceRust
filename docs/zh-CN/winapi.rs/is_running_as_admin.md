# is_running_as_admin 函数 (winapi.rs)

通过查询进程令牌中的提升状态，检查当前进程是否以管理员（提升）权限运行。

## 语法

```rust
pub fn is_running_as_admin() -> bool
```

## 参数

此函数不接受任何参数。

## 返回值

`bool` — 如果当前进程令牌指示进程已提升（以管理员身份运行），则返回 `true`；否则返回 `false`，包括任何中间 API 调用失败时。

## 说明

函数使用以下 Win32 API 调用序列查询当前进程令牌：

1. **OpenProcessToken** — 以 `TOKEN_QUERY` 访问权限打开当前进程（`GetCurrentProcess()`）的令牌。
2. **GetTokenInformation** — 查询 `TokenElevation` 信息类，填充 `TOKEN_ELEVATION` 结构。
3. **CloseHandle** — 查询后关闭令牌句柄。

结果由 `TOKEN_ELEVATION` 结构的 `TokenIsElevated` 字段确定。非零值表示进程已提升。

### 失败行为

如果 `OpenProcessToken` 或 `GetTokenInformation` 中的任何一个失败，函数返回 `false` 而不是传播错误。这种保守的默认值确保服务在无法检查令牌时将自己视为未提升，这会通过 [request_uac_elevation](request_uac_elevation.md) 触发 UAC 提升流。

### 在服务中的使用

此函数在 service 启动期间被调用，以确定是否需要 UAC 提升。如果 `is_running_as_admin()` 返回 `false` 且 CLI 标志 `noUAC` 未设置，服务会通过 [request_uac_elevation](request_uac_elevation.md) 启动一个提升的自身副本，并退出当前（未提升）进程。

### 令牌提升与权限检查

此函数检查令牌*提升状态*，而不是检查特定权限（如 `SeDebugPrivilege`）是否已启用。提升表示进程是通过 UAC 同意提示启动的，或者正在以高完整性令牌运行。提升后，仍然需要明确启用单个权限 — 参见 [enable_debug_privilege](enable_debug_privilege.md) 和 [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md)。

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用方** | `src/main.rs` 中的主启动逻辑 |
| **被调用方** | 无（叶函数） |
| **Win32 API** | [OpenProcessToken](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocesstoken), [GetTokenInformation](https://learn.microsoft.com/zh-cn/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation) (`TokenElevation`), [GetCurrentProcess](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocess), [CloseHandle](https://learn.microsoft.com/zh-cn/windows/win32/api/handleapi/nf-handleapi-closehandle) |
| **特权** | 不需要 — 始终可用进程自身令牌的 `TOKEN_QUERY` 访问权限 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [winapi.rs](README.md) |
| UAC 提升请求 | [request_uac_elevation](request_uac_elevation.md) |
| 调试权限启用 | [enable_debug_privilege](enable_debug_privilege.md) |
| 基础优先级权限启用 | [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md) |

*文档版本：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*