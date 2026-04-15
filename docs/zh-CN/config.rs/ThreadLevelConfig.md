# ThreadLevelConfig 类型 (config.rs)

`ThreadLevelConfig` 结构体保存单个进程规则的所有线程级调度设置。它定义了主线程 CPU 集合、模块前缀过滤器（带可选的每前缀 CPU 覆盖和线程优先级）、要跟踪的顶级线程数量，以及理想处理器分配规则。线程级配置与进程级配置分离，以便调度器可以在每次轮询迭代中独立管理每线程的 CPU 分配。

## 语法

```AffinityServiceRust/src/config.rs#L40-46
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
|------|------|------|
| `name` | `String` | 此规则适用的小写可执行文件名（例如 `"game.exe"`）。当同一进程同时存在进程级和线程级规则时，与对应的 [ProcessLevelConfig](ProcessLevelConfig.md) 条目匹配。 |
| `prime_threads_cpus` | `List<[u32; CONSUMER_CPUS]>` | 可用于主线程调度的 CPU 索引合集。当进程有 CPU 密集型线程时，调度器会将它们的亲和性设置到这些 CPU。这是配置中 `prime_cpus` 字段指定的所有每段 CPU 别名的并集。空列表表示不激活主线程调度（除非 `track_top_x_threads` 为负值，表示仅跟踪模式）。 |
| `prime_threads_prefixes` | `Vec<PrimePrefix>` | [PrimePrefix](PrimePrefix.md) 条目列表，根据线程启动模块名称过滤哪些线程有资格进行主线程调度。每个前缀可以可选地覆盖 CPU 集合和线程优先级。当列表只包含一个空前缀字符串的条目时，所有线程均有资格。 |
| `track_top_x_threads` | `i32` | 控制要跟踪和可选调度的顶级 CPU 消耗线程数量。**正值**（`?N`）：跟踪前 N 个线程并对其应用主线程调度。**负值**（`??N`）：仅出于监控目的跟踪前 N 个线程，不应用主线程调度。**零**：跟踪数量从 `prime_threads_cpus` 中的 CPU 数量推导。 |
| `ideal_processor_rules` | `Vec<IdealProcessorRule>` | [IdealProcessorRule](IdealProcessorRule.md) 条目列表，根据线程启动模块名称为线程分配理想处理器。调度器使用这些规则为每个符合条件的线程设置首选 CPU，按总 CPU 时间排名将前 N 个线程（N = 规则中的 CPU 数量）分配到指定的 CPU。 |

## 备注

### 与 ProcessLevelConfig 的关系

一条配置行可以同时为同一进程名称生成一个 [ProcessLevelConfig](ProcessLevelConfig.md) 和一个 `ThreadLevelConfig`。当任何进程级设置（优先级、亲和性、CPU 集合、I/O 优先级、内存优先级）非默认时，会创建进程级条目。当任何线程级设置（主线程 CPU、跟踪数量、理想处理器规则）非默认时，会创建线程级条目。这两种配置分别存储在 [ConfigResult](ConfigResult.md) 中按等级索引的独立 `HashMap` 集合中。

### 基于等级的调度

线程级配置按等级（轮询频率）组织。等级为 `1` 的规则在每次迭代中运行；等级为 `2` 的规则每隔一次迭代运行，以此类推。这允许开销较大的线程级分析以低于进程级设置的频率执行。

### 主线程调度流程

1. 调度器通过将 `name` 与运行中的进程可执行文件匹配来识别进程。
2. 它枚举进程的线程，并通过将每个线程的启动模块名称与 `prime_threads_prefixes` 匹配来进行过滤。
3. 线程按总 CPU 时间排名，选择前 N 个（N 由 `track_top_x_threads` 或 CPU 数量确定）。
4. 每个选中线程的亲和性被设置到 `prime_threads_cpus` 中的适当 CPU（或匹配的 `PrimePrefix` 中的每前缀 CPU 覆盖）。
5. 如果定义了 `ideal_processor_rules`，则使用类似的排名机制（按规则前缀过滤）单独应用理想处理器分配。

### track_top_x_threads 符号约定

| 值 | 配置语法 | 行为 |
|----|---------|------|
| 正值（`> 0`） | `?Nx*alias` | 跟踪前 N 个线程**并**应用主线程调度 |
| 负值（`< 0`） | `??N` | **仅**跟踪前 N 个线程用于监控（不应用主线程调度） |
| 零（`0`） | （默认） | 从 `prime_threads_cpus.len()` 推导跟踪数量 |

### 有效性检查

仅当以下条件至少满足一个时，`ThreadLevelConfig` 才会被插入到结果中：
- `prime_threads_cpus` 非空
- `track_top_x_threads` 非零
- `ideal_processor_rules` 非空

如果以上条件均不满足，即使配置行在其他方面有效，也不会创建线程级条目。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `config.rs` |
| 构造方式 | [parse_and_insert_rules](parse_and_insert_rules.md) |
| 存储位置 | [ConfigResult](ConfigResult.md)`.thread_level_configs` |
| 使用方 | `scheduler.rs`（主线程调度器）、`apply.rs` |
| 相关类型 | [PrimePrefix](PrimePrefix.md)、[IdealProcessorRule](IdealProcessorRule.md) |

## 另请参阅

| 资源 | 链接 |
|------|------|
| PrimePrefix | [PrimePrefix](PrimePrefix.md) |
| IdealProcessorRule | [IdealProcessorRule](IdealProcessorRule.md) |
| ProcessLevelConfig | [ProcessLevelConfig](ProcessLevelConfig.md) |
| ConfigResult | [ConfigResult](ConfigResult.md) |
| parse_and_insert_rules | [parse_and_insert_rules](parse_and_insert_rules.md) |
| scheduler 模块 | [scheduler.rs 概述](../scheduler.rs/README.md) |
| config 模块概述 | [README](README.md) |

---
*Commit: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)*