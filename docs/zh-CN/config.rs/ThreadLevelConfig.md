# ThreadLevelConfig 结构体 (config.rs)

每个进程的线程级配置，用于控制 Prime 线程调度和理想处理器分配。与 [ProcessLevelConfig](ProcessLevelConfig.md) 不同（后者在进程首次被看到时应用一次），`ThreadLevelConfig` 规则在每次轮询迭代期间进行评估，以跟踪线程活动并动态重新分配 CPU 资源。

## 语法

```rust
#[derive(Debug, Clone)]
pub struct ThreadLevelConfig {
    pub name: String,
    pub prime_threads_cpus: List<[u32; CONSUMER_CPUS]>,
    pub prime_threads_prefixes: Vec<PrimePrefix>,
    pub track_top_x_threads: i32,
    pub ideal_processor_rules: Vec<IdealProcessorRule>,
}
```

## 成员

| 成员 | 类型 | 描述 |
|--------|------|-------------|
| `name` | `String` | 小写进程名（例如 `"game.exe"`），用作线程级配置映射中的查找键。 |
| `prime_threads_cpus` | `List<[u32; CONSUMER_CPUS]>` | 有资格进行 Prime 线程绑定的所有 CPU 索引的并集。这是所有 [PrimePrefix](PrimePrefix.md) 条目的组合集。当线程被提升为 Prime 线程状态时，其 CPU 集合限制为此列表中的索引（或如果启用了前缀匹配，则为前缀特定的子集）。 |
| `prime_threads_prefixes` | `Vec<PrimePrefix>` | [PrimePrefix](PrimePrefix.md) 规则列表，控制哪些线程有资格进行 Prime 线程调度，以及每个前缀组获得哪些 CPU 子集。空前缀字符串匹配所有线程。多个条目允许将来自不同模块的线程路由到不同的 CPU 集合。 |
| `track_top_x_threads` | `i32` | 控制跟踪多少个顶级线程（按 CPU 周期消耗量）。正值启用前 N 个顶级线程的 Prime 线程调度。负值启用仅跟踪模式（收集指标但不进行 CPU 绑定）。零完全禁用线程跟踪。从 prime 字段中的 `?N`（正值）或 `??N`（负值）前缀解析。 |
| `ideal_processor_rules` | `Vec<IdealProcessorRule>` | [IdealProcessorRule](IdealProcessorRule.md) 条目列表，根据线程的起始模块前缀分配理想处理器提示。独立于 Prime 线程调度进行评估。 |

## 备注

### 与 ProcessLevelConfig 的关系

单行配置规则可以为同一进程生成 [ProcessLevelConfig](ProcessLevelConfig.md) 和 `ThreadLevelConfig`。[parse_and_insert_rules](parse_and_insert_rules.md) 函数确定线程级字段是否处于活动状态（非零主 CPUs、非零跟踪计数或非空理想处理器规则），并且仅当至少使用一个线程级功能时才创建 `ThreadLevelConfig`。

### Prime 线程调度

Prime 线程系统识别进程中 CPU 消耗最高的线程，并将它们固定到高性能核心。选择使用由 [ConfigConstants](ConfigConstants.md) 控制的迟滞，以避免频繁切换：

1. 每次迭代，按 CPU 周期增量对线程进行排名。
2. 相对份额超过 `entry_threshold` 的线程开始积累活动 streak。
3. 一旦线程的 streak 达到 `min_active_streak`，它会被提升为 Prime 线程状态并绑定到 `prime_threads_cpus`。
4. 仅当 Prime 线程的份额降至 `keep_threshold` 以下时才会被降级。

`track_top_x_threads` 字段限制参与此排名的线程数量。在具有许多线程的系统上，这避免了测量每个线程的周期。

### 仅跟踪模式

当 `track_top_x_threads` 为负数时（从 `??N` 语法解析），调度器收集线程周期统计信息并记录它们，但不执行任何 CPU 集合更改。这用于在承诺主配置之前分析线程行为。

### 基于前缀的 CPU 路由

`prime_threads_prefixes` 列表允许基于线程的起始模块将不同线程路由到不同的 CPU 子集。例如，游戏的渲染线程（从 `d3d11.dll` 开始）可以绑定到 P 核，而音频线程（从 `xaudio2.dll` 开始）可以进入 E 核。每个 [PrimePrefix](PrimePrefix.md) 还可以携带可选的 [ThreadPriority](../priority.rs/ThreadPriority.md) 提升。

### 理想处理器分配

`ideal_processor_rules` 字段独立于 Prime 线程调度运行。它为匹配特定模块前缀的线程设置理想处理器提示，Windows 调度器将其用作偏好（而非硬约束）。这是 Prime 线程绑定的轻量级替代方案。

### 配置字段格式

线程级设置从配置规则行的字段 4（Prime 线程）和字段 7（理想处理器）解析：

```
process.exe:priority:affinity:cpuset:prime_spec:io:memory:ideal_spec:grade
                                      ^字段 4                ^字段 7
```

Prime 线程规格支持几种形式：
- `*alias` — 将 Prime 线程固定到别名 CPUs
- `?8x*alias` — 跟踪前 8 个，固定到别名
- `??16` — 跟踪前 16 个，无绑定
- `*p@engine.dll;render.dll*e@audio.dll` — 基于前缀的路由，带有每前缀的 CPU 集合

### 存储结构

`ThreadLevelConfig` 实例存储在 `ConfigResult.thread_level_configs` 中，这是一个 `HashMap<u32, HashMap<String, ThreadLevelConfig>>`，其中外层键是等级（轮询频率层），内层键是小写进程名。

## 需求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **由...构建** | [parse_and_insert_rules](parse_and_insert_rules.md) |
| **由...使用** | [apply_prime_threads](../apply.rs/apply_prime_threads.md), [apply_ideal_processors](../apply.rs/apply_ideal_processors.md), 主轮询循环 |
| **依赖项** | [PrimePrefix](PrimePrefix.md), [IdealProcessorRule](IdealProcessorRule.md), [List](../collections.rs/README.md) |
| **权限** | 无（数据结构） |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [config.rs](README.md) |
| 进程级对应物 | [ProcessLevelConfig](ProcessLevelConfig.md) |
| Prime 线程前缀规则 | [PrimePrefix](PrimePrefix.md) |
| 理想处理器规则 | [IdealProcessorRule](IdealProcessorRule.md) |
| 迟滞常量 | [ConfigConstants](ConfigConstants.md) |
| Prime 线程调度器状态 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| 规则解析器 | [parse_and_insert_rules](parse_and_insert_rules.md) |
| 配置文件读取器 | [read_config](read_config.md) |

*文档针对 Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*