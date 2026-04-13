# event_trace 模块 (AffinityServiceRust)

`event_trace` 模块提供了一个精简的 ETW (Windows 事件跟踪) 消费者，用于实时监控进程的启动与停止。它订阅 `Microsoft-Windows-Kernel-Process` 提供程序以接收进程创建或终止的通知，使服务能够在新进程启动时立即应用配置规则，而无需等待下一个轮询间隔。该模块管理完整的 ETW (Windows 事件跟踪) 会话生命周期——启动、消费和停止跟踪会话——并通过通道分发进程事件，供主服务循环消费。

## 静态变量

| 静态变量 | 描述 |
|--------|-------------|
| [ETW_SENDER](ETW_SENDER.md) | 全局互斥锁保护的 MPSC 通道发送端，供 ETW (Windows 事件跟踪) 回调用于分发进程事件。 |
| [ETW_ACTIVE](ETW_ACTIVE.md) | 原子布尔标志，指示 ETW (Windows 事件跟踪) 跟踪会话当前是否处于活动状态。 |

## 结构体

| 结构体 | 描述 |
|--------|-------------|
| [EtwProcessEvent](EtwProcessEvent.md) | 轻量级值类型，携带进程 ID 和启动/停止标志，由 ETW (Windows 事件跟踪) 回调产生。 |
| [EtwProcessMonitor](EtwProcessMonitor.md) | 拥有 ETW (Windows 事件跟踪) 跟踪会话句柄和后台处理线程；提供 `start`、`stop` 和 `stop_existing_session` 方法。 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| ETW (Windows 事件跟踪) 错误报告中使用的 Win32 错误码转换 | [error_codes 模块](../error_codes.rs/README.md) |
| 消费 ETW (Windows 事件跟踪) 事件的服务主循环 | [main 模块](../main.rs/README.md) |
| 诊断日志基础设施 | [logging 模块](../logging.rs/README.md) |
| Windows API 封装（句柄、权限） | [winapi 模块](../winapi.rs/README.md) |