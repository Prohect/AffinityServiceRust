# SNAPSHOT_BUFFER 静态变量 (process.rs)

[`ProcessSnapshot::take`](ProcessSnapshot.md#processsnapshottake) 使用的全局缓冲区，用于存储 `NtQuerySystemInformation(SystemProcessInformation, ...)` 返回的原始字节输出。

## 语法

```rust
pub static SNAPSHOT_BUFFER: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| Mutex::new(vec![0u8; 32]));
```

## 成员

| 成员 | 类型 | 描述 |
|------|------|------|
| 内部值 | `Vec<u8>` | 接收内核返回的可变长度 `SYSTEM_PROCESS_INFORMATION` 结构体数组的字节缓冲区。初始大小为 32 字节；当内核返回 `STATUS_INFO_LENGTH_MISMATCH` 时，由 `ProcessSnapshot::take` 自动扩容。 |

## 备注

> **重要：** 请勿直接锁定或访问 `SNAPSHOT_BUFFER`。请始终通过 [`ProcessSnapshot`](ProcessSnapshot.md) RAII 包装器进行访问，该包装器会同时锁定 `SNAPSHOT_BUFFER` 和 [`PID_TO_PROCESS_MAP`](PID_TO_PROCESS_MAP.md)，确保两者保持一致。

该缓冲区通过 `once_cell::sync::Lazy` 分配一次，并在连续的快照之间重复使用，以摊销分配成本。当系统的进程列表增长超过当前容量时，`ProcessSnapshot::take` 可能会重新分配缓冲区。当 `ProcessSnapshot` 被销毁时，缓冲区会被清空（长度设为零），但底层分配的内存会被保留，供下一次快照使用。

### 生命周期与线程安全

`SNAPSHOT_BUFFER` 被包裹在 `Mutex` 中以保证独占访问。由于缓冲区内的原始 `SYSTEM_PROCESS_INFORMATION` 结构体包含指针（例如 `ImageName.Buffer`），这些指针仅在缓冲区内容未被修改时有效，因此 `ProcessSnapshot` 的借用会在快照使用期间保持互斥锁守卫的存活。

### 扩容策略

当 `NtQuerySystemInformation` 返回 `STATUS_INFO_LENGTH_MISMATCH`（`0xC0000004`）时：

1. 如果内核报告了所需长度（`return_len > 0`），缓冲区将被调整为该长度并向上对齐到 8 字节边界。
2. 否则，缓冲区容量翻倍。

随后在循环中重试调用，直到成功或返回其他 NTSTATUS 错误。

## 要求

| | |
|---|---|
| **模块** | `process.rs` |
| **Crate 依赖** | `once_cell`、`ntapi` |
| **同步机制** | `std::sync::Mutex` — 访问前需加锁 |
| **权限** | 除调用方 [`ProcessSnapshot::take`](ProcessSnapshot.md#processsnapshottake) 所需的权限外，无额外要求 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| ProcessSnapshot 结构体 | [ProcessSnapshot](ProcessSnapshot.md) |
| PID_TO_PROCESS_MAP 静态变量 | [PID_TO_PROCESS_MAP](PID_TO_PROCESS_MAP.md) |
| ProcessEntry 结构体 | [ProcessEntry](ProcessEntry.md) |
| NtQuerySystemInformation | [Microsoft Learn — NtQuerySystemInformation](https://learn.microsoft.com/en-us/windows/win32/api/winternl/nf-winternl-ntquerysysteminformation) |