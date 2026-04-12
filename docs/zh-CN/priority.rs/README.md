# priority.rs 模块 (priority.rs)

`priority` 模块定义了 Windows 进程和线程优先级的 Rust 枚举映射。每个枚举通过静态查找表实现与字符串名称和 Windows API 常量之间的双向转换。

## 概述

本模块为以下四类 Windows 优先级提供类型安全的枚举表示：

- **进程优先级类** — [`ProcessPriority`](ProcessPriority.md)，映射到 `PROCESS_CREATION_FLAGS`
- **I/O 优先级** — [`IOPriority`](IOPriority.md)，映射到 `NtSetInformationProcess` 的 I/O 优先级值
- **内存优先级** — [`MemoryPriority`](MemoryPriority.md)，映射到 `MEMORY_PRIORITY` 常量
- **线程优先级** — [`ThreadPriority`](ThreadPriority.md)，映射到 `THREAD_PRIORITY` 值

此外还包含一个 FFI 辅助结构体：

- **内存优先级信息** — [`MemoryPriorityInformation`](MemoryPriorityInformation.md)，`#[repr(C)]` 包装器，用于 `Get/SetProcessInformation`

## 设计模式

所有枚举遵循统一的查找表模式：

1. **静态 `TABLE` 常量** — 每个枚举定义一个 `&'static [(Self, &'static str, Option<WinConst>)]` 数组，集中管理变体、显示名称和 Windows 常量的映射关系。
2. **双向转换** — 提供 `as_str()` / `from_str()` 用于字符串互转，`as_win_const()` / `from_win_const()` 用于 Windows 常量互转。
3. **`None` 变体语义** — 每个枚举的 `None` 变体表示"不更改当前值"，其 Windows 常量映射为 `Option::None`。配置中指定 `none` 或省略该字段时使用此变体。

## 项目

### 枚举

| 名称 | 描述 |
| --- | --- |
| [ProcessPriority](ProcessPriority.md) | 进程优先级类，映射到 `PROCESS_CREATION_FLAGS`。7 个变体。 |
| [IOPriority](IOPriority.md) | I/O 优先级级别，通过 `NtSetInformationProcess` 设置。5 个变体。 |
| [MemoryPriority](MemoryPriority.md) | 内存页面替换优先级，映射到 `MEMORY_PRIORITY`。6 个变体。 |
| [ThreadPriority](ThreadPriority.md) | 线程优先级级别，映射到 `THREAD_PRIORITY`。11 个变体，含额外方法 `boost_one()` 和 `to_thread_priority_struct()`。 |

### 结构体

| 名称 | 描述 |
| --- | --- |
| [MemoryPriorityInformation](MemoryPriorityInformation.md) | `#[repr(C)]` 包装 `u32` 的元组结构体，用于 `Get/SetProcessInformation` 的 FFI 调用。 |

## 使用示例

配置解析时将字符串转换为枚举：

```rust
let priority = ProcessPriority::from_str("high");    // ProcessPriority::High
let io = IOPriority::from_str("normal");              // IOPriority::Normal
let mem = MemoryPriority::from_str("below normal");   // MemoryPriority::BelowNormal
let thread = ThreadPriority::from_str("above normal"); // ThreadPriority::AboveNormal
```

应用配置时将枚举转换为 Windows 常量：

```rust
if let Some(flags) = priority.as_win_const() {
    SetPriorityClass(handle, flags);
}
```

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | `src/priority.rs` |
| **依赖 crate** | `windows` (`Win32::System::Threading`) |
| **调用者** | [`apply_priority`](../apply.rs/apply_priority.md)、[`apply_io_priority`](../apply.rs/apply_io_priority.md)、[`apply_memory_priority`](../apply.rs/apply_memory_priority.md)、[`apply_prime_threads`](../apply.rs/apply_prime_threads.md) |
| **配置来源** | [`ProcessConfig`](../config.rs/ProcessConfig.md) 中的 `priority`、`io_priority`、`memory_priority` 字段 |

## 另请参阅

- [config.rs 模块](../config.md) — 配置解析，使用 `from_str()` 将配置值转换为枚举
- [apply.rs 模块](../apply.rs/README.md) — 配置应用，使用 `as_win_const()` 调用 Windows API
- [scheduler.rs 模块](../scheduler.rs/README.md) — Prime 线程调度器，使用 `ThreadPriority` 管理线程优先级提升