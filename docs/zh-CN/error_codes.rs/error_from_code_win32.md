# error_from_code_win32 函数 (error_codes.rs)

将 Win32 错误代码转换为人类可读的字符串表示。

## 语法

```rust
pub fn error_from_code_win32(code: u32) -> String
```

## 参数

`code`

一个 `u32` 类型的 Win32 错误代码。通常由 `GetLastError` 或其他 Windows API 函数返回。

## 返回值

返回一个 `String`，包含该错误代码对应的命名常量。

- 对于已知的错误代码，返回其符号名称（例如 `"ACCESS_DENIED"`、`"INVALID_HANDLE"`）。
- 对于未知的错误代码，格式化为十六进制字符串：`"WIN32_ERROR_CODE_0x{code:08X}"`。

## 备注

本函数维护一个约 45 个常见 Win32 错误代码的静态映射表，使用 `match` 语句实现。返回的字符串是简化的符号名称，省略了 `ERROR_` 前缀（与 Windows SDK 头文件中的 `ERROR_ACCESS_DENIED` 等完整常量名不同）。

### 已知错误代码映射表

| 代码 | 返回值 | Windows SDK 常量 |
| ---: | --- | --- |
| 0 | `"SUCCESS"` | `ERROR_SUCCESS` |
| 2 | `"FILE_NOT_FOUND"` | `ERROR_FILE_NOT_FOUND` |
| 5 | `"ACCESS_DENIED"` | `ERROR_ACCESS_DENIED` |
| 6 | `"INVALID_HANDLE"` | `ERROR_INVALID_HANDLE` |
| 8 | `"NOT_ENOUGH_MEMORY"` | `ERROR_NOT_ENOUGH_MEMORY` |
| 31 | `"ERROR_GEN_FAILURE"` | `ERROR_GEN_FAILURE` |
| 87 | `"INVALID_PARAMETER"` | `ERROR_INVALID_PARAMETER` |
| 122 | `"INSUFFICIENT_BUFFER"` | `ERROR_INSUFFICIENT_BUFFER` |
| 126 | `"MOD_NOT_FOUND"` | `ERROR_MOD_NOT_FOUND` |
| 127 | `"PROC_NOT_FOUND"` | `ERROR_PROC_NOT_FOUND` |
| 193 | `"BAD_EXE_FORMAT"` | `ERROR_BAD_EXE_FORMAT` |
| 565 | `"TOO_MANY_THREADS"` | `ERROR_TOO_MANY_THREADS` |
| 566 | `"THREAD_NOT_IN_PROCESS"` | `ERROR_THREAD_NOT_IN_PROCESS` |
| 567 | `"PAGEFILE_QUOTA_EXCEEDED"` | `ERROR_PAGEFILE_QUOTA_EXCEEDED` |
| 571 | `"IO_PRIVILEGE_FAILED"` | `ERROR_IO_PRIVILEGE_FAILED` |
| 577 | `"INVALID_IMAGE_HASH"` | `ERROR_INVALID_IMAGE_HASH` |
| 633 | `"DRIVER_FAILED_SLEEP"` | `ERROR_DRIVER_FAILED_SLEEP` |
| 998 | `"NOACCESS"` | `ERROR_NOACCESS` |
| 1003 | `"CALLER_CANNOT_MAP_VIEW"` | `ERROR_CALLER_CANNOT_MAP_VIEW` |
| 1006 | `"VOLUME_CHANGED"` | `ERROR_VOLUME_CHANGED` |
| 1007 | `"FULLSCREEN_MODE"` | `ERROR_FULLSCREEN_MODE` |
| 1008 | `"INVALID_HANDLE_STATE"` | `ERROR_INVALID_HANDLE_STATE` |
| 1058 | `"SERVICE_DISABLED"` | `ERROR_SERVICE_DISABLED` |
| 1060 | `"SERVICE_DOES_NOT_EXIST"` | `ERROR_SERVICE_DOES_NOT_EXIST` |
| 1062 | `"SERVICE_NOT_STARTED"` | `ERROR_SERVICE_NOT_STARTED` |
| 1073 | `"ALREADY_RUNNING"` | `ERROR_SERVICE_ALREADY_RUNNING` |
| 1314 | `"PRIVILEGE_NOT_HELD"` | `ERROR_PRIVILEGE_NOT_HELD` |
| 1330 | `"INVALID_ACCOUNT_NAME"` | `ERROR_INVALID_ACCOUNT_NAME` |
| 1331 | `"LOGON_FAILURE"` | `ERROR_LOGON_FAILURE` |
| 1332 | `"ACCOUNT_RESTRICTION"` | `ERROR_ACCOUNT_RESTRICTION` |
| 1344 | `"NO_LOGON_SERVERS"` | `ERROR_NO_LOGON_SERVERS` |
| 1346 | `"RPC_AUTH_LEVEL_MISMATCH"` | `ERROR_RPC_AUTH_LEVEL_MISMATCH` |
| 1444 | `"INVALID_THREAD_ID"` | `ERROR_INVALID_THREAD_ID` |
| 1445 | `"NON_MDICHILD_WINDOW"` | `ERROR_NON_MDICHILD_WINDOW` |
| 1450 | `"NO_SYSTEM_RESOURCES"` | `ERROR_NO_SYSTEM_RESOURCES` |
| 1453 | `"QUOTA_EXCEEDED"` | `ERROR_WORKING_SET_QUOTA` |
| 1455 | `"PAGEFILE_TOO_SMALL"` | `ERROR_COMMITMENT_LIMIT` |
| 1460 | `"TIMEOUT"` | `ERROR_TIMEOUT` |
| 1500 | `"EVT_INVALID_CHANNEL"` | `ERROR_EVT_INVALID_CHANNEL_PATH` |
| 1503 | `"EVT_CHANNEL_ALREADY_EXISTS"` | `ERROR_EVT_CHANNEL_ALREADY_EXISTS` |

### 调用场景

本函数主要在以下模块中被调用，用于将 Windows API 调用失败时 `GetLastError` 返回的错误代码转换为日志可读的格式：

- **apply.rs** — 在 [`log_error_if_new`](../apply.rs/log_error_if_new.md) 中格式化配置应用错误。
- **winapi.rs** — 在进程/线程句柄操作失败时记录错误。
- **logging.rs** — 在日志子系统中格式化错误信息。

### 示例

```rust
// 已知代码
assert_eq!(error_from_code_win32(5), "ACCESS_DENIED");
assert_eq!(error_from_code_win32(87), "INVALID_PARAMETER");

// 未知代码
assert_eq!(error_from_code_win32(9999), "WIN32_ERROR_CODE_0x0000270F");
```

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/error_codes.rs |
| **源码行** | L1–L46 |
| **调用者** | [apply.rs](../apply.rs/README.md)、winapi.rs、logging.rs |

## 另请参阅

- [error_codes.rs 模块概述](README.md)
- [error_from_ntstatus](error_from_ntstatus.md)
- [log_error_if_new](../apply.rs/log_error_if_new.md)