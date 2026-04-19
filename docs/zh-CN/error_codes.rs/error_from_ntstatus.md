# error_from_ntstatus 函数 (error_codes.rs)

将 NTSTATUS 代码映射为人类可读的常量名称。在整个项目中用于在原生 API 调用（`Nt*` / `Zw*` 函数）返回失败状态码时生成有意义的日志消息。

## 语法

```rust
pub fn error_from_ntstatus(status: i32) -> String
```

## 参数

`status: i32`

原生 Windows API 函数返回的 NTSTATUS 代码。NTSTATUS 值是有符号 32 位整数，其中高位表示严重性：第 31 位被置位的值（解释为 `i32` 时为负数）是错误码，零表示成功，较小的正值表示信息性状态。

## 返回值

`String` — 状态码的符号名称（例如 `"STATUS_ACCESS_DENIED"`）。如果代码不在查找表中，则返回格式化的回退字符串，形式为 `"NTSTATUS_0x{code:08X}"`，使用无符号表示。

## 备注

### 已识别的代码

该函数通过对 `status` 的无符号表示（通过 `i32::cast_unsigned` 获得）进行 `match` 来识别以下 NTSTATUS 代码：

| 代码 | 名称 |
|------|------|
| `0x00000000` | `STATUS_SUCCESS` |
| `0x00000001` | `STATUS_WAIT_1` |
| `0xC0000001` | `STATUS_UNSUCCESSFUL` |
| `0xC0000002` | `STATUS_NOT_IMPLEMENTED` |
| `0xC0000003` | `STATUS_INVALID_INFO_CLASS` |
| `0xC0000004` | `STATUS_INFO_LENGTH_MISMATCH` |
| `0xC0000008` | `STATUS_INVALID_HANDLE` |
| `0xC000000D` | `STATUS_INVALID_PARAMETER` |
| `0xC0000017` | `STATUS_NO_MEMORY` |
| `0xC0000018` | `STATUS_CONFLICTING_ADDRESSES` |
| `0xC0000022` | `STATUS_ACCESS_DENIED` |
| `0xC0000023` | `STATUS_BUFFER_TOO_SMALL` |
| `0xC0000034` | `STATUS_OBJECT_NAME_NOT_FOUND` |
| `0xC000004B` | `STATUS_THREAD_IS_TERMINATING` |
| `0xC0000061` | `STATUS_PRIVILEGE_NOT_HELD` |
| `0xC00000BB` | `STATUS_NOT_SUPPORTED` |
| `0xC000010A` | `STATUS_PROCESS_IS_TERMINATING` |

### 无符号转换

该函数使用 `i32::cast_unsigned(status)` 将有符号 `i32` 转换为其无符号 `u32` 位等价值后再进行匹配。这避免了使用负数字面量模式的需要，并且与 Windows SDK 头文件（例如 `ntstatus.h`）中 NTSTATUS 代码的惯用十六进制表示保持一致。

### 与 error_from_code_win32 的关系

该函数是 [error_from_code_win32](error_from_code_win32.md) 的 NTSTATUS 对应版本，后者处理 Win32 错误码。两个函数覆盖不同的错误码空间：

- **Win32 错误** — 由 `GetLastError()` 等函数返回，通常是较小的正整数。
- **NTSTATUS 代码** — 由 `Nt*`/`Zw*` 原生 API 函数返回，错误值通常为 `0xC*` 格式。

调用方根据产生错误码的 API 选择相应的函数。

### 回退格式

未知代码被格式化为 `"NTSTATUS_0x{code:08X}"`，其中 `code` 被转换为 `u32` 进行显示，确保无论输入的符号如何，都能产生一致的无符号十六进制输出。

## 要求

| | |
|---|---|
| **模块** | `src/error_codes.rs` |
| **调用方** | [apply_io_priority](../apply.rs/apply_io_priority.md)、[apply_memory_priority](../apply.rs/apply_memory_priority.md)、[winapi.rs](../winapi.rs/README.md) 中的原生 API 包装器 |
| **被调用方** | 无 |
| **Win32 API** | 无（纯查找函数） |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概述 | [error_codes.rs](README.md) |
| Win32 错误码查找 | [error_from_code_win32](error_from_code_win32.md) |
| IO 优先级应用 | [apply_io_priority](../apply.rs/apply_io_priority.md) |
| 内存优先级应用 | [apply_memory_priority](../apply.rs/apply_memory_priority.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*