# EtwProcessEvent 结构体 (event_trace.rs)

表示从 ETW（Windows 事件跟踪）实时跟踪会话接收到的单个进程生命周期事件。每个实例表示由 Microsoft-Windows-Kernel-Process 提供程序报告的一个进程已被创建或终止。

## 语法

```rust
#[derive(Debug, Clone)]
pub struct EtwProcessEvent {
    pub pid: u32,
    pub is_start: bool,
}
```

## 成员

| 成员 | 类型 | 描述 |
|--------|------|-------------|
| `pid` | `u32` | 从 ETW 事件的 `UserData` 载荷中提取的进程标识符（前 4 个字节）。 |
| `is_start` | `bool` | 如果事件表示进程启动（ETW 事件 ID 1）则为 `true`，如果表示进程停止（ETW 事件 ID 2）则为 `false`。 |

## 备注

- 该结构体的实例在 unsafe 的 `etw_event_callback` 函数内部创建，并通过全局 `ETW_SENDER` 通道发送到主服务线程上的消费者。
- 该结构体派生了 `Clone`，以便消费者可以保留事件的副本而无需担心生命周期问题，并派生了 `Debug` 用于诊断日志记录。
- `pid` 值直接来自内核事件载荷，在事件触发时有效。对于进程停止事件，当消费者处理消息时，PID 可能已经被回收，但在实践中这种情况很少见。

### 事件 ID 映射

| ETW 事件 ID | `is_start` 值 | 含义 |
|--------------|-------------------|---------|
| 1 | `true` | 进程启动 — 创建了新进程 |
| 2 | `false` | 进程停止 — 现有进程终止 |

来自 Microsoft-Windows-Kernel-Process 提供程序的所有其他事件 ID 都会被回调函数过滤掉，永远不会产生 `EtwProcessEvent`。

## 要求

| | |
|---|---|
| **模块** | `src/event_trace.rs` |
| **创建者** | `etw_event_callback` (unsafe extern "system" 函数) |
| **消费者** | 主服务循环，通过 [EtwProcessMonitor::start](EtwProcessMonitor.md) 返回的 `Receiver<EtwProcessEvent>` |
| **依赖项** | 无（纯数据结构） |
| **权限** | 无 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概览 | [event_trace.rs](README.md) |
| ETW 会话管理器 | [EtwProcessMonitor](EtwProcessMonitor.md) |
| 错误去重（消费者端） | [is_new_error](../logging.rs/is_new_error.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*