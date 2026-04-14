# EtwProcessEvent 结构体 (event_trace.rs)

表示从 ETW (Windows 事件跟踪) 实时跟踪会话接收到的进程生命周期事件。每个实例携带受影响进程的进程 ID 以及一个指示该事件是进程启动还是进程停止的标志。实例由内部 `etw_event_callback` 函数生成，并通过 MPSC 通道传递到主服务循环。

## 语法

```event_trace.rs
#[derive(Debug, Clone)]
pub struct EtwProcessEvent {
    pub pid: u32,
    pub is_start: bool,
}
```

## 成员

| 字段 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 被创建或终止的进程的进程标识符。从 ETW 事件记录的 `UserData` 载荷的前 4 个字节中提取。 |
| `is_start` | `bool` | 如果事件表示进程创建（ETW 事件 ID 1），则为 `true`；如果表示进程终止（ETW 事件 ID 2），则为 `false`。 |

## 备注

- 该结构体派生了 `Debug` 和 `Clone`，使其适合诊断日志记录以及在线程边界之间传递副本。它被有意设计为轻量级——两个标量字段，无堆内存分配——以最大限度减少高频 ETW 回调路径中的开销。
- 实例在 `extern "system"` 回调 `etw_event_callback` 内部创建，该回调运行在由 [EtwProcessMonitor::start](EtwProcessMonitor.md) 生成的 ETW 处理线程上。回调通过全局 [ETW_SENDER](ETW_SENDER.md) 通道发送每个事件。接收端由主服务循环持有，它使用 `is_start` 来决定是对新启动的进程应用配置规则，还是清理已终止进程的状态。
- `pid` 值直接来自 `Microsoft-Windows-Kernel-Process` 提供程序的事件载荷。对于 `ProcessStart` 事件（ID 1），这是新创建进程的 PID。对于 `ProcessStop` 事件（ID 2），这是已退出进程的 PID。
- 由于 ETW 回调从原始 `UserData` 字节中提取 PID，回调在读取之前会执行边界检查（`UserDataLength >= 4`）。如果检查失败，则不会生成 `EtwProcessEvent`。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `event_trace` |
| 生产者 | `etw_event_callback`（内部 `extern "system"` 函数） |
| 消费者 | [main 模块](../main.rs/README.md) 中的服务主循环 |
| 通道 | 通过 [ETW_SENDER](ETW_SENDER.md) 发送，从 [EtwProcessMonitor::start](EtwProcessMonitor.md) 返回的 `Receiver<EtwProcessEvent>` 接收 |
| ETW 提供程序 | `Microsoft-Windows-Kernel-Process` (`{22fb2cd6-0e7b-422b-a0c7-2fad1fd0e716}`) |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 生成这些事件的 ETW 会话管理器 | [EtwProcessMonitor](EtwProcessMonitor.md) |
| 回调使用的全局通道发送端 | [ETW_SENDER](ETW_SENDER.md) |
| 活跃会话标志 | [ETW_ACTIVE](ETW_ACTIVE.md) |
| event_trace 模块概述 | [event_trace 模块](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd