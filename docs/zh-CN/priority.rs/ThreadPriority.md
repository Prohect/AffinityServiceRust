# ThreadPriority 枚举 (priority.rs)

表示 Windows 线程优先级级别。每个变体映射到 `SetThreadPriority` 使用的整数常量。提供双向转换和优先级提升辅助方法。

## 语法

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadPriority {
    None,
    ErrorReturn,
    ModeBackgroundBegin,
    ModeBackgroundEnd,
    Idle,
    Lowest,
    BelowNormal,
    Normal,
    AboveNormal,
    Highest,
    TimeCritical,
}
```

## 成员

`None`

不更改线程优先级。用作配置中的占位符，表示不对线程优先级进行干预。

`ErrorReturn`

对应 Windows 常量 `THREAD_PRIORITY_ERROR_RETURN`（`0x7FFFFFFF`）。表示 `GetThreadPriority` 调用失败时的返回值，不应用于设置优先级。

`ModeBackgroundBegin`

对应 `THREAD_MODE_BACKGROUND_BEGIN`（`0x00010000`）。将当前线程置于后台处理模式，降低 I/O 和调度优先级。仅可用于当前线程。

`ModeBackgroundEnd`

对应 `THREAD_MODE_BACKGROUND_END`（`0x00020000`）。结束当前线程的后台处理模式。仅可用于当前线程。

`Idle`

对应 `THREAD_PRIORITY_IDLE`（`-15`）。线程仅在系统空闲时运行。

`Lowest`

对应 `THREAD_PRIORITY_LOWEST`（`-2`）。优先级比正常低两级。

`BelowNormal`

对应 `THREAD_PRIORITY_BELOW_NORMAL`（`-1`）。优先级比正常低一级。

`Normal`

对应 `THREAD_PRIORITY_NORMAL`（`0`）。默认线程优先级。

`AboveNormal`

对应 `THREAD_PRIORITY_ABOVE_NORMAL`（`1`）。优先级比正常高一级。

`Highest`

对应 `THREAD_PRIORITY_HIGHEST`（`2`）。优先级比正常高两级。

`TimeCritical`

对应 `THREAD_PRIORITY_TIME_CRITICAL`（`15`）。最高可用线程优先级，适用于对时间极度敏感的任务。

## 方法

| 方法 | 签名 | 描述 |
| --- | --- | --- |
| **as_str** | `pub fn as_str(&self) -> &'static str` | 返回变体的人类可读字符串名称（例如 `"normal"`、`"highest"`）。 |
| **as_win_const** | `pub fn as_win_const(&self) -> Option<i32>` | 返回对应的 Windows 整数常量。`None` 变体返回 `Option::None`。 |
| **from_str** | `pub fn from_str(s: &str) -> Self` | 从不区分大小写的字符串解析变体。无法识别的字符串返回 `ThreadPriority::None`。 |
| **from_win_const** | `pub fn from_win_const(val: i32) -> Self` | 从 Windows 整数常量反向查找变体。未匹配的值返回 `ThreadPriority::None`。 |
| **boost_one** | `pub fn boost_one(&self) -> Self` | 返回比当前高一级的优先级，上限为 `Highest`。 |
| **to_thread_priority_struct** | `pub fn to_thread_priority_struct(self) -> THREAD_PRIORITY` | 将变体转换为 Windows `THREAD_PRIORITY` 结构体，用于直接传递给 Win32 API。 |

### boost_one 提升映射

`boost_one()` 按以下规则返回高一级优先级：

| 当前值 | 返回值 |
| --- | --- |
| `None` | `None`（不变） |
| `ErrorReturn` | `ErrorReturn`（不变） |
| `ModeBackgroundBegin` | `ModeBackgroundBegin`（不变） |
| `ModeBackgroundEnd` | `ModeBackgroundEnd`（不变） |
| `Idle` | `Lowest` |
| `Lowest` | `BelowNormal` |
| `BelowNormal` | `Normal` |
| `Normal` | `AboveNormal` |
| `AboveNormal` | `Highest` |
| `Highest` | `Highest`（上限） |
| `TimeCritical` | `TimeCritical`（不变） |

特殊变体（`None`、`ErrorReturn`、`ModeBackgroundBegin`、`ModeBackgroundEnd`、`TimeCritical`）不参与提升序列，原样返回。常规优先级从 `Idle` 到 `Highest` 形成提升链，`Highest` 为上限。

## 备注

### 查找表模式

与本模块中其他枚举一致，`ThreadPriority` 使用静态常量 `TABLE` 存储 `(Self, &str, Option<i32>)` 三元组，实现所有转换方法的双向查找：

```rust
const TABLE: &'static [(Self, &'static str, Option<i32>)] = &[
    (Self::None, "none", None),
    (Self::ErrorReturn, "error", Some(0x7FFFFFFF_i32)),
    (Self::ModeBackgroundBegin, "background begin", Some(0x00010000_i32)),
    (Self::ModeBackgroundEnd, "background end", Some(0x00020000_i32)),
    (Self::Idle, "idle", Some(-15)),
    (Self::Lowest, "lowest", Some(-2)),
    (Self::BelowNormal, "below normal", Some(-1)),
    (Self::Normal, "normal", Some(0)),
    (Self::AboveNormal, "above normal", Some(1)),
    (Self::Highest, "highest", Some(2)),
    (Self::TimeCritical, "time critical", Some(15)),
];
```

### Prime 线程优先级提升

`boost_one()` 主要用于 prime 线程调度场景。当 [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) 将某线程选为 prime 线程时，会调用 `boost_one()` 将其原始优先级提升一级，以获得更多 CPU 调度时间。当该线程被降级时，恢复为 `original_priority`。

### to_thread_priority_struct

`to_thread_priority_struct()` 将 `as_win_const()` 的返回值包装为 Windows `THREAD_PRIORITY` 结构体（`THREAD_PRIORITY(i32)`）。若 `as_win_const()` 返回 `Option::None`（即 `ThreadPriority::None` 变体），则使用 `0`（等同于 `THREAD_PRIORITY_NORMAL`）。

### 背景模式变体

`ModeBackgroundBegin` 和 `ModeBackgroundEnd` 在 Windows 中仅适用于当前线程。对远程线程调用 `SetThreadPriority` 传入这些值会失败。本项目的配置解析器接受这些值，但在实际应用中应谨慎使用。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/priority.rs |
| **行号** | L162–L261 |
| **依赖** | `windows::Win32::System::Threading::THREAD_PRIORITY` |
| **消费者** | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md)、[ThreadStats](../scheduler.rs/ThreadStats.md)、[apply_prime_threads](../apply.rs/apply_prime_threads.md) |

## 另请参阅

- [priority.rs 模块概述](README.md)
- [ProcessPriority 枚举](ProcessPriority.md)
- [ThreadStats 结构体](../scheduler.rs/ThreadStats.md) — `original_priority` 字段存储此类型
- [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) — prime 线程的优先级提升/恢复逻辑
- [SetThreadPriority](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadpriority) — Windows API 参考