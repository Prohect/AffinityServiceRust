# error_from_code_win32 函数 (error_codes.rs)

将 `u32` 类型的 Win32 错误代码转换为其常用的符号名称字符串。此函数为在进程和线程管理、权限操作以及 ETW 跟踪过程中最常遇到的 Win32 错误代码提供人类可读的翻译。未识别的错误代码将格式化为零填充的十六进制字符串。

## 语法

```rust
pub fn error_from_code_win32(code: u32) -> String
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `code` | `u32` | Win32 错误代码，通常从 `GetLastError()` 或 `windows::core::Error` 的错误负载中获取。 |

## 返回值

返回一个包含错误代码符号名称的 `String`。如果代码未被识别，则返回格式为 `"WIN32_ERROR_CODE_0x{code:08X}"` 的字符串（例如 `"WIN32_ERROR_CODE_0x000003E9"`）。

## 已识别的错误代码

| 代码 | 返回字符串 | Win32 常量 |
|------|-----------|------------|
| `0` | `"SUCCESS"` | `ERROR_SUCCESS` |
| `2` | `"FILE_NOT_FOUND"` | `ERROR_FILE_NOT_FOUND` |
| `5` | `"ACCESS_DENIED"` | `ERROR_ACCESS_DENIED` |
| `6` | `"INVALID_HANDLE"` | `ERROR_INVALID_HANDLE` |
| `8` | `"NOT_ENOUGH_MEMORY"` | `ERROR_NOT_ENOUGH_MEMORY` |
| `31` | `"ERROR_GEN_FAILURE"` | `ERROR_GEN_FAILURE` |
| `87` | `"INVALID_PARAMETER"` | `ERROR_INVALID_PARAMETER` |
| `122` | `"INSUFFICIENT_BUFFER"` | `ERROR_INSUFFICIENT_BUFFER` |
| `126` | `"MOD_NOT_FOUND"` | `ERROR_MOD_NOT_FOUND` |
| `127` | `"PROC_NOT_FOUND"` | `ERROR_PROC_NOT_FOUND` |
| `193` | `"BAD_EXE_FORMAT"` | `ERROR_BAD_EXE_FORMAT` |
| `565` | `"TOO_MANY_THREADS"` | `ERROR_TOO_MANY_THREADS` |
| `566` | `"THREAD_NOT_IN_PROCESS"` | `ERROR_THREAD_NOT_IN_PROCESS` |
| `567` | `"PAGEFILE_QUOTA_EXCEEDED"` | `ERROR_PAGEFILE_QUOTA_EXCEEDED` |
| `571` | `"IO_PRIVILEGE_FAILED"` | `ERROR_IO_PRIVILEGE_FAILED` |
| `577` | `"INVALID_IMAGE_HASH"` | `ERROR_INVALID_IMAGE_HASH` |
| `633` | `"DRIVER_FAILED_SLEEP"` | `ERROR_DRIVER_FAILED_SLEEP` |
| `998` | `"NOACCESS"` | `ERROR_NOACCESS` |
| `1003` | `"CALLER_CANNOT_MAP_VIEW"` | `ERROR_CALLER_CANNOT_MAP_VIEW` |
| `1006` | `"VOLUME_CHANGED"` | `ERROR_VOLUME_CHANGED` |
| `1007` | `"FULLSCREEN_MODE"` | `ERROR_FULLSCREEN_MODE` |
| `1008` | `"INVALID_HANDLE_STATE"` | `ERROR_INVALID_HANDLE_STATE` |
| `1058` | `"SERVICE_DISABLED"` | `ERROR_SERVICE_DISABLED` |
| `1060` | `"SERVICE_DOES_NOT_EXIST"` | `ERROR_SERVICE_DOES_NOT_EXIST` |
| `1062` | `"SERVICE_NOT_STARTED"` | `ERROR_SERVICE_NOT_STARTED` |
| `1073` | `"ALREADY_RUNNING"` | `ERROR_SERVICE_ALREADY_RUNNING` |
| `1314` | `"PRIVILEGE_NOT_HELD"` | `ERROR_PRIVILEGE_NOT_HELD` |
| `1330` | `"INVALID_ACCOUNT_NAME"` | `ERROR_INVALID_ACCOUNT_NAME` |
| `1331` | `"LOGON_FAILURE"` | `ERROR_LOGON_FAILURE` |
| `1332` | `"ACCOUNT_RESTRICTION"` | `ERROR_ACCOUNT_RESTRICTION` |
| `1344` | `"NO_LOGON_SERVERS"` | `ERROR_NO_LOGON_SERVERS` |
| `1346` | `"RPC_AUTH_LEVEL_MISMATCH"` | `RPC_S_AUTHN_LEVEL_NOT_SUPPORTED` |
| `1444` | `"INVALID_THREAD_ID"` | `ERROR_INVALID_THREAD_ID` |
| `1445` | `"NON_MDICHILD_WINDOW"` | `ERROR_NON_MDICHILD_WINDOW` |
| `1450` | `"NO_SYSTEM_RESOURCES"` | `ERROR_NO_SYSTEM_RESOURCES` |
| `1453` | `"QUOTA_EXCEEDED"` | `ERROR_QUOTA_EXCEEDED` |
| `1455` | `"PAGEFILE_TOO_SMALL"` | `ERROR_COMMITMENT_LIMIT` |
| `1460` | `"TIMEOUT"` | `ERROR_TIMEOUT` |
| `1500` | `"EVT_INVALID_CHANNEL"` | `ERROR_EVT_INVALID_CHANNEL_PATH` |
| `1503` | `"EVT_CHANNEL_ALREADY_EXISTS"` | `ERROR_EVT_CHANNEL_ALREADY_EXISTS` |

## 备注

- 该函数使用 `match` 语句对字面整数值进行 O(1) 分发（由 Rust 编译器编译为跳转表或二分查找）。不使用哈希映射或外部查找表。

- 每个匹配分支通过 `.to_string()` 分配一个新的 `String`。仅将结果用于格式化或日志记录的调用者可能需要在热路径中考虑此分配开销。

- 已识别的错误代码集合是针对 AffinityServiceRust 遇到的特定错误场景精心筛选的：进程/线程句柄操作、权限管理、ETW 会话管理、模块枚举和服务控制。这**不是**所有 Win32 错误代码的完整映射。

- 回退格式 `"WIN32_ERROR_CODE_0x{code:08X}"` 使用大写十六进制并进行 8 位零填充，生成类似 `WIN32_ERROR_CODE_0x000003E9` 的值。这使得未识别的错误代码可以方便地在 Microsoft 文档中查找或使用 `net helpmsg` 命令查询。

- 在 AffinityServiceRust 上下文中最常见的几个错误代码：
  - `5` (`ACCESS_DENIED`) — 目标进程受保护或已提升权限，而调用者缺少 `SeDebugPrivilege`。
  - `6` (`INVALID_HANDLE`) — 句柄已被提前关闭或从未有效。
  - `87` (`INVALID_PARAMETER`) — API 收到超出范围的参数（例如无效的 CPU 集合 ID、无效的优先级类别）。
  - `1314` (`PRIVILEGE_NOT_HELD`) — 所需的权限（例如 `SeDebugPrivilege`、`SeIncreaseBasePriorityPrivilege`）尚未在进程令牌上启用。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `error_codes.rs` |
| **调用者** | [`get_process_handle`](../winapi.rs/get_process_handle.md)、[`get_thread_handle`](../winapi.rs/get_thread_handle.md)、[`is_affinity_unset`](../winapi.rs/is_affinity_unset.md)、[`EtwProcessMonitor::start`](../event_trace.rs/EtwProcessMonitor.md)、`apply.rs` 规则应用逻辑 |
| **被调用者** | 无（纯函数，无副作用） |
| **依赖** | 仅标准库（`String`、`format!`） |
| **平台** | 函数本身与平台无关；其映射的错误代码是 Windows 特有的。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| error_from_ntstatus 函数 | [error_from_ntstatus](error_from_ntstatus.md) |
| error_codes 模块概述 | [README](README.md) |
| get_process_handle 函数 | [get_process_handle](../winapi.rs/get_process_handle.md) |
| get_thread_handle 函数 | [get_thread_handle](../winapi.rs/get_thread_handle.md) |
| is_affinity_unset 函数 | [is_affinity_unset](../winapi.rs/is_affinity_unset.md) |
| EtwProcessMonitor 结构体 | [EtwProcessMonitor](../event_trace.rs/EtwProcessMonitor.md) |
| logging 模块 | [logging.rs](../logging.rs/README.md) |

---
*提交：[37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
