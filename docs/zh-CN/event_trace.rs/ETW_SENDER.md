# ETW_SENDER 静态变量 (event_trace.rs)

全局发送端，供 ETW 回调通过 `mpsc` 通道分发进程事件。

## 语法

```rust
static ETW_SENDER: Lazy<Mutex<Option<Sender<EtwProcessEvent>>>> = Lazy::new(|| Mutex::new(None));
```

## 备注

ETW 事件回调是 `extern "system"` 函数指针，无法捕获任何环境变量。此全局发送端提供回调与主循环通信的机制。在 `EtwProcessMonitor::start()` 期间设置为 `Some(sender)`，在 `stop()` 期间清除为 `None`。

## 另请参阅

- [EtwProcessMonitor](EtwProcessMonitor.md)
- [EtwProcessEvent](EtwProcessEvent.md)