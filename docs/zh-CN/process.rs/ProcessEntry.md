# ProcessEntry 结构体 (process.rs)

表示系统快照中的单个进程，封装了原生 `SYSTEM_PROCESS_INFORMATION` 结构，并提供延迟线程解析和缓存的名称查找。

`ProcessEntry` 存储了指向快照缓冲区中线程数组的原始指针，延迟到首次调用 `get_threads` 或 `get_thread` 时才将其解析为 `HashMap<u32, SYSTEM_THREAD_INFORMATION>`。进程映像名称在构造时即被解析并转换为小写，以便在配置查找期间进行高效匹配。

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

| 成员 | 类型 | 描述 |
|------|------|------|
| `process` | `SYSTEM_PROCESS_INFORMATION` | 原始 NT 进程信息结构。设为公开以便直接访问字段（如 `UniqueProcessId`、`NumberOfThreads`、`WorkingSetSize`）。 |
| `threads` | `HashMap<u32, SYSTEM_THREAD_INFORMATION>` | 延迟填充的线程 ID（TID）到线程信息的映射。在调用 `get_threads` 之前为空。私有字段。 |
| `threads_base_ptr` | `usize` | 快照缓冲区中 `Threads` 柔性数组成员的基地址，存储为 `usize` 以满足 `Send` 要求。私有字段。 |
| `name` | `String` | 小写的进程映像名称（如 `"explorer.exe"`），在构造时从 `ImageName` `UNICODE_STRING` 解析而来。私有字段。 |

## 方法

### new

```rust
pub fn new(
    process: SYSTEM_PROCESS_INFORMATION,
    threads_base_ptr: *const SYSTEM_THREAD_INFORMATION,
) -> Self
```

从原始进程信息结构和指向其线程数组的指针构造一个 `ProcessEntry`。

映像名称从 `process.ImageName`（UTF-16LE `UNICODE_STRING`）解码并立即转换为小写。如果映像名称缓冲区为空或长度为零，名称将被设置为空字符串。线程映射保持为空；线程在首次通过 `get_threads` 访问时延迟解析。

| 参数 | 描述 |
|------|------|
| `process` | 该进程的 `SYSTEM_PROCESS_INFORMATION` 结构的副本。 |
| `threads_base_ptr` | 指向 `Threads` 柔性数组成员第一个元素的指针。内部存储为 `usize`。在拥有该条目的 `ProcessSnapshot` 生命周期内必须保持有效。 |

**返回值：** 一个新的 `ProcessEntry`，包含已解析的名称和延迟的线程映射。

### get_threads

```rust
pub fn get_threads(&mut self) -> &HashMap<u32, SYSTEM_THREAD_INFORMATION>
```

返回线程信息映射，首次调用时从原始线程数组指针延迟填充。

如果内部线程映射的长度与 `process.NumberOfThreads` 不匹配，映射将被清空并通过遍历原始 `SYSTEM_THREAD_INFORMATION` 数组重新填充。每个线程以其 TID（`ClientId.UniqueThread`）为键。在后续调用中，如果线程数量没有变化，将直接返回缓存的映射。

**返回值：** 以线程 ID 为键的 `HashMap<u32, SYSTEM_THREAD_INFORMATION>` 的引用。

> [!IMPORTANT]
> 原始指针的解引用仅在父级 `ProcessSnapshot`（及其后备缓冲区）存活期间是安全的。在快照被释放后调用此方法属于未定义行为。

### get_thread

```rust
pub fn get_thread(&mut self, tid: u32) -> Option<&SYSTEM_THREAD_INFORMATION>
```

返回指定 TID 的线程信息，如果该线程不属于此进程则返回 `None`。

内部调用 `get_threads` 以确保映射已填充，然后执行哈希查找。

| 参数 | 描述 |
|------|------|
| `tid` | 要查找的线程 ID。 |

**返回值：** 如果线程存在则返回 `Some(&SYSTEM_THREAD_INFORMATION)`，否则返回 `None`。

### get_name

```rust
pub fn get_name(&self) -> &str
```

返回小写的进程映像名称（如 `"chrome.exe"`）。

此值在 `new` 期间计算一次并缓存。对于系统空闲进程（PID 0）或 `ImageName` 缓冲区为空的任何进程，返回空字符串。

**返回值：** 引用缓存的小写名称的字符串切片。

### get_name_original_case

```rust
pub fn get_name_original_case(&self) -> String
```

返回保留快照缓冲区中原始大小写的进程映像名称。

与 `get_name` 不同，此方法每次调用时从 `process` 字段重新读取 `ImageName` `UNICODE_STRING`，且不进行小写转换。适用于需要保留原始大小写的显示或日志场景。

**返回值：** 包含原始大小写映像名称的新 `String`，如果缓冲区为空则返回空字符串。

> [!NOTE]
> 此方法解引用 `ImageName.Buffer` 指针，该指针指向快照缓冲区。仅在父级 `ProcessSnapshot` 存活期间是安全的。

### pid

```rust
pub fn pid(&self) -> u32
```

返回进程 ID。

从底层 `SYSTEM_PROCESS_INFORMATION` 结构中提取 `UniqueProcessId`，通过 `usize` 转换为 `u32`。

**返回值：** `u32` 类型的 PID。

### thread_count

```rust
pub fn thread_count(&self) -> u32
```

返回操作系统报告的该进程线程数。

此方法从底层 `SYSTEM_PROCESS_INFORMATION` 读取 `NumberOfThreads`，**不**需要线程映射已填充。

**返回值：** `u32` 类型的线程计数。

## 备注

### 延迟线程解析

`ProcessEntry` 的主要设计目标是避免为未被任何配置规则匹配的进程解析线程数组。在典型的系统快照中，包含数百个进程和数千个线程，但只有少数进程匹配用户定义的规则。将 `O(n)` 的线程数组遍历延迟到 `get_threads` 调用时，使每次快照的开销与*匹配的*进程数成正比。

### 安全性和 Send

`ProcessEntry` 将线程数组指针存储为 `usize` 而非原始 `*const`，以允许该类型实现 `Send`。提供了 `unsafe impl Send for ProcessEntry`，其约定是实例仅通过 `Mutex`（经由 `PID_TO_PROCESS_MAP`）访问，且快照缓冲区的生命周期长于所有引用。

### 克隆

`ProcessEntry` 派生了 `Clone`。克隆的实例共享相同的 `threads_base_ptr` 值，这意味着克隆同样仅在快照缓冲区生命周期内有效。`threads` `HashMap` 是深度克隆的，因此已经解析了线程的克隆副本不需要重新解析。

## 要求

| 要求 | 值 |
|------|---|
| 模块 | `process.rs` |
| 构造者 | [`ProcessSnapshot::take`](ProcessSnapshot.md) |
| 使用者 | [`apply_affinity`](../apply.rs/apply_affinity.md)、[`apply_prime_threads`](../apply.rs/apply_prime_threads.md)、[`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md)、[`prefetch_all_thread_cycles`](../apply.rs/prefetch_all_thread_cycles.md) |
| NT API | `SYSTEM_PROCESS_INFORMATION`、`SYSTEM_THREAD_INFORMATION` (ntapi) |
| 权限 | 无（数据从已使用适当权限捕获的快照中读取） |

## 另请参阅

| 链接 | 描述 |
|------|------|
| [ProcessSnapshot](ProcessSnapshot.md) | 捕获快照并拥有缓冲区生命周期的 RAII 包装器。 |
| [SNAPSHOT_BUFFER](SNAPSHOT_BUFFER.md) | 支撑快照数据的全局缓冲区。 |
| [PID_TO_PROCESS_MAP](PID_TO_PROCESS_MAP.md) | 按 PID 存储 `ProcessEntry` 实例的全局映射。 |
| [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) | 从 `ProcessEntry` 消费线程周期数据以进行主力线程选择。 |