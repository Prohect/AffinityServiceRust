# main 模块 (AffinityServiceRust)

`main` 模块是 AffinityServiceRust 服务的入口点和顶层协调器。它声明所有子模块、解析命令行参数、加载配置、管理主轮询循环，并协调将进程级和线程级调度策略应用于正在运行的 Windows 进程。该模块集成了 ETW（Windows 事件跟踪）以实现响应式进程检测，支持配置文件和黑名单文件的热重载，并提供辅助模式，如进程发现（`-find`）、日志分析（`-processLogs`）、配置验证和配置转换。

## 函数

| 函数 | 描述 |
|----------|-------------|
| [apply_process_level](apply_process_level.md) | 对单个进程应用一次性的进程级设置（优先级、亲和性、CPU 集合、I/O 优先级、内存优先级）。 |
| [apply_thread_level](apply_thread_level.md) | 对单个进程应用每次迭代的线程级设置（主线程调度、理想处理器分配、周期跟踪）。 |
| [apply_config](apply_config.md) | 协调对匹配进程的进程级和线程级配置应用，合并结果并记录日志。 |
| [log_apply_results](log_apply_results.md) | 格式化并输出单次 `apply_config` 调用产生的变更和错误的日志。 |
| [process_logs](process_logs.md) | 扫描 `.find.log` 文件以发现先前未知的进程，通过 Everything 搜索（`es.exe`）解析其可执行文件路径，并将结果写入文件。 |
| [process_find](process_find.md) | 使用 Toolhelp API 枚举正在运行的进程，并记录尚未被配置或黑名单覆盖的进程。 |
| [main](main.md) | 程序入口点。解析 CLI 参数、加载配置、获取特权、启动 ETW 监视器，并运行主轮询/应用循环。 |

## 结构体 / 枚举

此模块未定义任何公共结构体或枚举。此模块中使用的所有数据类型均从兄弟模块导入，如 `config`、`apply`、`scheduler` 和 `cli`。

## 另请参阅

| 相关模块 | 链接 |
|----------------|------|
| scheduler | [scheduler 模块](../scheduler.rs/README.md) |
| priority | [priority 模块](../priority.rs/README.md) |
| apply | [apply 模块](../apply.rs/README.md) |
| config | [config 模块](../config.rs/README.md) |
| cli | [cli 模块](../cli.rs/README.md) |
| logging | [logging 模块](../logging.rs/README.md) |
| event_trace | [event_trace 模块](../event_trace.rs/README.md) |
| winapi | [winapi 模块](../winapi.rs/README.md) |
| process | [process 模块](../process.rs/README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
