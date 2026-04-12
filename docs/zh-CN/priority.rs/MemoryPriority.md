# MemoryPriority 枚举 (priority.rs)

表示进程的内存页面优先级级别。内存优先级影响操作系统在内存压力下对进程页面的替换顺序——优先级较低的页面会被优先换出。

## 语法

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryPriority {
    None,
    VeryLow,
    Low,
    Medium,
    BelowNormal,
    Normal,
}
```

## 成员

`None`

不更改内存优先级。`as_win_const()` 返回 `None`，表示跳过此设置。

`VeryLow`

最低内存优先级。映射到 `MEMORY_PRIORITY_VERY_LOW` (1)。此级别的页面在内存压力下最先被替换。

`Low`

低内存优先级。映射到 `MEMORY_PRIORITY_LOW` (2)。

`Medium`

中等内存优先级。映射到 `MEMORY_PRIORITY_MEDIUM` (3)。

`BelowNormal`

低于正常的内存优先级。映射到 `MEMORY_PRIORITY_BELOW_NORMAL` (4)。

`Normal`

默认内存优先级。映射到 `MEMORY_PRIORITY_NORMAL` (5)。大多数进程以此级别运行，其页面在内存压力下最后被替换。

## 方法

| 方法 | 签名 | 描述 |
| --- | --- | --- |
| **as_str** | `pub fn as_str(&self) -> &'static str` | 返回变体的人类可读字符串（如 `"very low"`、`"normal"`）。 |
| **as_win_const** | `pub fn as_win_const(&self) -> Option<MEMORY_PRIORITY>` | 返回对应的 `MEMORY_PRIORITY` 常量，`None` 变体返回 `Option::None`。 |
| **from_str** | `pub fn from_str(s: &str) -> Self` | 从字符串解析（不区分大小写）。无法识别的字符串返回 `MemoryPriority::None`。 |
| **from_win_const** | `pub fn from_win_const(val: u32) -> &'static str` | 从 Windows 常量值反查人类可读名称。无法识别的值返回 `"unknown"`。 |

## 备注

`MemoryPriority` 遵循本模块中所有优先级枚举的统一模式：通过静态 `TABLE` 实现双向转换。

```rust
const TABLE: &'static [(Self, &'static str, Option<MEMORY_PRIORITY>)] = &[
    (Self::None,        "none",         None),
    (Self::VeryLow,     "very low",     Some(MEMORY_PRIORITY_VERY_LOW)),
    (Self::Low,         "low",          Some(MEMORY_PRIORITY_LOW)),
    (Self::Medium,      "medium",       Some(MEMORY_PRIORITY_MEDIUM)),
    (Self::BelowNormal, "below normal", Some(MEMORY_PRIORITY_BELOW_NORMAL)),
    (Self::Normal,      "normal",       Some(MEMORY_PRIORITY_NORMAL)),
];
```

### 与 MemoryPriorityInformation 的关系

实际调用 Windows API 时，`MEMORY_PRIORITY` 值被包装在 [MemoryPriorityInformation](MemoryPriorityInformation.md) 结构体中，作为 `SetProcessInformation` 的参数传递。`MemoryPriority` 枚举负责配置级别的逻辑表示和字符串转换，而 `MemoryPriorityInformation` 是 FFI 层的 C 兼容包装器。

### 页面替换行为

Windows 内存管理器维护一个工作集修整策略，当系统内存不足时，优先级较低的页面会首先从工作集中移除。降低进程的内存优先级可以：

- 减少该进程对内存缓存的占用
- 让更关键的进程保留更多物理内存页面
- 适用于后台或低重要性进程

### 配置文件用法

在配置文件中使用 `memory_priority` 字段设置：

```
ProcessName.exe | memory_priority = below normal
```

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/priority.rs |
| **行号** | L112–L161 |
| **Windows 类型** | `MEMORY_PRIORITY`（来自 `windows::Win32::System::Threading`） |
| **Windows API** | [SetProcessInformation](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessinformation)、[GetProcessInformation](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessinformation) |
| **调用者** | [apply_memory_priority](../apply.rs/apply_memory_priority.md) |

## 另请参阅

- [MemoryPriorityInformation](MemoryPriorityInformation.md) — 用于 FFI 调用的 `#[repr(C)]` 包装器
- [ProcessPriority](ProcessPriority.md) — 进程优先级类枚举
- [IOPriority](IOPriority.md) — I/O 优先级枚举
- [priority.rs 模块概述](README.md)