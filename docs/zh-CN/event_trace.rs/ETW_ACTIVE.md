# ETW_ACTIVE 静态变量 (event_trace.rs)

一个原子布尔标志，指示 ETW (Windows 事件跟踪) 跟踪会话当前是否处于活动状态。此标志通过 `SeqCst` 排序在加载和存储操作中跨线程共享，无需互斥锁开销，以确保一致的可见性。它保护 [EtwProcessMonitor::stop](EtwProcessMonitor.md) 方法免受冗余拆除操作的影响，并标示整体会话生命周期状态。

## 语法

```event_trace.rs
static ETW_ACTIVE: AtomicBool = AtomicBool::new(false);
```

## 备注

`ETW_ACTIVE` 初始化为 `false`，并在 ETW (Windows 事件跟踪) 会话生命周期中经历以下状态转换：

| 状态 | 设置者 | 含义 |
|------|--------|------|
| `false` → `true` | [EtwProcessMonitor::start](EtwProcessMonitor.md) | 跟踪会话已成功启动，后台处理线程已生成。 |
| `true` → `false` | [EtwProcessMonitor::stop](EtwProcessMonitor.md) | 跟踪会话正在拆除。调用 `CloseTrace` 和带有 `EVENT_TRACE_CONTROL_STOP` 的 `ControlTraceW`，并等待后台线程加入。 |

### 线程安全

`ETW_ACTIVE` 对所有加载和存储操作使用 `Ordering::SeqCst`，这是最强的内存排序保证。这确保了：

- 一个线程上的 `stop` 方法能看到另一个线程上 `start` 设置的 `true` 值。
- 第二次调用 `stop`（包括来自 `Drop` 的隐式调用）能观察到第一次调用设置的 `false` 值并立即短路返回。

### 防止双重停止

`stop` 方法在入口处检查 `ETW_ACTIVE`，如果已经为 `false` 则立即返回。这防止了在显式调用 `stop` 后又通过 [EtwProcessMonitor](EtwProcessMonitor.md) 的 `Drop` 实现再次调用时出现的双重调用问题。

### 模块私有可见性

此静态变量**未**标记为 `pub` —— 它是 `event_trace` 模块的内部实现。外部代码仅通过 [EtwProcessMonitor](EtwProcessMonitor.md) API 与 ETW (Windows 事件跟踪) 会话进行交互。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `event_trace` |
| 类型 | `AtomicBool`（来自 `std::sync::atomic`） |
| 调用者 | [EtwProcessMonitor::start](EtwProcessMonitor.md)、[EtwProcessMonitor::stop](EtwProcessMonitor.md) |
| 被调用者 | *（无 —— 原子原语）* |
| 权限 | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 全局 ETW (Windows 事件跟踪) 事件发送通道 | [ETW_SENDER](ETW_SENDER.md) |
| ETW (Windows 事件跟踪) 会话管理器结构体 | [EtwProcessMonitor](EtwProcessMonitor.md) |
| 进程事件载荷 | [EtwProcessEvent](EtwProcessEvent.md) |
| 模块概述 | [event_trace 模块](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd