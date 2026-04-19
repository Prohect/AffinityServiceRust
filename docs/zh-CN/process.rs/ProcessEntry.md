# ProcessEntry 结构体 (process.rs)

表示系统进程快照中的单个进程。封装了原生 `SYSTEM_PROCESS_INFORMATION` 结构体，以及缓存的小写进程名称和指向线程数组的原始指针。实现了 `Clone` 和 `Send`（通过显式的 unsafe impl，其合理性基于 Mutex 保护的访问模式）。

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

| 成员 | 类型 | 描述 |
|------|------|------|
| `process` | `SYSTEM_PROCESS_INFORMATION` | 原始 Windows 内核结构体，包含进程指标：PID、线程数、工作集大小、时间信息、句柄数以及 `ImageName` UNICODE_STRING。 |
| `threads_base_ptr` | `usize` | 快照缓冲区中紧跟在进程结构体之后的 `SYSTEM_THREAD_INFORMATION` 数组的基地址。存储为 `usize` 而非原始指针，以允许派生 `Clone`。仅在父级 [ProcessSnapshot](ProcessSnapshot.md) 存活期间有效。 |
| `name` | `String` | 进程映像名称的小写副本，在构造时从 `process.ImageName` 提取。在整个服务中用于不区分大小写的配置匹配。 |

## 方法

### new

```rust
pub fn new(
    process: SYSTEM_PROCESS_INFORMATION,
    threads_base_ptr: *const SYSTEM_THREAD_INFORMATION,
) -> Self
```

从原生进程信息结构体和指向其线程数组的指针构造一个 `ProcessEntry`。

**参数**

| 参数 | 类型 | 描述 |
|------|------|------|
| `process` | `SYSTEM_PROCESS_INFORMATION` | 从快照缓冲区复制的进程信息结构体。 |
| `threads_base_ptr` | `*const SYSTEM_THREAD_INFORMATION` | 指向嵌入在快照缓冲区中的线程数组第一个元素的指针。存储时转换为 `usize`。 |

**备注**

- 从 `ImageName` UNICODE_STRING 字段提取进程映像名称，通过 `String::from_utf16_lossy` 将 UTF-16 转换为 Rust `String`。
- 名称立即通过 `.to_lowercase()` 转换为小写，用于不区分大小写的匹配。
- 如果 `ImageName.Length` 为零或缓冲区指针为空（如系统空闲进程 PID 0 的情况），则存储空字符串。

---

### get_threads

```rust
pub fn get_threads(&self) -> HashMap<u32, SYSTEM_THREAD_INFORMATION>
```

将快照缓冲区中的原始线程数组解析为以线程 ID (TID) 为键的 `HashMap`。

**返回值**

`HashMap<u32, SYSTEM_THREAD_INFORMATION>` — 从每个线程的 `ClientId.UniqueThread`（转换为 `u32`）到其完整 `SYSTEM_THREAD_INFORMATION` 结构体的映射。

**备注**

- 从 `threads_base_ptr` 开始迭代 `process.NumberOfThreads` 个条目。
- 返回的映射在每次调用时重新构造；结果不会在 `ProcessEntry` 内部缓存。
- 如果 `threads_base_ptr` 为空（存储为 `0usize`），则立即返回空映射。
- **安全性：** 指针运算仅在父级 [ProcessSnapshot](ProcessSnapshot.md) 存活且其缓冲区未被清除时有效。在快照被丢弃后调用此方法属于未定义行为。

---

### get_name

```rust
#[inline]
pub fn get_name(&self) -> &str
```

返回缓存的小写进程映像名称。

**返回值**

`&str` — 对小写名称字符串的借用引用。对于系统空闲进程（PID 0）返回 `""`。

---

### get_name_original_case

```rust
#[inline]
pub fn get_name_original_case(&self) -> String
```

以原始大小写形式重新读取进程映像名称。

**返回值**

`String` — 保留内核原始大小写的进程名称，例如 `"svchost.exe"` 或 `"MsMpEng.exe"`。如果映像名称缓冲区为空或长度为零，则返回空字符串。

**备注**

- 与 [get_name](#get_name) 不同，此方法每次调用都会执行新的 UTF-16 转换。
- 直接从 `process.ImageName.Buffer` 指针读取，因此与 [get_threads](#get_threads) 具有相同的生命周期约束。
- 当前标记为 `#[allow(dead_code)]`；可用于诊断/日志记录场景。

---

### pid

```rust
#[inline]
pub fn pid(&self) -> u32
```

返回进程标识符。

**返回值**

`u32` — 原生结构体中的 `UniqueProcessId` 字段，通过 `usize` 转换为 `u32`。

---

### thread_count

```rust
#[inline]
pub fn thread_count(&self) -> u32
```

返回此进程中的线程数量。

**返回值**

`u32` — 原生 `SYSTEM_PROCESS_INFORMATION` 结构体中的 `NumberOfThreads` 字段。

## 备注

### Send 安全性

`ProcessEntry` 包含一个 `SYSTEM_PROCESS_INFORMATION` 值，其中含有原始指针（例如 `ImageName.Buffer`）。显式的 `unsafe impl Send for ProcessEntry` 的合理性基于访问模式：所有 `ProcessEntry` 实例都存在于 `PID_TO_PROCESS_MAP` 中，该映射受 `Mutex` 保护。没有任何 `ProcessEntry` 在没有互斥锁保护的情况下跨线程发送。

### 生命周期约束

`threads_base_ptr` 和 `process.ImageName.Buffer` 指针引用的是 [ProcessSnapshot](ProcessSnapshot.md) 拥有的快照缓冲区。通过这些指针的所有访问必须在快照存活期间进行。`name` 字段是安全的拥有所有权的 `String` 副本，没有此约束。

### Clone 行为

克隆 `ProcessEntry` 会按值复制 `SYSTEM_PROCESS_INFORMATION` 结构体（包括其嵌入的原始指针）并克隆 `name` 字符串。克隆的条目共享相同的基于指针的字段，并受与原始条目相同的生命周期约束。

## 要求

| | |
|---|---|
| **模块** | `src/process.rs` |
| **调用方** | 主轮询循环、[apply_prime_threads](../apply.rs/apply_prime_threads.md)、以及从 [ProcessSnapshot](ProcessSnapshot.md) 迭代进程的任何代码 |
| **依赖** | `ntapi::ntexapi::SYSTEM_PROCESS_INFORMATION`、`ntapi::ntexapi::SYSTEM_THREAD_INFORMATION` |
| **权限** | 无（操作已捕获的快照数据） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块概述 | [process.rs](README.md) |
| 快照 RAII 封装器 | [ProcessSnapshot](ProcessSnapshot.md) |
| 线程级统计跟踪 | [ThreadStats](../scheduler.rs/ThreadStats.md) |
| 进程句柄管理 | [ProcessHandle](../winapi.rs/ProcessHandle.md) |

*文档记录自提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*
