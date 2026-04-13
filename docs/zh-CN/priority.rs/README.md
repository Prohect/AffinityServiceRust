# priority 模块 (AffinityServiceRust)

`priority` 模块定义了强类型枚举和辅助类型，用于表示 Windows 进程优先级类、I/O 优先级级别、内存优先级级别和线程优先级级别。每个枚举都携带一个查找表，在 Rust 变体、人类可读的字符串名称和对应的 Win32 常量值之间进行映射。该模块在每个优先级枚举上提供统一的 `as_str`、`as_win_const`、`from_str` 和 `from_win_const` 转换方法，使得在配置文本、Rust 逻辑和 Windows API 调用之间进行往返转换变得简单直接。每个枚举上的 `None` 变体充当哨兵值，表示"未请求更改"。

## 枚举

| 枚举 | 描述 |
|------|------|
| [ProcessPriority](ProcessPriority.md) | Windows 进程优先级类（从 `Idle` 到 `Realtime`），包装 `PROCESS_CREATION_FLAGS`。 |
| [IOPriority](IOPriority.md) | I/O 优先级级别（从 `VeryLow` 到 `High`），与 `NtSetInformationProcess` 配合使用。 |
| [MemoryPriority](MemoryPriority.md) | 内存优先级级别（从 `VeryLow` 到 `Normal`），包装 `MEMORY_PRIORITY` 常量。 |
| [ThreadPriority](ThreadPriority.md) | 线程优先级级别（从 `Idle` 到 `TimeCritical`），包括后台模式哨兵值和 `boost_one` 辅助方法。 |

## 结构体

| 结构体 | 描述 |
|--------|------|
| [MemoryPriorityInformation](MemoryPriorityInformation.md) | `#[repr(C)]` 新类型包装器，包装 `u32`，用作 `SetProcessInformation` / `GetProcessInformation` 内存优先级调用的缓冲区。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 配置解析（读取优先级字符串） | [config 模块](../config.rs/README.md) |
| 规则应用（调用 `as_win_const`） | [apply 模块](../apply.rs/README.md) |
| 错误码转换 | [error_codes 模块](../error_codes.rs/README.md) |
| 日志记录和失败跟踪 | [logging 模块](../logging.rs/README.md) |