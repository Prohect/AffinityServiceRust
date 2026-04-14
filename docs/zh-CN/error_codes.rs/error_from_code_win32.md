# error_from_code_win32 函数 (error_codes.rs)

将 Win32 错误代码映射为人类可读的符号名称字符串。此函数对 AffinityServiceRust 中最常遇到的 Win32 错误代码提供静态查找，在 Windows API 调用失败时为日志消息提供有意义的诊断输出。无法识别的代码将格式化为十六进制回退字符串。

## 语法

```error_codes.rs
pub fn error_from_code_win32(code: u32) -> String
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `code` | `u32` | 要翻译的 Win32 错误代码。通常从 `GetLastError` 或 `windows::core::Error` 中嵌入的错误代码获取。 |

## 返回值

一个 `String`，包含该错误代码对应的符号常量名称（例如 `"ACCESS_DENIED"`、`"INVALID_HANDLE"`）。如果该代码不在静态查找表中，则返回格式为 `"WIN32_ERROR_CODE_0x00000000"` 的十六进制字符串。

## 备注

该函数使用 `match` 语句覆盖以下 Win32 错误代码：

| 代码 | 符号名称 |
|------|----------|
| 0 | `SUCCESS` |
| 2 | `FILE_NOT_FOUND` |
| 5 | `ACCESS_DENIED` |
| 6 | `INVALID_HANDLE` |
| 8 | `NOT_ENOUGH_MEMORY` |
| 31 | `ERROR_GEN_FAILURE` |
| 87 | `INVALID_PARAMETER` |
| 122 | `INSUFFICIENT_BUFFER` |
| 126 | `MOD_NOT_FOUND` |
| 127 | `PROC_NOT_FOUND` |
| 193 | `BAD_EXE_FORMAT` |
| 565 | `TOO_MANY_THREADS` |
| 566 | `THREAD_NOT_IN_PROCESS` |
| 567 | `PAGEFILE_QUOTA_EXCEEDED` |
| 571 | `IO_PRIVILEGE_FAILED` |
| 577 | `INVALID_IMAGE_HASH` |
| 633 | `DRIVER_FAILED_SLEEP` |
| 998 | `NOACCESS` |
| 1003 | `CALLER_CANNOT_MAP_VIEW` |
| 1006 | `VOLUME_CHANGED` |
| 1007 | `FULLSCREEN_MODE` |
| 1008 | `INVALID_HANDLE_STATE` |
| 1058 | `SERVICE_DISABLED` |
| 1060 | `SERVICE_DOES_NOT_EXIST` |
| 1062 | `SERVICE_NOT_STARTED` |
| 1073 | `ALREADY_RUNNING` |
| 1314 | `PRIVILEGE_NOT_HELD` |
| 1330 | `INVALID_ACCOUNT_NAME` |
| 1331 | `LOGON_FAILURE` |
| 1332 | `ACCOUNT_RESTRICTION` |
| 1344 | `NO_LOGON_SERVERS` |
| 1346 | `RPC_AUTH_LEVEL_MISMATCH` |
| 1444 | `INVALID_THREAD_ID` |
| 1445 | `NON_MDICHILD_WINDOW` |
| 1450 | `NO_SYSTEM_RESOURCES` |
| 1453 | `QUOTA_EXCEEDED` |
| 1455 | `PAGEFILE_TOO_SMALL` |
| 1460 | `TIMEOUT` |
| 1500 | `EVT_INVALID_CHANNEL` |
| 1503 | `EVT_CHANNEL_ALREADY_EXISTS` |

这是一个静态表，而非对 `FormatMessage` 的调用。它避免了 `FormatMessageW` 的开销和区域设置依赖行为，同时提供一致的、便于 grep 搜索的日志输出。该表覆盖了 AffinityServiceRust 在进程句柄获取、优先级设置、亲和性操作、CPU 集合管理、ETW (Windows 事件跟踪) 会话控制和特权操作过程中实际可能遇到的错误代码。

每个返回的字符串有意省略了官方 Win32 头文件中使用的 `ERROR_` 前缀（例如返回 `"ACCESS_DENIED"` 而非 `"ERROR_ACCESS_DENIED"`），以使日志行更短同时保持无歧义。十六进制回退格式（`WIN32_ERROR_CODE_0x{:08X}`）便于在 Microsoft 文档中查找未记录的代码。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `error_codes` |
| **调用方** | [log_error_if_new](../apply.rs/log_error_if_new.md)、[EtwProcessMonitor::start](../event_trace.rs/EtwProcessMonitor.md)、[get_process_handle](../winapi.rs/get_process_handle.md)、[get_thread_handle](../winapi.rs/get_thread_handle.md) |
| **被调用方** | *（无 ── 纯映射）* |
| **Win32 API** | *（无 ── 此函数不调用任何 Win32 API）* |
| **特权** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| NTSTATUS 代码翻译 | [error_from_ntstatus](error_from_ntstatus.md) |
| 日志中的错误去重 | [is_new_error](../logging.rs/is_new_error.md) |
| error_codes 模块概述 | [error_codes 模块](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd