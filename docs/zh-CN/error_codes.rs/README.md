# error_codes.rs 模块 (error_codes.rs)

`error_codes` 模块提供 Windows 错误代码到人类可读字符串的转换功能。它将数值错误代码映射为命名常量字符串，便于日志记录和调试。

## 概述

Windows API 使用两套不同的错误代码体系：

- **Win32 错误代码** — 由大多数 Win32 API 函数通过 `GetLastError` 返回的 `u32` 值。
- **NTSTATUS 代码** — 由 NT 原生 API（如 `NtSetInformationProcess`、`NtQueryInformationProcess`）返回的 `i32` 值。

本模块为每套体系提供一个转换函数，将已知代码映射为其官方命名常量（例如 `ACCESS_DENIED`、`STATUS_ACCESS_DENIED`），未知代码则格式化为十六进制字符串以便排查。

## 项目

### 函数

| 名称 | 描述 |
| --- | --- |
| [error_from_code_win32](error_from_code_win32.md) | 将 Win32 错误代码（`u32`）转换为人类可读的字符串。 |
| [error_from_ntstatus](error_from_ntstatus.md) | 将 NTSTATUS 代码（`i32`）转换为人类可读的字符串。 |

## 备注

本模块不依赖任何外部 crate 或 Windows API 调用——它是纯粹的 `match` 查找表。映射表仅涵盖项目实际会遇到的常见代码子集，而非完整的 Windows 错误代码空间。

这两个函数在整个项目中被广泛调用，主要用于：

- [`apply.rs`](../apply.rs/README.md) 中的 [`log_error_if_new`](../apply.rs/log_error_if_new.md) — 将 Win32 错误代码转换为日志消息。
- [`winapi.rs`](../winapi.rs/README.md) — 在进程/线程句柄操作失败时格式化错误信息。
- [`logging.rs`](../logging.rs/README.md) — 在错误去重和日志输出中使用。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | `src/error_codes.rs` |
| **行号** | L1–L70 |
| **外部依赖** | 无 |
| **调用者** | `src/apply.rs`、`src/winapi.rs`、`src/logging.rs` |