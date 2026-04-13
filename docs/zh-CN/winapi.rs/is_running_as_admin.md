# is_running_as_admin 函数 (winapi.rs)

通过查询进程令牌的 `TOKEN_ELEVATION` 信息，检查当前进程是否以管理员（提权）特权运行。此函数在启动时用于确定是否需要在服务循环开始前请求 UAC 提权。

## 语法

```rust
pub fn is_running_as_admin() -> bool
```

## 参数

无。

## 返回值

| 值 | 描述 |
|------|------|
| `true` | 当前进程令牌具有提权状态（`TokenIsElevated != 0`），表示进程正在以完整的管理员特权运行。 |
| `false` | 进程未提权，或令牌查询链中的任何步骤失败（令牌打开失败、`GetTokenInformation` 失败）。该函数在出错时默认返回 `false` 而非 panic，因此调用方可以安全地使用该结果来决定是否尝试提权。 |

## 备注

### 算法

该函数依次执行三个 Win32 调用：

1. **`OpenProcessToken`** — 以 `TOKEN_QUERY` 访问权限打开当前进程的令牌。
2. **`GetTokenInformation`** — 查询 `TokenElevation` 信息类，填充一个 `TOKEN_ELEVATION` 结构体。
3. **`CloseHandle`** — 无论 `GetTokenInformation` 的结果如何，都关闭令牌句柄。

令牌句柄在返回前始终会被关闭，即使在失败路径上也是如此，以防止句柄泄漏。

### 失败行为

链中的任何失败都会导致函数返回 `false`：

| 失败点 | 行为 |
|--------|------|
| `OpenProcessToken` 失败 | 立即返回 `false`。 |
| `GetTokenInformation` 失败 | 关闭令牌句柄，返回 `false`。 |
| 成功但 `TokenIsElevated == 0` | 关闭令牌句柄，返回 `false`（未提权）。 |

不记录任何错误——该函数设计为静默运行，因为它在启动序列的早期阶段被调用，此时日志记录可能尚未完全初始化。

### 在启动流程中的使用

主函数调用 `is_running_as_admin()` 来决定是否调用 [request_uac_elevation](request_uac_elevation.md)。典型流程如下：

1. 解析 CLI 参数。
2. 调用 `is_running_as_admin()`。
3. 如果返回 `false` 且未设置 `--no_uac` 标志，则调用 [request_uac_elevation](request_uac_elevation.md) 以管理员权限重新启动进程。
4. 如果返回 `true`，继续执行 [enable_debug_privilege](enable_debug_privilege.md) 和主服务循环。

### UAC 与令牌提权

在启用了 UAC 的 Windows 上，属于管理员组的用户默认以过滤后的（非提权）令牌运行进程。当授予提权（例如通过"以管理员身份运行"或 UAC 提示）时，进程会收到完整的、未过滤的令牌，其中 `TokenIsElevated` 被设置为非零值。此函数检测的正是这种区别。

### 与特权的关系

提权是成功启用 `SeDebugPrivilege`（[enable_debug_privilege](enable_debug_privilege.md)）和 `SeIncreaseBasePriorityPrivilege`（[enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md)）等特权的前提条件。如果没有提权，这些特权调整调用将静默失败，服务将以降低的能力运行（无法打开受保护进程或设置实时优先级）。

## 要求

| | |
|---|---|
| **模块** | `winapi` (`src/winapi.rs`) |
| **可见性** | `pub` |
| **调用方** | [`main`](../main.rs/README.md)（启动序列） |
| **被调用方** | `GetCurrentProcess`、`OpenProcessToken`、`GetTokenInformation`（`TokenElevation`）、`CloseHandle`（Win32） |
| **API** | [`OpenProcessToken`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocesstoken)、[`GetTokenInformation`](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation) |
| **特权** | 无 — 令牌自查询不需要提权 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| UAC 提权请求 | [request_uac_elevation](request_uac_elevation.md) |
| 调试特权启用 | [enable_debug_privilege](enable_debug_privilege.md) |
| 基本优先级特权启用 | [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md) |
| 服务主入口点 | [main 模块](../main.rs/README.md) |
| TOKEN_ELEVATION (MSDN) | [Microsoft Learn — TOKEN_ELEVATION](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/ns-securitybaseapi-token_elevation) |