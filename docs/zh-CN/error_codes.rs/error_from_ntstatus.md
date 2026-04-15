# error_from_ntstatus 函数 (error_codes.rs)

将 `i32` 类型的 NTSTATUS 值转换为其已知的符号名称字符串。此函数为进程和线程管理操作中常见的 NT 原生 API 状态码提供人类可读的翻译，例如 `STATUS_ACCESS_DENIED`、`STATUS_INVALID_HANDLE` 和 `STATUS_PROCESS_IS_TERMINATING`。未知的状态码将被格式化为十六进制字符串。

## 语法

```rust
pub fn error_from_ntstatus(status: i32) -> String
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `status` | `i32` | 由 NT 原生 API 调用（如 `NtQueryInformationProcess`、`NtSetInformationProcess`、`NtQueryInformationThread`、`NtQuerySystemInformation`）返回的 NTSTATUS 值。NTSTATUS 值为有符号 32 位整数，其中负值表示错误，零表示成功，正值表示信息或警告状态。 |

## 返回值

返回一个包含状态码符号名称的 `String`。如果状态码无法识别，函数将返回格式为 `"NTSTATUS_0x{code:08X}"` 的十六进制字符串。

### 可识别的状态码

| NTSTATUS 值 | 无符号十六进制 | 返回字符串 |
|-------------|---------------|-----------|
| `0` | `0x00000000` | `STATUS_SUCCESS` |
| `1` | `0x00000001` | `STATUS_WAIT_1` |
| `-1073741823` | `0xC0000001` | `STATUS_UNSUCCESSFUL` |
| `-1073741822` | `0xC0000002` | `STATUS_NOT_IMPLEMENTED` |
| `-1073741821` | `0xC0000003` | `STATUS_INVALID_INFO_CLASS` |
| `-1073741820` | `0xC0000004` | `STATUS_INFO_LENGTH_MISMATCH` |
| `-1073741816` | `0xC0000008` | `STATUS_INVALID_HANDLE` |
| `-1073741811` | `0xC000000D` | `STATUS_INVALID_PARAMETER` |
| `-1073741801` | `0xC0000017` | `STATUS_NO_MEMORY` |
| `-1073741800` | `0xC0000018` | `STATUS_CONFLICTING_ADDRESSES` |
| `-1073741790` | `0xC0000022` | `STATUS_ACCESS_DENIED` |
| `-1073741789` | `0xC0000023` | `STATUS_BUFFER_TOO_SMALL` |
| `-1073741772` | `0xC0000034` | `STATUS_OBJECT_NAME_NOT_FOUND` |
| `-1073741749` | `0xC000004B` | `STATUS_THREAD_IS_TERMINATING` |
| `-1073741727` | `0xC0000061` | `STATUS_PRIVILEGE_NOT_HELD` |
| `-1073741637` | `0xC00000BB` | `STATUS_NOT_SUPPORTED` |
| `-1073741558` | `0xC000010A` | `STATUS_PROCESS_IS_TERMINATING` |

### 回退格式

无法识别的状态码将被格式化为：

```text
NTSTATUS_0xC0000XXX
```

原始 `i32` 值在进行十六进制格式化之前会被转换为 `u32`，以生成常规的无符号 NTSTATUS 表示形式。

## 备注

- 该函数使用 `i32::cast_unsigned(status)` 将有符号 `i32` 转换为其无符号 `u32` 位等价值后再进行匹配。这是必要的，因为 NTSTATUS 错误码（高两位的严重性位为 `11`）通常以无符号十六进制值表示（如 `0xC0000022`），但在 Rust 绑定中存储为负的 `i32` 值。

- 可识别的状态码集合涵盖了 AffinityServiceRust 运行期间最常见的状态——特别是由 `NtQuerySystemInformation`、`NtQueryInformationProcess`、`NtQueryInformationThread`、`NtSetInformationProcess` 和 `NtSetTimerResolution` 返回的状态。

- `STATUS_INFO_LENGTH_MISMATCH`（`0xC0000004`）在本项目中尤为重要：当传递给 `NtQuerySystemInformation` 的缓冲区太小时，[`ProcessSnapshot::take`](../process.rs/ProcessSnapshot.md) 使用它作为重试信号。

- `STATUS_PROCESS_IS_TERMINATING`（`0xC000010A`）和 `STATUS_THREAD_IS_TERMINATING`（`0xC000004B`）在尝试查询或设置正在退出的进程/线程的属性时经常出现——这在系统级进程管理器中是正常现象。

- 与处理 Win32 错误码（来自 `GetLastError` 的 `u32` 值）的 [`error_from_code_win32`](error_from_code_win32.md) 不同，此函数处理 NTSTATUS 值（由 NT 原生 API 直接返回的 `i32` 值）。这两个错误码空间是不同的，不应混用。

- 该函数每次调用都会分配一个新的 `String`。对于同一状态码被反复翻译的热路径，调用者应考虑缓存结果或有条件地记录日志（如 [`is_new_error`](../logging.rs/is_new_error.md) 所做的那样）。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `error_codes.rs` |
| **调用方** | `winapi.rs`、`process.rs` —— 任何 NT 原生 API 返回需要记录或显示的 NTSTATUS 值的地方。 |
| **被调用方** | 无（纯函数，无副作用） |
| **API** | 无 —— 这是一个查找表，不是 API 包装器 |
| **平台** | 逻辑上与平台无关；它翻译的状态码是 Windows NT 特有的。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| error_from_code_win32 函数 | [error_from_code_win32](error_from_code_win32.md) |
| ProcessSnapshot::take | [ProcessSnapshot](../process.rs/ProcessSnapshot.md) |
| error_codes 模块概述 | [README](README.md) |
| winapi 模块 | [winapi.rs](../winapi.rs/README.md) |

---
> Commit SHA: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
