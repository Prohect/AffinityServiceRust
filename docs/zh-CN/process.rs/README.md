# process.rs 模块 (process.rs)

`process` 模块通过 `NtQuerySystemInformation` 系统调用提供进程和线程枚举功能。它以单次内核调用捕获系统中所有进程及其线程的快照，并将原始 NT 数据结构解析为安全的 Rust 抽象。

## 概述

本模块是整个应用程序获取运行进程信息的基础设施层。主循环在每次迭代开始时调用 [`ProcessSnapshot::take`](ProcessSnapshot.md) 获取系统快照，随后通过 PID 查找匹配配置规则的进程，再通过 [`ProcessEntry`](ProcessEntry.md) 访问进程的线程列表和元数据。快照缓冲区和进程映射表作为全局静态变量（[`SNAPSHOT_BUFFER`](SNAPSHOT_BUFFER.md)、[`PID_TO_PROCESS_MAP`](PID_TO_PROCESS_MAP.md)）跨迭代复用。

**设计要点：**

- **单次系统调用** — `NtQuerySystemInformation(SystemProcessInformation)` 一次性返回所有进程和线程信息，避免多次调用的开销和一致性问题。
- **动态缓冲区增长** — 当缓冲区不足时（`STATUS_INFO_LENGTH_MISMATCH`），自动扩容并重试。缓冲区在快照之间复用以减少内存分配。
- **生命周期安全** — `ProcessSnapshot<'a>` 持有对缓冲区的可变借用，确保在快照存活期间缓冲区不会被释放或修改。`ProcessEntry` 中的线程数据通过原始指针延迟解析，指向快照缓冲区中的 `SYSTEM_THREAD_INFORMATION` 数组。
- **延迟线程解析** — `ProcessEntry` 在首次调用 `get_threads()` 时才将原始线程指针解析为 `HashMap`，避免对未使用进程的不必要解析开销。

## 项目

### 结构体

| 名称 | 描述 |
| --- | --- |
| [ProcessSnapshot](ProcessSnapshot.md) | 系统进程快照，通过单次 `NtQuerySystemInformation` 调用捕获所有进程和线程。 |
| [ProcessEntry](ProcessEntry.md) | 单个进程条目，包含进程信息和延迟解析的线程集合。 |

### 静态变量

| 名称 | 描述 |
| --- | --- |
| [SNAPSHOT_BUFFER](SNAPSHOT_BUFFER.md) | 快照数据的全局缓冲区，跨迭代复用。 |
| [PID_TO_PROCESS_MAP](PID_TO_PROCESS_MAP.md) | 每次快照填充的全局进程映射表。 |

## 数据流

```text
NtQuerySystemInformation
        │
        ▼
┌─────────────────────┐
│   ProcessSnapshot   │  ← 拥有 buffer 的可变借用
│                     │
│  pid_to_process:    │
│   HashMap<u32,      │
│    ProcessEntry>    │
└────────┬────────────┘
         │
         ▼
┌─────────────────────┐
│   ProcessEntry      │  ← 存储原始线程指针
│                     │
│  get_threads()      │  ← 首次调用时延迟解析
│  get_name()         │
│  pid()              │
└─────────────────────┘
```

## 内存安全

`ProcessSnapshot` 的 `Drop` 实现在析构时清空 `pid_to_process` 映射并清除缓冲区内容。这确保了 `ProcessEntry` 中持有的原始指针不会在缓冲区释放后被意外使用。

`ProcessEntry` 中的 `threads_base_ptr` 是指向快照缓冲区内部的原始指针（存储为 `usize`）。这是一个有意的 `unsafe` 设计决策——线程数据直接存在于 `NtQuerySystemInformation` 返回的 `SYSTEM_PROCESS_INFORMATION` 结构体之后，使用独立分配来复制这些数据会带来不必要的开销。生命周期参数 `'a` 确保快照不会比其底层缓冲区存活更久。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | `src/process.rs` |
| **外部 crate** | `ntapi`（`NtQuerySystemInformation`、`SYSTEM_PROCESS_INFORMATION`、`SYSTEM_THREAD_INFORMATION`） |
| **调用者** | `src/main.rs` 中的主循环 |
| **消费者** | [`apply.rs`](../apply.rs/README.md)、[`scheduler.rs`](../scheduler.rs/README.md)、[`apply_config_process_level`](../main.rs/apply_config_process_level.md)、[`apply_config_thread_level`](../main.rs/apply_config_thread_level.md) |

## 另请参阅

- [ProcessSnapshot](ProcessSnapshot.md)
- [ProcessEntry](ProcessEntry.md)
- [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) — 使用 `ProcessEntry` 的线程数据进行调度决策
- [apply.rs 模块](../apply.rs/README.md) — 使用 `ProcessEntry` 应用配置