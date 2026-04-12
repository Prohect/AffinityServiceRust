# ProcessSnapshot<'a> struct (process.rs)

通过单次 `NtQuerySystemInformation` 系统调用捕获所有进程和线程的快照。提供按 PID 索引的进程查找和按名称的过滤查询。

## 语法

```rust
pub struct ProcessSnapshot<'a> {
    buffer: &'a mut Vec<u8>,
    pub pid_to_process: HashMap<u32, ProcessEntry>,
}
```

## 成员

`buffer`

对外部分配的字节缓冲区的可变引用。该缓冲区持有 `NtQuerySystemInformation` 返回的原始 `SYSTEM_PROCESS_INFORMATION` 链表数据。生命周期参数 `'a` 确保缓冲区在快照存活期间不会被释放，从而保证 [`ProcessEntry`](ProcessEntry.md) 中延迟解析的线程指针始终有效。

`pid_to_process`

`HashMap<u32, ProcessEntry>` — 以进程 ID 为键的进程条目映射表。公开字段，调用者可直接按 PID 查找或遍历所有进程。

## 方法

| 方法 | 签名 | 描述 |
| --- | --- | --- |
| **take** | `pub fn take(buffer: &'a mut Vec<u8>) -> Result<Self, i32>` | 执行系统调用并构建快照。 |
| **get_by_name** | `pub fn get_by_name(&self, name: String) -> Vec<&ProcessEntry>` | 按进程名（小写）过滤并返回所有匹配的条目引用。 |

## 备注

### take — 缓冲区动态增长

`take` 方法调用 `NtQuerySystemInformation(SystemProcessInformation, ...)` 获取系统中所有进程和线程的信息。如果缓冲区不足（返回 `STATUS_INFO_LENGTH_MISMATCH`，即 `0xC0000004`），方法会自动按以下策略增长缓冲区并重试：

- 若系统返回了 `return_length`，则使用 `((return_length / 8) + 1) * 8` 进行 8 字节对齐分配。
- 否则将缓冲区大小翻倍。

调用者在外层循环中复用同一个 `Vec<u8>` 缓冲区，避免每次迭代重新分配内存。缓冲区只会增长、不会缩小，因此在稳定运行后不再需要重分配。

### 生命周期保证

`ProcessSnapshot` 借用 `buffer` 的可变引用，Rust 的借用检查器确保：

1. 缓冲区在快照存活期间不会被移动或释放。
2. [`ProcessEntry`](ProcessEntry.md) 中存储的原始线程指针（`threads_base_ptr`）在快照生命周期内始终指向有效内存。

### Drop 实现

`ProcessSnapshot` 实现了 `Drop` trait。在析构时：

1. 清空 `pid_to_process` 映射，释放所有 [`ProcessEntry`](ProcessEntry.md) 实例。
2. 清空 `buffer`（长度归零，但保留已分配的容量供下次复用）。

这确保了在 `ProcessEntry` 的线程指针被清理之后，不会有悬挂引用指向已释放的缓冲区内存。

### 解析过程

`take` 成功后，方法遍历 `SYSTEM_PROCESS_INFORMATION` 链表（通过 `NextEntryOffset` 字段链接），为每个进程创建一个 [`ProcessEntry`](ProcessEntry.md)，以 `UniqueProcessId` 为键插入 `pid_to_process`。线程数据此时**不会**被解析——仅保存线程数组的基址指针，待 [`ProcessEntry::get_threads`](ProcessEntry.md) 首次调用时才延迟解析。

### get_by_name

`get_by_name` 对 `pid_to_process` 中所有条目调用 [`ProcessEntry::get_name()`](ProcessEntry.md)（返回小写名称），与传入的 `name` 参数进行比较，返回所有匹配条目的引用向量。适用于按进程名查找多个实例的场景（例如多个 `chrome.exe` 进程）。

### 错误处理

若 `NtQuerySystemInformation` 返回非 `STATUS_INFO_LENGTH_MISMATCH` 的负数状态码，`take` 将返回 `Err(status)` 供调用者处理。常见错误状态码可参考 NTSTATUS 文档。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/process.rs |
| **行号** | L4–L72 |
| **crate 依赖** | `ntapi::ntexapi::{NtQuerySystemInformation, SYSTEM_PROCESS_INFORMATION, SystemProcessInformation}` |
| **调用者** | `src/main.rs` 主循环 |
| **Windows API** | [NtQuerySystemInformation](https://learn.microsoft.com/en-us/windows/win32/api/winternl/nf-winternl-ntquerysysteminformation) |

## 另请参阅

- [process.rs 模块概述](README.md)
- [ProcessEntry](ProcessEntry.md)
- [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) — 使用快照数据进行线程调度