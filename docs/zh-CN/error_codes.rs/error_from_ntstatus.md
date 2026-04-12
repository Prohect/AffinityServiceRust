# error_from_ntstatus 函数 (error_codes.rs)

将 NTSTATUS 错误代码转换为人类可读的字符串表示。

## 语法

```rust
pub fn error_from_ntstatus(status: i32) -> String
```

## 参数

`status`

一个 `i32` 类型的 NTSTATUS 状态代码。NTSTATUS 是 Windows NT 内核层 API 返回的状态码体系，其高位表示严重性级别（成功、信息、警告、错误）。负值（最高位为 1）表示错误。

## 返回值

返回一个 `String`，包含该 NTSTATUS 代码对应的命名常量。

- 对于已知代码，返回对应的符号名称（例如 `"STATUS_SUCCESS"`、`"STATUS_ACCESS_DENIED"`）。
- 对于未知代码，格式化为十六进制字符串：`"NTSTATUS_0x{code:08X}"`。

## 备注

函数内部首先通过 `i32::cast_unsigned` 将有符号的 `status` 转换为无符号整数进行匹配，这是因为 NTSTATUS 错误代码的官方定义使用无符号十六进制值（如 `0xC0000022`），而 Rust 函数签名接受 `i32` 以匹配 Windows API 的实际返回类型。

### 已知代码映射表

| 代码（十六进制） | 代码（十进制） | 返回字符串 |
| --- | --- | --- |
| `0x00000000` | 0 | `STATUS_SUCCESS` |
| `0x00000001` | 1 | `STATUS_WAIT_1` |
| `0xC0000001` | -1073741823 | `STATUS_UNSUCCESSFUL` |
| `0xC0000002` | -1073741822 | `STATUS_NOT_IMPLEMENTED` |
| `0xC0000003` | -1073741821 | `STATUS_INVALID_INFO_CLASS` |
| `0xC0000004` | -1073741820 | `STATUS_INFO_LENGTH_MISMATCH` |
| `0xC0000008` | -1073741816 | `STATUS_INVALID_HANDLE` |
| `0xC000000D` | -1073741811 | `STATUS_INVALID_PARAMETER` |
| `0xC0000017` | -1073741801 | `STATUS_NO_MEMORY` |
| `0xC0000018` | -1073741800 | `STATUS_CONFLICTING_ADDRESSES` |
| `0xC0000022` | -1073741790 | `STATUS_ACCESS_DENIED` |
| `0xC0000023` | -1073741789 | `STATUS_BUFFER_TOO_SMALL` |
| `0xC0000034` | -1073741772 | `STATUS_OBJECT_NAME_NOT_FOUND` |
| `0xC000004B` | -1073741749 | `STATUS_THREAD_IS_TERMINATING` |
| `0xC0000061` | -1073741727 | `STATUS_PRIVILEGE_NOT_HELD` |
| `0xC00000BB` | -1073741637 | `STATUS_NOT_SUPPORTED` |
| `0xC000010A` | -1073741558 | `STATUS_PROCESS_IS_TERMINATING` |

共映射 **17** 个已知 NTSTATUS 代码（2 个成功/等待代码 + 15 个错误代码）。

### NTSTATUS 代码结构

NTSTATUS 的 32 位值具有以下结构：

- **Bit 31（Sev 高位）**：严重性——`0` = 成功/信息，`1` = 警告/错误
- **Bit 30（Sev 低位）**：严重性——与 Bit 31 组合确定级别
- **Bit 29（C）**：客户代码标志——`1` = 用户定义代码
- **Bit 28（N）**：保留位
- **Bits 27–16**：设施代码（Facility）
- **Bits 15–0**：状态代码

本函数映射的错误代码均以 `0xC0` 开头，表示严重性级别为"错误"（Severity = 3）。

### 典型使用场景

本函数主要用于格式化以下 NT 原生 API 调用的返回值：

- `NtSetInformationProcess` / `NtQueryInformationProcess` — 设置/查询 I/O 优先级时使用
- `NtSetTimerResolution` — 设置系统计时器分辨率时使用

这些 API 直接返回 `NTSTATUS` 而非 Win32 错误代码，因此需要单独的转换函数。

### 与 error_from_code_win32 的区别

| 特性 | error_from_code_win32 | error_from_ntstatus |
| --- | --- | --- |
| 输入类型 | `u32`（无符号） | `i32`（有符号） |
| 代码来源 | Win32 API (`GetLastError`) | NT 原生 API（`NTSTATUS` 返回值） |
| 已知映射数量 | ~45 | 17 |
| 未知格式 | `WIN32_ERROR_CODE_0x{:08X}` | `NTSTATUS_0x{:08X}` |
| 命名前缀 | 无统一前缀 | `STATUS_` |

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/error_codes.rs |
| **源码行** | L47–L70 |
| **调用者** | [apply_io_priority](../apply.rs/apply_io_priority.md)、[apply_memory_priority](../apply.rs/apply_memory_priority.md)（通过日志记录） |

## 另请参阅

- [error_from_code_win32](error_from_code_win32.md) — Win32 错误代码转换函数
- [error_codes.rs 模块概述](README.md)
- [apply_io_priority](../apply.rs/apply_io_priority.md) — 使用 NTSTATUS 返回值的 I/O 优先级设置
- [apply_memory_priority](../apply.rs/apply_memory_priority.md) — 使用 NTSTATUS 返回值的内存优先级设置