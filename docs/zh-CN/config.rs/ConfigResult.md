# ConfigResult 类型 (config.rs)

`ConfigResult` 是配置解析器的聚合输出。它包含所有按等级组织的进程级和线程级规则、可调常量、别名/分组/规则统计信息，以及解析过程中产生的所有错误和警告。它是 [`read_config`](read_config.md) 的主要返回类型，也是主循环在将规则应用到运行中的进程时所参考的核心数据结构。

## 语法

```AffinityServiceRust/src/config.rs#L162-175
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
|------|------|------|
| `process_level_configs` | `HashMap<u32, HashMap<String, ProcessLevelConfig>>` | 进程级规则，按等级（轮询频率）索引，再按小写进程名称索引。每个条目包含优先级、亲和性、CPU 集合、I/O 优先级和内存优先级设置。 |
| `thread_level_configs` | `HashMap<u32, HashMap<String, ThreadLevelConfig>>` | 线程级规则，按等级索引，再按小写进程名称索引。每个条目包含主线程 CPU、前缀过滤、跟踪数量和理想处理器规则。 |
| `constants` | [`ConfigConstants`](ConfigConstants.md) | 可调数值常量（`MIN_ACTIVE_STREAK`、`KEEP_THRESHOLD`、`ENTRY_THRESHOLD`），控制主线程调度器的行为。从 `@NAME = value` 行填充，如果未指定则使用默认值。 |
| `constants_count` | `usize` | 从配置文件中成功解析的 `@CONSTANT` 指令数量。 |
| `aliases_count` | `usize` | 从配置文件中成功解析的 `*alias = cpu_spec` 指令数量。 |
| `groups_count` | `usize` | 配置文件中遇到的 `{ }` 进程分组块数量。 |
| `group_members_count` | `usize` | 从所有分组块中收集的单独进程名称总数。 |
| `process_rules_count` | `usize` | 解析的进程规则总数（包括单独规则和分组展开的成员）。 |
| `redundant_rules_count` | `usize` | 对同一进程名称重复定义的规则计数。当遇到重复规则时，先前的定义会被覆盖并发出警告。 |
| `errors` | `Vec<String>` | 致命解析错误列表。当此向量非空时，配置被视为无效且不应被应用。 |
| `warnings` | `Vec<String>` | 非致命警告列表（例如，未知优先级字符串、重复规则、空分组）。存在警告时配置仍然可用。 |
| `thread_level_configs_count` | `usize` | 在所有等级中插入的线程级配置条目总数。 |

## 方法

### `is_valid`

```AffinityServiceRust/src/config.rs#L178-180
pub fn is_valid(&self) -> bool {
    self.errors.is_empty()
}
```

如果配置不包含错误且可安全应用，则返回 `true`。仅有警告不会导致 `is_valid` 返回 `false`。

### `total_rules`

```AffinityServiceRust/src/config.rs#L182-186
pub fn total_rules(&self) -> usize {
    let a: usize = self.process_level_configs.values().map(|grade_configs| grade_configs.len()).sum();
    let b: usize = self.thread_level_configs.values().map(|grade_configs| grade_configs.len()).sum();
    a + b
}
```

返回所有等级中活跃规则的总数，通过汇总进程级和线程级配置中所有内部 `HashMap` 条目的长度来计算。

### `print_report`

```AffinityServiceRust/src/config.rs#L188-217
pub fn print_report(&self)
```

将解析结果的人类可读摘要打印到日志中。当配置有效时，报告进程分组和进程规则的数量。当存在错误时，分别以 `✗` 或 `⚠` 为前缀打印每个错误和警告，随后输出错误计数。当 `redundant_rules_count > 0` 时，也会打印重复规则警告。

## 备注

- **基于等级的组织**：`process_level_configs` 和 `thread_level_configs` 都使用 `HashMap<u32, HashMap<String, ...>>` 结构。外部键是*等级*——一个正整数（≥ 1），决定规则的应用频率。等级为 `1` 表示规则在每次轮询循环中运行；等级为 `N` 表示每第 N 次循环运行一次。这种设计使主循环能够高效地仅选择适用于当前迭代的规则。

- **派生 Default**：`ConfigResult` 派生了 `Default`，它将两个 `HashMap` 集合初始化为空，将所有计数器设为 `0`，并为错误和警告创建空的 `Vec`。`constants` 字段接收 `ConfigConstants::default()` 的值（`min_active_streak = 2`、`keep_threshold = 0.69`、`entry_threshold = 0.42`）。

- **重复规则处理**：当一个进程名称出现在多个规则中时，[`parse_and_insert_rules`](parse_and_insert_rules.md) 会覆盖先前的条目并递增 `redundant_rules_count`。同时会向 `warnings` 推送一条警告，标识行号和进程名称。

- **热重载安全性**：[`hotreload_config`](hotreload_config.md) 将配置文件解析为新的 `ConfigResult`，仅当 `is_valid()` 返回 `true` 时才替换活跃配置。如果重载验证失败，则保留先前的 `ConfigResult` 并记录错误。

- **拆分为进程级和线程级**：一条配置行可能同时在 `process_level_configs` 和 `thread_level_configs` 中产生条目。拆分发生在 [`parse_and_insert_rules`](parse_and_insert_rules.md) 内部：进程级设置（优先级、亲和性、CPU 集合、I/O 优先级、内存优先级）进入 `process_level_configs`，线程级设置（主线程 CPU、前缀、跟踪、理想处理器）进入 `thread_level_configs`。只有线程级设置的进程（例如，仅有理想处理器规则）在 `process_level_configs` 中没有条目，反之亦然。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `config.rs` |
| 构造方式 | [`read_config`](read_config.md)（通过 `ConfigResult::default()`） |
| 填充方 | [`parse_and_insert_rules`](parse_and_insert_rules.md)、[`parse_constant`](parse_constant.md)、[`parse_alias`](parse_alias.md)、[`collect_group_block`](collect_group_block.md) |
| 使用方 | `main.rs` 主循环、[`hotreload_config`](hotreload_config.md)、`apply.rs` |
| API | 内部 |
| 权限 | 无 |

## 另请参阅

| 资源 | 链接 |
|------|------|
| ProcessLevelConfig | [ProcessLevelConfig](ProcessLevelConfig.md) |
| ThreadLevelConfig | [ThreadLevelConfig](ThreadLevelConfig.md) |
| ConfigConstants | [ConfigConstants](ConfigConstants.md) |
| read_config | [read_config](read_config.md) |
| parse_and_insert_rules | [parse_and_insert_rules](parse_and_insert_rules.md) |
| hotreload_config | [hotreload_config](hotreload_config.md) |
| config 模块概述 | [README](README.md) |

---
*提交：[37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*