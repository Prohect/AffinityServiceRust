# is_running_as_admin 函数 (winapi.rs)

通过查询进程令牌的提升状态，检查当前进程是否以管理员（提升）权限运行。

## 语法

```rust
pub fn is_running_as_admin() -> bool
```

## 参数

此函数不接受参数。

## 返回值

如果当前进程令牌指示进程正在以提升权限（管理员）运行，则返回 `true`。如果进程未提升，或任何底层 Windows API 调用失败，则返回 `false`。

## 备注

该函数执行以下步骤：

1. 通过 `GetCurrentProcess` 获取当前进程的句柄。
2. 使用 `OpenProcessToken` 以 `TOKEN_QUERY` 访问权限打开进程令牌。
3. 通过 `GetTokenInformation` 查询令牌的 `TokenElevation` 信息。
4. 检查 `TOKEN_ELEVATION.TokenIsElevated` 字段——非零值表示进程已提升。
5. 在返回之前关闭令牌句柄。

如果 `OpenProcessToken` 或 `GetTokenInformation` 失败，函数返回 `false` 作为保守默认值（假定未提升）。

此函数通常在启动早期被调用，以确定是否需要 UAC 提升。如果返回 `false` 且服务需要管理员权限，调用方可以调用 [request_uac_elevation](request_uac_elevation.md) 以提升权限重新启动进程。

### 平台说明

- **仅限 Windows。** 使用 Win32 安全 API 中的 `TOKEN_ELEVATION` 和 `TokenElevation`。
- 在禁用 UAC 的系统上，该函数仍然根据令牌返回正确的提升状态。
- 该函数不缓存其结果。每次调用都会重新查询令牌，但实际上进程的提升状态在启动后不会改变。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `winapi.rs` |
| **调用方** | `main.rs` — 启动逻辑，用于决定是否需要 UAC 提升。 |
| **被调用方** | `GetCurrentProcess`、`OpenProcessToken`、`GetTokenInformation`、`CloseHandle`（Win32 API） |
| **API** | [GetTokenInformation](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation)，使用 `TokenElevation` 类 |
| **权限** | 无需特殊权限——每个进程都可以查询自己的令牌。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| request_uac_elevation | [request_uac_elevation](request_uac_elevation.md) |
| enable_debug_privilege | [enable_debug_privilege](enable_debug_privilege.md) |
| enable_inc_base_priority_privilege | [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md) |
| winapi 模块概述 | [README](README.md) |

---
*提交：[29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
