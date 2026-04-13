# error_codes 模块 (AffinityServiceRust)

`error_codes` 模块提供 Windows 错误代码的人类可读翻译。它将数值型 Win32 错误代码和 NTSTATUS 值映射为对应的符号常量名称（例如 `5` → `"ACCESS_DENIED"`、`0xC0000022` → `"STATUS_ACCESS_DENIED"`），使 Windows API 调用失败时的日志输出更具可读性。无法识别的代码将格式化为十六进制回退字符串。

## 函数

| 函数 | 描述 |
|------|------|
| [error_from_code_win32](error_from_code_win32.md) | 将 Win32 错误代码（`u32`）映射为人类可读的符号名称字符串。 |
| [error_from_ntstatus](error_from_ntstatus.md) | 将 NTSTATUS 代码（`i32`）映射为人类可读的符号名称字符串。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 日志记录与错误去重 | [logging 模块](../logging.rs/README.md) |
| ETW (Windows 事件跟踪) 会话管理（使用错误格式化） | [event_trace 模块](../event_trace.rs/README.md) |
| 规则应用（错误格式化的主要使用者） | [apply 模块](../apply.rs/README.md) |
| Windows API 封装 | [winapi 模块](../winapi.rs/README.md) |