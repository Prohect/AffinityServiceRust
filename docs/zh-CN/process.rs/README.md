# process 模块 (AffinityServiceRust)

`process` 模块通过封装原生 `NtQuerySystemInformation` API，提供高效的进程和线程枚举功能。它捕获系统上所有正在运行的进程及其线程的时间点快照，并将它们存储在可重用的缓冲区和以 PID 为键的查找映射中。快照模型使用 RAII 语义——当 `ProcessSnapshot` 被丢弃时，缓冲区和映射会自动清除，防止过时的数据在调度周期之间残留。

## 函数

此模块不导出独立函数。所有功能都通过 `ProcessSnapshot` 和 `ProcessEntry` 的方法暴露。

## 结构体

| 结构体 | 描述 |
|--------|------|
| [ProcessSnapshot](ProcessSnapshot.md) | RAII 封装器，通过 `NtQuerySystemInformation` 捕获所有进程和线程的时间点快照，丢弃时自动清理。 |
| [ProcessEntry](ProcessEntry.md) | 表示快照中的单个进程，封装 `SYSTEM_PROCESS_INFORMATION` 并提供缓存的进程名称和延迟线程枚举。 |

## 静态变量

| 静态变量 | 描述 |
|----------|------|
| [SNAPSHOT_BUFFER](SNAPSHOT_BUFFER.md) | `Lazy<Mutex<Vec<u8>>>` — `NtQuerySystemInformation` 结果的共享后备缓冲区。不应直接访问；请改用 `ProcessSnapshot`。 |
| [PID_TO_PROCESS_MAP](PID_TO_PROCESS_MAP.md) | `Lazy<Mutex<HashMap<u32, ProcessEntry>>>` — 由 `ProcessSnapshot::take` 填充的共享 PID 到进程的查找映射。不应直接访问；请改用 `ProcessSnapshot`。 |

## 另请参阅

| 链接 | 描述 |
|------|------|
| [winapi.rs](../winapi.rs/README.md) | 底层 Windows API 封装，包括句柄管理和 CPU 集合操作。 |
| [collections.rs](../collections.rs/README.md) | 项目中使用的类型别名（`HashMap`、`HashSet`、`List`）和容量常量。 |
| [event_trace.rs](../event_trace.rs/README.md) | 基于 ETW 的实时进程监控，作为基于轮询的快照的补充。 |

---
> Commit: `7221ea0694670265d4eb4975582d8ed2ae02439d`
