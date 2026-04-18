# ProcessPriority 枚举 (priority.rs)

将 Windows 进程优先级类别表示为强类型的 Rust 枚举。每个变体映射到 Win32 `SetPriorityClass` API 使用的对应 `PROCESS_CREATION_FLAGS` 常量，`None` 哨兵值表示不应进行优先级更改。该类型提供人类可读字符串名称、枚举变体和原始 Win32 常量值之间的双向转换。

## 语法

```AffinityServiceRust/src/priority.rs#L10-18
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessPriority {
    None,
    Idle,
    BelowNormal,
    Normal,
    AboveNormal,
    High,
    Realtime,
}
```

## 成员

| 变体 | Win32 常量 | 数值 | 字符串表示 | 描述 |
|------|-----------|------|-----------|------|
| `None` | *（无）* | N/A | `"none"` | 哨兵值，表示未请求优先级更改。`as_win_const()` 返回 `None`。 |
| `Idle` | `IDLE_PRIORITY_CLASS` | `0x00000040` | `"idle"` | 进程仅在系统空闲时运行。空闲优先级进程中的线程会被任何更高优先级类别的线程抢占。 |
| `BelowNormal` | `BELOW_NORMAL_PRIORITY_CLASS` | `0x00004000` | `"below normal"` | 优先级高于 `Idle` 但低于 `Normal`。适用于不应影响交互响应性的后台工作。 |
| `Normal` | `NORMAL_PRIORITY_CLASS` | `0x00000020` | `"normal"` | 大多数进程的默认优先级类别。无特殊调度处理。 |
| `AboveNormal` | `ABOVE_NORMAL_PRIORITY_CLASS` | `0x00008000` | `"above normal"` | 优先级高于 `Normal` 但低于 `High`。适用于对延迟敏感的前台工作。 |
| `High` | `HIGH_PRIORITY_CLASS` | `0x00000080` | `"high"` | 高优先级进程中的线程会抢占 `Normal` 和 `BelowNormal` 进程中的线程。应谨慎使用以避免使其他进程饥饿。 |
| `Realtime` | `REALTIME_PRIORITY_CLASS` | `0x00000100` | `"real time"` | 最高可能的优先级类别。线程会抢占所有其他线程，包括执行关键任务的操作系统线程。**需要 `SeIncreaseBasePriorityPrivilege` 和管理员权限。** 不当使用可能导致系统不稳定。 |

## 方法

### `as_str`

```AffinityServiceRust/src/priority.rs#L30-35
pub fn as_str(&self) -> &'static str
```

返回该变体的人类可读字符串名称（例如 `"below normal"`、`"high"`）。如果变体未在内部查找表中找到则返回 `"unknown"`（对于正确构造的值不应出现此情况）。

### `as_win_const`

```AffinityServiceRust/src/priority.rs#L37-39
pub fn as_win_const(&self) -> Option<PROCESS_CREATION_FLAGS>
```

返回对应该变体的 Win32 `PROCESS_CREATION_FLAGS` 常量，包装在 `Some` 中。对于 `ProcessPriority::None` 哨兵值返回 `None`，表示不应进行 API 调用。

### `from_str`

```AffinityServiceRust/src/priority.rs#L41-47
pub fn from_str(s: &str) -> Self
```

将不区分大小写的字符串解析为 `ProcessPriority` 变体。输入在与查找表比较之前会被转为小写。如果字符串不匹配任何已知的优先级名称，则返回 `ProcessPriority::None`。

**注意：** 这是一个固有方法，而非 `std::str::FromStr` trait 的实现。

### `from_win_const`

```AffinityServiceRust/src/priority.rs#L49-55
pub fn from_win_const(val: u32) -> &'static str
```

给定一个表示 `PROCESS_CREATION_FLAGS` 常量的原始 `u32` 值，返回对应的人类可读字符串名称。如果值不匹配任何已知的优先级类别，则返回 `"unknown"`。此方法返回字符串而非枚举变体，主要用于从操作系统回读值时的诊断日志记录。

## 备注

- 所有转换由定义为枚举关联常量的单一 `const TABLE` 数组驱动。这种表驱动设计确保字符串名称、枚举变体和 Win32 常量保持同步，添加新变体只需添加一个表条目。
- 字符串表示 `"real time"`（两个单词，带空格）与 Windows 任务管理器显示的名称匹配，也是配置文件中期望的格式。
- 在桌面系统上将进程设置为 `Realtime` 优先级**极其危险**。它可能使关键的操作系统线程（包括鼠标/键盘输入线程和磁盘 I/O 线程）饥饿，可能导致系统无响应。AffinityServiceRust 中的 apply 引擎仅在明确配置且持有所需权限时才会尝试此调用。
- `ProcessPriority::None` 变体在整个配置系统中用作默认/无操作哨兵。当配置解析器遇到缺失或为空的 `priority` 字段时，会生成 `ProcessPriority::None`，这会导致 apply 引擎完全跳过 `SetPriorityClass` 调用。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `priority.rs` |
| 调用者 | `config` 模块（反序列化）、`apply::apply_priority`（进程级应用） |
| Win32 API | `SetPriorityClass`（间接——枚举提供由 apply 模块使用的常量） |
| 权限 | `Realtime` 需要 `SeIncreaseBasePriorityPrivilege`；`High` 建议具有管理员权限 |
| 依赖 | `windows::Win32::System::Threading::PROCESS_CREATION_FLAGS` 及相关常量 |

## 另请参阅

| 参考 | 链接 |
|------|------|
| IOPriority | [IOPriority](IOPriority.md) |
| MemoryPriority | [MemoryPriority](MemoryPriority.md) |
| ThreadPriority | [ThreadPriority](ThreadPriority.md) |
| MemoryPriorityInformation | [MemoryPriorityInformation](MemoryPriorityInformation.md) |
| apply_process_level | [apply_process_level](../main.rs/apply_process_level.md) |
| priority 模块概述 | [README](README.md) |

---
*提交：[29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
