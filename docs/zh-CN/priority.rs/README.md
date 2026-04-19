# priority 模块 (AffinityServiceRust)

`priority` 模块为 Windows 优先级级别提供了类型安全的 Rust 枚举，实现了人类可读的字符串名称与 Win32 数值常量之间的双向转换。每个枚举遵循相同模式：一个表示"无配置值"的 `None` 变体、用于 DRY 查找转换的 `TABLE` 常量，以及四个标准方法（`as_str`、`as_win_const`、`from_str`、`from_win_const`）。这些枚举在整个配置解析器和应用引擎中使用，用于表示优先级设置，而无需暴露原始 Win32 数值。

## 枚举

| 名称 | 描述 |
|------|------|
| [ProcessPriority](ProcessPriority.md) | 将进程优先级类名称（`"idle"`、`"normal"`、`"high"` 等）映射到 `PROCESS_CREATION_FLAGS` 常量。 |
| [IOPriority](IOPriority.md) | 将 IO 优先级级别名称（`"very low"`、`"low"`、`"normal"`、`"high"`）映射到 NT IO 优先级 `u32` 值（0–3）。 |
| [MemoryPriority](MemoryPriority.md) | 将内存优先级级别名称映射到 `MEMORY_PRIORITY` 常量，用于 `SetProcessInformation`。 |
| [ThreadPriority](ThreadPriority.md) | 将线程优先级级别名称映射到 `i32` Win32 线程优先级值。还包括用于优先级提升和 FFI 转换的额外方法。 |

## 结构体

| 名称 | 描述 |
|------|------|
| [MemoryPriorityInformation](MemoryPriorityInformation.md) | 用于 Win32 `MEMORY_PRIORITY_INFORMATION` 互操作的 `#[repr(C)]` 包装器，围绕 `u32`。 |

## 参见

| 主题 | 链接 |
|------|------|
| 应用引擎（这些枚举的消费者） | [apply.rs](../apply.rs/README.md) |
| 配置解析器（从字符串创建这些枚举） | [config.rs](../config.rs/README.md) |
| 进程优先级应用 | [apply_priority](../apply.rs/apply_priority.md) |
| IO 优先级应用 | [apply_io_priority](../apply.rs/apply_io_priority.md) |
| 内存优先级应用 | [apply_memory_priority](../apply.rs/apply_memory_priority.md) |
| 线程级调度 | [apply_prime_threads](../apply.rs/apply_prime_threads.md) |
| 源代码文件 | [`src/priority.rs`](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/priority.rs) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*