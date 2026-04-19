# enable_debug_privilege 函数 (winapi.rs)

在当前进程令牌上启用 `SeDebugPrivilege` 特权。此特权允许服务打开受保护和系统进程的句柄，否则这些进程会拒绝访问，从而实现对所有运行进程的完全进程/线程检查和修改。

## 语法

```rust
pub fn enable_debug_privilege(no_debug_priv: bool)
```

## 参数

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `no_debug_priv` | `bool` | 如果为 `true`，函数将记录消息并立即返回，不尝试启用特权。由 `-noDebugPriv` CLI 标志控制。 |

## 返回值

此函数不返回任何值。成功或失败通过日志消息传达。

## 说明

函数执行以下 Win32 API 调用序列来启用特权：

1. **OpenProcessToken** — 使用 `TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY` 访问权限打开当前进程令牌（`GetCurrentProcess()`）。
2. **LookupPrivilegeValueW** — 将 `SE_DEBUG_NAME` 字符串常量解析为标识本地系统上 `SeDebugPrivilege` 的 `LUID`。
3. **AdjustTokenPrivileges** — 通过传递具有 `SE_PRIVILEGE_ENABLED` 设置为解析后的 LUID 的 `TOKEN_PRIVILEGES` 结构来启用特权。
4. **CloseHandle** — 无论成功还是失败，在调整完成后关闭令牌句柄。

### 失败行为

每个 API 调用都会单独检查。如果任何调用失败，函数将记录描述性错误消息并提前返回。无论何时成功打开令牌句柄，都会始终关闭它。失败不是致命的——服务继续运行，但能力降低（无法打开某些受保护进程的句柄）。

### CLI 标志的早期退出

当 `no_debug_priv` 为 `true`（用户在命令行上传递了 `-noDebugPriv`）时，函数记录 `"SeDebugPrivilege disabled by -noDebugPriv flag"` 并返回，不接触令牌。这允许用户在受限环境中运行服务，以降低特权进行测试，或者在无法授予特权的环境中运行。

### 何时需要此特权？

没有 `SeDebugPrivilege`，针对由其他用户、系统进程或受保护进程拥有的进程的 `OpenProcess` 和 `OpenThread` 调用将因 `ERROR_ACCESS_DENIED`（5）而失败。启用特权允许服务：

- 打开所有进程的句柄（包括 `csrss.exe`、`lsass.exe`、`System` 等）
- 查询和修改系统上任何线程的调度参数
- 在保护进程中枚举模块以进行地址解析

### 特权与提升

`SeDebugPrivilege` 通常存在于提升的管理员进程的令牌中，但默认情况下被禁用。此函数*启用*已经存在的特权——它不授予令牌没有的特权。以没有令牌中特权的非管理员身份运行将导致 `AdjustTokenPrivileges` 成功，但特权实际上不会被启用（可以通过 `GetLastError() == ERROR_NOT_ALL_ASSIGNED` 检测此条件，但此处未执行此检查）。

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用方** | `src/main.rs` 中的主启动逻辑 |
| **被调用方** | 无（叶函数） |
| **Win32 API** | [OpenProcessToken](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocesstoken), [LookupPrivilegeValueW](https://learn.microsoft.com/zh-cn/windows/win32/api/winbase/nf-winbase-lookupprivilegevaluew) (`SE_DEBUG_NAME`), [AdjustTokenPrivileges](https://learn.microsoft.com/zh-cn/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges), [GetCurrentProcess](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocess), [CloseHandle](https://learn.microsoft.com/zh-cn/windows/win32/api/handleapi/nf-handleapi-closehandle) |
| **特权** | 要求调用进程的令牌已经包含 `SeDebugPrivilege`（提升的管理员令牌的标准配置）。函数启用它；它不能添加它。 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [winapi.rs](README.md) |
| 配套特权启用 | [enable_inc_base_priority_privilege](enable_inc_base_priority_privilege.md) |
| 提升检查 | [is_running_as_admin](is_running_as_admin.md) |
| UAC 提升请求 | [request_uac_elevation](request_uac_elevation.md) |
| 进程句柄打开（从此特权受益） | [get_process_handle](get_process_handle.md) |
| 线程句柄打开（从此特权受益） | [get_thread_handle](get_thread_handle.md) |

*文档针对提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*