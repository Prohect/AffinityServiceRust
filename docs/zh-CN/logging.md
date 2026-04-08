# Logging 模块文档

日志基础设施和错误去重。

## 概述

本模块提供：
- 双路日志（控制台 vs 文件）
- 带日期戳的日志文件
- 错误去重以防止日志垃圾
- 用于进程发现的独立 `.find.log`

## 调用者

- 所有模块通过 `log!` 宏
- [main.rs](main.md) - 常规日志
- [apply.rs](apply.md) - 带去重的错误日志
- [winapi.rs](winapi.md) - Find 模式日志

## 宏

### log!

主要日志宏。

```rust
macro_rules! log {
    ($($arg:tt)*) => { ... }
}
```

**使用：**
```rust
log!("Process {} started", pid);
log!("Error: {}", e);
```

**输出格式：**`[HH:MM:SS] message`

**目标：**
- 如果 `use_console() == true` 则为控制台
- 否则为文件 `logs/YYYYMMDD.log`

**垃圾箱模式：**如果 `DUST_BIN_MODE` 为 true，则消息被抑制。

## 静态状态

### LOCALTIME_BUFFER

一致时间显示的共享时间戳。

```rust
pub static LOCALTIME_BUFFER: Lazy<Mutex<DateTime<Local>>>
```

**更新：**[main.rs](main.md#main-loop) 每次循环迭代

**目的：**确保同一次循环中的所有日志条目共享相同的时间戳

### FINDS_SET

`-find` 模式的去重集合。

```rust
static FINDS_SET: Lazy<Mutex<HashSet<String>>>
```

**目的：**防止每个会话多次记录相同的进程名。

### FINDS_FAIL_SET

跟踪 `-find` 模式中 `ACCESS_DENIED` 失败的进程。

```rust
pub static FINDS_FAIL_SET: Lazy<Mutex<HashSet<String>>>
```

**使用者：**`is_affinity_unset()` - 跳过重试已知失败的进程

### PID_MAP_FAIL_ENTRY_SET

错误去重映射。

```rust
static PID_MAP_FAIL_ENTRY_SET: Lazy<Mutex<HashMap<u32, HashMap<ApplyFailEntry, bool>>>>
```

结构：`pid → { (pid, name, operation, error_code) → alive }`

### DUST_BIN_MODE

防止 UAC 提升前记录日志。

```rust
pub static DUST_BIN_MODE: Lazy<Mutex<bool>>
```

**目的：**避免进程以提升重启时出现重复日志。

**设置者：**[main.rs](main.md) 最初设置，提升后清除。

### USE_CONSOLE

输出模式标志。

```rust
static USE_CONSOLE: Lazy<Mutex<bool>>
```

**设置者：**`-console` CLI 标志或 `-validate` 模式

### LOG_FILE / FIND_LOG_FILE

延迟初始化的日志文件句柄。

```rust
static LOG_FILE: Lazy<Mutex<File>>
static FIND_LOG_FILE: Lazy<Mutex<File>>
```

**路径：**
- `logs/YYYYMMDD.log` - 常规日志
- `logs/YYYYMMDD.find.log` - Find 模式日志

## 枚举

### Operation

跟踪去重操作。

```rust
pub enum Operation {
    OpenProcess2processQueryLimitedInformation,
    OpenProcess2processSetLimitedInformation,
    OpenProcess2processQueryInformation,
    OpenProcess2processSetInformation,
    OpenThread,
    SetPriorityClass,
    GetProcessAffinityMask,
    SetProcessAffinityMask,
    GetProcessDefaultCpuSets,
    SetProcessDefaultCpuSets,
    QueryThreadCycleTime,
    SetThreadSelectedCpuSets,
    SetThreadPriority,
    NtQueryInformationProcess2ProcessInformationIOPriority,
    NtSetInformationProcess2ProcessInformationIOPriority,
    GetProcessInformation2ProcessMemoryPriority,
    SetProcessInformation2ProcessMemoryPriority,
    SetThreadIdealProcessorEx,
    GetThreadIdealProcessorEx,
    InvalidHandle,
}
```

## 错误去重

### is_new_error

检查此错误是否之前未记录过。

```rust
pub fn is_new_error(
    pid: u32,
    process_name: &str,
    operation: Operation,
    error_code: u32
) -> bool
```

**返回：**如果第一次看到此 (pid, name, operation, code) 组合则返回 `true`

**算法：**
1. 创建 `ApplyFailEntry` 键
2. 检查此 PID 的映射中是否已存在条目
3. 如果条目存在但进程名不同，清除映射（PID 重用）
4. 如果条目存在 → 返回 `false`
5. 如果是新的 → 以 `alive=true` 插入 → 返回 `true`

**使用示例：**
```rust
if is_new_error(pid, name, Operation::SetPriorityClass, error_code) {
    log!("Failed to set priority: {}", error_code);
}
```

### purge_fail_map

从错误跟踪中移除陈旧条目。

```rust
pub fn purge_fail_map(pids_and_names: &[(u32, String)])
```

**算法：**
1. 将所有条目标记为死亡（`alive = false`）
2. 将当前运行的进程标记为存活
3. 移除所有死亡条目

**被调用者：**每次循环迭代进程枚举后的 [main.rs](main.md#main-loop)

**目的：**防止无限制增长并处理 PID 重用

## 日志函数

### log_message

主要日志函数（由 `log!` 宏使用）。

```rust
pub fn log_message(args: &str)
```

**检查：**
- 如果 `DUST_BIN_MODE` 为 true 则提前返回
- 使用 `LOCALTIME_BUFFER` 作为时间戳
- 基于 `USE_CONSOLE` 路由到控制台或文件

### log_pure_message

记录不带时间戳。

```rust
pub fn log_pure_message(args: &str)
```

**用于：**多行日志条目中的延续行。

### log_to_find

记录到 find 日志文件。

```rust
pub fn log_to_find(msg: &str)
```

**输出：**`[HH:MM:SS] message` 到 `.find.log`

**被调用者：**错误函数在 [`-find` 模式](main.md#-find-flag) 记录访问被拒绝时

### log_process_find

记录来自 `-find` 模式的发现进程（去重）。

```rust
pub fn log_process_find(process_name: &str)
```

**去重：**使用 `FINDS_SET` 每个会话只记录一次进程。

**输出：**`[HH:MM:SS] find process.exe`

**被调用者：**[main.rs](main.md#-find-flag) `-find` 模式

## 访问器

### use_console

返回控制台标志的引用。

```rust
pub fn use_console() -> &'static Mutex<bool>
```

### logger / find_logger

返回日志文件句柄的引用。

```rust
pub fn logger() -> &'static Mutex<File>
pub fn find_logger() -> &'static Mutex<File>
```

## 文件路径

### get_log_path

基于当前日期生成日志文件路径。

```rust
fn get_log_path(suffix: &str) -> PathBuf
```

**格式：**`logs/YYYYMMDD{suffix}.log`

**示例：**
- `logs/20240115.log`
- `logs/20240115.find.log`

**目录创建：**如果不存在则创建 `logs/`。

## 错误条目结构

### ApplyFailEntry

错误去重映射的键。

```rust
struct ApplyFailEntry {
    pid: u32,
    process_name: String,
    operation: Operation,
    error_code: u32,
}
```

**字段：**
- `operation`: [`Operation`](#operation) - 失败的操作

**相等性：**所有字段必须匹配条目才被视为相等。

**哈希：**派生自所有字段。

## 使用模式

### 标准日志

```rust
log!("Starting service with interval: {}ms", interval);
```

### 带去重的错误日志

```rust
let error_code = unsafe { GetLastError().0 };
if is_new_error(pid, name, Operation::SetAffinityMask, error_code) {
    log_to_find(&format!("Failed: {}", error_code));
}
```

### 多行输出

```rust
log_message(&format!("Process {}:", pid));
for line in details {
    log_pure_message(&format!("  {}", line));
}
```

### Find 模式日志

```rust
// 在主循环中
if !in_configs && !blacklist.contains(&name) {
    log_process_find(&name);
}
```

## 依赖

- `chrono` - 日期/时间处理
- `once_cell` - 延迟初始化
- `std::collections` - 去重用的 HashMap/HashSet
- `std::fs` - 文件操作
- `std::io` - Write trait

## 线程安全

所有公共函数通过 `Mutex` 保护是线程安全的：
- `LOCALTIME_BUFFER` - Mutex<DateTime>
- `FINDS_SET` - Mutex<HashSet>
- `FINDS_FAIL_SET` - Mutex<HashSet>
- `PID_MAP_FAIL_ENTRY_SET` - Mutex<HashMap>
- `USE_CONSOLE` - Mutex<bool>
- `DUST_BIN_MODE` - Mutex<bool>
- `LOG_FILE` - Mutex<File>
- `FIND_LOG_FILE` - Mutex<File>

## 文件轮换

日志文件基于日期：
- 每天创建新文件
- 无自动清理
- 文件在 `logs/` 目录中累积

## 性能考虑

- `log_message()` 获取多个锁 - 避免在热循环中使用
- `is_new_error()` 使用 HashMap - O(1) 查找
- `purge_fail_map()` 每次循环运行一次 - O(n) n 为错误数
- 文件写入由 OS 缓冲
