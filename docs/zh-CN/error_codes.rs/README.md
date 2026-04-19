# error_codes 模块 (AffinityServiceRust)

`error_codes` 模块为 Win32 和 NTSTATUS 错误代码提供人类可读的错误消息查找。应用程序其余部分不直接在日志输出中显示原始数字代码，而是调用这些函数来获取熟悉的符号名称（例如 `ACCESS_DENIED`、`STATUS_INVALID_HANDLE`），这些名称与 Windows SDK 头文件中定义的常量匹配。未知代码则回退到十六进制格式化的字符串。

## 函数

| 名称 | 描述 |
|------|-------------|
| [error_from_code_win32](error_from_code_win32.md) | 将 Win32 `u32` 错误代码映射为其符号名称（例如 `5` → `"ACCESS_DENIED"`）。对于未映射的代码，返回 `"WIN32_ERROR_CODE_0x{code:08X}"`。 |
| [error_from_ntstatus](error_from_ntstatus.md) | 将 NTSTATUS `i32` 状态代码映射为其符号名称（例如 `0xC0000022` → `"STATUS_ACCESS_DENIED"`）。对于未映射的代码，返回 `"NTSTATUS_0x{code:08X}"`。 |

## 备注

两个函数都使用 `match` 语句针对精心筛选的错误代码子集进行匹配，这些错误代码在操作进程句柄、线程优先级、亲和性掩码和其他 Windows 资源管理 API 时常见。该子集故意保持小巧——只包含在 AffinityServiceRust 正常运行或调试过程中观察到的代码。这样可保持查找速度（编译器生成的跳转表），并避免引入完整的 Windows SDK 错误目录。

这些函数是纯函数——它们不执行任何 I/O，不持有状态，并且可以从任何上下文安全调用。

## 另请参阅

| 主题 | 链接 |
|-------|------|
| apply 模块中的错误去重 | [log_error_if_new](../apply.rs/log_error_if_new.md) |
| apply 模块（主要使用者） | [apply.rs](../apply.rs/README.md) |
| Windows API 包装器 | [winapi.rs](../winapi.rs/README.md) |

*文档对应提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*