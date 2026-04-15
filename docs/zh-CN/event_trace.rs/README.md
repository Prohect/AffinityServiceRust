# event_trace 模块 (AffinityServiceRust)

`event_trace` 模块提供了一个精简的 ETW（Windows 事件跟踪）消费者，用于实时监控进程的启动和停止。它使用 `Microsoft-Windows-Kernel-Process` 提供程序（GUID `{22fb2cd6-0e7b-422b-a0c7-2fad1fd0e716}`）来接收进程创建或终止的通知，从而无需轮询即可实现响应式的规则应用。

该模块在后台线程上管理一个 ETW 实时跟踪会话。全局通道（`ETW_SENDER`）通过 `mpsc::Sender` 将操作系统级别的 `extern "system"` 回调与安全的 Rust 代码桥接起来。`EtwProcessMonitor` 结构体拥有会话的生命周期，并在销毁时自动清理资源。

## 函数

本模块不直接公开公共函数。所有功能均通过 `EtwProcessMonitor` 结构体的方法访问。

## 结构体

| 结构体 | 描述 |
|--------|------|
| [EtwProcessEvent](EtwProcessEvent.md) | 从 ETW 接收的进程事件，包含进程 ID 以及该事件是启动事件还是停止事件。 |
| [EtwProcessMonitor](EtwProcessMonitor.md) | 管理用于进程监控的 ETW 实时跟踪会话，包括会话设置、后台线程生命周期和清理。 |

## 静态变量

| 静态变量 | 描述 |
|----------|------|
| [ETW_SENDER](ETW_SENDER.md) | 全局 `Mutex<Option<Sender<EtwProcessEvent>>>`，供 ETW 回调向消费者发送事件。之所以需要全局静态变量，是因为 ETW 回调是一个 `extern "system"` 函数指针，无法捕获状态。 |
| [ETW_ACTIVE](ETW_ACTIVE.md) | `AtomicBool` 标志，指示 ETW 会话当前是否处于活动状态。用于防止冗余的停止操作。 |

## 另请参阅

| 链接 | 描述 |
|------|------|
| [process.rs 模块](../process.rs/README.md) | 与 ETW 配合使用的进程快照枚举，用于进程数据查找。 |
| [logging.rs 模块](../logging.rs/README.md) | 用于诊断和错误报告的日志工具。 |
| [error_codes.rs 模块](../error_codes.rs/README.md) | ETW API 调用失败时使用的 Win32 错误码翻译。 |

---
源代码提交: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
