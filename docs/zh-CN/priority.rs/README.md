# priority 模块 (AffinityServiceRust)

`priority` 模块定义了强类型的 Rust 枚举和辅助类型，将 Windows 进程、线程、I/O 和内存优先级级别映射到对应的 Win32 API 常量。每种类型都提供了人类可读的字符串名称、Rust 枚举变体和 Windows API 所需的原始数值（`PROCESS_CREATION_FLAGS`、`THREAD_PRIORITY`、`MEMORY_PRIORITY` 和未公开的 I/O 优先级整数）之间的双向转换。该模块由配置解析器使用，用于反序列化用户提供的优先级字符串；也由 apply 引擎使用，用于向 `SetPriorityClass`、`SetThreadPriority`、`NtSetInformationProcess` 及相关 Win32 调用传递正确的常量。

## 枚举

| 枚举 | 描述 |
|------|------|
| [ProcessPriority](ProcessPriority.md) | 将六个 Windows 进程优先级类别（`Idle` 到 `Realtime`）加上 `None` 哨兵值映射到 `PROCESS_CREATION_FLAGS` 值。 |
| [IOPriority](IOPriority.md) | 将 Windows I/O 优先级提示（`VeryLow`、`Low`、`Normal`、`High`）加上 `None` 哨兵值映射到 `NtSetInformationProcess` 使用的原始 `u32` 值。 |
| [MemoryPriority](MemoryPriority.md) | 将 Windows 内存优先级级别（`VeryLow` 到 `Normal`）加上 `None` 哨兵值映射到 `MEMORY_PRIORITY` 常量。 |
| [ThreadPriority](ThreadPriority.md) | 将完整的 Windows 线程优先级级别集合（`Idle` 到 `TimeCritical`，包括后台模式令牌）加上 `None` 哨兵值映射到 `SetThreadPriority` 使用的 `i32` 值。还提供了用于单步优先级提升的 `boost_one` 方法。 |

## 结构体

| 结构体 | 描述 |
|--------|------|
| [MemoryPriorityInformation](MemoryPriorityInformation.md) | 一个 `#[repr(C)]` 的 `u32` 新类型包装器，与 `NtSetInformationProcess` 所期望的 `MEMORY_PRIORITY_INFORMATION` 结构体布局一致。 |

## 另请参阅

| 参考 | 链接 |
|------|------|
| main 模块 | [main.rs](../main.rs/README.md) |
| scheduler 模块 | [scheduler.rs](../scheduler.rs/README.md) |
| apply 模块 | [apply.rs](../apply.rs/README.md) |
| config 模块 | [config.rs](../config.rs/README.md) |

---
Commit: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
