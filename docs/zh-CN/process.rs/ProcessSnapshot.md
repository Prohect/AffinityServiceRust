# ProcessSnapshot 结构体 (process.rs)

RAII 守卫，使用 \`NtQuerySystemInformation(SystemProcessInformation)\` 捕获所有运行中进程及其线程的时间点快照。该结构体持有对共享缓冲区和进程映射的可变引用，当快照被丢弃时，两者都会自动清除。

## 语法

```rust
pub struct ProcessSnapshot<'a> {
    buffer: &'a mut Vec<u8>,
    pub pid_to_process: &'a mut HashMap<u32, ProcessEntry>,
}
```

## 成员

| 成员 | 类型 | 描述 |
|------|------|------|
| \`buffer\` | \`&'a mut Vec<u8>\` | 对原始字节缓冲区的可变引用，该缓冲区保存 \`SYSTEM_PROCESS_INFORMATION\` 链表（由 \`NtQuerySystemInformation\` 返回）。通过 [SNAPSHOT_BUFFER](README.md) 跨迭代复用。 |
| \`pid_to_process\` | \`&'a mut HashMap<u32, ProcessEntry>\` | 对在 \`take()\` 期间填充的进程映射的可变引用。以 PID 为键，值为 [ProcessEntry](ProcessEntry.md) 结构体。设为公开以便调用方可迭代和查询进程。 |

## 方法

### take

```rust
pub fn take(
    buffer: &'a mut Vec<u8>,
    pid_to_process: &'a mut HashMap<u32, ProcessEntry>,
) -> Result<Self, i32>
```

通过调用 \`NtQuerySystemInformation(SystemProcessInformation)\` 捕获系统上所有进程和线程的快照。

**参数**

| 参数 | 类型 | 描述 |
|------|------|------|
| \`buffer\` | \`&'a mut Vec<u8>\` | 用于接收系统信息的字节缓冲区的可变引用。如果缓冲区太小，会动态增长。通常来源于 \`SNAPSHOT_BUFFER\`。 |
| \`pid_to_process\` | \`&'a mut HashMap<u32, ProcessEntry>\` | 将被填充进程条目的 HashMap 的可变引用。每次调用开始时会被清除。通常来源于 \`PID_TO_PROCESS_MAP\`。 |

**返回值**

`Result<ProcessSnapshot<'a>, i32>` — 成功时返回 RAII 快照守卫。失败时返回 NTSTATUS 错误码（`i32`）。

**备注**

- 该函数在重试循环中调用 \`NtQuerySystemInformation\`。如果调用返回 \`STATUS_INFO_LENGTH_MISMATCH\`（\`0xC0000004\`），缓冲区将被重新分配为 \`ReturnLength\` 输出参数指示的大小（向上对齐到 8 字节边界），或者在 \`ReturnLength\` 为零时翻倍。
- 成功调用后，原始缓冲区作为 \`SYSTEM_PROCESS_INFORMATION\` 结构体链表进行遍历。每个条目的 \`NextEntryOffset\` 字段指向下一个条目；偏移量为零表示最后一个条目。
- 对于每个进程，通过 \`ProcessEntry::new()\` 构造一个 [ProcessEntry](ProcessEntry.md)，它会提取小写进程名并存储线程数组的基指针。
- 缓冲区和映射在**丢弃时被清除**，因此在快照丢弃后，从 \`pid_to_process\` 派生的所有 [ProcessEntry](ProcessEntry.md) 引用将变为无效。

### Drop

```rust
impl<'a> Drop for ProcessSnapshot<'a> {
    fn drop(&mut self);
}
```

当快照离开作用域时，清除 \`pid_to_process\` 和 \`buffer\`。这确保了存储在 [ProcessEntry](ProcessEntry.md) 中的过期原始指针（\`threads_base_ptr\` 字段）在底层缓冲区被复用后不会被解引用。

## 备注

`ProcessSnapshot` 遵循 RAII 模式，将已解析的进程数据的生命周期绑定到原始缓冲区的生命周期。由于 \`SYSTEM_PROCESS_INFORMATION.Threads\` 是附加在每个结构体之后的变长数组，线程数据仅在原始缓冲区存活期间有效。\`Drop\` 实现强制执行此不变量。

### 典型用法

```rust
let mut buffer = SNAPSHOT_BUFFER.lock().unwrap();
let mut map = PID_TO_PROCESS_MAP.lock().unwrap();
let snapshot = ProcessSnapshot::take(&mut buffer, &mut map)?;
// 使用 snapshot.pid_to_process 迭代进程
// snapshot 在此处被丢弃，清除缓冲区和映射表
```

### 缓冲区增长策略

| 条件 | 操作 |
|------|------|
| \`ReturnLength > 0\` | 分配 \`((ReturnLength / 8) + 1) * 8\` 字节（8 字节对齐的上限） |
| \`ReturnLength == 0\` | 将当前缓冲区容量翻倍 |

初始缓冲区大小为 32 字节（来自 \`SNAPSHOT_BUFFER\`），这在首次调用时总会触发至少一次重新分配。后续调用复用上一次成功调用的容量，因此在第一次迭代之后重新分配变得很少发生。

## 要求

| | |
|---|---|
| **模块** | \`src/process.rs\` |
| **调用方** | \`src/main.rs\` 中的主轮询循环 |
| **被调用方** | \`NtQuerySystemInformation\` (ntdll), [ProcessEntry::new](ProcessEntry.md) |
| **NT API** | [NtQuerySystemInformation](https://learn.microsoft.com/zh-cn/windows/win32/api/winternl/nf-winternl-ntquerysysteminformation)，使用 \`SystemProcessInformation\`（类别 5） |
| **特权** | 基本枚举不需要特殊特权；[SeDebugPrivilege](../winapi.rs/enable_debug_privilege.md) 可扩展对受保护进程的可见性 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概览 | [process.rs](README.md) |
| 进程数据容器 | [ProcessEntry](ProcessEntry.md) |
| 线程句柄管理 | [ThreadHandle](../winapi.rs/ThreadHandle.md) |
| Prime 线程调度器 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |

*文档记录自提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
