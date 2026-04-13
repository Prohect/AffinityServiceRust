# ProcessConfig 结构体 (config.rs)

保存单个进程规则的完整已解析配置。每个字段对应配置文件规则行中一个以冒号分隔的段。服务循环通过进程名称（不区分大小写）匹配正在运行的进程，并通过 Windows API 应用此结构体所描述的设置。

## 语法

```rust
#[derive(Debug, Clone)]
pub struct ProcessConfig {
    pub name: String,
    pub priority: ProcessPriority,
    pub affinity_cpus: Vec<u32>,
    pub cpu_set_cpus: Vec<u32>,
    pub cpu_set_reset_ideal: bool,
    pub prime_threads_cpus: Vec<u32>,
    pub prime_threads_prefixes: Vec<PrimePrefix>,
    pub track_top_x_threads: i32,
    pub io_priority: IOPriority,
    pub memory_priority: MemoryPriority,
    pub ideal_processor_rules: Vec<IdealProcessorRule>,
}
```

## 成员

| 成员 | 类型 | 描述 |
|------|------|------|
| `name` | `String` | 小写的进程映像名称（例如 `"game.exe"`）。用作匹配正在运行进程时的查找键。 |
| `priority` | `ProcessPriority` | 要应用的 Windows 进程优先级类。`ProcessPriority::None` 表示不更改。对应规则字段 1（例如 `high`、`above normal`、`none`）。 |
| `affinity_cpus` | `Vec<u32>` | 用于进程 CPU 亲和性掩码的已排序 CPU 索引列表。空向量表示不更改亲和性。通过 `SetProcessAffinityMask` 设置。对应规则字段 2。 |
| `cpu_set_cpus` | `Vec<u32>` | 用于进程默认 CPU 集的已排序 CPU 索引列表。空向量表示不更改 CPU 集。通过 `SetProcessDefaultCpuSets` 设置。对应规则字段 3。 |
| `cpu_set_reset_ideal` | `bool` | 当为 `true` 时，在应用 CPU 集后调用 `reset_thread_ideal_processors`，将线程理想处理器分配到 `cpu_set_cpus` 上。在配置规则中通过在 cpuset 字段值前加 `@` 前缀启用（例如 `@*alias` 或 `@0-7`）。 |
| `prime_threads_cpus` | `Vec<u32>` | 用于主线程固定的聚合 CPU 索引。这是主线程字段中引用的所有 CPU 集的并集。空向量表示禁用此进程的主线程调度。对应规则字段 4。 |
| `prime_threads_prefixes` | `Vec<PrimePrefix>` | 用于主线程选择的模块名称前缀过滤器列表。每个条目可以携带自己的 CPU 子集和线程优先级覆盖。当不存在 `@` 前缀规范时，包含一个前缀为空字符串且 `cpus: None` 的通配符条目。 |
| `track_top_x_threads` | `i32` | 控制线程跟踪粒度。正值（例如 `8`）跟踪并调度按周期增量排名前 N 的线程。负值（例如 `-16`）仅跟踪而不应用主线程调度（仅观察模式）。`0` 使用默认行为。通过主线程字段中的 `?N`（正值）或 `??N`（负值）前缀设置。 |
| `io_priority` | `IOPriority` | 通过 `NtSetInformationProcess` 应用的 I/O 优先级提示。`IOPriority::None` 表示不更改。对应规则字段 5。 |
| `memory_priority` | `MemoryPriority` | 通过 `SetProcessInformation` 应用的内存优先级。`MemoryPriority::None` 表示不更改。对应规则字段 6。 |
| `ideal_processor_rules` | `Vec<IdealProcessorRule>` | 从规则字段 7 解析的理想处理器分配规则列表。每条规则将一组 CPU 映射到可选的模块名称前缀过滤器。空向量表示不进行理想处理器管理。 |

## 备注

### 规则行格式

配置文件规则行采用以下冒号分隔的格式：

```
name:priority:affinity:cpuset:prime_cpus:io_priority:memory_priority:ideal_processor:grade
```

字段 1–2（`priority`、`affinity`）是必需的；所有后续字段都是可选的，省略时默认为 `None`/空/`0`/`1`。

### 等级系统

`grade` 字段（字段 8，默认为 `1`）不存储在 `ProcessConfig` 本身上。相反，它决定条目被放入 [ConfigResult](ConfigResult.md)`.configs` 中的哪个子映射。`configs` 字段是一个 `HashMap<u32, HashMap<String, ProcessConfig>>`，以等级为键，允许服务循环优先处理更高等级的规则。

### 组展开

当在配置文件中定义进程组时（使用 `{ member1: member2 }:rule` 语法），[parse_and_insert_rules](parse_and_insert_rules.md) 会为每个成员创建一个 `ProcessConfig` 克隆，每个克隆的 `name` 字段设置为单独的成员名称。

### 冗余规则检测

如果在解析新规则时，某个进程名称已存在于任何等级映射中，[parse_and_insert_rules](parse_and_insert_rules.md) 会递增冗余规则计数器并发出警告。之前的定义将被覆盖。

### CPU 别名解析

`affinity_cpus`、`cpu_set_cpus` 和 `prime_threads_cpus` 字段支持 CPU 别名引用（例如 `*perf`），在解析期间由 [resolve_cpu_spec](resolve_cpu_spec.md) 解析。解析后的整数索引存储在结构体中。

### 应用顺序

apply 模块按以下顺序处理每个进程的 `ProcessConfig` 字段：

1. **进程级别**（应用一次）：`priority` → `affinity_cpus` → `cpu_set_cpus`（+ 可选的 `cpu_set_reset_ideal`）→ `io_priority` → `memory_priority`
2. **线程级别**（每次循环迭代应用）：`prime_threads_cpus`/`prime_threads_prefixes` → `ideal_processor_rules`

### 默认实例

所有字段设置为 `None`/空/`0` 的 `ProcessConfig` 不会对匹配的进程应用任何更改。这等同于规则行 `name:none:0`。

## 要求

| | |
|---|---|
| **模块** | `config` (`src/config.rs`) |
| **构造者** | [parse_and_insert_rules](parse_and_insert_rules.md) |
| **存储于** | [ConfigResult](ConfigResult.md)`.configs` |
| **使用者** | [apply_config_process_level](../main.rs/apply_config_process_level.md)、[apply_config_thread_level](../main.rs/apply_config_thread_level.md)、[apply_priority](../apply.rs/apply_priority.md)、[apply_affinity](../apply.rs/apply_affinity.md)、[apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md)、[apply_io_priority](../apply.rs/apply_io_priority.md)、[apply_memory_priority](../apply.rs/apply_memory_priority.md)、[apply_prime_threads](../apply.rs/apply_prime_threads.md)、[apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |
| **依赖** | [PrimePrefix](PrimePrefix.md)、[IdealProcessorRule](IdealProcessorRule.md)、`ProcessPriority`、`IOPriority`、`MemoryPriority` |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 主线程前缀过滤器 | [PrimePrefix](PrimePrefix.md) |
| 理想处理器规则 | [IdealProcessorRule](IdealProcessorRule.md) |
| 规则字段解析 | [parse_and_insert_rules](parse_and_insert_rules.md) |
| 配置文件读取器 | [read_config](read_config.md) |
| 已解析配置聚合 | [ConfigResult](ConfigResult.md) |
| 进程优先级枚举 | [ProcessPriority](../priority.rs/ProcessPriority.md) |
| I/O 优先级枚举 | [IOPriority](../priority.rs/IOPriority.md) |
| 内存优先级枚举 | [MemoryPriority](../priority.rs/MemoryPriority.md) |
| CPU 规范解析 | [parse_cpu_spec](parse_cpu_spec.md) |
| CPU 别名解析 | [resolve_cpu_spec](resolve_cpu_spec.md) |