# EtwProcessMonitor 结构体（event_trace.rs）

管理 ETW（Windows 事件跟踪）实时跟踪会话的生命周期，该会话用于监控进程的启动和停止事件。监视器订阅 **Microsoft-Windows-Kernel-Process** 提供者，并通过标准的 `mpsc` 通道传递 [EtwProcessEvent](EtwProcessEvent.md) 值。

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

| 成员 | 类型 | 描述 |
|--------|------|-------------|
| `control_handle` | `CONTROLTRACE_HANDLE` | 由 `StartTraceW` 返回的句柄，用于控制（停止）ETW 会话。 |
| `trace_handle` | `PROCESSTRACE_HANDLE` | 由 `OpenTraceW` 返回的句柄，用于关闭跟踪消费者并解除阻塞处理线程。 |
| `properties_buf` | `Vec<u8>` | 支持 `EVENT_TRACE_PROPERTIES` 结构的堆分配缓冲区以及附加的会话名称。在会话持续期间保持活动状态，因为 `ControlTraceW` 在停止时需要它。 |
| `process_thread` | `Option<thread::JoinHandle<()>>` | 运行 `ProcessTrace` 的后台线程的 Join 句柄。在 `stop()` 期间线程加入后设置为 `None`。 |

## 方法

### start

```rust
pub fn start() -> Result<(Self, Receiver<EtwProcessEvent>), String>
```

创建并开始新的 ETW 实时跟踪会话以进行进程监控。

**返回值**

`Result<(EtwProcessMonitor, Receiver<EtwProcessEvent>), String>` — 成功时，返回监视器实例和通道接收器，该接收器生成 [EtwProcessEvent](EtwProcessEvent.md) 值。失败时，返回可读的错误字符串。

**备注**

启动顺序按顺序执行六个步骤：

1. **创建通道** — 分配一个 `mpsc` 通道，并将发送者安装到全局 [ETW_SENDER](README.md) 静态变量中，以便 OS 回调可以访问它。
2. **准备 `EVENT_TRACE_PROPERTIES`** — 分配足够大的缓冲区以容纳属性结构以及 UTF-16 会话名称 `"AffinityServiceRust_EtwProcessMonitor"`。配置为实时模式，使用 QPC 时间戳。
3. **停止现有会话** — 调用 `stop_existing_session` 以清理具有相同名称的任何陈旧会话（例如，来自以前的崩溃）。
4. **开始跟踪** — 调用 `StartTraceW` 创建 ETW 会话并获得控制句柄。
5. **启用提供者** — 调用 `EnableTraceEx2`，使用 `Microsoft-Windows-Kernel-Process` 提供者 GUID (`{22fb2cd6-0e7b-422b-a0c7-2fad1fd0e716}`) 和 `WINEVENT_KEYWORD_PROCESS` (`0x10`) 关键字，级别为 `TRACE_LEVEL_INFORMATION`。
6. **打开并处理跟踪** — 调用 `OpenTraceW`，带有 `PROCESS_TRACE_MODE_REAL_TIME | PROCESS_TRACE_MODE_EVENT_RECORD` 和 [etw_event_callback](README.md) 函数指针。启动名为 `"etw-process-trace"` 的后台线程调用 `ProcessTrace`，该线程在跟踪关闭之前会阻塞。

如果任何步骤失败，在返回错误之前释放所有先前获取的资源（停止会话，清除发送者）。

### stop

```rust
pub fn stop(&mut self)
```

停止 ETW 跟踪会话并释放所有资源。

**备注**

关闭顺序为：

1. 检查并清除全局 `ETW_ACTIVE` 标志。如果已经非活动，则立即返回（幂等）。
2. 在 `trace_handle` 上调用 `CloseTrace` 以解除阻塞后台线程中的 `ProcessTrace` 调用。
3. 使用 `EVENT_TRACE_CONTROL_STOP` 调用 `ControlTraceW` 终止 ETW 会话。
4. 加入后台处理线程。
5. 清除全局 `ETW_SENDER` 以丢弃通道发送者，这将使接收器检测到通道断开。

### stop_existing_session

```rust
fn stop_existing_session(wide_name: &[u16])
```

尝试停止给定名称的任何预存在的 ETW 会话。这是一个不需要 `EtwProcessMonitor` 实例的静态辅助函数。

**参数**

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `wide_name` | `&[u16]` | 要停止的以 null 结尾的 UTF-16 会话名称。 |

**备注**

分配一个临时的 `EVENT_TRACE_PROPERTIES` 缓冲区，并使用 `EVENT_TRACE_CONTROL_STOP` 调用 `ControlTraceW`。错误被静默忽略，因为会话可能不存在。

## 特质实现

### Drop

```rust
impl Drop for EtwProcessMonitor {
    fn drop(&mut self) {
        self.stop();
    }
}
```

`Drop` 实现调用 `stop()`，确保即使提前返回或 panic 时，当监视器超出范围时 ETW 会话总是被清理。

## 备注

- 一次只应该有一个 `EtwProcessMonitor` 处于活动状态，因为回调通过单个全局发送者（`ETW_SENDER`）通信。启动第二个监视器会替换发送者，使第一个监视器的接收者脱离关系。
- 会话名称 `"AffinityServiceRust_EtwProcessMonitor"` 是一个固定常量。如果服务崩溃而没有调用 `stop()`，陈旧的会话会保留在内核中，直到下一次调用 `start()` 通过 `stop_existing_session` 清理它。
- `ProcessTrace` 是一个阻塞的 Win32 调用，仅在跟踪句柄关闭时才返回。这就是为什么它在专用后台线程上运行，而不是在主线程上。
- `properties_buf` 必须在整个会话生命周期内保持有效并且地址相同，因为 `ControlTraceW` 在停止时会写回到同一个缓冲区。

## 要求

| | |
|---|---|
| **模块** | `src/event_trace.rs` |
| **调用方** | 主服务循环（`src/main.rs`），调度器（`src/scheduler.rs`） |
| **被调用方** | [etw_event_callback](README.md)（注册为 OS 回调） |
| **Win32 API** | [StartTraceW](https://learn.microsoft.com/zh-cn/windows/win32/api/evntrace/nf-evntrace-starttracew), [EnableTraceEx2](https://learn.microsoft.com/zh-cn/windows/win32/api/evntrace/nf-evntrace-enabletraceex2), [OpenTraceW](https://learn.microsoft.com/zh-cn/windows/win32/api/evntrace/nf-evntrace-opentracew), [ProcessTrace](https://learn.microsoft.com/zh-cn/windows/win32/api/evntrace/nf-evntrace-processtrace), [CloseTrace](https://learn.microsoft.com/zh-cn/windows/win32/api/evntrace/nf-evntrace-closetrace), [ControlTraceW](https://learn.microsoft.com/zh-cn/windows/win32/api/evntrace/nf-evntrace-controltracew) |
| **权限** | 管理员或 `SeSystemProfilePrivilege`（ETW 内核级提供者所需） |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 模块概述 | [event_trace.rs](README.md) |
| 事件负载 | [EtwProcessEvent](EtwProcessEvent.md) |
| 错误代码辅助 | [error_from_code_win32](../error_codes.rs/README.md) |
| 调度器集成 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*