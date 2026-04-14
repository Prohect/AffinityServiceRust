# PID_TO_PROCESS_MAP 静态变量 (process.rs)

全局延迟初始化的从进程 ID（`u32`）到 [`ProcessEntry`](ProcessEntry.md) 的映射，由 `Mutex` 保护。此静态变量存储由 [`ProcessSnapshot::take`](ProcessSnapshot.md#take) 获取的最近一次进程快照的解析结果。它是快照基础设施的实现细节，**不得直接访问**；请改用 [`ProcessSnapshot`](ProcessSnapshot.md)。

## 语法

```rust
pub static PID_TO_PROCESS_MAP: Lazy<Mutex<HashMap<u32, ProcessEntry>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
```

## 类型

`once_cell::sync::Lazy<std::sync::Mutex<std::collections::HashMap<u32, ProcessEntry>>>`

## 备注

`PID_TO_PROCESS_MAP` 是 [`ProcessSnapshot::take`](ProcessSnapshot.md#take) 解析 `NtQuerySystemInformation` 返回的 `SYSTEM_PROCESS_INFORMATION` 记录时所填充的后备存储。每个键是一个 Windows 进程 ID，每个值是一个包含对应 `SYSTEM_PROCESS_INFORMATION` 结构和延迟解析线程数据的 [`ProcessEntry`](ProcessEntry.md)。

### 为何禁止直接访问

映射的内容仅在填充它的 [`ProcessSnapshot`](ProcessSnapshot.md) 存活期间有效。当快照被销毁时，映射和底层的 [`SNAPSHOT_BUFFER`](SNAPSHOT_BUFFER.md) 都会被清空。销毁后获取的任何 `ProcessEntry` 包含悬垂的内部指针（`threads_base_ptr` 字段指向已释放的缓冲区）。在快照生命周期之外访问映射属于**未定义行为**。

请始终通过 `ProcessSnapshot` 获取数据：

```rust
let mut buf = SNAPSHOT_BUFFER.lock().unwrap();
let mut map = PID_TO_PROCESS_MAP.lock().unwrap();
let snapshot = ProcessSnapshot::take(&mut buf, &mut map)?;
// 在此处安全地使用 snapshot.pid_to_process
// snapshot 在作用域结束时被销毁，清空两个静态变量
```

### 线程安全

`Mutex` 确保了互斥访问。由于 [`ProcessSnapshot::take`](ProcessSnapshot.md#take) 需要同时持有缓冲区和映射的 `&mut` 引用，同一时刻只能存在一个快照。

## 要求

| &nbsp; | &nbsp; |
|---|---|
| **模块** | `process` (`src/process.rs`) |
| **Crate 依赖** | `once_cell`、`ntapi` |
| **初始化方** | [`ProcessSnapshot::take`](ProcessSnapshot.md#take) |
| **清理方** | [`ProcessSnapshot::drop`](ProcessSnapshot.md) |
| **权限** | 无（仅初始化；快照捕获间接需要 `SeDebugPrivilege`） |

## 另请参阅

| 主题 | 描述 |
|---|---|
| [`SNAPSHOT_BUFFER`](SNAPSHOT_BUFFER.md) | 支撑 `NtQuerySystemInformation` 数据的全局缓冲区 |
| [`ProcessSnapshot`](ProcessSnapshot.md) | 安全管理两个静态变量的 RAII 包装器 |
| [`ProcessEntry`](ProcessEntry.md) | 作为映射值存储的单个进程条目记录 |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd