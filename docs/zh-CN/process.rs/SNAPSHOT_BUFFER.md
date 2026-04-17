# SNAPSHOT_BUFFER 静态变量 (process.rs)

`NtQuerySystemInformation` 结果的共享后备缓冲区。此静态变量提供 [`ProcessSnapshot::take`](ProcessSnapshot.md) 用系统进程信息数据填充的原始字节存储。它以较小的初始容量进行延迟初始化，并在快照捕获过程中根据需要动态调整大小。

## 语法

```rust
pub static SNAPSHOT_BUFFER: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| Mutex::new(vec![0u8; 32]));
```

## 类型

`once_cell::sync::Lazy<std::sync::Mutex<Vec<u8>>>`

## 备注

> **警告：** 请勿直接访问此静态变量。请改用 [`ProcessSnapshot`](ProcessSnapshot.md) 结构体，它管理缓冲区的生命周期并确保正确性。

### 初始容量

缓冲区以 32 字节的容量初始化——这是故意设置得很小的。在首次调用 [`ProcessSnapshot::take`](ProcessSnapshot.md) 时，`NtQuerySystemInformation` 将返回 `STATUS_INFO_LENGTH_MISMATCH`，从而促使快照逻辑将缓冲区重新分配为 API 的 `return_length` 输出参数所报告的大小（向上取整到 8 字节边界）。在首次成功快照之后，缓冲区保留其扩大后的容量以供后续调用使用，避免重复的重新分配。

### 生命周期

1. **首次访问** — `Lazy` 使用 32 字节的向量初始化互斥锁。
2. **快照捕获** — [`ProcessSnapshot::take`](ProcessSnapshot.md) 锁定互斥锁，使用缓冲区调用 `NtQuerySystemInformation`，如果当前容量不足，可能会调整其大小。
3. **快照销毁** — 当 `ProcessSnapshot` 被销毁时，缓冲区被清空（`Vec::clear()`），释放数据但保留已分配的容量以供下一个快照周期使用。
4. **进程退出** — 缓冲区永远不会被显式释放；它作为 `'static` 资源在进程的整个生命周期中存在。

### 线程安全

缓冲区由 `Mutex` 保护。同一时间只有一个线程可以持有锁，确保并发的快照尝试被串行化。在实践中，快照是从单线程上的主调度循环中获取的，因此预计不会出现争用。

### 与 PID_TO_PROCESS_MAP 的关系

此缓冲区和 [`PID_TO_PROCESS_MAP`](PID_TO_PROCESS_MAP.md) 始终一起使用。`ProcessSnapshot::take` 方法需要对两者的可变引用，并且 `ProcessSnapshot` 结构体在其生命周期内持有对两者的引用。`SYSTEM_PROCESS_INFORMATION` 结构体内的原始指针（存储在进程条目中）指向此缓冲区，因此在任何 `ProcessEntry` 引用存活期间，**不得**修改或释放此缓冲区。

### 为什么使用全局静态变量？

缓冲区被存储为全局静态变量而非局部变量，是为了在调度循环迭代之间实现缓冲区的重用。通过在快照之间保留已分配的容量，应用程序避免了在每个周期重复进行大量分配（通常为 1-4 MB）。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `process.rs` |
| **可见性** | `pub`（但不应直接访问；请使用 `ProcessSnapshot`） |
| **使用者** | [`ProcessSnapshot::take`](ProcessSnapshot.md) |
| **依赖** | `once_cell::sync::Lazy`、`std::sync::Mutex` |
| **Win32 API** | `NtQuerySystemInformation`（通过 `ntapi` crate，由 `ProcessSnapshot::take` 调用） |
| **平台** | Windows |

## 另请参阅

| 主题 | 链接 |
|------|------|
| ProcessSnapshot 结构体 | [ProcessSnapshot](ProcessSnapshot.md) |
| PID_TO_PROCESS_MAP 静态变量 | [PID_TO_PROCESS_MAP](PID_TO_PROCESS_MAP.md) |
| ProcessEntry 结构体 | [ProcessEntry](ProcessEntry.md) |
| process 模块概述 | [README](README.md) |

---
*提交：[37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
