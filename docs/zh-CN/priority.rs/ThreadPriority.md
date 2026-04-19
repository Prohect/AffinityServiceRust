# ThreadPriority 枚举（priority.rs）

Windows 线程优先级级别的类型安全表示。在人类可读的字符串名称和 Win32 `i32` 线程优先级常量之间进行映射。提供了额外的 `boost_one` 方法用于逐步提升线程的优先级级别，Prime 线程提升算法中使用此方法。

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

| 变体 | 字符串名称 | Win32 值 | 描述 |
|---------|-------------|-------------|-------------|
| `None` | `"none"` | *(None)* | 未配置线程优先级。哨兵值，不会产生 Win32 API 调用。 |
| `ErrorReturn` | `"error"` | `0x7FFFFFFF` | `GetThreadPriority` 失败时返回的值（`THREAD_PRIORITY_ERROR_RETURN`）。 |
| `ModeBackgroundBegin` | `"background begin"` | `0x00010000` | 进入后台处理模式。仅对当前线程有效。 |
| `ModeBackgroundEnd` | `"background end"` | `0x00020000` | 退出后台处理模式。仅对当前线程有效。 |
| `Idle` | `"idle"` | `-15` | `THREAD_PRIORITY_IDLE` — `IDLE_PRIORITY_CLASS` 的基础优先级为 1，`REALTIME_PRIORITY_CLASS` 为 16。 |
| `Lowest` | `"lowest"` | `-2` | `THREAD_PRIORITY_LOWEST` — 比正常低 2 级。 |
| `BelowNormal` | `"below normal"` | `-1` | `THREAD_PRIORITY_BELOW_NORMAL` — 比正常低 1 级。 |
| `Normal` | `"normal"` | `0` | `THREAD_PRIORITY_NORMAL` — 默认线程优先级。 |
| `AboveNormal` | `"above normal"` | `1` | `THREAD_PRIORITY_ABOVE_NORMAL` — 比正常高 1 级。 |
| `Highest` | `"highest"` | `2` | `THREAD_PRIORITY_HIGHEST` — 比正常高 2 级。 |
| `TimeCritical` | `"time critical"` | `15` | `THREAD_PRIORITY_TIME_CRITICAL` — `IDLE_PRIORITY_CLASS` 的基础优先级为 15，`REALTIME_PRIORITY_CLASS` 为 31。 |

## 方法

### as_str

```rust
pub fn as_str(&self) -> &'static str
```

返回变体的人类可读字符串名称（例如，`"above normal"`）。如果在 `TABLE` 中找不到该变体则返回 `"unknown"`（对于正确构造的值不应发生）。

### as_win_const

```rust
pub fn as_win_const(&self) -> Option<i32>
```

返回 Win32 `i32` 线程优先级常量，或 `None` 用于 `ThreadPriority::None`。

### from_str

```rust
pub fn from_str(s: &str) -> Self
```

将大小写不敏感的字符串解析为 `ThreadPriority` 变体。如果字符串不匹配任何已知名称则返回 `ThreadPriority::None`。

### from_win_const

```rust
pub fn from_win_const(val: i32) -> Self
```

将 Win32 `i32` 线程优先级值转换回 `ThreadPriority` 变体。如果值不匹配任何已知常量则返回 `ThreadPriority::None`。

**注意：** 与 [`ProcessPriority::from_win_const`](ProcessPriority.md)、[`IOPriority::from_win_const`](IOPriority.md) 和 [`MemoryPriority::from_win_const`](MemoryPriority.md) 返回 `&'static str` 不同，此方法直接返回 `ThreadPriority` 枚举变体。这种差异存在是因为线程优先级被程序化使用（例如，在 `boost_one` 中）而不仅用于显示。

### boost_one

```rust
pub fn boost_one(&self) -> Self
```

返回标准优先级梯级中的下一个更高优先级级别。由 [`apply_prime_threads_promote`](../apply.rs/apply_prime_threads_promote.md) 中的 Prime 线程提升算法用于逐步提升线程的优先级。

**提升梯级：**

| 输入 | 输出 |
|-------|--------|
| `Idle` | `Lowest` |
| `Lowest` | `BelowNormal` |
| `BelowNormal` | `Normal` |
| `Normal` | `AboveNormal` |
| `AboveNormal` | `Highest` |
| `Highest` | `Highest` *(达到上限)* |
| `TimeCritical` | `TimeCritical` *(保持不变)* |
| `None` | `None` *(保持不变)* |
| `ErrorReturn` | `ErrorReturn` *(保持不变)* |
| `ModeBackgroundBegin` | `ModeBackgroundBegin` *(保持不变)* |
| `ModeBackgroundEnd` | `ModeBackgroundEnd` *(保持不变)* |

提升在 `Highest` 处达到上限 — 永远不会提升到可能导致系统不稳定的 `TimeCritical`。特殊变体（`None`、`ErrorReturn`、`ModeBackgroundBegin`、`ModeBackgroundEnd`）保持不变返回。

### to_thread_priority_struct

```rust
pub fn to_thread_priority_struct(self) -> THREAD_PRIORITY
```

将枚举转换为 `windows::Win32::System::Threading::THREAD_PRIORITY` 新类型包装器，用于 Win32 FFI 调用中的直接使用。如果 `as_win_const()` 返回 `None` 则回退到 `THREAD_PRIORITY(0)`（正常优先级）。

## 备注

### TABLE 常量

所有转换都由单个 `TABLE` 常量驱动：

```rust
const TABLE: &'static [(Self, &'static str, Option<i32>)] = &[...];
```

这个 `(变体、名称、win32_value)` 元组的数组是所有双向映射的单一真实来源，遵循本模块中其他优先级枚举相同的 DRY 模式。

### 后台模式变体

`ModeBackgroundBegin` 和 `ModeBackgroundEnd` 是特殊的线程优先级值，它们将调用线程切换到后台处理模式或退出该模式。在后台模式下，系统会降低线程的调度优先级、IO 优先级和内存优先级。这些值仅在对**当前**线程应用时有效 — 使用它们与远程线程的 `SetThreadPriority` 将失败。AffinityServiceRust 通常不会在远程线程上设置这些值；它们是为了完整性和 `from_win_const` 往返而包含的。

### 平台说明

- 线程优先级值是 `i32` 有符号整数，而进程优先级类是 `u32` 标志。
- `THREAD_PRIORITY` 新类型在 `windows`  crate 中包装了一个 `i32`。`to_thread_priority_struct` 方法为需要类型化 FFI 结构的调用方产生此包装器。
- `SetThreadPriority` 需要目标线程句柄上的 `THREAD_SET_INFORMATION` 访问权限。

## 要求

| | |
|---|---|
| **模块** | [`src/priority.rs`](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/priority.rs) |
| **使用者** | [`apply_prime_threads_promote`](../apply.rs/apply_prime_threads_promote.md)、[`apply_prime_threads_demote`](../apply.rs/apply_prime_threads_demote.md)、[配置解析](../config.rs/README.md) |
| **Win32 类型** | [`THREAD_PRIORITY`](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadpriority) |
| **权限** | `THREAD_SET_INFORMATION`（通过 `SetThreadPriority` 设置时） |

## 参见

| 主题 | 链接 |
|-------|------|
| 进程优先级枚举 | [ProcessPriority](ProcessPriority.md) |
| IO 优先级枚举 | [IOPriority](IOPriority.md) |
| 内存优先级枚举 | [MemoryPriority](MemoryPriority.md) |
| Prime 线程提升 | [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md) |
| Prime 线程降级 | [apply_prime_threads_demote](../apply.rs/apply_prime_threads_demote.md) |
| 模块概述 | [priority.rs](README.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*