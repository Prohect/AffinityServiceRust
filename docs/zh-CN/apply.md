# Apply 模块文档

将配置设置应用到目标进程。

## 概述

本模块实现了应用进程配置的核心逻辑：
- 进程优先级类
- CPU 亲和性（硬掩码）
- CPU 集（软偏好）
- I/O 优先级
- 内存优先级
- Prime 线程调度
- 理想处理器分配

## 调用者

- `main.rs` 中的 `apply_config()` - 主编排函数
- Apply 函数之间的内部交叉调用

## 数据结构

### ApplyConfigResult

收集配置应用过程中的更改和错误。

```rust
pub struct ApplyConfigResult {
    pub changes: Vec<String>,   // 人类可读的更改描述
    pub errors: Vec<String>,    // 带上下文的错误消息
}
```

由 main.rs 中的 [`apply_config()`](main.md#apply_config-function) 返回。

**方法：**
- `new()` - 创建空结果
- `add_change(change: String)` - 添加更改消息（格式：`"$operation details"`）
- `add_error(error: String)` - 添加错误消息（格式：`"$fn_name: [$operation][$error] details"`）
- `is_empty() -> bool` - 检查是否有更改或错误

## Apply 函数

### apply_config (位于 main.rs)

编排将配置设置应用到目标进程的所有操作。

**被调用者：**[main.rs](main.md#main-loop) 主循环

**流程：**
1. 获取进程句柄
2. 应用优先级
3. 应用亲和性（捕获 current_mask 用于过滤）
4. 应用 CPU 集
5. 应用 I/O 优先级
6. 应用内存优先级
7. 如果启用了 prime/ideal/跟踪：
   - 释放模块缓存
   - 在调度器中设置存活状态
   - 预取线程周期
   - 应用 prime 线程
   - 应用理想处理器
   - 更新线程统计

### apply_priority

设置进程优先级类。

```rust
pub fn apply_priority(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
)
```

**参数：**
- `config`: [`ProcessConfig`](config.md#processconfig)
- `process_handle`: [`ProcessHandle`](winapi.md#processhandle)
- `apply_config_result`: [`ApplyConfigResult`](#applyconfigresult)

**Windows API:** `GetPriorityClass`, `SetPriorityClass`

**更改日志：**`"Priority: {old} -> {new}"`

### apply_affinity

设置硬 CPU 亲和性掩码。

```rust
pub fn apply_affinity(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    current_mask: &mut usize,  // 输出：填充当前掩码
    process_handle: &ProcessHandle,
    process: &mut ProcessEntry,
    apply_config_result: &mut ApplyConfigResult,
)
```

**参数：**
- `config`: [`ProcessConfig`](config.md#processconfig)
- `process_handle`: [`ProcessHandle`](winapi.md#processhandle)
- `apply_config_result`: [`ApplyConfigResult`](#applyconfigresult)
- `process`: [`ProcessEntry`](process.md#processentry)

**副作用：**`current_mask` 被填充为进程的当前亲和性掩码

**Windows API:** `GetProcessAffinityMask`, `SetProcessAffinityMask`

**更改日志：**`"Affinity: {old:#X} -> {new:#X}"`

**后续操作：**如果亲和性更改，调用 `reset_thread_ideal_processors()`

### reset_thread_ideal_processors

重置所有线程的理想处理器。

```rust
pub fn reset_thread_ideal_processors(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    cpus: &[u32],
    process: &mut ProcessEntry,
    apply_config_result: &mut ApplyConfigResult,
)
```

**参数：**
- `config`: [`ProcessConfig`](config.md#processconfig)
- `process`: [`ProcessEntry`](process.md#processentry)
- `apply_config_result`: [`ApplyConfigResult`](#applyconfigresult)

**目的：**在指定的 CPU 上重新分配线程理想处理器。在亲和性更改后或 CPU 集更改时（当 `cpu_set_reset_ideal` 启用时）使用。

**参数：**
- `cpus` - 用于分配线程理想处理器的 CPU 索引集合

**算法：**
1. 按总 CPU 时间（内核 + 用户）对线程排序
2. 在指定的 CPU 上轮询分配理想处理器
3. 应用随机偏移以避免聚集
4. 如果已在目标 CPU 上则跳过分配（延迟设置）

**被调用者：**
- `apply_affinity()` - 亲和性更改后（传递 `&config.affinity_cpus`）
- `main.rs` 中的 `apply_config()` - CPU 集更改后当 `config.cpu_set_reset_ideal` 为 true 时（传递 `&config.cpu_set_cpus`）

**Windows API:** `OpenThread`, `SetThreadIdealProcessorEx`

### apply_process_default_cpuset

通过 CPU 集设置软 CPU 偏好。

```rust
pub fn apply_process_default_cpuset(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    process: &mut ProcessEntry,
    apply_config_result: &mut ApplyConfigResult,
)
```

**参数：**
- `config`: [`ProcessConfig`](config.md#processconfig)
- `process_handle`: [`ProcessHandle`](winapi.md#processhandle)
- `process`: [`ProcessEntry`](process.md#processentry)
- `apply_config_result`: [`ApplyConfigResult`](#applyconfigresult)

**Windows API:** `GetProcessDefaultCpuSets`, `SetProcessDefaultCpuSets`

**注意：**查询最初可能失败并返回错误 122（INSUFFICIENT_BUFFER）- 这是预期的。

**后续操作：**如果 `config.cpu_set_reset_ideal` 为 true，调用者应该使用 `&config.cpu_set_cpus` 调用 `reset_thread_ideal_processors()` 以重新分配线程理想处理器。

### apply_io_priority

设置 I/O 优先级。

```rust
pub fn apply_io_priority(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
)
```

**参数：**
- `config`: [`ProcessConfig`](config.md#processconfig)
- `process_handle`: [`ProcessHandle`](winapi.md#processhandle)
- `apply_config_result`: [`ApplyConfigResult`](#applyconfigresult)

**Windows API:** `NtQueryInformationProcess`, `NtSetInformationProcess` (class 33)

**特权：**I/O 优先级 "high" 需要 `SeIncreaseBasePriorityPrivilege` + 管理员

**更改日志：**`"IO Priority: {old} -> {new}"`

### apply_memory_priority

设置内存页面优先级。

```rust
pub fn apply_memory_priority(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
)
```

**参数：**
- `config`: [`ProcessConfig`](config.md#processconfig)
- `process_handle`: [`ProcessHandle`](winapi.md#processhandle)
- `apply_config_result`: [`ApplyConfigResult`](#applyconfigresult)

**Windows API:** `GetProcessInformation`, `SetProcessInformation` (`ProcessMemoryPriority`)

**更改日志：**`"Memory Priority: {old} -> {new}"`

## Prime 线程调度

### prefetch_all_thread_cycles

预取线程周期计数以进行 prime 线程选择。

```rust
pub fn prefetch_all_thread_cycles(
    pid: u32,
    config: &ProcessConfig,
    process: &mut ProcessEntry,
    prime_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

**参数：**
- `config`: [`ProcessConfig`](config.md#processconfig)
- `process`: [`ProcessEntry`](process.md#processentry)
- `prime_scheduler`: [`PrimeThreadScheduler`](scheduler.md#primethreadscheduler)
- `apply_config_result`: [`ApplyConfigResult`](#applyconfigresult)

**算法：**
1. 获取按 CPU 时间增量排序的线程
2. 保留前 N 个线程（N = 逻辑 CPU 数 × 2）
3. 为没有缓存句柄的线程打开句柄
4. 为每个线程查询 `QueryThreadCycleTime`
5. 计算周期增量
6. 在调度器中更新活跃连击

**优化：**仅为可能被选中的线程打开句柄（按 CPU 时间排名前）。

**Windows API:** `OpenThread`, `QueryThreadCycleTime`

### apply_prime_threads

Prime 线程调度的主编排函数。

```rust
pub fn apply_prime_threads(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    current_mask: &mut usize,
    process: &mut ProcessEntry,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

**参数：**
- `config`: [`ProcessConfig`](config.md#processconfig)
- `prime_core_scheduler`: [`PrimeThreadScheduler`](scheduler.md#primethreadscheduler)
- `process`: [`ProcessEntry`](process.md#processentry)
- `apply_config_result`: [`ApplyConfigResult`](#applyconfigresult)

**算法：**
1. 如果启用，设置跟踪信息
2. 按 CPU 时间增量对线程排序
3. 选择候选池（4× prime 槽位数或 CPU 数）
4. 包括之前固定的线程以进行降级检查
5. 获取候选的周期增量
6. **选择：**应用滞后以选择 prime 线程
7. **提升：**分配 CPU 集并提升优先级
8. **降级：**移除 CPU 集并恢复优先级
9. 清理已退出线程的句柄

**Windows API:** `SetThreadSelectedCpuSets`, `SetThreadPriority`

### apply_prime_threads_select

使用滞后选择顶部线程。

```rust
pub fn apply_prime_threads_select(
    pid: u32,
    prime_count: usize,
    tid_with_delta_cycles: &mut [(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
)
```

使用 `|ts| !ts.pinned_cpu_set_ids.is_empty()` 作为"当前已分配"检查调用 [`select_top_threads_with_hysteresis()`](scheduler.md#select_top_threads_with_hysteresis)。

### apply_prime_threads_promote

将选中的线程提升为 prime 状态。

```rust
pub fn apply_prime_threads_promote(
    pid: u32,
    config: &ProcessConfig,
    current_mask: &mut usize,
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

**参数：**
- `prime_core_scheduler`: [`PrimeThreadScheduler`](scheduler.md#primethreadscheduler)
- `config`: [`ProcessConfig`](config.md#processconfig)
- `apply_config_result`: [`ApplyConfigResult`](#applyconfigresult)

**对于每个选中的线程：**
1. 将起始地址解析为模块名
2. 匹配前缀规则
3. 按当前亲和性掩码过滤 CPU（如果已设置）
4. 应用 `SetThreadSelectedCpuSets`
5. 提升线程优先级（显式或自动）

**更改日志：**
- `"Thread {tid} -> (promoted, [{cpus}], cycles={cycles}, start={module})"`
- `"Thread {tid} -> ({action}: {old} -> {new})"` (优先级)

### apply_prime_threads_demote

降级不再符合条件的线程。

```rust
pub fn apply_prime_threads_demote(
    pid: u32,
    config: &ProcessConfig,
    process: &mut ProcessEntry,
    tid_with_delta_cycles: &[(u32, u64, bool)],
    prime_core_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

**参数：**
- `process`: [`ProcessEntry`](process.md#processentry)
- `prime_core_scheduler`: [`PrimeThreadScheduler`](scheduler.md#primethreadscheduler)
- `config`: [`ProcessConfig`](config.md#processconfig)
- `apply_config_result`: [`ApplyConfigResult`](#applyconfigresult)

**对于每个不在选中集中但有固定 CPU 的线程：**
1. 清除 CPU 集分配（使用空值调用 `SetThreadSelectedCpuSets`）
2. 恢复原始线程优先级
3. **始终清除**`pinned_cpu_set_ids` 以防止重试循环

**更改日志：**`"Thread {tid} -> (demoted, start={module})"`

## 理想处理器分配

### apply_ideal_processors

根据起始模块为线程分配理想处理器。

```rust
pub fn apply_ideal_processors(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process: &mut ProcessEntry,
    prime_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

**参数：**
- `config`: [`ProcessConfig`](config.md#processconfig)
- `process`: [`ProcessEntry`](process.md#processentry)
- `prime_scheduler`: [`PrimeThreadScheduler`](scheduler.md#primethreadscheduler)
- `apply_config_result`: [`ApplyConfigResult`](#applyconfigresult)

**每条规则的算法：**
1. 按模块前缀过滤线程（如果指定）
2. 按周期计数排序
3. 使用滞后选择前 N 个（与 prime 调度相同）
4. 对于选中的线程：
   - 如果不在空闲池 CPU 上，设置理想处理器
   - 在 `IdealProcessorState` 中跟踪分配
5. 对于降级的线程：
   - 恢复原始理想处理器

**延迟设置优化：**如果线程当前理想处理器在空闲池中，跳过系统调用。

**Windows API:** `SetThreadIdealProcessorEx`

**更改日志：**包括 `start=module+offset`（例如 `start=cs2.exe+0xEA60`）

## 辅助函数

### get_handles

从 ProcessHandle 中提取读写句柄。

```rust
#[inline(always)]
fn get_handles(process_handle: &ProcessHandle) -> (Option<HANDLE>, Option<HANDLE>)
```

优先使用完整句柄而非受限句柄。

### log_error_if_new

仅当对此 pid/操作组合是新的错误时才记录。

```rust
#[inline(always)]
fn log_error_if_new(
    pid: u32,
    process_name: &str,
    operation: Operation,
    error_code: u32,
    apply_config_result: &mut ApplyConfigResult,
    format_msg: impl FnOnce() -> String,
)
```

使用 `logging::is_new_error()` 进行去重。

## 错误处理

所有函数使用 `log_error_if_new()` 防止日志垃圾：
- 访问被拒绝错误每个 (pid, 进程名, 操作) 只记录一次
- 无效句柄错误单独跟踪
- 每次循环清除死 PID 的错误映射

## 依赖

- `crate::config` - 配置结构和 CPU 工具
- `crate::error_codes` - 错误代码转换
- `crate::logging` - 错误跟踪和日志
- `crate::priority` - 优先级枚举
- `crate::process` - 进程和线程枚举
- `crate::scheduler` - Prime 线程调度器
- `crate::winapi` - Windows API 包装器
- `rand` - 理想处理器重置的随机偏移
- `windows` - Win32 API
