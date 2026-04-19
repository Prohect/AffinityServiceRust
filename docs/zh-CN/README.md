# AffinityServiceRust 文档（zh-CN）

AffinityServiceRust 是一个用 Rust 编写的 Windows 服务，它通过 INI 风格的配置文件管理进程 CPU 亲和性、优先级、IO 优先级、内存优先级和线程调度。它支持配置热重载、基于 ETW 的响应式进程检测、带迟滞的 Prime 线程调度，以及用于 Process Lasso 配置转换的工具。

## 模块

| 模块 | 描述 |
|--------|-------------|
| [main.rs](main.rs/README.md) | 应用程序入口点和主循环协调器 |
| [cli.rs](cli.rs/README.md) | 命令行参数解析和帮助输出 |
| [config.rs](config.rs/README.md) | 配置文件解析、验证、热重载 |
| [apply.rs](apply.rs/README.md) | 核心强制引擎——将所有设置应用到进程 |
| [scheduler.rs](scheduler.rs/README.md) | 带迟滞选择的 Prime 线程调度器 |
| [process.rs](process.rs/README.md) | 通过 NtQuerySystemInformation 获取进程快照 |
| [winapi.rs](winapi.rs/README.md) | Windows API 封装（句柄、CPU 集合、权限） |
| [event_trace.rs](event_trace.rs/README.md) | ETW 消费者，用于实时监控进程的启动/停止 |
| [logging.rs](logging.rs/README.md) | 日志基础设施，支持日志轮转和错误去重 |
| [priority.rs](priority.rs/README.md) | Windows 优先级级别的安全类型枚举 |
| [collections.rs](collections.rs/README.md) | 面向性能的集合类型别名和常量 |
| [error_codes.rs](error_codes.rs/README.md) | 人类可读的 Win32/NTSTATUS 错误查找 |

## 架构概览

该服务遵循基于循环的强制架构：

1. **配置解析** — `config.rs` 读取并验证 INI 风格的配置文件，生成一组进程规则。
2. **主循环** — `main.rs` 协调轮询周期并协调所有子系统。
3. **进程快照** — `process.rs` 通过 `NtQuerySystemInformation` 获取系统范围的进程快照。
4. **规则匹配** — 将每个运行中的进程与加载的配置规则进行匹配。
5. **应用强制** — `apply.rs` 将 CPU 亲和性、优先级、IO 优先级、内存优先级和线程调度设置应用到匹配的进程。
6. **睡眠/ETW 等待** — 循环在重复之前睡眠或等待来自 `event_trace.rs` 的 ETW 进程启动事件。

支持配置热重载：服务检测配置文件更改并重新解析而无需重启。`scheduler.rs` 中的 Prime 线程调度器使用迟滞来避免线程在核心之间过度迁移。

## 区域设置

| 区域设置 | 链接 |
|--------|------|
| en-US | [en-US](../en-US/README.md) |
| zh-CN | (本页) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*