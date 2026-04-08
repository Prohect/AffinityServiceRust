# Process 模块文档

通过 NtQuerySystemInformation 进行进程和线程枚举。

## 概述

本模块使用原生 Windows API 提供高效的进程快照：
- 单次系统调用捕获所有进程和线程
- 延迟线程解析以提高内存效率
- 原始指针的安全生命周期管理

## 调用者

- [main.rs](main.md#main-loop) - 主循环进程枚举
- [apply.rs](apply.md) - 用于亲和性/理想处理器操作的线程迭代

## 数据结构

### ProcessSnapshot

捕获系统范围的进程信息。

```rust
pub struct ProcessSnapshot {
    buffer: Vec<u8>,                              // 原始 SYSTEM_PROCESS_INFORMATION 数据
    pub pid_to_process: HashMap<u32, ProcessEntry>, // 解析的进程条目
}
```

**字段：**
- `pid_to_process`: 按 PID 索引的 [`ProcessEntry`](#processentry) HashMap

**生命周期：**`buffer` 必须比 `ProcessEntry` 中的任何引用（threads 指针）存活更久。

**Drop 实现：**清除两个集合以释放内存。

### ProcessEntry

单个进程信息，延迟线程解析。

```rust
pub struct ProcessEntry {
    pub process: SYSTEM_PROCESS_INFORMATION,      // 原生系统结构
    threads: HashMap<u32, SYSTEM_THREAD_INFORMATION>, // 解析的线程（延迟）
    threads_base_ptr: usize,                      // 线程数组的原始指针
    name: String,                                 // 进程名（小写）
}
```

## 方法

### ProcessSnapshot::take

捕获所有进程和线程的快照。

```rust
pub fn take() -> Result<Self, i32>
```

**算法：**
1. 从 1KB 缓冲区开始
2. 调用 `NtQuerySystemInformation(SystemProcessInformation, ...)`
3. 如果 `STATUS_INFO_LENGTH_MISMATCH`，增长缓冲区并重试
4. 解析 `SYSTEM_PROCESS_INFORMATION` 结构链表
5. 为每个进程创建 `ProcessEntry`

**错误：**失败时返回负 NTSTATUS。

**示例：**
```rust
match ProcessSnapshot::take() {
    Ok(processes) => {
        for (pid, entry) in &processes.pid_to_process {
            println!("{}: {}", pid, entry.get_name());
        }
    }
    Err(status) => eprintln!("Failed: 0x{:08X}", status),
}
```

### ProcessSnapshot::get_by_name

按名称查找进程（不区分大小写）。

```rust
pub fn get_by_name(&self, name: String) -> Vec<&ProcessEntry>
```

**注意：**返回绑定到快照生命周期的引用。

### ProcessEntry::new

从原始系统数据创建 ProcessEntry。

```rust
pub fn new(
    process: SYSTEM_PROCESS_INFORMATION,
    threads_base_ptr: *const SYSTEM_THREAD_INFORMATION
) -> Self
```

**安全：**`threads_base_ptr` 必须保持有效（指向快照缓冲区）。

### ProcessEntry::get_threads

返回线程信息映射，首次调用时延迟填充。

```rust
#[inline]
pub fn get_threads(&mut self) -> &HashMap<u32, SYSTEM_THREAD_INFORMATION>
```

**延迟加载：**
- 首次调用：从原始指针解析线程数组到 HashMap
- 后续调用：返回缓存的 HashMap

**线程结构：**
```rust
SYSTEM_THREAD_INFORMATION {
    KernelTime: LARGE_INTEGER,
    UserTime: LARGE_INTEGER,
    CreateTime: LARGE_INTEGER,
    WaitTime: ULONG,
    StartAddress: PVOID,
    ClientId: CLIENT_ID { UniqueProcess, UniqueThread },
    Priority: KPRIORITY,
    BasePriority: LONG,
    ContextSwitches: ULONG,
    ThreadState: THREAD_STATE,
    WaitReason: KWAIT_REASON,
    // ... 附加字段
}
```

### ProcessEntry::get_thread

通过 TID 获取单个线程。

```rust
#[inline]
pub fn get_thread(&mut self, tid: u32) -> Option<&SYSTEM_THREAD_INFORMATION>
```

### ProcessEntry::get_name

获取进程名（小写）。

```rust
#[inline]
pub fn get_name(&self) -> &str
```

### ProcessEntry::get_name_original_case

获取进程名（来自系统的原始大小写）。

```rust
#[inline]
pub fn get_name_original_case(&self) -> String
```

### ProcessEntry::pid

获取进程 ID。

```rust
#[inline]
pub fn pid(&self) -> u32
```

### ProcessEntry::thread_count

获取线程数。

```rust
#[inline]
pub fn thread_count(&self) -> u32
```

## SYSTEM_PROCESS_INFORMATION 结构

使用的主要字段：

| 字段 | 描述 |
|-------|-------------|
| `NextEntryOffset` | 到下一个进程的偏移（0 = 最后一个） |
| `NumberOfThreads` | 线程计数 |
| `ImageName` | UNICODE_STRING 进程名 |
| `UniqueProcessId` | 进程 ID |
| `Threads[]` | 线程信息结构的内联数组 |

## 安全考虑

### 原始指针处理

本模块使用不安全代码：
1. 调用 `NtQuerySystemInformation`
2. 解引用 `SYSTEM_PROCESS_INFORMATION` 指针
3. 迭代线程数组

**维护的不变式：**
- 缓冲区在解析前正确调整大小
- 快照存在时指针有效
- 解引用前进行空检查

### 内存布局

Windows API 在连续缓冲区中返回链表：
```
[Process1][Gap][Process2][Gap][Process3][Gap]...
     ↓
 [Threads1...]
```

`Threads` 数组在每个进程结构内内联。

## 性能特征

| 操作 | 复杂度 | 注意 |
|-----------|------------|-------|
| `take()` | O(P + T) | 单次系统调用，P=进程，T=线程 |
| `get_threads()` | O(T_p) | 延迟，T_p=进程中的线程 |
| `pid()` | O(1) | 直接字段访问 |
| `get_name()` | O(1) | 缓存字符串引用 |

## 与 ToolHelp 比较

| 特性 | ProcessSnapshot | ToolHelp32 |
|---------|-----------------|------------|
| 系统调用 | 1 | 1 + N（进程）+ M（线程） |
| 原子性 | 单次快照 | 可能出现竞争条件 |
| 内存 | 缓冲区大小（~1MB） | 最小 |
| 速度 | 更快 | 线程多则更慢 |
| 信息 | 完整的 SYSTEM_THREAD_INFORMATION | 有限 |

## 依赖

- `ntapi::ntexapi` - `NtQuerySystemInformation`、结构
- `std::collections::HashMap` - 进程/线程查找
- `std::slice` - UNICODE_STRING 解析

## 平台要求

- Windows XP 或更高版本
- 查询无需特殊特权
- 无提升时某些系统进程返回有限信息
