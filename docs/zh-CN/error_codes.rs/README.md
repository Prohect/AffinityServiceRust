# error_codes 模块 (AffinityServiceRust)

`error_codes` 模块提供 Windows 错误代码的人类可读字符串翻译。它包含两个查找函数，分别将数值型 Win32 错误代码和 NTSTATUS 值映射为其常见的符号名称（例如 `ACCESS_DENIED`、`STATUS_INVALID_HANDLE`）。未知代码将格式化为十六进制字符串。这些函数在 AffinityServiceRust 中广泛用于诊断日志记录和错误报告。

## 函数

| 函数 | 描述 |
|------|------|
| [error_from_code_win32](error_from_code_win32.md) | 将 `u32` Win32 错误代码转换为其符号名称字符串。 |
| [error_from_ntstatus](error_from_ntstatus.md) | 将 `i32` NTSTATUS 值转换为其符号名称字符串。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| winapi 模块 | [../winapi.rs/README.md](../winapi.rs/README.md) |
| logging 模块 | [../logging.rs/README.md](../logging.rs/README.md) |
| event_trace 模块 | [../event_trace.rs/README.md](../event_trace.rs/README.md) |

---

> Commit SHA: `7221ea0694670265d4eb4975582d8ed2ae02439d`
