# ETW_SENDER 静态变量 (event_trace.rs)

ETW 事件回调使用的全局通道发送端，用于将进程事件传递给消费者。由于 ETW 回调是一个 `extern "system"` 函数指针，无法捕获任何状态，因此需要一个全局静态变量来桥接操作系统级别的回调与 Rust 的 `mpsc` 通道基础设施。

## 语法

```rust
static ETW_SENDER: Lazy<Mutex<Option<Sender<EtwProcessEvent>>>> = Lazy::new(|| Mutex::new(None));
```

## 类型

`once_cell::sync::Lazy<std::sync::Mutex<Option<std::sync::mpsc::Sender<EtwProcessEvent>>>>`

## 备注

### 生命周期

1. **初始状态** — `None`。静态变量在首次访问时初始化为 `Mutex::new(None)`。
2. **激活** — 当调用 [`EtwProcessMonitor::start`](EtwProcessMonitor.md) 时，它会创建一个 `mpsc::channel`，将 `Sender` 端安装到 `ETW_SENDER` 中，并将 `Receiver` 端返回给调用者。
3. **使用中** — `etw_event_callback` 函数（一个 `unsafe extern "system"` 回调，由操作系统为每个 ETW 事件调用）锁定互斥量，检查是否为 `Some(ref sender)`，并通过通道发送 [`EtwProcessEvent`](EtwProcessEvent.md) 实例。
4. **停用** — 当调用 [`EtwProcessMonitor::stop`](EtwProcessMonitor.md)（或监视器被丢弃）时，发送端被替换为 `None`，这会导致接收端观察到通道已断开。

### 为什么使用全局静态变量？

Windows ETW 要求事件记录回调必须是原始函数指针（`unsafe extern "system" fn(*mut EVENT_RECORD)`）。Rust 闭包和 trait 对象在捕获环境状态时无法用作原始函数指针。全局 `ETW_SENDER` 静态变量提供了一个固定的、已知的位置，回调可以在此找到通道发送端，而无需捕获任何变量。

### 线程安全

- `Mutex` 确保来自 ETW 回调线程和控制线程（调用 `start` / `stop` 的线程）的并发访问得到正确同步。
- ETW 回调线程为每个事件短暂获取锁，通过通道发送事件。如果锁被污染（例如，持有锁期间发生了 panic），回调会静默丢弃该事件。
- 来自 `once_cell` 的 `Lazy` 包装器确保线程安全的一次性初始化。

### 回调中的错误处理

回调使用防御性访问模式：

```rust
if let Ok(guard) = ETW_SENDER.lock()
    && let Some(ref sender) = *guard
{
    let _ = sender.send(EtwProcessEvent { pid, is_start });
}
```

- 如果互斥量被污染，事件会被静默丢弃。
- 如果发送端为 `None`（监视器未启动或已停止），事件会被静默丢弃。
- 如果通道已断开（接收端被丢弃），`sender.send()` 返回 `Err`，通过 `let _` 忽略。

### 清理

当 ETW 会话通过 [`EtwProcessMonitor::stop`](EtwProcessMonitor.md) 停止或监视器被丢弃时，全局发送端被设置回 `None`：

```rust
if let Ok(mut guard) = ETW_SENDER.lock() {
    *guard = None;
}
```

这在 [`EtwProcessMonitor::start`](EtwProcessMonitor.md) 期间的错误路径中也会发生，如果 `StartTraceW`、`EnableTraceEx2` 或 `OpenTraceW` 失败，确保不会残留悬空的发送端。

## 要求

| 要求 | 值 |
|-------------|-------|
| **模块** | `event_trace.rs` |
| **可见性** | 私有（模块内部） |
| **访问者** | `etw_event_callback`（操作系统调用的 ETW 回调）、[`EtwProcessMonitor::start`](EtwProcessMonitor.md)、[`EtwProcessMonitor::stop`](EtwProcessMonitor.md) |
| **依赖项** | `once_cell::sync::Lazy`、`std::sync::Mutex`、`std::sync::mpsc::Sender`、[`EtwProcessEvent`](EtwProcessEvent.md) |
| **平台** | 仅限 Windows |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| ETW_ACTIVE 静态变量 | [ETW_ACTIVE](ETW_ACTIVE.md) |
| EtwProcessEvent 结构体 | [EtwProcessEvent](EtwProcessEvent.md) |
| EtwProcessMonitor 结构体 | [EtwProcessMonitor](EtwProcessMonitor.md) |
| event_trace 模块概述 | [README](README.md) |

---
> Commit SHA: `7221ea0694670265d4eb4975582d8ed2ae02439d`
