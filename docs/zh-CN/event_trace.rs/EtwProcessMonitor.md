# EtwProcessMonitor 结构体 (event_trace.rs)

管理 ETW 实时跟踪会话的进程监控器。处理会话创建、提供程序启用、后台事件处理和清理。

## 语法

```rust
pub struct EtwProcessMonitor {
    control_handle: CONTROLTRACE_HANDLE,
    trace_handle: PROCESSTRACE_HANDLE,
    properties_buf: Vec<u8>,
    process_thread: Option<thread::JoinHandle<()>>,
}
```

## 成员

`control_handle`

由 `StartTraceW` 返回的句柄，用于控制（停止）跟踪会话。

`trace_handle`

由 `OpenTraceW` 返回的句柄，用于 `ProcessTrace` 和 `CloseTrace`。

`properties_buf`

包含 `EVENT_TRACE_PROPERTIES` 结构和会话名称的字节缓冲区，供 `StartTraceW` 和 `ControlTraceW` 使用。

`process_thread`

运行 `ProcessTrace` 的后台线程的 join 句柄。在 `stop()` 期间取出并等待。

## 方法

| 方法 | 签名 | 描述 |
| --- | --- | --- |
| **start** | `pub fn start() -> Result<(Self, Receiver<EtwProcessEvent>), String>` | 启动新的 ETW 会话并返回监控器和事件接收端。 |
| **stop** | `pub fn stop(&mut self)` | 停止 ETW 会话，等待后台线程结束并清理。 |

### start

创建并配置 ETW 实时跟踪会话：

1. 安装全局 `ETW_SENDER` 供回调使用。
2. 准备带有 `EVENT_TRACE_REAL_TIME_MODE` 的 `EVENT_TRACE_PROPERTIES`。
3. 停止同名的现有会话（清理上次崩溃残留）。
4. 调用 `StartTraceW` 创建会话。
5. 调用 `EnableTraceEx2` 启用 `Microsoft-Windows-Kernel-Process` 提供程序 GUID 和 `WINEVENT_KEYWORD_PROCESS` 关键字。
6. 调用 `OpenTraceW` 设置 `PROCESS_TRACE_MODE_REAL_TIME | PROCESS_TRACE_MODE_EVENT_RECORD`。
7. 生成后台线程调用 `ProcessTrace`（阻塞直到会话停止）。

如果任何步骤失败，返回 `Err(String)` 并清理部分状态。

### stop

停止跟踪会话并清理所有资源：

1. 调用 `CloseTrace` 解除 `ProcessTrace` 的阻塞。
2. 调用 `ControlTraceW` 发送 `EVENT_TRACE_CONTROL_STOP`。
3. 等待后台处理线程结束。
4. 清除全局 `ETW_SENDER`。

也通过 `Drop` 实现自动调用。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/event_trace.rs |
| **创建者** | [`main`](../main.rs/main.md) |
| **Windows API** | `StartTraceW`、`EnableTraceEx2`、`OpenTraceW`、`ProcessTrace`、`CloseTrace`、`ControlTraceW` |

## 另请参阅

- [EtwProcessEvent](EtwProcessEvent.md)
- [ETW_SENDER](ETW_SENDER.md)
- [event_trace.rs 模块概述](README.md)