# ProcessPriority 枚举 (priority.rs)

表示 Windows 进程优先级类。每个变体映射到一个 `PROCESS_CREATION_FLAGS` 常量，用于 `SetPriorityClass` API 调用。

## 语法

```rust
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

| 变体 | 字符串值 | Windows 常量 | 说明 |
| --- | --- | --- | --- |
| `None` | `"none"` | — | 不更改优先级（跳过操作）。 |
| `Idle` | `"idle"` | `IDLE_PRIORITY_CLASS` | 最低优先级，仅在系统空闲时运行。 |
| `BelowNormal` | `"below normal"` | `BELOW_NORMAL_PRIORITY_CLASS` | 低于正常优先级。 |
| `Normal` | `"normal"` | `NORMAL_PRIORITY_CLASS` | 默认优先级。 |
| `AboveNormal` | `"above normal"` | `ABOVE_NORMAL_PRIORITY_CLASS` | 高于正常优先级。 |
| `High` | `"high"` | `HIGH_PRIORITY_CLASS` | 高优先级，适用于时间关键型任务。 |
| `Realtime` | `"real time"` | `REALTIME_PRIORITY_CLASS` | 最高优先级，可能影响系统稳定性。 |

## 方法

### as_str

返回当前变体对应的人类可读字符串。

```rust
pub fn as_str(&self) -> &'static str
```

**返回值**

返回 `TABLE` 中对应的字符串值（例如 `"idle"`、`"normal"`）。若未找到匹配项，返回 `"unknown"`。

---

### as_win_const

返回当前变体对应的 Windows `PROCESS_CREATION_FLAGS` 常量。

```rust
pub fn as_win_const(&self) -> Option<PROCESS_CREATION_FLAGS>
```

**返回值**

返回 `Some(PROCESS_CREATION_FLAGS)` 或 `None`（当变体为 `None` 时）。调用者可通过检查 `None` 返回值来判断是否应跳过优先级设置操作。

---

### from_str

从字符串解析 `ProcessPriority` 变体。

```rust
pub fn from_str(s: &str) -> Self
```

**参数**

`s` — 优先级名称字符串（不区分大小写）。

**返回值**

返回匹配的 `ProcessPriority` 变体。若无匹配项，返回 `ProcessPriority::None`。

---

### from_win_const

从 Windows 常量的原始 `u32` 值反查对应的优先级名称字符串。

```rust
pub fn from_win_const(val: u32) -> &'static str
```

**参数**

`val` — `PROCESS_CREATION_FLAGS` 的内部 `u32` 值。

**返回值**

返回匹配的优先级名称字符串。若无匹配项，返回 `"unknown"`。

## 备注

`ProcessPriority` 遵循本模块中所有枚举共享的查找表模式：

- **静态查找表 `TABLE`**：一个 `&'static [(Self, &str, Option<WinConst>)]` 切片，在编译时定义所有变体、字符串名称和 Windows 常量之间的映射关系。
- **双向转换**：支持从字符串到枚举（`from_str`）和从枚举到字符串（`as_str`）的转换，同时支持从 Windows 常量到枚举的反向映射（`from_win_const`）。
- **`None` 语义**：`None` 变体表示"不更改"，其 Windows 常量为 `None`（`Option::None`），调用者据此跳过对应的 API 调用。

配置文件解析器（[`ProcessConfig`](../config.rs/ProcessConfig.md)）使用 `from_str` 将用户输入转换为枚举值。应用层（[`apply_priority`](../apply.rs/apply_priority.md)）使用 `as_win_const` 获取传递给 `SetPriorityClass` 的实际值。

`Realtime` 优先级需要管理员权限。如果进程没有足够的权限，`SetPriorityClass` 调用将失败。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | `src/priority.rs` |
| **行号** | L8–L58 |
| **派生特征** | `Debug`、`Clone`、`Copy`、`PartialEq`、`Eq` |
| **Windows 依赖** | `windows::Win32::System::Threading::PROCESS_CREATION_FLAGS` |

## 另请参阅

- [priority.rs 模块概述](README.md)
- [IOPriority](IOPriority.md)
- [MemoryPriority](MemoryPriority.md)
- [ThreadPriority](ThreadPriority.md)
- [apply_priority 函数](../apply.rs/apply_priority.md)
- [ProcessConfig 结构体](../config.rs/ProcessConfig.md)