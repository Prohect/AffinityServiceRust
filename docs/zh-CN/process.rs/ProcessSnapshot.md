# ProcessSnapshot 结构体 (process.rs)

提供一个基于 RAII 的系统级进程快照封装，通过 `NtQuerySystemInformation` 获取。当 `ProcessSnapshot` 被销毁时，后备缓冲区和已解析的进程映射会被清除，确保在快照的有效生命周期之外不会残留指向内核返回数据的过期指针。

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
| `buffer` | `&'a mut Vec<u8>` | 指向快照后备原始字节缓冲区的可变引用。由 `NtQuerySystemInformation` 使用 `SystemProcessInformation` 填充。当遇到 `STATUS_INFO_LENGTH_MISMATCH` 时缓冲区会自动增长，成功后会截断到实际返回长度。此成员为私有——调用者通过公共 API 进行交互。 |
| `pid_to_process` | `&'a mut HashMap<u32, ProcessEntry>` | 指向以 PID 为键的映射的可变引用，在 `take` 过程中填充。每个值是一个 [`ProcessEntry`](ProcessEntry.md)，它封装了一个 `SYSTEM_PROCESS_INFORMATION` 结构体并提供惰性线程枚举。此成员为公有，以便主循环可以按 PID 迭代、查找和修改条目。 |

## 生命周期参数 `'a`

缓冲区和映射都从调用者拥有的静态变量（[`SNAPSHOT_BUFFER`](SNAPSHOT_BUFFER.md) 和 [`PID_TO_PROCESS_MAP`](PID_TO_PROCESS_MAP.md)）中借用。生命周期 `'a` 将 `ProcessSnapshot` 绑定到这些 `MutexGuard` 借用，保证存储在每个 [`ProcessEntry`](ProcessEntry.md) 中的原始指针在快照存在期间始终有效。

## 方法

| 方法 | 描述 |
|------|------|
| [`take`](#take) | 通过调用 `NtQuerySystemInformation` 捕获新的进程快照。 |
| [`get_by_name`](#get_by_name) | 返回所有镜像名称与给定字符串匹配的进程条目。 |
| [`drop`](#drop-实现) | 当快照超出作用域时清除进程映射和缓冲区。 |

---

### take

捕获所有正在运行的进程及其线程的快照。

#### 语法

```rust
pub fn take(
    buffer: &'a mut Vec<u8>,
    pid_to_process: &'a mut HashMap<u32, ProcessEntry>,
) -> Result<Self, i32>
```

#### 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `buffer` | `&'a mut Vec<u8>` | 可复用的字节缓冲区。首次调用时通常为 32 字节；函数会根据需要增长它。容量在多次调用间保留，因此后续快照很少需要重新分配。 |
| `pid_to_process` | `&'a mut HashMap<u32, ProcessEntry>` | 要填充的映射。解析开始前会被清除。以 PID（`UniqueProcessId` 转换为 `u32`）为键。 |

#### 返回值

| 值 | 描述 |
|----|------|
| `Ok(ProcessSnapshot)` | 有效的快照。调用者拥有 RAII 守卫；销毁时会清除数据。 |
| `Err(i32)` | `NtQuerySystemInformation` 返回的 NTSTATUS 错误码（除 `STATUS_INFO_LENGTH_MISMATCH` 以外的任何负值，后者在内部处理）。 |

#### 备注

1. **缓冲区增长策略** — 当 `NtQuerySystemInformation` 返回 `STATUS_INFO_LENGTH_MISMATCH`（`0xC0000004`）时，函数使用更大的缓冲区重试。如果内核提供了 `return_len`，新大小为 `((return_len / 8) + 1) * 8`（8 字节对齐）；否则缓冲区大小翻倍。此循环持续进行直到调用成功或返回不同的错误。

2. **链表遍历** — `SYSTEM_PROCESS_INFORMATION` 条目通过 `NextEntryOffset` 形成内存中的链表。函数从偏移量 0 开始遍历此链表，为每个节点构造一个 [`ProcessEntry`](ProcessEntry.md)，直到 `NextEntryOffset == 0`。

3. **线程指针** — 每个 `SYSTEM_PROCESS_INFORMATION` 之后紧跟其线程数组（`Threads` 柔性数组成员）。此数组的基指针由 [`ProcessEntry::new`](ProcessEntry.md) 捕获，以便 `get_threads` 可以稍后惰性解析它。这些指针仅在 `buffer` 存活期间有效——`Drop` 实现确保清理。

4. **安全性** — 函数体包裹在 `unsafe` 中，因为它解引用了内核返回的原始指针。正确性依赖于 `NtQuerySystemInformation` 写入有效的 `SYSTEM_PROCESS_INFORMATION` 结构体，以及缓冲区在快照的整个生命周期内保持固定。

5. **典型调用模式**：
   ```rust
   let mut buf = SNAPSHOT_BUFFER.lock().unwrap();
   let mut map = PID_TO_PROCESS_MAP.lock().unwrap();
   let snapshot = ProcessSnapshot::take(&mut buf, &mut map)?;
   // 使用 snapshot.pid_to_process …
   // snapshot 在此处被销毁——缓冲区和映射被清除
   ```

---

### get_by_name

返回所有小写镜像名称与给定字符串匹配的进程条目的引用。

#### 语法

```rust
pub fn get_by_name(&self, name: String) -> Vec<&ProcessEntry>
```

#### 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `name` | `String` | 要搜索的进程镜像名称（例如 `"explorer.exe"`）。与存储在每个 [`ProcessEntry`](ProcessEntry.md) 中的小写名称进行比较。 |

#### 返回值

包含零个或多个匹配条目引用的 `Vec<&ProcessEntry>`。当多个进程共享相同的镜像名称时，可能有多个条目匹配。

#### 备注

- 此方法在源代码中标记为 `#[allow(dead_code)]`，主要用于诊断或交互式使用。
- 比较使用 [`ProcessEntry::new`](ProcessEntry.md) 预先缓存的小写名称，因此 `name` 参数也应为小写才能匹配。

---

### Drop 实现

当 `ProcessSnapshot` 超出作用域时清除所有快照数据。

#### 语法

```rust
impl<'a> Drop for ProcessSnapshot<'a> {
    fn drop(&mut self);
}
```

#### 备注

`Drop` 实现先调用 `self.pid_to_process.clear()` 然后调用 `self.buffer.clear()`。这至关重要，因为 [`ProcessEntry`](ProcessEntry.md) 对象持有指向缓冲区的原始 `threads_base_ptr` 指针。先清除映射确保没有 `ProcessEntry` 的生存期超过它引用的缓冲区内存。

注意 `Vec::clear` 将长度设为零但保留已分配的容量，因此缓冲区内存会在下一次 `take` 调用时被复用而无需重新分配（除非系统已增长）。

## 要求

| | |
|---|---|
| **模块** | `process.rs` |
| **调用者** | [`main.rs`](../main.rs/README.md) 主循环、[`apply.rs`](../apply.rs/README.md) apply 函数 |
| **被调用者** | `NtQuerySystemInformation`（ntdll）、[`ProcessEntry::new`](ProcessEntry.md) |
| **API** | `ntapi::ntexapi::NtQuerySystemInformation` 使用 `SystemProcessInformation` 信息类 |
| **权限** | 建议具有 `SeDebugPrivilege` 以获得完整的系统可见性；若没有该权限，内核可能会省略某些进程条目 |

## 另请参阅

| 链接 | 描述 |
|------|------|
| [`ProcessEntry`](ProcessEntry.md) | 支持惰性线程解析的单个进程封装 |
| [`SNAPSHOT_BUFFER`](SNAPSHOT_BUFFER.md) | 快照的全局后备缓冲区 |
| [`PID_TO_PROCESS_MAP`](PID_TO_PROCESS_MAP.md) | 全局 PID 到 ProcessEntry 的映射 |
| [`PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md) | 从快照中消费线程数据以进行主力线程调度决策 |
| [NtQuerySystemInformation (MSDN)](https://learn.microsoft.com/en-us/windows/win32/api/winternl/nf-winternl-ntquerysysteminformation) | 底层 Windows API |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd