# MemoryPriorityInformation 类型 (priority.rs)

一个 `#[repr(C)]` 的 `u32` 新类型包装器，与 Windows `NtSetInformationProcess` API 期望的 `MEMORY_PRIORITY_INFORMATION` 结构体的二进制布局匹配。此结构体用于通过未公开的 `ProcessMemoryPriority` 信息类设置进程的内存优先级时使用，确保 Rust 侧的数据与内核期望的 C 结构体在布局上兼容。

## 语法

```rust
#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct MemoryPriorityInformation(pub u32);
```

## 成员

| 成员 | 类型 | 描述 |
|------|------|------|
| `0`（元组字段） | `u32` | 原始内存优先级值。对应于 Windows SDK 定义的 `MEMORY_PRIORITY_*` 常量：`MEMORY_PRIORITY_VERY_LOW` (1)、`MEMORY_PRIORITY_LOW` (2)、`MEMORY_PRIORITY_MEDIUM` (3)、`MEMORY_PRIORITY_BELOW_NORMAL` (4) 或 `MEMORY_PRIORITY_NORMAL` (5)。该值通常通过调用 [`MemoryPriority::as_win_const()`](MemoryPriority.md) 并从结果 `MEMORY_PRIORITY` 包装器中提取内部 `.0` 字段来获取。 |

## 备注

- `#[repr(C)]` 属性保证该结构体的内存布局与包含单个 `ULONG` 字段的 C `struct` 匹配，这正是调用 `NtSetInformationProcess` 使用 `ProcessMemoryPriority` 信息类时 Windows 所期望的布局。
- 该结构体派生了 `PartialEq`、`Eq`、`Clone` 和 `Copy`，使其适合比较和值类型语义。
- 与本模块中的其他优先级类型不同，`MemoryPriorityInformation` 是一个结构体而非枚举，因为它直接充当 FFI 边界类型。对应的用户层逻辑枚举是 [`MemoryPriority`](MemoryPriority.md)，它提供字符串转换和查找表功能。
- 此类型**未**派生 `Debug`。如果需要调试输出，可以通过 `.0` 直接访问内部的 `u32` 值。
- 该结构体在 `apply` 模块中通过指针传递给 `NtSetInformationProcess`。结构体的大小（`std::mem::size_of::<MemoryPriorityInformation>()`）作为信息长度参数传递给 NT API 调用。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `priority.rs` |
| 调用方 | `apply` 模块（在设置进程内存优先级时，用作 `NtSetInformationProcess` 调用的数据缓冲区） |
| Win32 API | 对应于传递给 `NtSetInformationProcess`（信息类 `ProcessMemoryPriority`）的 `MEMORY_PRIORITY_INFORMATION` |
| 权限 | 对其他用户拥有的进程设置内存优先级时，可能需要 `SeDebugPrivilege` |

## 另请参阅

| 参考 | 链接 |
|------|------|
| MemoryPriority 枚举 | [MemoryPriority](MemoryPriority.md) |
| ProcessPriority | [ProcessPriority](ProcessPriority.md) |
| IOPriority | [IOPriority](IOPriority.md) |
| ThreadPriority | [ThreadPriority](ThreadPriority.md) |
| priority 模块概述 | [README](README.md) |
| apply_process_level | [apply_process_level](../main.rs/apply_process_level.md) |

---
*Commit: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)*