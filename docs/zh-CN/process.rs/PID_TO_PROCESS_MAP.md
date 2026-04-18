# PID_TO_PROCESS_MAP 静态变量 (process.rs)

全局共享的 PID 到进程查找映射，由 [`ProcessSnapshot::take`](ProcessSnapshot.md) 在每次快照捕获期间填充。此静态变量保存以进程 ID 为键的已解析 [`ProcessEntry`](ProcessEntry.md) 对象，支持按 PID 进行高效的 O(1) 进程信息查找。

## 语法

```rust
pub static PID_TO_PROCESS_MAP: Lazy<Mutex<HashMap<u32, ProcessEntry>>> =
    Lazy::new(|| Mutex::new(HashMap::default()));
```

## 类型

`once_cell::sync::Lazy<std::sync::Mutex<HashMap<u32, ProcessEntry>>>`

## 成员

| 组件 | 类型 | 描述 |
|------|------|------|
| 键 | `u32` | 进程标识符 (PID)。 |
| 值 | [`ProcessEntry`](ProcessEntry.md) | 已解析的进程信息，包括名称、线程计数和原始 `SYSTEM_PROCESS_INFORMATION` 数据。 |

## 备注

> **请勿直接访问** — 请改用 [`ProcessSnapshot`](ProcessSnapshot.md) 结构体。

此静态变量旨在仅通过 [`ProcessSnapshot`](ProcessSnapshot.md) RAII 包装器访问，该包装器管理其生命周期（填充和清理）。直接访问会绕过快照的安全保证，可能导致读取到过时或部分填充的数据。

### 生命周期

1. **初始化。** 映射在首次访问时通过 `once_cell::sync::Lazy` 惰性初始化为空 `HashMap`。
2. **填充。** [`ProcessSnapshot::take`](ProcessSnapshot.md) 清空映射，然后通过解析 [`SNAPSHOT_BUFFER`](SNAPSHOT_BUFFER.md) 中的原始 `SYSTEM_PROCESS_INFORMATION` 结构重新填充。每个进程以其 PID 作为键插入。
3. **清理。** 当 `ProcessSnapshot` 被丢弃时，映射通过 `pid_to_process.clear()` 被清空，确保快照周期之间不会残留过时的条目。

### 线程安全

映射受 `std::sync::Mutex` 保护。调用者在读取或写入之前必须获取锁。在实践中，每个调度周期在获取新快照时获取一次锁，生成的 `MutexGuard` 通过 `ProcessSnapshot` 结构体的借用引用在快照的整个生命周期内持有。

### 数据有效性

存储在此映射中的 [`ProcessEntry`](ProcessEntry.md) 值包含指向 [`SNAPSHOT_BUFFER`](SNAPSHOT_BUFFER.md) 的原始指针（`threads_base_ptr`）。这些指针仅在快照缓冲区当前内容的生命周期内有效。当 `ProcessSnapshot` 被丢弃且缓冲区被清空后，任何保留的 `ProcessEntry` 引用都将包含悬垂指针。`ProcessSnapshot` 的 RAII 设计通过在丢弃时同时清空映射和缓冲区来防止此问题。

### HashMap 类型

此处使用的 `HashMap` 是项目自定义的 [`collections`](../collections.rs/README.md) 模块中 `FxHashMap` 的类型别名，使用快速的非加密哈希函数（Fx 哈希），针对 PID 等整数键进行了优化。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `process.rs` |
| **类型** | `Lazy<Mutex<HashMap<u32, ProcessEntry>>>` |
| **可见性** | `pub`（但应仅通过 [`ProcessSnapshot`](ProcessSnapshot.md) 访问） |
| **填充者** | [`ProcessSnapshot::take`](ProcessSnapshot.md) |
| **清理者** | `ProcessSnapshot::drop` |
| **依赖** | `once_cell::sync::Lazy`、`std::sync::Mutex`、[`HashMap`](../collections.rs/HashMap.md)、[`ProcessEntry`](ProcessEntry.md) |
| **平台** | Windows |

## 另请参阅

| 主题 | 链接 |
|------|------|
| ProcessSnapshot 结构体 | [ProcessSnapshot](ProcessSnapshot.md) |
| ProcessEntry 结构体 | [ProcessEntry](ProcessEntry.md) |
| SNAPSHOT_BUFFER 静态变量 | [SNAPSHOT_BUFFER](SNAPSHOT_BUFFER.md) |
| collections 模块 | [collections.rs](../collections.rs/README.md) |
| HashMap 类型别名 | [HashMap](../collections.rs/HashMap.md) |
| process 模块概述 | [README](README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
