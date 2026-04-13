# EtwProcessMonitor 结构体 (event_trace.rs)

管理一个 ETW (Windows 事件跟踪) 实时跟踪会话，用于监控来自 `Microsoft-Windows-Kernel-Process` 提供程序的进程启动和停止事件。`EtwProcessMonitor` 封装了 ETW 消费者会话的完整生命周期——启动跟踪、启用内核进程提供程序、打开跟踪以进行实时消费、在后台线程上处理事件，以及在停止或析构时清理所有资源。进程事件通过 `start` 返回的 `mpsc::Receiver<EtwProcessEvent>` 通道传递给调用者。

## 语法

```event_trace.rs
pub struct EtwProcessMonitor {
    control_handle: CONTROLTRACE_HANDLE,
    trace_handle: PROCESSTRACE_HANDLE,
    properties_buf: Vec<u8>,
    process_thread: Option<thread::JoinHandle<()>>,
}
```

## 成员

| 字段 | 类型 | 描述 |
|------|------|------|
| `control_handle` | `CONTROLTRACE_HANDLE` | 由 `StartTraceW` 返回的句柄，用于通过 `ControlTraceW` 控制（停止）跟踪会话。 |
| `trace_handle` | `PROCESSTRACE_HANDLE` | 由 `OpenTraceW` 返回的句柄，用于通过 `CloseTrace` 关闭跟踪消费者，从而解除后台线程上 `ProcessTrace` 调用的阻塞。 |
| `properties_buf` | `Vec<u8>` | 堆分配的缓冲区，用于保存 `EVENT_TRACE_PROPERTIES` 结构体及其后续的宽字符串会话名称。在会话期间保持存活，因为 `ControlTraceW(EVENT_TRACE_CONTROL_STOP)` 会写回此缓冲区。 |
| `process_thread` | `Option<thread::JoinHandle<()>>` | 运行 `ProcessTrace` 的后台线程的 join 句柄。在 `stop` 期间线程被 join 后设置为 `None`。 |

## 方法

### start

```event_trace.rs
pub fn start() -> Result<(Self, Receiver<EtwProcessEvent>), String>
```

启动一个新的 ETW (Windows 事件跟踪) 实时跟踪会话，并返回监视器句柄与 [EtwProcessEvent](EtwProcessEvent.md) 值的接收器配对。

**启动流程：**

1. 创建一个 `mpsc::channel`，并将发送端安装到全局 [ETW_SENDER](ETW_SENDER.md) 中。
2. 分配并初始化一个配置为实时模式、使用 QPC 时间戳的 `EVENT_TRACE_PROPERTIES` 结构体。
3. 调用 [stop_existing_session](#stop_existing_session) 清理上次崩溃遗留的孤立会话。
4. 调用 `StartTraceW` 创建名为 `"AffinityServiceRust_EtwProcessMonitor"` 的命名会话。
5. 调用 `EnableTraceEx2` 启用 `Microsoft-Windows-Kernel-Process` 提供程序（GUID `{22fb2cd6-0e7b-422b-a0c7-2fad1fd0e716}`），关键字为 `WINEVENT_KEYWORD_PROCESS`（`0x10`），级别为 `TRACE_LEVEL_INFORMATION`。
6. 调用 `OpenTraceW`，使用 `PROCESS_TRACE_MODE_REAL_TIME | PROCESS_TRACE_MODE_EVENT_RECORD` 和 `etw_event_callback` 函数指针。
7. 将 [ETW_ACTIVE](ETW_ACTIVE.md) 设置为 `true`。
8. 生成一个名为 `"etw-process-trace"` 的后台线程，该线程调用 `ProcessTrace`（阻塞直到跟踪关闭）。

如果任何步骤失败，所有先前获取的资源将被清理，全局发送端将在返回 `Err(String)` 之前被清除，错误消息包含从 [error_from_code_win32](../error_codes.rs/error_from_code_win32.md) 翻译的 Win32 错误代码的描述。

**返回值**

成功时返回 `Ok((EtwProcessMonitor, Receiver<EtwProcessEvent>))`。调用者应轮询或迭代接收器以处理传入的 [EtwProcessEvent](EtwProcessEvent.md) 值。失败时返回 `Err(String)`，描述哪个 ETW API 调用失败以及相关的错误代码。

### stop

```event_trace.rs
pub fn stop(&mut self)
```

停止 ETW (Windows 事件跟踪) 跟踪会话并释放所有相关资源。此方法是幂等的——在第一次成功停止后多次调用不会产生任何效果。

**关闭流程：**

1. 检查 [ETW_ACTIVE](ETW_ACTIVE.md)；如果已经为 `false` 则立即返回。
2. 将 `ETW_ACTIVE` 设置为 `false`。
3. 对 `trace_handle` 调用 `CloseTrace`，解除后台线程上运行的 `ProcessTrace` 调用的阻塞。
4. 使用 `EVENT_TRACE_CONTROL_STOP` 调用 `ControlTraceW` 以终止跟踪会话。
5. Join 后台处理线程（等待其退出）。
6. 将全局 [ETW_SENDER](ETW_SENDER.md) 清除为 `None`，丢弃发送端并关闭通道。

`stop` 返回后，从 `start` 获取的接收器将不再产生事件，任何挂起的 `recv` 调用将返回 `Err(RecvError)`。

### stop_existing_session

```event_trace.rs
fn stop_existing_session(wide_name: &[u16])
```

尝试停止任何先前存在的同名 ETW (Windows 事件跟踪) 会话。这是一个在 `start` 期间调用的私有辅助方法，用于清理异常终止（崩溃、终止、调试器分离）后可能残留的孤立会话。它会静默忽略错误，因为该会话可能不存在。

**参数**

| 参数 | 类型 | 描述 |
|------|------|------|
| `wide_name` | `&[u16]` | 要停止的以 null 结尾的 UTF-16 会话名称。始终为编码为 UTF-16 的 `"AffinityServiceRust_EtwProcessMonitor"`。 |

## Drop

`EtwProcessMonitor` 实现了 `Drop`，将操作委托给 `stop()`。这确保即使调用者忘记显式停止，或者监视器因错误路径而离开作用域，ETW (Windows 事件跟踪) 会话也总是会被清理。

```event_trace.rs
impl Drop for EtwProcessMonitor {
    fn drop(&mut self) {
        self.stop();
    }
}
```

## 备注

- **会话命名：** 会话以名称 `"AffinityServiceRust_EtwProcessMonitor"` 注册。系统上同一时间只能存在一个具有给定名称的 ETW (Windows 事件跟踪) 会话。启动时的 `stop_existing_session` 清理处理了服务先前实例崩溃而未停止其跟踪的情况。
- **回调架构：** 由于 ETW (Windows 事件跟踪) 通过 `extern "system"` 函数指针回调（`etw_event_callback`）传递事件，因此无法直接传递闭包或接收器。相反，该模块使用全局 [ETW_SENDER](ETW_SENDER.md) `Lazy<Mutex<Option<Sender<EtwProcessEvent>>>>` 来桥接从回调到 Rust 的 `mpsc` 通道。
- **线程模型：** `ProcessTrace` 是一个阻塞调用，在跟踪关闭之前不会返回。它运行在一个专用的后台线程（`"etw-process-trace"`）上，以避免阻塞服务的主循环。
- **事件过滤：** ETW (Windows 事件跟踪) 提供程序以关键字 `0x10`（`WINEVENT_KEYWORD_PROCESS`）启用，这将传递限制为仅进程生命周期事件（启动/停止）。回调进一步过滤为事件 ID 1（启动）和事件 ID 2（停止），丢弃所有其他事件。
- **权限：** 启动 ETW (Windows 事件跟踪) 跟踪会话通常需要管理员权限。AffinityServiceRust 已以提升权限运行，因此这不是额外的要求。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `event_trace` |
| 调用者 | [main](../main.rs/README.md)（创建并拥有监视器，生命周期贯穿整个服务循环） |
| 被调用者 | [error_from_code_win32](../error_codes.rs/error_from_code_win32.md)、[ETW_SENDER](ETW_SENDER.md)、[ETW_ACTIVE](ETW_ACTIVE.md) |
| Win32 API | `StartTraceW`、`EnableTraceEx2`、`OpenTraceW`、`ProcessTrace`、`CloseTrace`、`ControlTraceW` |
| ETW 提供程序 | `Microsoft-Windows-Kernel-Process`（`{22fb2cd6-0e7b-422b-a0c7-2fad1fd0e716}`） |
| 权限 | 管理员提升（从服务上下文继承） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 事件载荷结构体 | [EtwProcessEvent](EtwProcessEvent.md) |
| ETW 回调的全局发送端 | [ETW_SENDER](ETW_SENDER.md) |
| ETW 会话的活动标志 | [ETW_ACTIVE](ETW_ACTIVE.md) |
| Win32 错误代码翻译 | [error_from_code_win32](../error_codes.rs/error_from_code_win32.md) |
| event_trace 模块概述 | [event_trace 模块](README.md) |