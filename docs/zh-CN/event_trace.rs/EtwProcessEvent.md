# EtwProcessEvent 结构体 (event_trace.rs)

从 ETW 接收的进程启动/停止事件，包含进程 ID 和事件是进程创建还是终止。

## 语法

```rust
#[derive(Debug, Clone)]
pub struct EtwProcessEvent {
    pub pid: u32,
    pub is_start: bool,
}
```

## 成员

`pid`

从 ETW 事件的 `UserData` 中提取的进程标识符（前 4 字节）。

`is_start`

如果事件是进程启动（ETW 事件 ID 1）则为 `true`，如果是进程停止（ETW 事件 ID 2）则为 `false`。

## 备注

事件通过 `mpsc` 通道从 ETW 回调发送到主循环。主循环每次迭代排空通道并使用事件：

- **启动事件**：将 PID 添加到 `process_level_pending` 以立即应用规则。
- **停止事件**：从调度器状态、错误去重映射和已应用跟踪集中清理该 PID。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/event_trace.rs |
| **创建者** | `etw_event_callback` |
| **消费者** | [`main`](../main.rs/main.md) 主循环 |

## 另请参阅

- [EtwProcessMonitor](EtwProcessMonitor.md)
- [event_trace.rs 模块概述](README.md)