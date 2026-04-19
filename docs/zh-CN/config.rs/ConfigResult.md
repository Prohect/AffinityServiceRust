# ConfigResult 结构体 (config.rs)

配置解析器的聚合输出。包含所有按等级（应用频率）组织的已解析进程级和线程级规则，以及解析统计信息、错误和警告。这是 [read_config](read_config.md) 的主要返回类型，也是主服务循环和 [hotreload_config](hotreload_config.md) 所处理的结构体。

## 语法

```rust
#[derive(Debug, Default)]
pub struct ConfigResult {
    pub process_level_configs: HashMap<u32, HashMap<String, ProcessLevelConfig>>,
    pub thread_level_configs: HashMap<u32, HashMap<String, ThreadLevelConfig>>,
    pub constants: ConfigConstants,
    pub constants_count: usize,
    pub aliases_count: usize,
    pub groups_count: usize,
    pub group_members_count: usize,
    pub process_rules_count: usize,
    pub redundant_rules_count: usize,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub thread_level_configs_count: usize,
}
```

## 成员

| 成员 | 类型 | 描述 |
|--------|------|-------------|
| `process_level_configs` | `HashMap<u32, HashMap<String, ProcessLevelConfig>>` | 按等级（外层）和小写进程名（内层）索引的进程级规则。等级 `1` 是默认值，表示规则在每个轮询迭代中应用；更高的等级会降低应用频率。 |
| `thread_level_configs` | `HashMap<u32, HashMap<String, ThreadLevelConfig>>` | 按等级（外层）和小写进程名（内层）索引的线程级规则。这些规则在每个轮询迭代中评估，用于线程级调度（Prime 线程、理想处理器）。 |
| `constants` | [ConfigConstants](ConfigConstants.md) | 用于 Prime 线程迟滞的调优常量。从配置文件中的 `@CONSTANT = value` 行填充；如果未指定则使用 `Default` 值。 |
| `constants_count` | `usize` | 成功解析的 `@CONSTANT` 指令数量。 |
| `aliases_count` | `usize` | 成功解析的 `*alias = cpu_spec` 别名定义数量。 |
| `groups_count` | `usize` | 成功解析的 `{ }` 组块数量。 |
| `group_members_count` | `usize` | 所有已解析组块中包含的进程名总数。 |
| `process_rules_count` | `usize` | 尝试为其生成规则的进程名总数（包括单个进程和组成员）。 |
| `redundant_rules_count` | `usize` | 为同一进程名覆盖先前定义规则的数量。每个冗余规则还会生成一条警告。 |
| `errors` | `Vec<String>` | 致命解析错误。任何非空的错误列表都会使 [is_valid](#is_valid) 返回 `false`，并阻止配置被应用。消息包含行号。 |
| `warnings` | `Vec<String>` | 非致命解析警告（例如，未知优先级名称被视为 `none`、冗余规则、空组）。即使存在警告，配置仍然可用。 |
| `thread_level_configs_count` | `usize` | 插入的线程级配置条目的运行计数。每当为进程名生成有效的线程级规则时递增。 |

## 方法

### is_valid

```rust
pub fn is_valid(&self) -> bool
```

如果 `errors` 向量为空，则返回 `true`，表示配置已无致命错误地解析，可以安全应用。

**返回值**

`bool` — 当未记录解析错误时为 `true`；否则为 `false`。

### total_rules

```rust
pub fn total_rules(&self) -> usize
```

返回所有等级中活动规则的总数，结合进程级和线程级配置。这计算内层 `HashMap` 中的条目数，而不是原始 `process_rules_count` 计数器。

**返回值**

`usize` — 每个等级中所有进程级配置条目和所有线程级配置条目的总和。

### print_report

```rust
pub fn print_report(&self)
```

记录解析结果的人类可读摘要。行为取决于有效性：

- **有效配置：** 如果有组存在，则记录组统计信息、进程规则总数，以及任何以 `⚠` 前缀的警告。
- **无效配置：** 记录所有以 `✗` 前缀的错误、所有以 `⚠` 前缀的警告，以及最终的错误计数摘要。
- **存在冗余规则：** 即使对于有效配置，也会打印警告，以提醒用户存在重复定义。

## 备注

### 两级 HashMap 结构

外层 `HashMap<u32, ...>` 键是**等级**，一个 ≥ 1 的无符号整数，控制规则的应用频率。等级 1 规则在每个轮询周期运行；等级 *N* 规则每 *N* 个周期运行一次。这允许用户为不需要持续监控的进程配置较低频率的执行。内层 `HashMap<String, ...>` 将小写的进程名映射到它们各自的配置。

### 进程级与线程级分离

单行配置规则可以同时生成 [ProcessLevelConfig](ProcessLevelConfig.md) 和 [ThreadLevelConfig](ThreadLevelConfig.md)。[parse_and_insert_rules](parse_and_insert_rules.md) 中的解析器独立确定有效性：仅当优先级、亲和性、CPU 集合、IO 优先级或内存优先级中至少有一个非默认值时，才会创建进程级条目；仅当指定了主 CPUs、跟踪或理想处理器规则时，才会创建线程级条目。如果两者都不有效，则发出警告。

### 冗余检测

当为已存在于任何等级中的进程名插入规则时，旧条目将被覆盖，`redundant_rules_count` 将递增。这是警告，不是错误——最后定义的规则生效。

### 默认派生

该结构体派生 `Default`，将所有 `HashMap` 和 `Vec` 初始化为空，所有计数器设为零，并将 `constants` 设为 `ConfigConstants::default()`。这由 [read_config](read_config.md) 在开始解析之前用于创建初始累加器。

## 需求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **调用方** | [read_config](read_config.md), [hotreload_config](hotreload_config.md), 主轮询循环, [parse_and_insert_rules](parse_and_insert_rules.md), [parse_constant](parse_constant.md), [parse_alias](parse_alias.md) |
| **依赖项** | [ProcessLevelConfig](ProcessLevelConfig.md), [ThreadLevelConfig](ThreadLevelConfig.md), [ConfigConstants](ConfigConstants.md), [HashMap](../collections.rs/README.md) |
| **权限** | 无（仅为数据结构） |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [config.rs](README.md) |
| 进程级规则结构体 | [ProcessLevelConfig](ProcessLevelConfig.md) |
| 线程级规则结构体 | [ThreadLevelConfig](ThreadLevelConfig.md) |
| 调优常量 | [ConfigConstants](ConfigConstants.md) |
| 配置文件解析器 | [read_config](read_config.md) |
| 规则插入逻辑 | [parse_and_insert_rules](parse_and_insert_rules.md) |
| 带验证的热重载 | [hotreload_config](hotreload_config.md) |
| 应用引擎 | [apply.rs](../apply.rs/README.md) |

*文档针对 Commit：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*