# ProcessLevelConfig 结构体 (config.rs)

表示单个 Windows 进程的完整进程级调优参数集。每个实例捕获所需的优先级类、CPU 亲和性掩码、CPU 集合分配、IO 优先级和内存优先级。当服务首次发现匹配的进程时，这些设置会被应用一次，除非配置被热重载，否则在随后的轮询迭代中不会重新应用。

## 语法

```rust
#[derive(Debug, Clone)]
pub struct ProcessLevelConfig {
    pub name: String,
    pub priority: ProcessPriority,
    pub affinity_cpus: List<[u32; CONSUMER_CPUS]>,
    pub cpu_set_cpus: List<[u32; CONSUMER_CPUS]>,
    pub cpu_set_reset_ideal: bool,
    pub io_priority: IOPriority,
    pub memory_priority: MemoryPriority,
}
```

## 成员

| 成员 | 类型 | 描述 |
|--------|------|-------------|
| `name` | `String` | 小写进程名（例如，`"game.exe"`），用作配置哈希表中的查找键。对运行的进程名进行不区分大小写的匹配。 |
| `priority` | [ProcessPriority](../priority.rs/ProcessPriority.md) | 进程的期望 Windows 优先级类。当设置为 `ProcessPriority::None` 时，服务不会修改进程的优先级。有效值包括 `Idle`、`BelowNormal`、`Normal`、`AboveNormal`、`High` 和 `RealTime`。 |
| `affinity_cpus` | `List<[u32; CONSUMER_CPUS]>` | 定义硬 CPU 亲和性掩码的逻辑处理器索引排序列表。空列表表示不应用亲和性更改。设置后，服务调用 `SetProcessAffinityMask`，并立即在新 CPU 集合上重新分布线程理想处理器。 |
| `cpu_set_cpus` | `List<[u32; CONSUMER_CPUS]>` | 进程默认 CPU 集合（软亲和性）的逻辑处理器索引排序列表。空列表表示不更改 CPU 集合。使用 Windows CPU 集合 API（`SetProcessDefaultCpuSets`），它提供比硬亲和性更柔和的调度提示。 |
| `cpu_set_reset_ideal` | `bool` | 当为 `true` 时，在应用 CPU 集合后重置所有线程理想处理器分配。通过在配置文件中用 `@` 前缀前缀 CPU 集合规格来触发（例如，`@*ecore`）。当将 CPU 集合与理想处理器规则结合使用时非常有用，以确保在 CPU 集合更改后重新分布线程。 |
| `io_priority` | [IOPriority](../priority.rs/IOPriority.md) | 进程的期望 IO 优先级。通过 `NtSetInformationProcess` 使用 `ProcessIoPriority` 设置。当设置为 `IOPriority::None` 时，不应用 IO 优先级更改。有效值包括 `VeryLow`、`Low`、`Normal` 和 `High`。 |
| `memory_priority` | [MemoryPriority](../priority.rs/MemoryPriority.md) | 进程的期望内存优先级。通过 `SetProcessInformation` 使用 `ProcessMemoryPriority` 设置。当设置为 `MemoryPriority::None` 时，不应用内存优先级更改。有效值包括 `VeryLow`、`Low`、`Medium`、`BelowNormal` 和 `Normal`。 |

## 备注

### 配置文件格式

`ProcessLevelConfig` 由具有以下位置字段的配置规则行构建：

```
process.exe:priority:affinity:cpuset:prime:io_priority:memory_priority:ideal:grade
```

进程名之后的前两个字段（`priority` 和 `affinity`）是必需的。省略的字段默认为其"不更改"值（`None`/空列表/`false`）。

### 解析时的验证

[parse_and_insert_rules](parse_and_insert_rules.md) 函数仅在至少有一个进程级字段具有非默认值时创建 `ProcessLevelConfig`。如果所有进程级字段都处于默认值（例如，规则仅指定线程级设置如主 CPU），则不插入 `ProcessLevelConfig`，只创建 [ThreadLevelConfig](ThreadLevelConfig.md)。

### 亲和性与 CPU 集合

`affinity_cpus` 和 `cpu_set_cpus` 都控制进程可以使用哪些处理器，但它们在强制执行方面有所不同：

| 功能 | 亲和性 (`affinity_cpus`) | CPU 集合 (`cpu_set_cpus`) |
|---------|---------------------------|--------------------------|
| **API** | `SetProcessAffinityMask` | `SetProcessDefaultCpuSets` |
| **强制执行** | 硬——线程不能在被排除的 CPU 上运行 | 软——调度器偏好列出的 CPU，但可能会溢出 |
| **范围** | 限制整个进程 | 设置默认提示；单个线程可以覆盖 |
| **最大 CPU** | 64（单个处理器组） | 支持跨处理器组的 >64 CPU |

### 基于等级的组织

`ProcessLevelConfig` 实例存储在 [ConfigResult](ConfigResult.md) 内的两级 `HashMap<u32, HashMap<String, ProcessLevelConfig>>` 中。外层键是*等级*（应用频率），内层键是进程名。等级 1 规则在每个轮询迭代中检查；更高等级的规则检查频率较低，减少低优先级进程的开销。

### 冗余检测

如果多个规则为同一进程名定义 `ProcessLevelConfig`，解析器会发出警告，后面的定义覆盖前面的定义。`ConfigResult` 中的 `redundant_rules_count` 计数器跟踪发生了多少次此类覆盖。

## 需求

| | |
|---|---|
| **模块** | [`src/config.rs`](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/config.rs) |
| **创建者** | [parse_and_insert_rules](parse_and_insert_rules.md) |
| **使用者** | [apply_priority](../apply.rs/apply_priority.md), [apply_affinity](../apply.rs/apply_affinity.md), [apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md), [apply_io_priority](../apply.rs/apply_io_priority.md), [apply_memory_priority](../apply.rs/apply_memory_priority.md) |
| **依赖项** | [ProcessPriority](../priority.rs/ProcessPriority.md), [IOPriority](../priority.rs/IOPriority.md), [MemoryPriority](../priority.rs/MemoryPriority.md), [List](../collections.rs/README.md) |
| **权限** | `PROCESS_SET_INFORMATION`（写入）, `PROCESS_QUERY_LIMITED_INFORMATION`（读取） |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [config.rs](README.md) |
| 线程级对应项 | [ThreadLevelConfig](ThreadLevelConfig.md) |
| 规则解析器 | [parse_and_insert_rules](parse_and_insert_rules.md) |
| 配置文件读取器 | [read_config](read_config.md) |
| 聚合解析结果 | [ConfigResult](ConfigResult.md) |
| CPU 规格解析器 | [parse_cpu_spec](parse_cpu_spec.md) |
| CPU 别名解析 | [resolve_cpu_spec](resolve_cpu_spec.md) |
| 优先级枚举 | [ProcessPriority](../priority.rs/ProcessPriority.md), [IOPriority](../priority.rs/IOPriority.md), [MemoryPriority](../priority.rs/MemoryPriority.md) |

*文档为 Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*