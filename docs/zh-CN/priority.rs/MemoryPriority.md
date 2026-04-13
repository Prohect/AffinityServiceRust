# MemoryPriority 枚举 (priority.rs)

表示可通过 Windows `SetProcessInformation` API（使用 `ProcessMemoryPriority` 信息类）分配给进程的内存优先级级别。内存优先级影响内存管理器修剪进程工作集页面的速度以及在备用列表中对这些页面的优先排序方式。较低的内存优先级会导致页面在内存压力下更快被回收。`None` 变体充当哨兵值，表示配置中未请求更改内存优先级。

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

| 变体 | Win32 常量 | 值 | 描述 |
|---------|---------------|-------|-------------|
| `None` | *(无)* | — | 哨兵值：未请求更改内存优先级。`as_win_const` 返回 `None`。 |
| `VeryLow` | `MEMORY_PRIORITY_VERY_LOW` | 1 | 最低内存优先级。在内存压力下，页面最先被修剪和回收。 |
| `Low` | `MEMORY_PRIORITY_LOW` | 2 | 低内存优先级。页面在中等及更高级别之前被修剪。 |
| `Medium` | `MEMORY_PRIORITY_MEDIUM` | 3 | 中等内存优先级。修剪行为均衡。 |
| `BelowNormal` | `MEMORY_PRIORITY_BELOW_NORMAL` | 4 | 低于正常的内存优先级。页面在中等之后、正常之前被修剪。 |
| `Normal` | `MEMORY_PRIORITY_NORMAL` | 5 | 默认内存优先级。页面在备用列表中保留时间最长。 |

## 方法

### as_str

```rust
pub fn as_str(&self) -> &'static str
```

返回此变体的人类可读字符串名称（例如 `"very low"`、`"normal"`）。如果在查找表中未找到该变体（对于正确构造的值不会发生），则返回 `"unknown"`。

### as_win_const

```rust
pub fn as_win_const(&self) -> Option<MEMORY_PRIORITY>
```

返回用于 `SetProcessInformation` 的对应 `MEMORY_PRIORITY` 常量，对于 `MemoryPriority::None` 哨兵变体返回 `None`。

### from_str

```rust
pub fn from_str(s: &str) -> Self
```

将不区分大小写的字符串解析为 `MemoryPriority` 变体。可识别的值包括 `"none"`、`"very low"`、`"low"`、`"medium"`、`"below normal"` 和 `"normal"`。无法识别的字符串默认返回 `MemoryPriority::None`。

**注意：** 这是一个固有方法，不是 `std::str::FromStr` trait 的实现。它不返回 `Result`；无法识别的输入会静默映射为 `None`。

### from_win_const

```rust
pub fn from_win_const(val: u32) -> &'static str
```

查找原始 `u32` 内存优先级常量值，并返回对应的人类可读名称字符串。如果值与任何已知的 `MEMORY_PRIORITY` 常量不匹配，则返回 `"unknown"`。此方法用于读取进程当前内存优先级时的诊断日志记录。

## 备注

`MemoryPriority` 枚举与 [MemoryPriorityInformation](MemoryPriorityInformation.md) 配合使用，后者作为传递给 `SetProcessInformation` 和 `GetProcessInformation` 的 `#[repr(C)]` 缓冲区。在应用内存优先级时，服务调用 `as_win_const()` 获取 `MEMORY_PRIORITY` 值，将其包装在 `MemoryPriorityInformation` 结构体中，然后传递给 Windows API。

所有转换方法使用单一的 `const TABLE` 数组，将每个变体与其字符串名称和可选的 Win32 常量配对。这确保了字符串 ↔ 常量 ↔ 变体映射始终保持一致。

内存优先级与进程优先级类和 I/O 优先级是正交的；三者可以在同一进程上独立设置。

## 要求

| 要求 | 值 |
|-------------|-------|
| 模块 | `priority` |
| 调用者 | [apply_memory_priority](../apply.rs/apply_memory_priority.md)、[read_config](../config.rs/read_config.md)、[parse_and_insert_rules](../config.rs/parse_and_insert_rules.md) |
| Win32 API | `SetProcessInformation`、`GetProcessInformation`（`ProcessMemoryPriority`） |
| 权限 | 具有 `PROCESS_SET_INFORMATION` 访问权限的标准进程句柄 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 用于内存优先级 API 调用的 C 兼容缓冲区 | [MemoryPriorityInformation](MemoryPriorityInformation.md) |
| 进程优先级类枚举 | [ProcessPriority](ProcessPriority.md) |
| I/O 优先级枚举 | [IOPriority](IOPriority.md) |
| 线程优先级枚举 | [ThreadPriority](ThreadPriority.md) |
| 模块概览 | [priority 模块](README.md) |