# ThreadPriority 枚举 (priority.rs)

将完整的 Windows 线程优先级级别映射为强类型的 Rust 枚举变体，提供人类可读的字符串名称、枚举变体和 Win32 `SetThreadPriority` API 所需的原始 `i32` 值之间的双向转换。该枚举涵盖从 `Idle`（−15）到 `TimeCritical`（15）的标准调度级别、特殊的后台模式令牌 `ModeBackgroundBegin` / `ModeBackgroundEnd`、`ErrorReturn` 哨兵值，以及表示不请求优先级更改的 `None` 变体。`boost_one` 方法支持主线程提升时的单步优先级提升。

## 语法

```AffinityServiceRust/src/priority.rs#L159-L172
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadPriority {
    None,
    ErrorReturn,         // 0x7FFFFFFF
    ModeBackgroundBegin, // 0x00010000 (use only for current thread)
    ModeBackgroundEnd,   // 0x00020000 (use only for current thread)
    Idle,                // -15
    Lowest,              // -2
    BelowNormal,         // -1
    Normal,              // 0
    AboveNormal,         // 1
    Highest,             // 2
    TimeCritical,        // 15
}
```

## 成员

| 变体 | Win32 值 | 字符串键 | 描述 |
|------|----------|---------|------|
| `None` | *（不调用 API）* | `"none"` | 哨兵值，表示不应进行优先级更改。`as_win_const` 方法对此变体返回 `None`。 |
| `ErrorReturn` | `0x7FFFFFFF` | `"error"` | 表示 `GetThreadPriority` 失败时返回的 `THREAD_PRIORITY_ERROR_RETURN` 值。通常不用作 `SetThreadPriority` 的输入。 |
| `ModeBackgroundBegin` | `0x00010000` | `"background begin"` | 将线程降至后台处理模式，降低其调度优先级、I/O 优先级和内存优先级。**必须仅对调用线程自身设置。** |
| `ModeBackgroundEnd` | `0x00020000` | `"background end"` | 恢复先前进入后台模式的线程的正常处理模式。**必须仅对调用线程自身设置。** |
| `Idle` | `-15` | `"idle"` | `THREAD_PRIORITY_IDLE`。最低的常规调度优先级。线程仅在没有其他线程就绪时运行。 |
| `Lowest` | `-2` | `"lowest"` | `THREAD_PRIORITY_LOWEST`。比正常低两个级别。 |
| `BelowNormal` | `-1` | `"below normal"` | `THREAD_PRIORITY_BELOW_NORMAL`。比正常低一个级别。 |
| `Normal` | `0` | `"normal"` | `THREAD_PRIORITY_NORMAL`。大多数线程的默认优先级。 |
| `AboveNormal` | `1` | `"above normal"` | `THREAD_PRIORITY_ABOVE_NORMAL`。比正常高一个级别。 |
| `Highest` | `2` | `"highest"` | `THREAD_PRIORITY_HIGHEST`。比正常高两个级别。 |
| `TimeCritical` | `15` | `"time critical"` | `THREAD_PRIORITY_TIME_CRITICAL`。最高的常规调度优先级。使用时需极度谨慎，因为它可能会饿死其他线程。 |

## 方法

### `as_str`

```AffinityServiceRust/src/priority.rs#L186-L191
pub fn as_str(&self) -> &'static str
```

返回此变体的人类可读字符串名称（例如 `"normal"`、`"above normal"`）。如果变体在内部查找表中找不到（对于有效的枚举值不应发生），则返回 `"unknown"`。

### `as_win_const`

```AffinityServiceRust/src/priority.rs#L193-L195
pub fn as_win_const(&self) -> Option<i32>
```

返回此变体对应的 Win32 `i32` 常量，对于 `ThreadPriority::None` 哨兵则返回 `None`。返回值适合传递给 `SetThreadPriority`（包装在 `THREAD_PRIORITY` 中之后）。

### `from_str`

```AffinityServiceRust/src/priority.rs#L197-L204
pub fn from_str(s: &str) -> Self
```

将不区分大小写的字符串（例如 `"Above Normal"`、`"idle"`）解析为相应的 `ThreadPriority` 变体。如果字符串与任何已知优先级名称都不匹配，则返回 `ThreadPriority::None`。

### `from_win_const`

```AffinityServiceRust/src/priority.rs#L206-L212
pub fn from_win_const(val: i32) -> Self
```

将原始的 `i32` Win32 线程优先级值（例如 `GetThreadPriority` 返回的值）转换回对应的 `ThreadPriority` 变体。如果值与任何已知常量都不匹配，则返回 `ThreadPriority::None`。

### `boost_one`

```AffinityServiceRust/src/priority.rs#L215-L228
pub fn boost_one(&self) -> Self
```

返回标准调度阶梯中的下一个更高优先级级别。当主线程被提升为 prime 状态时，主线程引擎使用此方法将线程优先级提升一级。映射关系如下：

| 输入 | 输出 |
|------|------|
| `Idle` | `Lowest` |
| `Lowest` | `BelowNormal` |
| `BelowNormal` | `Normal` |
| `Normal` | `AboveNormal` |
| `AboveNormal` | `Highest` |
| `Highest` | `Highest` *（封顶）* |
| `TimeCritical` | `TimeCritical` *（封顶）* |
| `None` | `None` |
| `ErrorReturn` | `ErrorReturn` |
| `ModeBackgroundBegin` | `ModeBackgroundBegin` |
| `ModeBackgroundEnd` | `ModeBackgroundEnd` |

该函数将提升封顶在 `Highest` — 永远不会将线程提升到 `TimeCritical`。特殊变体（`None`、`ErrorReturn`、`ModeBackgroundBegin`、`ModeBackgroundEnd`）原样返回。

### `to_thread_priority_struct`

```AffinityServiceRust/src/priority.rs#L230-L232
pub fn to_thread_priority_struct(self) -> THREAD_PRIORITY
```

将此枚举变体转换为 `windows` crate API 所需的 `windows::Win32::System::Threading::THREAD_PRIORITY` 新类型结构。调用 `as_win_const()` 并将结果包装在 `THREAD_PRIORITY(...)` 中，如果变体为 `None` 则默认使用 `0`。

## 备注

- 内部查找表 `TABLE` 是一个 `&'static` 的 `(Self, &'static str, Option<i32>)` 元组切片，确保所有转换都是零分配的，在一个小的固定大小数组（11 个条目）上进行常量时间线性扫描。
- `from_str` 方法在匹配前将输入转为小写进行不区分大小写的比较。所有表条目使用小写字符串键。
- `ModeBackgroundBegin` 和 `ModeBackgroundEnd` 是特殊的 Win32 值，只能应用于**当前**线程。尝试通过 `SetThreadPriority` 使用任意线程句柄对远程线程设置这些值将会因 `ERROR_ACCESS_DENIED` 而失败。AffinityServiceRust 服务不对远程线程使用这些变体。
- [`ThreadStats`](../scheduler.rs/ThreadStats.md) 中的 `original_priority` 字段存储一个 `Option<ThreadPriority>`，以便服务可以在线程失去 prime 状态或进程退出时快照并随后恢复线程的调度优先级。
- `boost_one` 被设计为默认安全——它永远不会提升到 `TimeCritical`，如果广泛应用可能导致系统不稳定。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `priority.rs` |
| 调用者 | 配置解析器（`config.rs`）、应用引擎（`apply.rs`）、[`ThreadStats`](../scheduler.rs/ThreadStats.md) |
| 被调用者 | 无（具有转换方法的纯数据类型） |
| Win32 API | 对应 `SetThreadPriority` 接受的和 `GetThreadPriority` 返回的值 |
| 依赖 | `windows::Win32::System::Threading::THREAD_PRIORITY` |
| 权限 | 根据进程优先级类别，设置 `TimeCritical` 或提升到 `Normal` 以上可能需要 `SeIncreaseBasePriorityPrivilege` |

## 另请参阅

| 参考 | 链接 |
|------|------|
| ProcessPriority | [ProcessPriority](ProcessPriority.md) |
| IOPriority | [IOPriority](IOPriority.md) |
| MemoryPriority | [MemoryPriority](MemoryPriority.md) |
| MemoryPriorityInformation | [MemoryPriorityInformation](MemoryPriorityInformation.md) |
| ThreadStats | [ThreadStats](../scheduler.rs/ThreadStats.md) |
| priority 模块概述 | [README](README.md) |

---
*提交：[29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
