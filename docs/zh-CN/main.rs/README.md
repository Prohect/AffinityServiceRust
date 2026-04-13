# main.rs 模块 (main.rs)

`main` 模块是 AffinityServiceRust 的应用程序入口点，负责命令行解析、特权管理、配置热重载、ETW 响应式进程检测、grade 调度、进程发现以及各种工具模式的分发。

## 概述

本模块包含以下核心职责：

- **CLI 解析与模式分发** — 解析命令行参数，根据模式标志分发到帮助、转换、验证、日志处理等子流程
- **特权管理** — 请求 `SeDebugPrivilege` 和 `SeIncreaseBasePriorityPrivilege`，处理 UAC 提升
- **配置加载与热重载** — 首次加载配置文件，主循环中通过 [`hotreload_config`](../config.rs/hotreload_config.md) 和 [`hotreload_blacklist`](../config.rs/hotreload_blacklist.md) 自动重载
- **ETW 响应式进程检测** — 通过 [`EtwProcessMonitor`](../event_trace.rs/EtwProcessMonitor.md) 监听进程启动/停止事件，实现即时规则应用
- **两级配置应用** — 进程级设置（一次性）和线程级设置（每次迭代）分离
- **Grade 调度** — 按 grade 值控制规则执行频率
- **进程发现（find 模式）** — 通过 [`process_find`](process_find.md) 扫描未管理进程

### 两级应用

配置应用分为两个级别：

- **进程级** ([`apply_config_process_level`](apply_config_process_level.md)) — 一次性设置（优先级、亲和性、CPU 集、I/O 优先级、内存优先级），每进程应用一次。通过 `process_level_applied: HashSet<u32>` 跟踪。
- **线程级** ([`apply_config_thread_level`](apply_config_thread_level.md)) — 每次迭代设置（Prime 线程调度、理想处理器分配），每次循环应用。

### 响应式进程检测

来自 [`EtwProcessMonitor`](../event_trace.rs/EtwProcessMonitor.md) 的 ETW 事件实现即时规则应用：
- **进程启动** → PID 添加到 `process_level_pending` → 下次快照时立即应用进程级规则，绕过 grade 调度
- **进程停止** → 从调度器、错误跟踪和已应用集合中清理 PID

## 项目

### 函数

| 名称 | 描述 |
| --- | --- |
| [apply_config_process_level](apply_config_process_level.md) | 应用进程级设置（一次性）：优先级、亲和性、CPU 集、I/O/内存优先级。 |
| [apply_config_thread_level](apply_config_thread_level.md) | 应用线程级设置（每次迭代）：Prime 调度、理想处理器、周期跟踪。 |
| [process_find](process_find.md) | 在 `-find` 模式下扫描未管理进程。 |
| [process_logs](process_logs.md) | 处理 `.find.log` 文件以发现可执行文件路径。 |
| [main](main.md) | 入口点 — CLI、特权、配置、UAC、ETW、主循环。 |

## 执行流程

1. **CLI 解析** — 通过 `parse_args` 解析命令行参数到 `CliArgs`
2. **模式分发** — 帮助/转换/自动分组/验证/日志处理模式提前退出
3. **配置加载** — 通过 `read_config` 加载并验证配置文件
4. **特权获取** — 请求 `SeDebugPrivilege` 和 `SeIncreaseBasePriorityPrivilege`
5. **计时器分辨率** — 通过 [`set_timer_resolution`](../winapi.rs/set_timer_resolution.md)（如配置）
6. **UAC 提升** — 非管理员时请求 `request_uac_elevation`
7. **清理子进程** — 调用 `terminate_child_processes`
8. **ETW 启动** — [`EtwProcessMonitor::start()`](../event_trace.rs/EtwProcessMonitor.md) 开始响应式监控（不可用时回退到轮询）
9. **主循环**：
   - 获取 `ProcessSnapshot`
   - 排空 ETW 事件：启动事件 → `process_level_pending`，停止事件 → 清理
   - 立即应用 `process_level_pending` 中的进程级规则
   - Grade 迭代：应用进程级（如未应用）和线程级规则
   - Find 模式扫描
   - 热重载配置和黑名单
   - 休眠指定间隔
10. **ETW 停止** — 清理关闭 ETW 监控器

## Grade 调度机制

| Grade | 执行频率 | 典型用途 |
| --- | --- | --- |
| 1 | 每次循环 | 需要频繁调整的关键进程 |
| 2 | 每 2 次循环 | 中等频率的进程 |
| 5 | 每 5 次循环 | 低频率或不常变化的进程 |

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | `src/main.rs` |
| **关键依赖** | `CliArgs`、`ProcessConfig`、`PrimeThreadScheduler`、`ProcessSnapshot`、[`EtwProcessMonitor`](../event_trace.rs/EtwProcessMonitor.md) |