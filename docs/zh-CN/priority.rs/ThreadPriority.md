# ThreadPriority 枚举 (priority.rs)

表示 Windows 线程的优先级。此枚举封装了 `SetThreadPriority` 和 `GetThreadPriority` 使用的有符号整数常量，包括从 `Idle` (−15) 到 `TimeCritical` (15) 的标准调度级别，以及用于后台处理模式转换和错误返回的特殊哨兵值。`None` 变体表示未请求线程优先级更改。该枚举提供 Rust 变体、显示字符串和 Win32 整数值之间的往返转换，以及一个 `boost_one` 方法，用于主线程调度器的增量优先级提升。

## 语法

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadPriority {
    None,
    ErrorReturn,         // 0x7FFFFFFF
    ModeBackgroundBegin, // 0x00010000
    ModeBackgroundEnd,   // 0x00020000
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

| 变体 | Win32 值 | 描述 |
|---------|-------------|-------------|
| `None` | *(无)* | 哨兵值 — 未请求线程优先级更改。 |
| `ErrorReturn` | `0x7FFFFFFF` | `GetThreadPriority` 失败时返回的值 (`THREAD_PRIORITY_ERROR_RETURN`)。 |
| `ModeBackgroundBegin` | `0x00010000` | 将线程降低到后台处理模式。只能用于调用线程本身。 |
| `ModeBackgroundEnd` | `0x00020000` | 结束调用线程的后台处理模式。只能用于调用线程本身。 |
| `Idle` | `−15` | `THREAD_PRIORITY_IDLE` — 最低调度优先级。 |
| `Lowest` | `−2` | `THREAD_PRIORITY_LOWEST` — 比正常低两个级别。 |
| `BelowNormal` | `−1` | `THREAD_PRIORITY_BELOW_NORMAL` — 比正常低一个级别。 |
| `Normal` | `0` | `THREAD_PRIORITY_NORMAL` — 默认调度优先级。 |
| `AboveNormal` | `1` | `THREAD_PRIORITY_ABOVE_NORMAL` — 比正常高一个级别。 |
| `Highest` | `2` | `THREAD_PRIORITY_HIGHEST` — 比正常高两个级别。 |
| `TimeCritical` | `15` | `THREAD_PRIORITY_TIME_CRITICAL` — 最高调度优先级。 |

## 方法

### as_str

```rust
pub fn as_str(&self) -> &'static str
```

返回该变体的可读字符串表示（例如 `"below normal"`、`"time critical"`）。如果变体未在内部查找表中找到，则返回 `"unknown"`（对于有效变体不应出现此情况）。

### as_win_const

```rust
pub fn as_win_const(&self) -> Option<i32>
```

返回该变体对应的 Win32 整数常量，对于 `None` 哨兵变体返回 `None`。

### from_str

```rust
pub fn from_str(s: &str) -> Self
```

将大小写不敏感的字符串解析为 `ThreadPriority` 变体。如果字符串不匹配任何已知的优先级名称，则返回 `ThreadPriority::None`。此方法通过将输入转为小写后与内部查找表进行比较。

### from_win_const

```rust
pub fn from_win_const(val: i32) -> Self
```

通过 Win32 整数值查找 `ThreadPriority` 变体。如果值不匹配任何已知常量，则返回 `ThreadPriority::None`。

### boost_one

```rust
pub fn boost_one(&self) -> Self
```

返回下一个更高的标准优先级，上限为 `Highest`。此方法由主线程调度器使用，用于递增提升持续高 CPU 利用率的线程。

**提升链：** `Idle` → `Lowest` → `BelowNormal` → `Normal` → `AboveNormal` → `Highest` → `Highest`（封顶）。

以下变体为恒等映射（返回自身不变）：`None`、`ErrorReturn`、`ModeBackgroundBegin`、`ModeBackgroundEnd`、`TimeCritical`。

### to_thread_priority_struct

```rust
pub fn to_thread_priority_struct(self) -> THREAD_PRIORITY
```

将变体转换为 `windows::Win32::System::Threading::THREAD_PRIORITY` 结构体，以便直接用于 Win32 API。如果 `as_win_const` 返回 `None`，则回退为 `THREAD_PRIORITY(0)`（正常）。

## 备注

- 内部查找表 (`TABLE`) 将所有变体到字符串到常量的映射存储在单个 `&'static` 切片中，确保所有四个转换方法共享同一权威数据源。
- `ModeBackgroundBegin` 和 `ModeBackgroundEnd` 不是标准调度级别；它们是改变线程调度和 I/O 行为的控制码。Win32 文档规定这些只能应用于当前线程 — 将其应用于远程线程是未定义行为。AffinityServiceRust 不会将这些变体用于远程线程操作。
- `boost_one` 永远不会提升超过 `Highest`。故意排除了向 `TimeCritical` 的提升，因为 `TimeCritical` 会抢占大多数系统线程，如果广泛应用可能导致系统不稳定。
- 与 `ProcessPriority::from_win_const` 和 `IOPriority::from_win_const` 返回 `&'static str` 不同，`ThreadPriority::from_win_const` 返回 `Self`。这允许调用者进一步操作变体（例如调用 `boost_one`）。

## 要求

| 要求 | 值 |
|-------------|-------|
| 模块 | `priority` |
| 调用者 | [apply_prime_threads](../apply.rs/apply_prime_threads.md)、[apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md)、[apply_prime_threads_demote](../apply.rs/apply_prime_threads_demote.md)、[parse_and_insert_rules](../config.rs/parse_and_insert_rules.md) |
| 被调用者 | *(无 — 纯数据映射)* |
| Win32 API | `SetThreadPriority`、`GetThreadPriority`（通过 [apply 模块](../apply.rs/README.md) 间接使用） |
| 权限 | 为其他会话中的进程设置高于 `Normal` 的线程优先级可能需要 `SeDebugPrivilege`。 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 进程级优先级类 | [ProcessPriority](ProcessPriority.md) |
| I/O 优先级 | [IOPriority](IOPriority.md) |
| 内存优先级 | [MemoryPriority](MemoryPriority.md) |
| 主线程调度逻辑 | [scheduler 模块](../scheduler.rs/README.md) |
| 线程句柄获取 | [get_thread_handle](../winapi.rs/get_thread_handle.md) |
| 配置解析 | [config 模块](../config.rs/README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd