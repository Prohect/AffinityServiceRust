# event_trace 模块 (AffinityServiceRust)

`event_trace` 模块实现了最小的 ETW（Windows 事件跟踪）消费者，用于实时进程启动/停止监控。它订阅 **Microsoft-Windows-Kernel-Process** 提供程序（`{22fb2cd6-0e7b-422b-a0c7-2fad1fd0e716}`），并通过标准 `mpsc` 通道交付 `EtwProcessEvent` 消息，使主服务循环能够在进程创建或终止时立即响应并应用配置规则——无需轮询。

## 静态变量

| 名称 | 类型 | 描述 |
|------|------|-------------|
| `ETW_SENDER` | `Lazy<Mutex<Option<Sender<EtwProcessEvent>>>>` | 全局通道发送者，由操作系统调用的 [etw_event_callback](#etw_event_callback) 使用，用于转发事件。由于回调是一个 `extern "system"` 函数指针，因此需要全局变量来桥接到 Rust 的通道基础设施。 |
| `ETW_ACTIVE` | `AtomicBool` | 指示当前是否有 ETW 会话活动的标志。[stop](EtwProcessMonitor.md#stop) 通过检查此标志来避免重复清理。 |

## 函数

| 名称 | 描述 |
|------|------|
| <a id="etw_event_callback"></a>`etw_event_callback` | `unsafe extern "system"` 回调，由操作系统为每个 ETW 事件记录调用。从 `UserData` 的前 4 个字节中提取进程 ID，将事件 ID 1 映射为启动，事件 ID 2 映射为停止，并通过 `ETW_SENDER` 发送 [EtwProcessEvent](EtwProcessEvent.md)。非进程事件和空记录被静默丢弃。 |

## 结构体

| 名称 | 描述 |
|------|------|
| [EtwProcessEvent](EtwProcessEvent.md) | 轻量级值，表示从 ETW 接收的进程启动或停止事件。 |
| [EtwProcessMonitor](EtwProcessMonitor.md) | RAII 句柄，负责 ETW 会话的生命周期——创建、后台处理和清理。 |

## 常量

| 名称 | 值 | 描述 |
|------|-----|------|
| `KERNEL_PROCESS_GUID` | `{22fb2cd6-0e7b-422b-a0c7-2fad1fd0e716}` | Microsoft-Windows-Kernel-Process ETW 提供程序的 GUID。 |
| `WINEVENT_KEYWORD_PROCESS` | `0x10` | 选择进程生命周期事件的关键词掩码位。 |
| `SESSION_NAME` | `"AffinityServiceRust_EtwProcessMonitor"` | 在 ETW 子系统注册的跟踪会话名称。用于检测和在启动时清理残留的会话。 |

## 参见

| 主题 | 链接 |
|-------|------|
| 主服务循环（ETW 消费者） | [main.rs](../main.rs/README.md) |
| 错误代码格式化 | [error_codes.rs](../error_codes.rs/README.md) |
| Microsoft-Windows-Kernel-Process 提供程序 | [Microsoft 文档](https://learn.microsoft.com/zh-cn/windows/win32/etw/event-tracing-portal) |
| EVENT_TRACE_PROPERTIES | [Microsoft 文档](https://learn.microsoft.com/zh-cn/windows/win32/api/evntrace/ns-evntrace-event_trace_properties) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*