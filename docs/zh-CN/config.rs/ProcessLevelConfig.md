# ProcessLevelConfig 类型 (config.rs)

`ProcessLevelConfig` 结构体保存单个进程规则的所有进程级设置。每个实例代表 AffinityServiceRust 将应用于匹配进程的完整 OS 级属性集，包括优先级类别、硬 CPU 亲和性、软 CPU 集合偏好、I/O 优先级和内存优先级。

## 语法

```AffinityServiceRust/src/config.rs#L30-38
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
|------|------|------|
| `name` | `String` | 此规则匹配的小写可执行文件名称（例如 `"game.exe"`）。这是在 `ConfigResult.process_level_configs` 哈希映射中用于查找的键。 |
| `priority` | `ProcessPriority` | 要应用的 Windows 进程优先级类别（例如 `Idle`、`BelowNormal`、`Normal`、`AboveNormal`、`High`、`RealTime`）。值为 `ProcessPriority::None` 表示不改变优先级。 |
| `affinity_cpus` | `List<[u32; CONSUMER_CPUS]>` | 构成进程硬亲和性掩码的已排序 CPU 索引列表。非空时，进程（及其子线程）通过 `SetProcessAffinityMask` 被限制在这些逻辑处理器上。空列表表示不应用亲和性更改。 |
| `cpu_set_cpus` | `List<[u32; CONSUMER_CPUS]>` | 通过 Windows CPU 集合 API（`SetProcessDefaultCpuSets`）定义软 CPU 偏好的已排序 CPU 索引列表。与硬亲和性不同，CPU 集合是建议性的——操作系统在高负载下可能会将线程调度到其他核心。空列表表示未配置 CPU 集合。 |
| `cpu_set_reset_ideal` | `bool` | 当为 `true` 时，在应用 CPU 集合后，进程中所有线程的理想处理器分配将被重置。这通过在配置文件的 cpuset 字段前添加 `@` 前缀触发（例如 `@*ecore`）。 |
| `io_priority` | `IOPriority` | 要应用于进程的 I/O 优先级级别（例如 `VeryLow`、`Low`、`Normal`、`High`）。`IOPriority::None` 表示不更改。注意 `High` 通常需要管理员权限。 |
| `memory_priority` | `MemoryPriority` | 要应用于进程的内存页面优先级（例如 `VeryLow`、`Low`、`Medium`、`BelowNormal`、`Normal`）。`MemoryPriority::None` 表示不更改。较低的内存优先级会导致工作集管理器更积极地回收进程的页面。 |

## 备注

- `ProcessLevelConfig` 实例由 [`parse_and_insert_rules`](parse_and_insert_rules.md) 创建，仅当至少一个进程级字段具有非默认值时才会创建。如果所有字段都为 `None` 或空，则不会插入 `ProcessLevelConfig`（不过如果存在线程级字段，同一进程仍可能创建 [`ThreadLevelConfig`](ThreadLevelConfig.md)）。

- 该结构体存储在 `ConfigResult.process_level_configs` 中，它是一个两级 `HashMap<u32, HashMap<String, ProcessLevelConfig>>`。外部键是**等级**（规则应用频率），内部键是小写进程名称。等级为 `1` 表示规则在每次轮询循环中运行；等级为 `N` 表示规则每第 N 次循环运行一次。

- 硬亲和性（`affinity_cpus`）和软 CPU 集合（`cpu_set_cpus`）服务于不同目的，可以独立使用或组合使用：
  - **亲和性**是一种严格约束，会被子进程继承。
  - **CPU 集合**是一种调度提示，不会被继承，且可以被操作系统覆盖。

- `cpu_set_reset_ideal` 标志解决了一个特定的 Windows 调度行为问题：设置 CPU 集合时不会自动清除先前分配的理想处理器。启用此标志可确保线程被重新分配到新的 CPU 集合中，而不是继续固定在先前的理想处理器上。

- 该结构体派生了 `Debug` 和 `Clone`。克隆是必要的，因为同一规则可能应用于分组块中的多个进程。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `config.rs` |
| 构造方式 | [`parse_and_insert_rules`](parse_and_insert_rules.md) |
| 存储位置 | [`ConfigResult`](ConfigResult.md)`.process_level_configs` |
| 使用方 | `apply.rs`（进程级应用逻辑） |
| 依赖 | `ProcessPriority`、`IOPriority`、`MemoryPriority` 来自 [`priority.rs`](../priority.rs/README.md)；`List` 和 `CONSUMER_CPUS` 来自 [`collections.rs`](../collections.rs/README.md) |

## 另请参阅

| 资源 | 链接 |
|------|------|
| ThreadLevelConfig | [ThreadLevelConfig](ThreadLevelConfig.md) |
| ConfigResult | [ConfigResult](ConfigResult.md) |
| parse_and_insert_rules | [parse_and_insert_rules](parse_and_insert_rules.md) |
| read_config | [read_config](read_config.md) |
| priority 模块 | [priority.rs 概述](../priority.rs/README.md) |
| config 模块概述 | [README](README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*