# event_trace.rs 模块

最小化的 ETW（Windows 事件跟踪）消费者，用于实时进程启动/停止监控。

使用 Microsoft-Windows-Kernel-Process 提供程序接收进程创建或终止的通知，实现响应式规则应用，而不仅依赖轮询。

## 概述

本模块提供基于 ETW 的进程监控器，监听来自 Windows 内核的进程启动和停止事件。当新进程启动时，其 PID 通过通道发送到主循环，使其能够立即应用进程级规则，而无需等待下一个轮询间隔。当进程停止时，其 PID 用于清理调度器状态和错误去重条目。

监控器在后台线程上运行，通过 `mpsc` 通道与主循环通信。

## 项目列表

### 静态变量

| 名称 | 说明 |
| --- | --- |
| [ETW_SENDER](ETW_SENDER.md) | 全局发送端，供 ETW 回调通过通道分发进程事件。 |
| [ETW_ACTIVE](ETW_ACTIVE.md) | 原子标志，指示 ETW 会话是否处于活动状态。 |

### 结构体

| 名称 | 说明 |
| --- | --- |
| [EtwProcessEvent](EtwProcessEvent.md) | 从 ETW 接收的进程启动/停止事件，包含 PID 和事件类型。 |
| [EtwProcessMonitor](EtwProcessMonitor.md) | 管理 ETW 实时跟踪会话的进程监控器，包括会话生命周期和后台处理线程。 |

## 架构

### ETW 会话生命周期

1. **启动** — `EtwProcessMonitor::start()` 创建名为 `"AffinityServiceRust_EtwProcessMonitor"` 的 ETW 实时会话，启用带有 `WINEVENT_KEYWORD_PROCESS` 关键字的 `Microsoft-Windows-Kernel-Process` 提供程序，并生成运行 `ProcessTrace` 的后台线程。
2. **回调** — `etw_event_callback` 外部函数接收 OS 的每个事件记录，从 `UserData` 提取 PID，并通过全局 `ETW_SENDER` 通道发送 `EtwProcessEvent`。
3. **消费** — 主循环每次迭代调用 `rx.try_recv()` 排空待处理事件。启动事件将 PID 添加到 `process_level_pending`；停止事件清理调度器和错误跟踪状态。
4. **停止** — `EtwProcessMonitor::stop()` 关闭跟踪、停止会话、等待后台线程结束并清除全局发送端。也通过 `Drop` 自动调用。

### 与主循环的集成

- **进程启动**：PID 添加到 `process_level_pending` 集合。在下一次快照时，无论 grade 调度如何，立即应用进程级规则。
- **进程停止**：从 `process_level_applied`、`process_level_pending`、`PID_MAP_FAIL_ENTRY_SET` 和调度器统计中移除 PID（通过 `drop_process_by_pid`）。

### 降级回退

如果 ETW 初始化失败（例如权限不足），服务将回退到仅轮询模式并输出警告日志。所有功能继续正常工作，只是没有响应式进程检测。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | `src/event_trace.rs` |
| **Windows 特性** | `Win32_System_Diagnostics_Etw` |
| **调用方** | [`main`](../main.rs/main.md) |
| **关键依赖** | `windows` crate ETW API、`once_cell::sync::Lazy`、`std::sync::mpsc` |

## 另请参阅

- [main.rs 模块概述](../main.rs/README.md) — 将 ETW 事件集成到主循环
- [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) — `drop_process_by_pid` 在进程退出时清理
- [PID_MAP_FAIL_ENTRY_SET](../logging.rs/PID_MAP_FAIL_ENTRY_SET.md) — 进程退出时清理错误条目