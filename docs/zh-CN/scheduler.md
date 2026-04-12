# Scheduler 模块文档

具有基于滞后的提升/降级的 Prime 线程调度器。

## 概述

`PrimeThreadScheduler` 管理 CPU 密集型线程的动态线程到 CPU 分配。它使用：
- 每线程 CPU 周期跟踪
- 滞后以防止抖动（进入 vs 保持阈值）
- 活跃连击计数以过滤短暂活跃的线程
- 用于细粒度线程放置的 CPU 集

## 调用者

- `apply_prime_threads()` 在 [apply.rs](apply.md#prime-thread-scheduling) - 主调度入口点
- `apply_ideal_processors()` 在 [apply.rs](apply.md#ideal-processor-assignment) - 理想处理器分配
- `prefetch_all_thread_cycles()` 在 [apply.rs](apply.md#prefetch_all_thread_cycles) - 周期数据收集
- [main.rs](main.md) - 进程生命周期管理

## 数据结构

### PrimeThreadScheduler

```rust
pub struct PrimeThreadScheduler {
    pub pid_to_process_stats: HashMap<u32, ProcessStats>,
    pub constants: ConfigConstants,
}
```

**字段：**
- `pid_to_process_stats` - 每进程线程统计 ([`ProcessStats`](#processstats))
- `constants` - 来自配置的滞后阈值 ([`ConfigConstants`](config.md#configconstants))

### ProcessStats

每进程跟踪数据。

```rust
pub struct ProcessStats {
    pub alive: bool,                          // 进程仍在运行
    pub tid_to_thread_stats: HashMap<u32, ThreadStats>,
    pub track_top_x_threads: i32,             // 跟踪模式（0=关, >0=跟踪+prime, <0=仅跟踪）
    pub process_name: String,
    pub process_id: u32,
}
```

### ThreadStats

每线程跟踪数据。

```rust
pub struct ThreadStats {
    pub last_total_time: i64,                 // 之前的总 CPU 时间
    pub cached_total_time: i64,               // 当前总 CPU 时间（来自系统信息）
    pub last_cycles: u64,                     // 之前的周期计数
    pub cached_cycles: u64,                   // 当前周期计数（来自 QueryThreadCycleTime）
    pub handle: Option<ThreadHandle>,         // 线程句柄容器（缓存）
    pub pinned_cpu_set_ids: Vec<u32>,         // 当前分配的 CPU 集 ID
    pub active_streak: u8,                    // 连续活跃间隔
    pub start_address: usize,                 // 线程起始地址（用于模块标识）
    pub original_priority: Option<ThreadPriority>, // 提升前的优先级
    pub last_system_thread_info: Option<SYSTEM_THREAD_INFORMATION>, // 完整线程信息（跟踪模式）
    pub ideal_processor: IdealProcessorState, // 理想处理器分配状态
    pub process_id: u32,
}
```

**句柄使用：**
`handle` 字段包含一个 [`ThreadHandle`](winapi.md#threadhandle)，它提供多个访问级别：
- `r_limited_handle` - 句柄存在时始终有效
- `r_handle` - 使用前检查 `is_invalid()`
- `w_limited_handle` - 使用前检查 `is_invalid()`
- `w_handle` - 使用前检查 `is_invalid()`

**自动清理：**ThreadHandle 的 `Drop` 实现在句柄被移除或线程退出时自动关闭所有有效句柄。

**类型引用：**
- `original_priority`: [`ThreadPriority`](priority.md#threadpriority)
- `ideal_processor`: [`IdealProcessorState`](#idealprocessorstate)

### IdealProcessorState

跟踪滞后下的理想处理器分配。

```rust
pub struct IdealProcessorState {
    pub current_group: u16,
    pub current_number: u8,
    pub previous_group: u16,
    pub previous_number: u8,
    pub is_assigned: bool,
}
```

## 滞后算法

调度器使用两阶段滞后防止线程提升/降级抖动：

### 阈值

- `entry_threshold`（默认 0.42）- 成为候选的最小最大周期百分比
- `keep_threshold`（默认 0.69）- 保持 prime 的最小最大周期百分比（高于进入）
- `min_active_streak`（默认 2）- 提升前连续活跃间隔

### 两阶段选择

**阶段 1（保持）：**当前分配的线程如果周期 >= keep_threshold% 的最大值则保持分配。

**阶段 2（提升）：**新线程被提升如果：
- 周期 >= entry_threshold% 的最大值
- 活跃连击 >= min_active_streak

这在进入和保持阈值之间创建了"死区"，其中：
- 非 prime 线程不会被提升
- Prime 线程不会被降级
- 防止接近阈值时的振荡

## 方法

### new

使用给定常量创建调度器。

```rust
pub fn new(constants: ConfigConstants) -> Self
```

### reset_alive / set_alive

进程存在性的生命周期跟踪。

```rust
pub fn reset_alive(&mut self)  // 将所有标记为死亡（循环开始）
pub fn set_alive(&mut self, pid: u32)  // 将特定进程标记为存活
```

### set_tracking_info

为进程配置线程跟踪。

```rust
pub fn set_tracking_info(&mut self, pid: u32, track_top_x_threads: i32, process_name: String)
```

**参数：**
- `track_top_x_threads` - 要跟踪的顶部线程数：
  - `0` - 不跟踪
  - `>0` - 跟踪并在进程退出时记录前 N 个
  - `<0` 相同但不进行 prime 调度（仅监控）

### get_thread_stats

获取或创建线程统计条目。

```rust
#[inline]
pub fn get_thread_stats(&mut self, pid: u32, tid: u32) -> &mut ThreadStats
```

### update_active_streaks

基于周期增量更新活跃连击计数器。

```rust
pub fn update_active_streaks(&mut self, pid: u32, tid_with_delta_cycles: &[(u32, u64)])
```

**算法：**
1. 查找所有线程中的最大周期
2. 计算 entry_min = max * entry_threshold
3. 计算 keep_min = max * keep_threshold
4. 对于每个线程：
   - 如果已有连击（>0）且周期 < keep_min → 重置连击
   - 如果无连击且周期 >= entry_min → 设置连击为 1
   - 如果有连击且周期 >= keep_min → 增加连击（上限 254）

### select_top_threads_with_hysteresis

核心滞后选择算法。

```rust
pub fn select_top_threads_with_hysteresis(
    &mut self,
    pid: u32,
    tid_with_delta_cycles: &mut [(u32, u64, bool)],  // (tid, cycles, is_selected)
    slot_count: usize,
    is_currently_assigned: fn(&ThreadStats) -> bool,
) -> usize
```

**返回：**选择的线程数

**参数：**
- `tid_with_delta_cycles` - 线程数据，第三个元素是输出标志
- `slot_count` - 最多选择的线程数
- `is_currently_assigned` - 检查线程是否已有资源的回调

**使用示例：**
```rust
// 用于 prime 调度
select_top_threads_with_hysteresis(pid, &mut candidates, prime_count, |ts| {
    !ts.pinned_cpu_set_ids.is_empty()  // 当前已分配 CPU 集
});

// 用于理想处理器
select_top_threads_with_hysteresis(pid, &mut candidates, cpu_count, |ts| {
    ts.ideal_processor.is_assigned  // 当前已分配理想处理器
});
```

### close_dead_process_handles

退出进程的清理。

```rust
pub fn close_dead_process_handles(&mut self)
```

**副作用：**
- 如果启用了跟踪，记录前 N 个线程
- 清除进程的模块缓存
- 从映射中移除进程统计

**句柄清理：**当统计从映射中移除时，线程句柄由 `ThreadHandle` 的 `Drop` 实现自动关闭。

**跟踪输出格式：**
```
Process name.exe (PID) exited. Top X threads by CPU cycles:
  [1] TID: 1234 | Cycles: 1234567890 | StartAddress: 0x7FF12345 (module.dll+0xABC)
    KernelTime: 1.234 s
    UserTime: 5.678 s
    CreateTime: 2024-01-15 10:30:45.123
    WaitTime: 0
    ClientId: PID 1234, TID 1234
    Priority: 8
    BasePriority: 8
    ContextSwitches: 12345
    ThreadState: 5
    WaitReason: 0
```

## 线程跟踪输出

跟踪的进程退出时，记录详细统计：

| 字段 | 描述 |
|-------|-------------|
| TID | 线程 ID |
| Cycles | 消耗的总 CPU 周期 |
| StartAddress | 解析为 `module.dll+offset` 格式 |
| KernelTime | 内核模式时间 |
| UserTime | 用户模式时间 |
| CreateTime | 线程创建时间戳 |
| WaitTime | 等待时间 |
| ClientId | PID 和 TID |
| Priority | 当前线程优先级 |
| BasePriority | 基础优先级 |
| ContextSwitches | 上下文切换次数 |
| ThreadState | 当前状态 |
| WaitReason | 如果等待则为等待原因 |

## 依赖

- `crate::config::ConfigConstants` - 阈值值
- `crate::logging::log_message` - 输出
- `crate::priority::ThreadPriority` - 优先级枚举
- `crate::winapi` - 模块解析
- `chrono` - 时间格式化
- `ntapi` - 系统线程信息
- `windows` - Win32 API

## 算法详情

### 为什么使用滞后？

没有滞后，接近阈值的线程会不断在 prime 和非 prime 状态之间翻转，导致：
- 过多的 CPU 集分配
- 缓存抖动
- 日志垃圾

进入（42%）和保持（69%）阈值之间的差距确保：
- 线程必须明显超过阈值才能进入
- 线程必须明显低于阈值才能退出
- 短暂波动不会导致更改

### 活跃连击

连击计数器防止仅短暂活跃的线程被提升：
- 线程必须连续 N 个间隔高于进入阈值
- 默认 N=2 意味着至少 2 * interval_ms 的持续活动
- 低于保持阈值时连击立即重置
