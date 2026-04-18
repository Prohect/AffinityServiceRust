# AffinityServiceRust 文档 (zh-CN)

AffinityServiceRust 是一个用 Rust 编写的高性能 Windows 进程管理服务。它持续监控运行中的进程，并根据配置文件中定义的规则对 CPU 亲和性、优先级、I/O 优先级和内存优先级进行动态调整。服务同时支持传统系统（通过亲和性掩码管理 ≤64 个逻辑处理器）与现代多核系统（通过跨处理器组的 CPU Sets）。

服务在轮询循环或 ETW 响应式模式下运行，将每个活跃进程与用户定义的规则进行匹配，并通过 Windows 内核 API 下发调度策略变更。主线程调度器（Prime Thread Scheduler）在每个轮询间隔内动态识别 CPU 密集型线程，并将其固定到指定的高性能核心上。

## 模块

| 模块 | 概述 | 条目 |
|------|------|------|
| [apply.rs](apply.rs/README.md) | 进程/线程设置应用 | 16 |
| [cli.rs](cli.rs/README.md) | 命令行参数解析 | 7 |
| [collections.rs](collections.rs/README.md) | 类型别名与容量常量 | 8 |
| [config.rs](config.rs/README.md) | 配置文件解析与热重载 | 25 |
| [error_codes.rs](error_codes.rs/README.md) | Win32/NTSTATUS 错误格式化 | 2 |
| [event_trace.rs](event_trace.rs/README.md) | ETW 进程启动/停止监控 | 4 |
| [logging.rs](logging.rs/README.md) | 日志记录、错误去重、查找输出 | 10 |
| [main.rs](main.rs/README.md) | 入口点与主轮询循环 | 7 |
| [priority.rs](priority.rs/README.md) | 优先级枚举（进程、IO、内存、线程） | 5 |
| [process.rs](process.rs/README.md) | 通过 NtQuerySystemInformation 获取进程快照 | 4 |
| [scheduler.rs](scheduler.rs/README.md) | 主线程调度器与统计跟踪 | 6 |
| [winapi.rs](winapi.rs/README.md) | Windows API 封装与句柄管理 | 27 |

## 架构概览

服务以 `main.rs` 中的中央轮询循环为核心，每次迭代依次执行：

1. **快照** — `process.rs` 调用 `NtQuerySystemInformation`，获取当前进程与线程列表。
2. **匹配** — `config.rs` 提供已解析的规则集；每个活跃进程按名称与规则进行匹配。
3. **应用** — `apply.rs` 将匹配规则的各项设置（优先级、亲和性/CPU Sets、I/O 优先级、内存优先级、理想处理器、主线程调度）分发至各进程及其线程。
4. **调度** — `scheduler.rs` 跨轮询间隔跟踪每线程的 CPU 时钟周期增量，识别最繁忙的"主"线程，并将其分配至配置的主核心。
5. **响应** — `event_trace.rs` 可选地通过 ETW 进程启动事件驱动循环，替代固定计时器，从而降低新启动进程的响应延迟。

## 核心概念

| 概念 | 相关位置 |
|------|----------|
| CPU 亲和性掩码（≤64 核） | [apply_affinity](apply.rs/apply_affinity.md) |
| CPU Sets（>64 核，跨组） | [apply_process_default_cpuset](apply.rs/apply_process_default_cpuset.md) |
| 主线程调度 | [apply_prime_threads](apply.rs/apply_prime_threads.md)、[scheduler.rs](scheduler.rs/README.md) |
| 理想处理器分配 | [apply_ideal_processors](apply.rs/apply_ideal_processors.md) |
| 热重载 | [config.rs](config.rs/README.md) |
| 规则等级 | [ProcessLevelConfig](config.rs/ProcessLevelConfig.md) |

## 参见

| 资源 | 链接 |
|------|------|
| 项目自述文件 | [../../README.zh-CN.md](../../README.zh-CN.md) |
| 文档索引 | [../README.md](../README.md) |
| en-US 区域 | [../en-US/README.md](../en-US/README.md) |

*提交：[29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*