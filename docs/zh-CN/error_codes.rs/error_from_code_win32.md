# error_from_code_win32 函数 (error_codes.rs)

将 Win32 错误码映射为人类可读的常量名称字符串。在整个项目中用于在 Windows API 调用失败时生成有意义的日志消息。

## 语法

```rust
pub fn error_from_code_win32(code: u32) -> String
```

## 参数

`code: u32`

由 `GetLastError` 返回或从 `HRESULT` 中提取的 Win32 错误码。常见值包括 `5`（ACCESS_DENIED）、`87`（INVALID_PARAMETER）和 `998`（NOACCESS）。

## 返回值

`String` — 错误码的符号名称（例如 `"ACCESS_DENIED"`、`"INVALID_PARAMETER"`）。如果该错误码不在查找表中，则返回格式为 `"WIN32_ERROR_CODE_0x{code:08X}"` 的十六进制格式字符串。

## 备注

该函数使用 `match` 语句对一组精选的 Win32 错误码进行匹配，这些错误码在进程和线程操作过程中经常遇到。该集合并非详尽无遗——它仅涵盖与 AffinityServiceRust 操作最相关的错误码：

### 已识别的错误码

| 错误码 | 名称 | 典型场景 |
|------|------|-----------------|
| 0 | `SUCCESS` | 操作成功完成 |
| 2 | `FILE_NOT_FOUND` | 找不到配置文件或模块路径 |
| 5 | `ACCESS_DENIED` | 权限不足，无法打开/修改进程 |
| 6 | `INVALID_HANDLE` | 过期或已关闭的进程/线程句柄 |
| 8 | `NOT_ENOUGH_MEMORY` | 系统内存耗尽 |
| 31 | `ERROR_GEN_FAILURE` | 一般设备或驱动程序故障 |
| 87 | `INVALID_PARAMETER` | Win32 API 调用的参数无效 |
| 122 | `INSUFFICIENT_BUFFER` | 缓冲区太小，无法容纳查询结果 |
| 126 | `MOD_NOT_FOUND` | 找不到 DLL 或模块 |
| 127 | `PROC_NOT_FOUND` | 在模块中找不到导出函数 |
| 193 | `BAD_EXE_FORMAT` | 无效的可执行映像 |
| 565 | `TOO_MANY_THREADS` | 超出线程限制 |
| 566 | `THREAD_NOT_IN_PROCESS` | 线程不属于目标进程 |
| 567 | `PAGEFILE_QUOTA_EXCEEDED` | 超出页面文件配额 |
| 571 | `IO_PRIVILEGE_FAILED` | I/O 特权操作失败 |
| 577 | `INVALID_IMAGE_HASH` | 代码完整性检查失败 |
| 633 | `DRIVER_FAILED_SLEEP` | 驱动程序无法进入睡眠状态 |
| 998 | `NOACCESS` | 无效的内存位置访问 |
| 1003 | `CALLER_CANNOT_MAP_VIEW` | 调用方无法映射内存视图 |
| 1006 | `VOLUME_CHANGED` | 卷已被外部更改 |
| 1007 | `FULLSCREEN_MODE` | 独占全屏模式冲突 |
| 1008 | `INVALID_HANDLE_STATE` | 句柄处于无效状态 |
| 1058 | `SERVICE_DISABLED` | Windows 服务已禁用 |
| 1060 | `SERVICE_DOES_NOT_EXIST` | 指定的服务不存在 |
| 1062 | `SERVICE_NOT_STARTED` | 服务尚未启动 |
| 1073 | `ALREADY_RUNNING` | 进程或服务已在运行 |
| 1314 | `PRIVILEGE_NOT_HELD` | 调用方未持有所需特权 |
| 1330 | `INVALID_ACCOUNT_NAME` | 无效的帐户名格式 |
| 1331 | `LOGON_FAILURE` | 登录尝试失败 |
| 1332 | `ACCOUNT_RESTRICTION` | 帐户限制阻止了操作 |
| 1344 | `NO_LOGON_SERVERS` | 没有可用的登录服务器 |
| 1346 | `RPC_AUTH_LEVEL_MISMATCH` | RPC 身份验证级别不匹配 |
| 1444 | `INVALID_THREAD_ID` | 无效的线程标识符 |
| 1445 | `NON_MDICHILD_WINDOW` | 不是 MDI 子窗口 |
| 1450 | `NO_SYSTEM_RESOURCES` | 系统资源不足 |
| 1453 | `QUOTA_EXCEEDED` | 超出配额 |
| 1455 | `PAGEFILE_TOO_SMALL` | 页面文件太小 |
| 1460 | `TIMEOUT` | 操作超时 |
| 1500 | `EVT_INVALID_CHANNEL` | 无效的事件通道 |
| 1503 | `EVT_CHANNEL_ALREADY_EXISTS` | 事件通道已存在 |

### 回退格式

对于不在表中的任何错误码，该函数返回类似 `"WIN32_ERROR_CODE_0x00000039"` 的字符串，使用零填充的 8 位大写十六进制。这确保未识别的错误在日志中仍然产生可搜索、无歧义的标识符。

### 设计原理

优先使用静态查找而非在运行时调用 `FormatMessage`，原因如下：避免在错误路径中进行额外的 Win32 API 调用；保持输出确定性和简洁性（常量名称而非本地化语句）；消除 `FormatMessage` 缓冲区的分配和清理开销。

## 要求

| | |
|---|---|
| **模块** | `src/error_codes.rs` |
| **调用方** | [log_error_if_new](../apply.rs/log_error_if_new.md)，以及整个 crate 中的日志辅助函数 |
| **被调用方** | 无（纯函数） |
| **Win32 API** | 无 |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 模块概述 | [error_codes.rs](README.md) |
| NTSTATUS 对应函数 | [error_from_ntstatus](error_from_ntstatus.md) |
| 错误去重 | [log_error_if_new](../apply.rs/log_error_if_new.md) |
| Microsoft Win32 错误码参考 | [System Error Codes](https://learn.microsoft.com/zh-cn/windows/win32/debug/system-error-codes) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*