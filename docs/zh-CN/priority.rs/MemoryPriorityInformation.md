# MemoryPriorityInformation 结构体 (priority.rs)

一个 `#[repr(C)]` 新类型包装器，包装了一个表示进程内存优先级的 `u32` 值。此结构体用作直接传递给 Windows `SetProcessInformation` 和 `GetProcessInformation` API 的内存缓冲区，用于查询或设置 `ProcessMemoryPriority`。C 兼容的内存布局保证了该结构体的内存表示与内核期望的 `MEMORY_PRIORITY_INFORMATION` 结构一致。

## 语法

```rust
#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct MemoryPriorityInformation(pub u32);
```

## 成员

| 成员 | 类型 | 描述 |
|------|------|------|
| `0` | `u32` | 原始内存优先级值。对应于 Win32 `MEMORY_PRIORITY_INFORMATION` 结构的 `MemoryPriority` 字段。有效值为 `0`（`MEMORY_PRIORITY_VERY_LOW`）到 `5`（`MEMORY_PRIORITY_NORMAL`）。 |

## 备注

这是一个具有单个公共 `u32` 字段的元组结构体，设计用于在调用带有 `ProcessMemoryPriority` 信息类的 `SetProcessInformation` / `GetProcessInformation` 时转换为原始指针或从原始指针转换而来。`#[repr(C)]` 属性确保结构体具有可预测的、C 兼容的内存布局且无填充，使其可安全地用作 FFI 调用中的类型化缓冲区。

内部 `u32` 中存储的数值对应于 [MemoryPriority](MemoryPriority.md) 枚举通过其 `as_win_const` 方法定义的常量：

| 值 | 常量 |
|----|------|
| `1` | `MEMORY_PRIORITY_VERY_LOW` |
| `2` | `MEMORY_PRIORITY_LOW` |
| `3` | `MEMORY_PRIORITY_MEDIUM` |
| `4` | `MEMORY_PRIORITY_BELOW_NORMAL` |
| `5` | `MEMORY_PRIORITY_NORMAL` |

该结构体派生了 `PartialEq`、`Eq`、`Clone` 和 `Copy`，以支持值语义和比较操作。它未派生 `Debug`；可通过 `.0` 直接检查内部值。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `priority` |
| 调用方 | [apply_memory_priority](../apply.rs/apply_memory_priority.md) |
| Win32 API | `SetProcessInformation`、`GetProcessInformation`（使用 `ProcessMemoryPriority`） |
| 对应头文件 | `MEMORY_PRIORITY_INFORMATION` (processthreadsapi.h) |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 具有命名级别的内存优先级枚举 | [MemoryPriority](MemoryPriority.md) |
| 进程优先级类枚举 | [ProcessPriority](ProcessPriority.md) |
| I/O 优先级枚举 | [IOPriority](IOPriority.md) |
| 内存优先级应用逻辑 | [apply_memory_priority](../apply.rs/apply_memory_priority.md) |
| 优先级模块概述 | [priority 模块](README.md) |