# ProcessEntry struct (process.rs)

表示系统进程快照中的单个进程条目，包装了 `SYSTEM_PROCESS_INFORMATION` 结构体，并提供延迟线程解析和便捷的访问方法。

## 语法

```rust
#[derive(Clone)]
pub struct ProcessEntry {
    pub process: SYSTEM_PROCESS_INFORMATION,
    threads: HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    threads_base_ptr: usize,
    name: String,
}
```

## 成员

`process`

底层的 `SYSTEM_PROCESS_INFORMATION` 结构体，包含进程的完整系统信息（PID、线程数、内存计数器、时间等）。此字段为 `pub`，允许调用者直接访问原始数据。

`threads`

线程 ID 到 `SYSTEM_THREAD_INFORMATION` 的哈希映射。此字段采用**延迟填充**策略——初始为空，仅在首次调用 `get_threads()` 时从原始指针解析并缓存。

`threads_base_ptr`

指向 `SYSTEM_PROCESS_INFORMATION::Threads` 数组起始位置的原始指针（存储为 `usize`）。此指针在 [ProcessSnapshot](ProcessSnapshot.md) 的生命周期内有效，用于延迟解析线程数据。

`name`

进程映像名称的小写形式，从 `SYSTEM_PROCESS_INFORMATION::ImageName`（UTF-16 `UNICODE_STRING`）转换而来。空名称对应 System Idle Process（PID 0）。

## 方法

| 方法 | 签名 | 描述 |
| --- | --- | --- |
| **new** | `pub fn new(process: SYSTEM_PROCESS_INFORMATION, threads_base_ptr: *const SYSTEM_THREAD_INFORMATION) -> Self` | 从原始系统信息和线程数组指针构造条目。进程名从 `ImageName` 解码为小写 UTF-8。 |
| **get_threads** | `pub fn get_threads(&mut self) -> &HashMap<u32, SYSTEM_THREAD_INFORMATION>` | 返回线程信息映射。首次调用时从原始指针延迟解析并缓存；后续调用直接返回缓存。 |
| **get_thread** | `pub fn get_thread(&mut self, tid: u32) -> Option<&SYSTEM_THREAD_INFORMATION>` | 按线程 ID 查找单个线程的信息。内部调用 `get_threads()` 确保缓存已填充。 |
| **get_name** | `pub fn get_name(&self) -> &str` | 返回进程名称的小写形式（例如 `"notepad.exe"`）。 |
| **get_name_original_case** | `pub fn get_name_original_case(&self) -> String` | 从原始 `ImageName` 指针重新解码，返回保留原始大小写的进程名称。 |
| **pid** | `pub fn pid(&self) -> u32` | 返回进程的 PID（从 `UniqueProcessId` 转换）。 |
| **thread_count** | `pub fn thread_count(&self) -> u32` | 返回 `NumberOfThreads` 字段的值。 |

## 备注

### 延迟线程解析

`ProcessEntry` 的设计核心是**延迟解析**——线程数据不在构造时解析，而是在首次需要时才从原始指针读取。这避免了为不需要线程信息的进程执行不必要的解析工作。

`get_threads()` 方法在内部检查 `self.threads.len()` 是否与 `self.process.NumberOfThreads` 匹配。若不匹配，则清空现有映射并重新从原始指针解析。这意味着如果线程数据发生变化（在同一快照内不会发生），映射也能正确刷新。

### 原始指针安全

`threads_base_ptr` 是一个裸指针（存储为 `usize`），指向 [ProcessSnapshot](ProcessSnapshot.md) 拥有的缓冲区内部。这些指针的安全性由以下机制保证：

- `ProcessSnapshot` 的生命周期参数 `'a` 绑定到底层缓冲区，确保缓冲区在快照存活期间不会被释放。
- `ProcessSnapshot` 的 `Drop` 实现在释放时先清空 `pid_to_process`（即所有 `ProcessEntry`），然后清空缓冲区。
- `get_threads()` 和 `get_name_original_case()` 中的 `unsafe` 块仅在缓冲区有效时被调用。

### 进程名处理

构造时通过 `String::from_utf16_lossy` 从 `UNICODE_STRING` 解码并调用 `to_lowercase()`，以便后续的名称匹配操作不区分大小写。如果需要原始大小写（例如日志输出），可使用 `get_name_original_case()`，它会从原始指针重新解码而不转换大小写。

对于 System Idle Process（PID 0），`ImageName.Length` 为 0 或 `Buffer` 为 null，此时名称设为空字符串。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/process.rs |
| **行号** | L74–L152 |
| **外部 crate** | `ntapi`（`SYSTEM_PROCESS_INFORMATION`、`SYSTEM_THREAD_INFORMATION`） |
| **Trait 实现** | `Clone` |
| **构造者** | [ProcessSnapshot::take](ProcessSnapshot.md) |
| **消费者** | [apply_affinity](../apply.rs/apply_affinity.md)、[apply_prime_threads](../apply.rs/apply_prime_threads.md)、[apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |

## 另请参阅

- [ProcessSnapshot](ProcessSnapshot.md) — 拥有底层缓冲区并构造 `ProcessEntry` 的快照容器
- [process.rs 模块概述](README.md)
- [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) — 使用 `ProcessEntry` 的线程数据进行 prime 线程选择