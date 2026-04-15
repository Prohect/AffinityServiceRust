# MemoryPriority 枚举 (priority.rs)

表示可通过 `NtSetInformationProcess` 使用 `ProcessMemoryPriority` 信息类分配给进程的 Windows 内存优先级级别。每个变体映射到 Windows SDK 中的一个 `MEMORY_PRIORITY` 常量。`None` 哨兵变体表示不应进行内存优先级更改。

## 语法

```AffinityServiceRust/src/priority.rs#L108-L116
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

| 变体 | Win32 常量 | 数值 | 描述 |
|------|-----------|------|------|
| `None` | *（无）* | 不适用 | 哨兵值，表示未请求内存优先级更改。`as_win_const()` 返回 `None`。 |
| `VeryLow` | `MEMORY_PRIORITY_VERY_LOW` | 1 | 最低内存优先级。在内存压力下，该进程的页面会最先被裁剪。适用于后台或维护进程。 |
| `Low` | `MEMORY_PRIORITY_LOW` | 2 | 低内存优先级。页面在正常优先级进程之前被裁剪。 |
| `Medium` | `MEMORY_PRIORITY_MEDIUM` | 3 | 中等内存优先级。介于低和低于正常之间的中间层级。 |
| `BelowNormal` | `MEMORY_PRIORITY_BELOW_NORMAL` | 4 | 低于正常的内存优先级。页面被裁剪的可能性略高于正常优先级进程。 |
| `Normal` | `MEMORY_PRIORITY_NORMAL` | 5 | 默认内存优先级。除非显式更改，否则分配给进程的标准级别。 |

## 方法

### `as_str`

```AffinityServiceRust/src/priority.rs#L127-L132
pub fn as_str(&self) -> &'static str
```

返回变体的人类可读字符串名称（例如 `"very low"`、`"normal"`）。如果在内部查找表中未找到该变体（对于合法值不会发生），则返回 `"unknown"`。

### `as_win_const`

```AffinityServiceRust/src/priority.rs#L134-L136
pub fn as_win_const(&self) -> Option<MEMORY_PRIORITY>
```

返回用 `Some` 包装的对应 `MEMORY_PRIORITY` Windows SDK 常量，对于 `MemoryPriority::None` 哨兵变体则返回 `None`。返回的值适合通过 [`MemoryPriorityInformation`](MemoryPriorityInformation.md) 包装结构体传递给 `NtSetInformationProcess`。

### `from_str`

```AffinityServiceRust/src/priority.rs#L138-L144
pub fn from_str(s: &str) -> Self
```

将不区分大小写的字符串解析为对应的 `MemoryPriority` 变体。输入在与查找表比较之前会被转换为小写。如果字符串与任何已知变体名称不匹配，则返回 `MemoryPriority::None`。

**识别的字符串：** `"none"`、`"very low"`、`"low"`、`"medium"`、`"below normal"`、`"normal"`

### `from_win_const`

```AffinityServiceRust/src/priority.rs#L146-L152
pub fn from_win_const(val: u32) -> &'static str
```

查找 Windows API 返回的原始 `u32` 内存优先级值对应的人类可读字符串名称。如果该值与任何已知常量不匹配，则返回 `"unknown"`。注意，此方法返回的是 `&'static str` 而非 `MemoryPriority` 变体。

## 备注

- 内部查找表（`TABLE`）定义为 `&'static` 的 `(Self, &'static str, Option<MEMORY_PRIORITY>)` 元组数组，为枚举变体、字符串名称和 Win32 常量之间的映射提供单一事实来源。
- 内存优先级影响 Windows 内存管理器的页面裁剪顺序。具有较低内存优先级的进程在系统面临内存压力时会最先被回收页面。这不影响内存分配的速度本身，只影响页面被换出的可能性。
- `MEMORY_PRIORITY` 类型从 `windows::Win32::System::Threading` 导入，内部封装一个 `u32` 值。
- `from_str` 方法**没有**实现标准 `std::str::FromStr` trait。它是一个独立的关联函数，对于无法识别的输入返回 `None` 变体而不是错误。
- 与 [`ProcessPriority`](ProcessPriority.md) 和 [`IOPriority`](IOPriority.md) 不同，内存优先级通过 `NtSetInformationProcess` 使用 [`MemoryPriorityInformation`](MemoryPriorityInformation.md) `#[repr(C)]` 包装器设置，而不是通过专用的 Win32 函数。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `priority.rs` |
| 调用方 | `config` 模块（反序列化）、`apply` 模块（`apply_memory_priority`） |
| 依赖 | `windows::Win32::System::Threading::MEMORY_PRIORITY`、`MEMORY_PRIORITY_VERY_LOW`、`MEMORY_PRIORITY_LOW`、`MEMORY_PRIORITY_MEDIUM`、`MEMORY_PRIORITY_BELOW_NORMAL`、`MEMORY_PRIORITY_NORMAL` |
| 平台 | Windows |

## 另请参阅

| 参考 | 链接 |
|------|------|
| MemoryPriorityInformation | [MemoryPriorityInformation](MemoryPriorityInformation.md) |
| ProcessPriority | [ProcessPriority](ProcessPriority.md) |
| IOPriority | [IOPriority](IOPriority.md) |
| ThreadPriority | [ThreadPriority](ThreadPriority.md) |
| priority 模块概述 | [README](README.md) |
| apply_process_level | [apply_process_level](../main.rs/apply_process_level.md) |

---
*Commit: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)*