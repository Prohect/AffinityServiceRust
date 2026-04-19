# process 模块 (AffinityServiceRust)

\`process\` 模块提供了一个基于 \`NtQuerySystemInformation(SystemProcessInformation)\` 构建的高性能进程快照机制。它将所有正在运行的进程及其线程的时间点视图捕获到可重用的缓冲区中，最大限度地减少了轮询迭代之间的分配开销。该模块公开了一个基于 RAII 的 [ProcessSnapshot](ProcessSnapshot.md) 类型，在 drop 时自动清除共享状态，以及一个 [ProcessEntry](ProcessEntry.md) 结构体，用于包装每个进程的数据，包括从原始内核结构中解析出的线程数组。

## 静态变量

| 名称 | 类型 | 描述 |
|------|------|------|
| \`SNAPSHOT_BUFFER\` | \`Lazy<Mutex<Vec<u8>>>\` | 共享字节缓冲区，由 [ProcessSnapshot::take](ProcessSnapshot.md) 使用，用于接收内核返回的原始 \`SYSTEM_PROCESS_INFORMATION\` 数据。跨迭代重用以避免重复分配；当内核报告 \`STATUS_INFO_LENGTH_MISMATCH\` 时动态增长。**请勿直接访问** —— 请使用 [ProcessSnapshot](ProcessSnapshot.md)。 |
| \`PID_TO_PROCESS_MAP\` | \`Lazy<Mutex<HashMap<u32, ProcessEntry>>>\` | 共享的 PID → [ProcessEntry](ProcessEntry.md) 映射表，在每次快照期间填充。在 [ProcessSnapshot](ProcessSnapshot.md) drop 时清除。**请勿直接访问** —— 请使用 [ProcessSnapshot](ProcessSnapshot.md)。 |

## 结构体

| 名称 | 描述 |
|------|------|
| [ProcessSnapshot](ProcessSnapshot.md) | RAII 守卫，将系统范围的进程快照捕获到共享缓冲区和进程映射表中。在 drop 时清除两者。 |
| [ProcessEntry](ProcessEntry.md) | 每个进程的数据包装器，包含 \`SYSTEM_PROCESS_INFORMATION\` 记录、指向原始线程数组的指针以及小写的进程名称。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 快照消费者 —— 主循环 | [main.rs](../main.rs/README.md) |
| 线程调度引擎 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| 进程句柄管理 | [ProcessHandle](../winapi.rs/ProcessHandle.md) |
| 配置应用 | [apply.rs](../apply.rs/README.md) |
| Windows API 包装器 | [winapi.rs](../winapi.rs/README.md) |

*文档记录自提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
