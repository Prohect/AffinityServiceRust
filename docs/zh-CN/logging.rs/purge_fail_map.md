# purge_fail_map 函数 (logging.rs) — 已移除

> **此函数已被移除。** 进程退出清理现在通过主循环中的 ETW 事件响应式处理。当从 [`EtwProcessMonitor`](../event_trace.rs/EtwProcessMonitor.md) 接收到进程停止事件时，主循环直接从 [`PID_MAP_FAIL_ENTRY_SET`](PID_MAP_FAIL_ENTRY_SET.md) 中移除该 PID 的条目，并在调度器上调用 [`drop_process_by_pid`](../scheduler.rs/PrimeThreadScheduler.md)。

## 另请参阅

- [event_trace.rs 模块概述](../event_trace.rs/README.md) — 响应式进程监控
- [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md)
- [logging.rs 模块概述](README.md)