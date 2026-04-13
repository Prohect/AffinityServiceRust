# ETW_ACTIVE 静态变量 (event_trace.rs)

原子标志，指示 ETW 会话是否处于活动状态。

## 语法

```rust
static ETW_ACTIVE: AtomicBool = AtomicBool::new(false);
```

## 备注

当 `EtwProcessMonitor::start()` 成功时设置为 `true`，当 `stop()` 被调用时恢复为 `false`。用于防止重复停止操作。

## 另请参阅

- [EtwProcessMonitor](EtwProcessMonitor.md)