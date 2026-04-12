# MemoryPriorityInformation struct (priority.rs)

`#[repr(C)]` 包装器结构体，封装一个 `u32` 值，用于通过 `GetProcessInformation` / `SetProcessInformation` 系统调用传递内存优先级信息。

## 语法

```rust
#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct MemoryPriorityInformation(pub u32);
```

## 成员

`0: u32`

内存优先级值，对应 Windows `MEMORY_PRIORITY` 常量。有效值范围为 0–5，由 [MemoryPriority](MemoryPriority.md) 枚举的 `as_win_const()` 方法提供。

| 值 | 对应 MemoryPriority | Windows 常量 |
| --- | --- | --- |
| 0 | VeryLow | `MEMORY_PRIORITY_VERY_LOW` |
| 1 | Low | `MEMORY_PRIORITY_LOW` |
| 2 | Medium | `MEMORY_PRIORITY_MEDIUM` |
| 3 | BelowNormal | `MEMORY_PRIORITY_BELOW_NORMAL` |
| 4 | — | — |
| 5 | Normal | `MEMORY_PRIORITY_NORMAL` |

## 备注

此结构体使用 `#[repr(C)]` 布局属性，确保其内存布局与 Windows API 期望的 C 结构体 `MEMORY_PRIORITY_INFORMATION` 完全一致。这使得它可以直接作为指针传递给 `SetProcessInformation` 和 `GetProcessInformation`，无需额外的序列化或类型转换。

元组结构体设计（`pub u32`）使得构造和解构都非常简洁：

```rust
// 构造
let info = MemoryPriorityInformation(5); // Normal

// 读取
let raw_value = info.0;
```

该结构体派生了 `PartialEq` 和 `Eq`，以支持对当前值和目标值的比较——在 [`apply_memory_priority`](../apply.rs/apply_memory_priority.md) 中用于判断是否需要执行更改。派生 `Clone` 和 `Copy` 使其可以按值传递，这对 FFI 边界的使用场景很重要。

### 与 MemoryPriority 枚举的关系

[MemoryPriority](MemoryPriority.md) 枚举提供类型安全的 Rust 级别抽象，而 `MemoryPriorityInformation` 是与 Windows API 直接交互的 FFI 层。典型用法是：

1. 从配置中获取 `MemoryPriority` 枚举值
2. 调用 `as_win_const()` 获取 `MEMORY_PRIORITY` 常量
3. 将常量值包装为 `MemoryPriorityInformation` 传递给系统调用

### Windows API 用法

```rust
// 设置内存优先级
let info = MemoryPriorityInformation(MEMORY_PRIORITY_NORMAL.0);
SetProcessInformation(
    handle,
    ProcessMemoryPriority,
    &info as *const _ as *const c_void,
    std::mem::size_of::<MemoryPriorityInformation>() as u32,
);

// 查询内存优先级
let mut info = MemoryPriorityInformation(0);
GetProcessInformation(
    handle,
    ProcessMemoryPriority,
    &mut info as *mut _ as *mut c_void,
    std::mem::size_of::<MemoryPriorityInformation>() as u32,
);
```

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/priority.rs |
| **源码行** | L109 |
| **布局** | `#[repr(C)]`，大小 4 字节，对齐 4 字节 |
| **Windows API** | [SetProcessInformation](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessinformation)、[GetProcessInformation](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessinformation) |

## 另请参阅

- [MemoryPriority 枚举](MemoryPriority.md) — 类型安全的内存优先级抽象
- [apply_memory_priority](../apply.rs/apply_memory_priority.md) — 应用内存优先级的函数
- [priority.rs 模块概述](README.md)