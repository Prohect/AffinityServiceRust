# EtwProcessEvent 结构体 (event_trace.rs)

从 ETW（Windows 事件跟踪）接收的进程事件，表示进程启动或进程停止通知。此结构体的实例由 ETW 回调函数生成，并通过 [`EtwProcessMonitor::start`](EtwProcessMonitor.md) 返回的 `mpsc::Receiver<EtwProcessEvent>` 通道传递给消费者。

## 语法

```rust
#[derive(Debug, Clone)]
pub struct EtwProcessEvent {
    pub pid: u32,
    pub is_start: bool,
}
```

## 成员

| 字段 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 从 ETW 事件的 `UserData` 载荷（前 4 个字节）中提取的进程标识符。这是被创建或终止的进程的 PID。 |
| `is_start` | `bool` | 如果事件表示进程创建（ETW 事件 ID `1` — `ProcessStart`），则为 `true`。如果事件表示进程终止（ETW 事件 ID `2` — `ProcessStop`），则为 `false`。 |

## 备注

- 该结构体派生了 `Debug` 和 `Clone`，允许用于诊断打印以及在通道边界和集合操作中自由复制。

- 实例在 `extern "system"` ETW 回调函数（`etw_event_callback`）内部构造，该回调从原始 `EVENT_RECORD.UserData` 指针中提取 PID，并从 `EVENT_RECORD.EventHeader.EventDescriptor.Id` 确定事件类型。只有事件 ID `1`（启动）和 `2`（停止）会生成 `EtwProcessEvent` 值；所有其他事件 ID 会被回调静默丢弃。

- 事件通过全局 [`ETW_SENDER`](ETW_SENDER.md) 通道发送。如果发送端已被丢弃或通道已满，事件将被静默丢失（回调使用 `let _ = sender.send(...)` 忽略发送错误）。

- 消费者（通常是主调度循环）通过 [`EtwProcessMonitor::start`](EtwProcessMonitor.md) 返回的 `mpsc::Receiver<EtwProcessEvent>` 接收这些事件，并利用它们在新进程出现时主动应用亲和性/优先级规则，而不是仅依赖 [process 模块](../process.rs/README.md) 基于轮询的快照。

### ETW 事件 ID 映射

| ETW 事件 ID | `is_start` 值 | 含义 |
|-------------|---------------|------|
| `1` | `true` | `ProcessStart` — 新进程已创建。 |
| `2` | `false` | `ProcessStop` — 现有进程已终止。 |

### ETW 提供程序详细信息

事件来源于 `Microsoft-Windows-Kernel-Process` 提供程序（GUID `{22fb2cd6-0e7b-422b-a0c7-2fad1fd0e716}`），并使用关键字过滤器 `WINEVENT_KEYWORD_PROCESS`（`0x10`），确保只有进程相关事件被传递到回调。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `event_trace.rs` |
| **生成者** | `etw_event_callback`（模块私有的 `extern "system"` 函数） |
| **消费者** | 主调度循环，通过 `mpsc::Receiver<EtwProcessEvent>` |
| **传递通道** | [`ETW_SENDER`](ETW_SENDER.md) 全局通道 |
| **平台** | 仅限 Windows — 需要 ETW 基础设施 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| EtwProcessMonitor 结构体 | [EtwProcessMonitor](EtwProcessMonitor.md) |
| ETW_SENDER 静态变量 | [ETW_SENDER](ETW_SENDER.md) |
| ETW_ACTIVE 静态变量 | [ETW_ACTIVE](ETW_ACTIVE.md) |
| process 模块 | [process.rs](../process.rs/README.md) |
| event_trace 模块概述 | [README](README.md) |

---
*提交：[37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
