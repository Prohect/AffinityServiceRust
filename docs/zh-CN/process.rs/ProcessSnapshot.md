# ProcessSnapshot 结构体 (process.rs)

`ProcessSnapshot` 结构体通过调用 `NtQuerySystemInformation` 的 `SystemProcessInformation` 类来捕获系统上所有正在运行的进程及其线程的时间点快照。它持有指向共享缓冲区和 PID 到进程查找映射的可变引用，两者在快照被丢弃时自动清除（RAII 语义）。这确保了过时的进程数据不会在调度周期之间残留。

## 语法

```rust
pub struct ProcessSnapshot<'a> {
    buffer: &'a mut Vec<u8>,
    pub pid_to_process: &'a mut HashMap<u32, ProcessEntry>,
}
```

## 成员

| 字段 | 类型 | 描述 |
|------|------|------|
| `buffer` | `&'a mut Vec<u8>` | 指向后备字节缓冲区的可变引用，该缓冲区存储 `NtQuerySystemInformation` 返回的原始 `SYSTEM_PROCESS_INFORMATION` 结构。此字段为私有。 |
| `pid_to_process` | `&'a mut HashMap<u32, ProcessEntry>` | 指向以 PID 为键的已解析 [`ProcessEntry`](ProcessEntry.md) 对象映射的可变引用。为公有字段，允许调用方在获取快照后按 PID 查找进程信息。 |

## 方法

### `ProcessSnapshot::take`

```rust
pub fn take(
    buffer: &'a mut Vec<u8>,
    pid_to_process: &'a mut HashMap<u32, ProcessEntry>,
) -> Result<Self, i32>
```

捕获一个新的进程快照。这是构造 `ProcessSnapshot` 的唯一方式。

#### 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `buffer` | `&'a mut Vec<u8>` | 指向可复用字节缓冲区的可变引用。通常通过锁定 [`SNAPSHOT_BUFFER`](SNAPSHOT_BUFFER.md) 静态变量获得。如果 `NtQuerySystemInformation` 返回 `STATUS_INFO_LENGTH_MISMATCH`，缓冲区会动态调整大小。 |
| `pid_to_process` | `&'a mut HashMap<u32, ProcessEntry>` | 指向可复用的 PID 到进程映射的可变引用。通常通过锁定 [`PID_TO_PROCESS_MAP`](PID_TO_PROCESS_MAP.md) 静态变量获得。每次调用开始时会先清除，然后重新填充。 |

#### 返回值

| 结果 | 描述 |
|------|------|
| `Ok(ProcessSnapshot)` | 成功构造的快照。`pid_to_process` 映射已填充所有正在运行的进程。 |
| `Err(i32)` | 来自 `NtQuerySystemInformation` 的 `NTSTATUS` 错误码（`STATUS_INFO_LENGTH_MISMATCH` 除外，该错误会在内部通过使用更大的缓冲区重试来处理）。 |

## 备注

### 快照算法

1. **带重试循环的查询。** 在循环中调用 `NtQuerySystemInformation(SystemProcessInformation, ...)`。如果调用返回 `STATUS_INFO_LENGTH_MISMATCH`（`-1073741820` / `0xC0000004`），则将缓冲区重新分配为 API 的 `return_len` 输出参数所指示的大小（向上取整到 8 字节边界），或者在 `return_len` 为零时将缓冲区大小翻倍。循环重试直到收到非不匹配状态。

2. **截断缓冲区。** 成功后，缓冲区被截断为实际返回的字节数（`return_len`），回收未使用的容量。

3. **解析链表。** 原始缓冲区包含通过 `NextEntryOffset` 连接的 `SYSTEM_PROCESS_INFORMATION` 结构链表。函数遍历此链表，为每个进程构造一个 [`ProcessEntry`](ProcessEntry.md) 并以 `UniqueProcessId` 为键将其插入 `pid_to_process`。

4. **返回快照。** 返回填充完毕的 `ProcessSnapshot`，它借用了缓冲区和映射。缓冲区在快照的整个生命周期内必须保持有效，因为 `ProcessEntry` 对象包含指向缓冲区内存的指针（用于线程信息数组和进程名称字符串）。

### Drop 行为

当 `ProcessSnapshot` 被丢弃时：
- 调用 `pid_to_process.clear()`，移除所有已解析的进程条目。
- 调用 `buffer.clear()`，将缓冲区长度置零（但不释放其容量，以便下次快照时复用已分配的内存）。

此清理至关重要，因为 [`ProcessEntry`](ProcessEntry.md) 对象存储了指向缓冲区的原始指针（`threads_base_ptr`）。同时清除映射和缓冲区可防止悬垂指针访问。

### 缓冲区大小调整策略

初始缓冲区较小（32 字节，如 [`SNAPSHOT_BUFFER`](SNAPSHOT_BUFFER.md) 中所定义）。首次调用时，`NtQuerySystemInformation` 几乎肯定会返回 `STATUS_INFO_LENGTH_MISMATCH`，导致缓冲区调整大小。新大小计算为 `((return_len / 8) + 1) * 8` 以对齐到 8 字节边界。在后续调用中，缓冲区保留其先前的容量（通过 `Vec::capacity()`），因此仅在系统进程数量显著增长时才需要调整大小。

### 不安全代码

`take` 的整个函数体包裹在 `unsafe` 中，原因是：
- `NtQuerySystemInformation` 是对 `ntdll.dll` 的 FFI 调用。
- 原始缓冲区被重新解释为 `SYSTEM_PROCESS_INFORMATION` 指针。
- 线程信息数组通过从 `SYSTEM_PROCESS_INFORMATION.Threads` 进行原始指针算术来访问。

### 典型使用模式

```text
let mut buf = SNAPSHOT_BUFFER.lock().unwrap();
let mut map = PID_TO_PROCESS_MAP.lock().unwrap();
let snapshot = ProcessSnapshot::take(&mut buf, &mut map)?;

for (pid, entry) in snapshot.pid_to_process.iter() {
    // 处理每个条目...
}
// snapshot 在此处被丢弃，同时清除 buf 和 map
```

### 平台说明

- **仅限 Windows。** 依赖于来自 `ntdll.dll` 的 NT 原生 `NtQuerySystemInformation` API（通过 `ntapi` crate 导入）。
- `SYSTEM_PROCESS_INFORMATION` 和 `SYSTEM_THREAD_INFORMATION` 类型来自 `ntapi::ntexapi` 模块。

## 要求

| 要求 | 值 |
|------|------|
| **模块** | `process.rs` |
| **调用方** | `scheduler.rs` — 主调度循环；`apply.rs` — 规则应用 |
| **被调用方** | `NtQuerySystemInformation`（ntdll，通过 `ntapi`），[`ProcessEntry::new`](ProcessEntry.md) |
| **静态变量** | [`SNAPSHOT_BUFFER`](SNAPSHOT_BUFFER.md)、[`PID_TO_PROCESS_MAP`](PID_TO_PROCESS_MAP.md) |
| **Win32 API** | `NtQuerySystemInformation` 配合 `SystemProcessInformation` |
| **权限** | 无显式要求，但 `SeDebugPrivilege` 可启用对受保护进程的枚举。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| ProcessEntry 结构体 | [ProcessEntry](ProcessEntry.md) |
| SNAPSHOT_BUFFER 静态变量 | [SNAPSHOT_BUFFER](SNAPSHOT_BUFFER.md) |
| PID_TO_PROCESS_MAP 静态变量 | [PID_TO_PROCESS_MAP](PID_TO_PROCESS_MAP.md) |
| collections 模块 | [collections.rs](../collections.rs/README.md) |
| winapi 模块 | [winapi.rs](../winapi.rs/README.md) |
| event_trace 模块 | [event_trace.rs](../event_trace.rs/README.md) |

---
> Commit SHA: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
