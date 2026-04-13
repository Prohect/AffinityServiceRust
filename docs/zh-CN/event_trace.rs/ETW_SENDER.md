# ETW_SENDER 静态变量 (event_trace.rs)

全局互斥锁保护的 MPSC 通道发送端，由 ETW (Windows 事件跟踪) 事件记录回调使用，用于将 [EtwProcessEvent](EtwProcessEvent.md) 值分发到主服务循环。由于 ETW 回调是一个没有闭包状态的 `extern "system"` 函数指针，发送端必须存储在全局静态变量中以便回调可以访问。

## 语法

```event_trace.rs
static ETW_SENDER: Lazy<Mutex<Option<Sender<EtwProcessEvent>>>> = Lazy::new(|| Mutex::new(None));
```

## 成员

| 组件 | 类型 | 描述 |
|------|------|------|
| 外层 | `Lazy<…>` | 通过 `once_cell::sync::Lazy` 延迟初始化。值在首次访问时创建。 |
| 中层 | `Mutex<…>` | 提供内部可变性和线程安全访问，支持从 ETW 回调线程和主线程同时访问。 |
| 内层 | `Option<Sender<EtwProcessEvent>>` | 当 ETW (Windows 事件跟踪) 会话处于活动状态时为 `Some(sender)`；当没有会话运行或清理之后为 `None`。 |

## 备注

- 该静态变量初始化为 `Mutex::new(None)`。当 [EtwProcessMonitor::start](EtwProcessMonitor.md) 被调用时，它会创建一个 `mpsc::channel`，将 `Sender` 端安装到此全局变量中，并将 `Receiver` 端返回给调用方。
- `extern "system"` ETW 事件记录回调 (`etw_event_callback`) 在每次事件分发时锁定此互斥锁，并通过通道发送一个 [EtwProcessEvent](EtwProcessEvent.md)。如果锁定或发送失败，事件将被静默丢弃——这是有意为之，以避免在 Windows 内核回调内部引发 panic。
- 当 [EtwProcessMonitor::stop](EtwProcessMonitor.md) 被调用时（或当监视器被丢弃时），内部 `Option` 被设回 `None`，这会丢弃 `Sender` 并导致接收端观察到通道已断开。
- 由于 `ETW_SENDER` 是一个进程级单例，同一时间只应有一个 `EtwProcessMonitor` 会话处于活动状态。启动第二个会话会覆盖发送端，导致之前的接收端成为孤立对象。

## 要求

| 要求 | 值 |
|------|---|
| 模块 | `event_trace` |
| Crate 依赖 | `once_cell` (`Lazy`)、`std::sync::Mutex`、`std::sync::mpsc::Sender` |
| 写入方 | [EtwProcessMonitor::start](EtwProcessMonitor.md) |
| 读取方 | `etw_event_callback`（模块私有 `extern "system"` 函数） |
| 清除方 | [EtwProcessMonitor::stop](EtwProcessMonitor.md) |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 会话活动状态的原子标志 | [ETW_ACTIVE](ETW_ACTIVE.md) |
| 通过通道发送的事件负载类型 | [EtwProcessEvent](EtwProcessEvent.md) |
| 管理 ETW (Windows 事件跟踪) 会话生命周期的监视器 | [EtwProcessMonitor](EtwProcessMonitor.md) |
| 模块概述 | [event_trace 模块](README.md) |