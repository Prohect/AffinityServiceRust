# EtwProcessMonitor 结构体 (event_trace.rs)

管理用于进程启动/停止监控的 ETW（Windows 事件跟踪）实时跟踪会话。`EtwProcessMonitor` 拥有会话的完整生命周期——包括控制句柄、跟踪句柄、属性缓冲区和后台处理线程——并在析构时自动清理所有资源。

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

| 字段 | 类型 | 描述 |
|------|------|------|
| `control_handle` | `CONTROLTRACE_HANDLE` | 由 `StartTraceW` 返回的句柄，用于通过 `ControlTraceW` 控制（停止）跟踪会话。 |
| `trace_handle` | `PROCESSTRACE_HANDLE` | 由 `OpenTraceW` 返回的句柄，用于实时消费事件以及通过 `CloseTrace` 关闭跟踪。 |
| `properties_buf` | `Vec<u8>` | `EVENT_TRACE_PROPERTIES` 结构体的后备缓冲区，包括附加的会话名称。在会话持续期间保持存活，因为 `ControlTraceW`（停止操作）需要有效的属性指针。 |
| `process_thread` | `Option<thread::JoinHandle<()>>` | 运行 `ProcessTrace` 的后台线程句柄。会话活跃时为 `Some`；当调用 `stop()` 并加入线程后变为 `None`（通过 `take()`）。 |

所有字段均为**私有**。外部代码仅通过其公共方法与 `EtwProcessMonitor` 交互。

## 方法

### `EtwProcessMonitor::start`

```rust
pub fn start() -> Result<(Self, Receiver<EtwProcessEvent>), String>
```

启动一个新的 ETW 跟踪会话，监控来自 `Microsoft-Windows-Kernel-Process` 提供程序的进程启动/停止事件。返回一个元组，包含监控器实例和一个 `mpsc::Receiver<EtwProcessEvent>`，调用者可从中读取事件。

#### 执行步骤

1. **创建通道** — 创建一个 `mpsc::channel`，并将发送端安装到全局 [ETW_SENDER](ETW_SENDER.md) 静态变量中，以便 `extern "system"` 回调函数能够访问它。
2. **准备属性** — 分配一个 `EVENT_TRACE_PROPERTIES` 缓冲区，附加会话名称 `"AffinityServiceRust_EtwProcessMonitor"`，配置为使用 QPC 时间戳的实时模式。
3. **停止残留会话** — 调用 `stop_existing_session` 来清除任何同名的残留会话（例如，之前崩溃遗留的会话）。
4. **启动跟踪** — 调用 `StartTraceW` 创建会话并获取 `control_handle`。
5. **启用提供程序** — 调用 `EnableTraceEx2` 订阅 `Microsoft-Windows-Kernel-Process` 提供程序，级别为 `TRACE_LEVEL_INFORMATION`，关键字为 `WINEVENT_KEYWORD_PROCESS` (0x10)。
6. **打开跟踪** — 以实时 + 事件记录模式调用 `OpenTraceW`，注册模块级 `etw_event_callback` 函数作为事件记录回调。
7. **生成后台线程** — 将 [ETW_ACTIVE](ETW_ACTIVE.md) 设置为 `true`，并生成一个名为 `"etw-process-trace"` 的线程，该线程调用 `ProcessTrace`，该调用会阻塞直到跟踪被关闭。

如果任何步骤失败，所有先前获取的资源都会被清理，并返回一个 `Err(String)`，其中包含描述性消息以及通过 [`error_from_code_win32`](../error_codes.rs/error_from_code_win32.md) 翻译的 Win32 错误代码。

### `EtwProcessMonitor::stop`

```rust
pub fn stop(&mut self)
```

停止 ETW 跟踪会话并释放所有关联资源：

1. 检查 [ETW_ACTIVE](ETW_ACTIVE.md)；如果已为 `false`，则立即返回（幂等操作）。
2. 将 `ETW_ACTIVE` 设置为 `false`。
3. 调用 `CloseTrace(trace_handle)` 以解除后台线程上 `ProcessTrace` 调用的阻塞。
4. 使用 `EVENT_TRACE_CONTROL_STOP` 调用 `ControlTraceW` 停止跟踪会话。
5. 加入后台线程（`process_thread.take().join()`）。
6. 清除全局 [ETW_SENDER](ETW_SENDER.md) 以释放通道发送端。

### `EtwProcessMonitor::stop_existing_session`（私有）

```rust
fn stop_existing_session(wide_name: &[u16])
```

尝试停止任何具有给定名称的已存在 ETW 会话。此方法在启动新会话之前调用，以处理应用程序先前实例崩溃而未清理其 ETW 会话的情况。失败会被静默忽略，因为会话可能不存在。

## 备注

- **RAII 清理。** `EtwProcessMonitor` 实现了 `Drop`，其委托给 `stop()`。这保证了即使监控器在没有显式调用 `stop()` 的情况下被析构，ETW 会话也会被正确销毁，防止留下孤立的系统级跟踪会话。

- **单实例约束。** 在同一系统上同时只能存在一个具有给定名称的 ETW 会话。`start()` 中的 `stop_existing_session` 调用处理了残留会话的情况，但尝试同时运行两个 AffinityServiceRust 实例将导致第二个 `StartTraceW` 失败。

- **全局回调桥接。** 由于 ETW 事件回调必须是 `extern "system"` 函数指针（不支持闭包或捕获状态），模块使用全局 [ETW_SENDER](ETW_SENDER.md) 静态变量将事件从回调桥接到 Rust 的 `mpsc` 通道。回调从 `UserData` 的前 4 个字节提取进程 ID，并发送一个 [EtwProcessEvent](EtwProcessEvent.md)，对于事件 ID 1（ProcessStart）设置 `is_start = true`，对于事件 ID 2（ProcessStop）设置 `is_start = false`。

- **提供程序详情。** `Microsoft-Windows-Kernel-Process` 提供程序 GUID 为 `{22fb2cd6-0e7b-422b-a0c7-2fad1fd0e716}`。关键字 `WINEVENT_KEYWORD_PROCESS` (`0x10`) 将事件过滤为仅包含进程生命周期事件（启动/停止），排除来自同一提供程序的线程和镜像加载事件。

- **线程命名。** 后台线程命名为 `"etw-process-trace"`，以便在调试器和线程分析器中进行诊断识别。

### 平台说明

- **仅限 Windows。** 依赖于 ETW 基础设施：`StartTraceW`、`EnableTraceEx2`、`OpenTraceW`、`ProcessTrace`、`CloseTrace`、`ControlTraceW`。
- 创建实时内核跟踪会话需要管理员权限。
- 会话使用 QPC（查询性能计数器）时间戳（`Wnode.ClientContext = 1`）以获得高精度事件计时。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `event_trace.rs` |
| **创建者** | `EtwProcessMonitor::start()` |
| **依赖** | [ETW_SENDER](ETW_SENDER.md)、[ETW_ACTIVE](ETW_ACTIVE.md)、[EtwProcessEvent](EtwProcessEvent.md)、[`error_from_code_win32`](../error_codes.rs/error_from_code_win32.md) |
| **Win32 API** | `StartTraceW`、`EnableTraceEx2`、`OpenTraceW`、`ProcessTrace`、`CloseTrace`、`ControlTraceW` |
| **ETW 提供程序** | `Microsoft-Windows-Kernel-Process` (`{22fb2cd6-0e7b-422b-a0c7-2fad1fd0e716}`) |
| **权限** | 需要管理员（提升）权限以创建内核跟踪会话 |
| **平台** | Windows |

## 另请参阅

| 主题 | 链接 |
|------|------|
| EtwProcessEvent 结构体 | [EtwProcessEvent](EtwProcessEvent.md) |
| ETW_SENDER 静态变量 | [ETW_SENDER](ETW_SENDER.md) |
| ETW_ACTIVE 静态变量 | [ETW_ACTIVE](ETW_ACTIVE.md) |
| error_from_code_win32 | [error_from_code_win32](../error_codes.rs/error_from_code_win32.md) |
| process 模块 | [process.rs](../process.rs/README.md) |
| event_trace 模块概览 | [README](README.md) |

---
> Commit SHA: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
