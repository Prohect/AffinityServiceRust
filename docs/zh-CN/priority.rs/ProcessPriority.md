# ProcessPriority 枚举（priority.rs）

Windows 进程优先级类的类型安全表示。提供人类可读的字符串名称与 Win32 `PROCESS_CREATION_FLAGS` 常量之间的双向转换，使用单一 `TABLE` 常量进行 DRY 查找逻辑。

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

| 变体 | 字符串名称 | Win32 常量 | 值 |
|---------|-------------|----------------|-------|
| `None` | `"none"` | *(无)* | N/A — 哨兵值，表示"不更改" |
| `Idle` | `"idle"` | `IDLE_PRIORITY_CLASS` | `0x00000040` |
| `BelowNormal` | `"below normal"` | `BELOW_NORMAL_PRIORITY_CLASS` | `0x00004000` |
| `Normal` | `"normal"` | `NORMAL_PRIORITY_CLASS` | `0x00000020` |
| `AboveNormal` | `"above normal"` | `ABOVE_NORMAL_PRIORITY_CLASS` | `0x00008000` |
| `High` | `"high"` | `HIGH_PRIORITY_CLASS` | `0x00000080` |
| `Realtime` | `"real time"` | `REALTIME_PRIORITY_CLASS` | `0x00000100` |

## 方法

### as_str

```rust
pub fn as_str(&self) -> &'static str
```

返回此变体的人类可读字符串名称（例如，`"above normal"`）。如果未在 `TABLE` 中找到该变体则返回 `"unknown"`（对于构造良好的值不应发生）。

### as_win_const

```rust
pub fn as_win_const(&self) -> Option<PROCESS_CREATION_FLAGS>
```

返回对应的 Win32 `PROCESS_CREATION_FLAGS` 常量，或对于 `None` 变体返回 `None`。返回值为 `None` 向调用方（例如 [`apply_priority`](../apply.rs/apply_priority.md)）发出信号，表示不应进行优先级更改。

### from_str

```rust
pub fn from_str(s: &str) -> Self
```

将大小写不敏感的字符串解析为 `ProcessPriority` 变体。未识别的字符串映射到 `ProcessPriority::None`。这在配置文件解析期间使用。

**参数**

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `s` | `&str` | 要解析的优先级名称（例如，`"High"`、`"below normal"`）。比较不区分大小写。 |

### from_win_const

```rust
pub fn from_win_const(val: u32) -> &'static str
```

将原始 Win32 优先级类值（如 `GetPriorityClass` 返回的）转换回人类可读的字符串名称。如果值不匹配任何已知常量则返回 `"unknown"`。这用于在更改日志消息中显示"之前"状态。

**参数**

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `val` | `u32` | 原始 `PROCESS_CREATION_FLAGS` 值（`.0` 字段）。 |

## 备注

### TABLE 驱动设计

所有四个转换方法都操作单个 `const TABLE` 数组：

```rust
const TABLE: &'static [(Self, &'static str, Option<PROCESS_CREATION_FLAGS>)]
```

每个条目是一个 `(变体，字符串名称，可选的 win32 常量)` 元组。这确保添加新的优先级级别时只需要一个表条目，而不需要更新四个单独的 `match` 臂。

### None 哨兵值

`None` 变体不是一个 Windows 优先级类——它表示没有配置的优先级。当 `as_win_const()` 返回 `None` 时，应用函数会跳过优先级设置步骤。这与 `Normal` 不同，后者会主动将优先级设置为 `NORMAL_PRIORITY_CLASS`。

### Realtime 优先级

`Realtime` 变体映射到 `REALTIME_PRIORITY_CLASS`，需要 `SeIncreaseBasePriorityPrivilege` 和管理员权限。在不适当的情况下使用此优先级类可能会导致系统不稳定。字符串表示使用 `"real time"`（带空格）以与 Windows 任务管理器显示名称匹配。

## 要求

| | |
|---|---|
| **模块** | [`src/priority.rs`](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/priority.rs) |
| **调用方** | [`apply_priority`](../apply.rs/apply_priority.md)，配置解析 ([`read_config`](../config.rs/read_config.md)) |
| **依赖项** | `windows::Win32::System::Threading::PROCESS_CREATION_FLAGS` 及相关常量 |
| **权限** | `Realtime` 需要 `SeIncreaseBasePriorityPrivilege` |

## 参见

| 主题 | 链接 |
|-------|------|
| 使用此枚举的应用函数 | [`apply_priority`](../apply.rs/apply_priority.md) |
| IO 优先级枚举 | [`IOPriority`](IOPriority.md) |
| 内存优先级枚举 | [`MemoryPriority`](MemoryPriority.md) |
| 线程优先级枚举 | [`ThreadPriority`](ThreadPriority.md) |
| 模块概述 | [priority.rs](README.md) |
| Win32 参考 | [`SetPriorityClass`](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-setpriorityclass) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*