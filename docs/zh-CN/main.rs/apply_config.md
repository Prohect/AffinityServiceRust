# apply_config 函数 (main.rs) — 已拆分

> **此函数已拆分为两个独立函数：**
> - [`apply_config_process_level`](apply_config_process_level.md) — 应用进程级设置（每进程一次性）：优先级、亲和性、CPU 集、I/O 优先级、内存优先级。
> - [`apply_config_thread_level`](apply_config_thread_level.md) — 应用线程级设置（每次轮询迭代）：Prime 线程调度、理想处理器分配、周期时间跟踪。
>
> 此拆分使得通过 ETW 实现响应式进程检测成为可能——进程级设置在检测到新进程时立即应用，而线程级设置继续按常规轮询计划执行。

## 另请参阅

- [apply_config_process_level](apply_config_process_level.md)
- [apply_config_thread_level](apply_config_thread_level.md)
- [main.rs 模块概述](README.md)