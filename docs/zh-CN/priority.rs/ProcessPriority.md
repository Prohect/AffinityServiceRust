# ProcessPriority 枚举 (priority.rs)

表示 Windows 进程优先级类别等级。每个变体映射到 `SetPriorityClass` Win32 API 使用的 `PROCESS_CREATION_FLAGS` 常量。`None` 变体作为哨兵值，表示给定的进程配置规则未请求优先级更改。

## 语法

```priority.rs
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

| 变体 | Win32 常量 | 值 | 描述 |
|---------|---------------|-------|-------------|
| `None` | — | — | 未请求优先级更改。作为哨兵值/默认值。 |
| `Idle` | `IDLE_PRIORITY_CLASS` | `0x00000040` | 最低优先级类别。线程仅在系统空闲时运行。 |
| `BelowNormal` | `BELOW_NORMAL_PRIORITY_CLASS` | `0x00004000` | 高于 Idle 但低于 Normal 的优先级。 |
| `Normal` | `NORMAL_PRIORITY_CLASS` | `0x00000020` | 大多数进程的默认优先级类别。 |
| `AboveNormal` | `ABOVE_NORMAL_PRIORITY_CLASS` | `0x00008000` | 高于 Normal 但低于 High 的优先级。 |
| `High` | `HIGH_PRIORITY_CLASS` | `0x00000080` | 高优先级。应谨慎使用，因为可能会使较低优先级的进程无法获得资源。 |
| `Realtime` | `REALTIME_PRIORITY_CLASS` | `0x00000100` | 最高优先级类别。需要 `SeIncreaseBasePriorityPrivilege` 权限。可以抢占操作系统线程。 |

## 方法

### as_str

```priority.rs
pub fn as_str(&self) -> &'static str
```

返回此变体的人类可读字符串名称（例如 `"idle"`、`"below normal"`、`"real time"`）。如果变体未在内部查找表中找到，则返回 `"unknown"`，对于格式正确的值不应出现此情况。`None` 变体返回 `"none"`。

### as_win_const

```priority.rs
pub fn as_win_const(&self) -> Option<PROCESS_CREATION_FLAGS>
```

返回此变体对应的 `PROCESS_CREATION_FLAGS` 值，如果变体为 `ProcessPriority::None`（未请求更改），则返回 `None`。

### from_str

```priority.rs
pub fn from_str(s: &str) -> Self
```

将不区分大小写的字符串解析为 `ProcessPriority` 变体。输入在与查找表匹配之前会被转换为小写。无法识别的字符串返回 `ProcessPriority::None`。

**参数**

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `s` | `&str` | 要解析的优先级名称。与 `"idle"`、`"below normal"`、`"normal"`、`"above normal"`、`"high"`、`"real time"` 进行不区分大小写的匹配。 |

### from_win_const

```priority.rs
pub fn from_win_const(val: u32) -> &'static str
```

查找原始 `u32` 值（`PROCESS_CREATION_FLAGS` 的内部值）并返回对应的人类可读字符串名称。如果值不匹配任何已知的优先级类别，则返回 `"unknown"`。

**参数**

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `val` | `u32` | 要查找的原始 Win32 优先级类别常量值。 |

## 备注

- 该枚举使用内部 `TABLE` 常量，其类型为 `(Self, &str, Option<PROCESS_CREATION_FLAGS>)` 元组数组，驱动所有转换方法。这确保了字符串名称、枚举变体和 Win32 常量保持同步。
- `from_str` 不是标准库的 `FromStr` trait——它是一个固有方法，在失败时返回 `ProcessPriority::None` 而不是错误。
- `from_win_const` 返回 `&'static str` 而不是 `Self`，与其他枚举的 `from_win_const` 同样返回 `&'static str` 一致。这主要用于读取运行中进程的当前优先级时的日志输出。
- 设置 `Realtime` 优先级需要 `SeIncreaseBasePriorityPrivilege` 权限和管理员提升。如果没有该权限，`SetPriorityClass` 调用将失败并返回 `ERROR_PRIVILEGE_NOT_HELD` (1314)。

## 要求

| 要求 | 值 |
|-------------|-------|
| 模块 | `priority` |
| 调用者 | [parse_and_insert_rules](../config.rs/parse_and_insert_rules.md)、[apply_priority](../apply.rs/apply_priority.md) |
| 被调用者 | — |
| Win32 API | `SetPriorityClass`、`GetPriorityClass`（常量的使用者） |
| 权限 | `SeIncreaseBasePriorityPrivilege`（用于 `Realtime` 变体） |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| I/O 优先级等级 | [IOPriority](IOPriority.md) |
| 内存优先级等级 | [MemoryPriority](MemoryPriority.md) |
| 线程优先级等级 | [ThreadPriority](ThreadPriority.md) |
| 每进程配置记录 | [ProcessConfig](../config.rs/ProcessConfig.md) |
| 优先级应用逻辑 | [apply_priority](../apply.rs/apply_priority.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd