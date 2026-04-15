# ProcessEntry 结构体 (process.rs)

表示系统快照中的单个进程，封装了原生 `SYSTEM_PROCESS_INFORMATION` 结构体，并提供缓存的小写进程名称和延迟线程枚举。`ProcessEntry` 是存储在由 [`ProcessSnapshot::take`](ProcessSnapshot.md) 构建的 PID 键控查找映射中的每进程数据单元。

## 语法

```rust
#[derive(Clone)]
pub struct ProcessEntry {
    pub process: SYSTEM_PROCESS_INFORMATION,
    threads_base_ptr: usize,
    name: String,
}
```

## 成员

| 字段 | 类型 | 可见性 | 描述 |
|------|------|--------|------|
| `process` | `SYSTEM_PROCESS_INFORMATION` | `pub` | 由 `NtQuerySystemInformation` 返回的原始 NT 进程信息结构体。包含 `UniqueProcessId`、`NumberOfThreads`、`ImageName` 以及各种资源计数器等字段。 |
| `threads_base_ptr` | `usize` | 私有 | 紧跟在快照缓冲区中进程条目之后的线程信息数组 (`SYSTEM_THREAD_INFORMATION[]`) 的基地址（以 `usize` 存储）。以数值而非原始指针存储，以满足 `Clone` 和 `Send` 的要求。 |
| `name` | `String` | 私有 | **小写形式**的进程映像名称，在构造期间从 UTF-16 `ImageName` 字段解码而来。对于系统空闲进程（PID 0），由于其 `ImageName.Buffer` 为 null，名称为空字符串。 |

## 方法

### `new`

```rust
pub fn new(
    process: SYSTEM_PROCESS_INFORMATION,
    threads_base_ptr: *const SYSTEM_THREAD_INFORMATION,
) -> Self
```

从原始 `SYSTEM_PROCESS_INFORMATION` 结构体和指向其线程数组的指针构造一个新的 `ProcessEntry`。在构造过程中，进程映像名称从 `ImageName.Buffer`（UTF-16）解码为小写 `String`。如果 `ImageName.Length` 为零或 `ImageName.Buffer` 为 null，则名称设为空字符串。

### `get_threads`

```rust
pub fn get_threads(&self) -> HashMap<u32, SYSTEM_THREAD_INFORMATION>
```

构建并返回一个 `HashMap`，将线程 ID (`u32`) 映射到对应的 `SYSTEM_THREAD_INFORMATION` 结构体。通过从 `threads_base_ptr` 开始迭代 `NumberOfThreads` 个条目，解析来自 `SYSTEM_PROCESS_INFORMATION` 的原始线程数组。每个线程的 `ClientId.UniqueThread` 用作映射键。

如果 `threads_base_ptr` 为 null，则返回空映射。

> **注意：** 此方法每次调用都会重新构建映射，不会在内部缓存结果。

### `get_name`

```rust
pub fn get_name(&self) -> &str
```

返回对缓存的小写进程名称的引用。这是一个 `#[inline]` 访问器，零分配开销。

### `get_name_original_case`

```rust
pub fn get_name_original_case(&self) -> String
```

从原始 `ImageName` UTF-16 缓冲区重新读取进程映像名称，**不进行**小写转换，以新 `String` 形式返回原始大小写的名称。此方法对 `ImageName.Buffer` 指针执行不安全读取，该指针仅在父级 [`ProcessSnapshot`](ProcessSnapshot.md) 存活且后备缓冲区未被清除时有效。

标记为 `#[allow(dead_code)]` —— 保留用于需要原始大小写的诊断或显示场景。

### `pid`

```rust
pub fn pid(&self) -> u32
```

返回从 `process.UniqueProcessId` 提取的进程标识符，通过 `usize` 转换为 `u32`。这是一个 `#[inline]` 访问器。

### `thread_count`

```rust
pub fn thread_count(&self) -> u32
```

返回进程中的线程数量，取自 `process.NumberOfThreads`。这是一个 `#[inline]` 访问器。

## 备注

### Send 安全性

`ProcessEntry` 有一个显式的 `unsafe impl Send for ProcessEntry` 声明。在以下不变量条件下这是安全的：

- `ProcessEntry` 实例仅通过 `Mutex` 保护的容器 (`PID_TO_PROCESS_MAP`) 访问，确保在任何给定时间只有单线程访问。
- `SYSTEM_PROCESS_INFORMATION` 内部的原始指针（如 `ImageName.Buffer`）指向由 [`ProcessSnapshot`](ProcessSnapshot.md) 拥有的快照缓冲区。这些指针仅在该缓冲区的生命周期内有效，在快照被丢弃后不得解引用。

### 生命周期耦合

`threads_base_ptr` 和 `process.ImageName.Buffer` 指向由父级 [`ProcessSnapshot`](ProcessSnapshot.md) 拥有的 [`SNAPSHOT_BUFFER`](SNAPSHOT_BUFFER.md)。一旦快照被丢弃（即缓冲区被清除），这些指针就变成悬垂指针。解引用这些指针的方法（`get_threads`、`get_name_original_case`）仅在快照存活期间调用才是安全的。

`name` 字段（小写 `String`）是拥有所有权的副本，因此 `get_name()` 无论快照生命周期如何都可以安全调用。

### Clone 行为

`ProcessEntry` 派生了 `Clone`。克隆的实例共享相同的 `threads_base_ptr` 值（数值地址）和相同的 `SYSTEM_PROCESS_INFORMATION` 内容。克隆的条目与原始条目具有相同的指针有效性生命周期约束。

### 名称规范化

进程名称以小写形式存储，以便与配置规则进行不区分大小写的匹配。转换对 UTF-16 解码后的名称使用 `String::to_lowercase()`，遵循 Rust 的 Unicode 小写规则（对于 ASCII 进程名称，这与 Windows 不区分大小写的文件名比较一致）。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `process.rs` |
| **创建者** | `ProcessEntry::new`（从 [`ProcessSnapshot::take`](ProcessSnapshot.md) 调用） |
| **存储位置** | [`PID_TO_PROCESS_MAP`](PID_TO_PROCESS_MAP.md) |
| **依赖项** | `ntapi::ntexapi::{SYSTEM_PROCESS_INFORMATION, SYSTEM_THREAD_INFORMATION}`、[`HashMap`](../collections.rs/HashMap.md) |
| **平台** | 仅限 Windows |

## 另请参阅

| 主题 | 链接 |
|------|------|
| ProcessSnapshot 结构体 | [ProcessSnapshot](ProcessSnapshot.md) |
| SNAPSHOT_BUFFER 静态变量 | [SNAPSHOT_BUFFER](SNAPSHOT_BUFFER.md) |
| PID_TO_PROCESS_MAP 静态变量 | [PID_TO_PROCESS_MAP](PID_TO_PROCESS_MAP.md) |
| HashMap 类型别名 | [HashMap](../collections.rs/HashMap.md) |
| winapi 模块 | [winapi.rs](../winapi.rs/README.md) |
| process 模块概述 | [README](README.md) |

---
> Commit SHA: `b0df9da35213b050501fab02c3020ad4dbd6c4e0`
