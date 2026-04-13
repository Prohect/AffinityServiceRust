# main 模块（AffinityServiceRust）

`main` 模块是 AffinityServiceRust 的入口点和顶层协调器。它将 CLI 解析、配置加载、进程快照枚举以及核心应用循环串联在一起，对受管进程执行进程级和线程级设置。该模块还提供了日志处理和未管理进程发现的实用入口点。

主轮询循环通过 `NtQuerySystemInformation` 定期获取进程快照，将运行中的进程与已加载的配置规则进行匹配，并委托 `apply` 模块执行实际的 Win32/NT API 调用。进程级设置（进程优先级、CPU 亲和性、CPU 集合、IO 优先级、内存优先级）在每个进程的生命周期内仅应用一次。线程级设置（主线程调度、理想处理器分配）在每次轮询迭代中重新评估。可选的基于 ETW 的实时进程监视器补充了轮询方式，以更快地响应新进程启动。

## 函数

| 函数 | 描述 |
|------|------|
| [apply_config_process_level](apply_config_process_level.md) | 应用一次性进程级设置：优先级类别、CPU 亲和性、CPU 集合、IO 优先级和内存优先级。 |
| [apply_config_thread_level](apply_config_thread_level.md) | 应用每次迭代的线程级设置：主线程调度、理想处理器分配和周期时间跟踪。 |
| [process_logs](process_logs.md) | 处理 `-find` 模式生成的 `.find.log` 文件，以发现新的未管理进程并解析其可执行文件路径。 |
| [process_find](process_find.md) | 通过 `CreateToolhelp32Snapshot` 枚举运行中的进程，并记录那些具有默认（未管理）亲和性的进程。 |
| [main](main.md) | 程序入口点。解析 CLI 参数、加载配置、管理轮询循环、ETW 集成、热重载和基于等级的调度。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| CLI 参数解析 | [cli.rs](../cli.rs/README.md) |
| 配置文件解析 | [config.rs](../config.rs/README.md) |
| 应用函数（进程级和线程级） | [apply.rs](../apply.rs/README.md) |
| 主线程调度器 | [scheduler.rs](../scheduler.rs/README.md) |
| ETW 进程监视器 | [event_trace.rs](../event_trace.rs/README.md) |
| 进程快照 | [process.rs](../process.rs/README.md) |
| Win32/NT API 封装 | [winapi.rs](../winapi.rs/README.md) |
| 日志基础设施 | [logging.rs](../logging.rs/README.md) |