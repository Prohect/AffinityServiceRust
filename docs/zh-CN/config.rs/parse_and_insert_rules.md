# parse_and_insert_rules 函数 (config.rs)

解析配置行中的规则字段，并为提供的列表中的每个成员将进程级和线程级配置条目插入到 [`ConfigResult`](ConfigResult.md) 中。这是核心的规则解释函数，将原始的冒号分隔字段字符串转换为完全解析的 [`ProcessLevelConfig`](ProcessLevelConfig.md) 和 [`ThreadLevelConfig`](ThreadLevelConfig.md) 实例。

## 语法

```AffinityServiceRust/src/config.rs#L424-741
fn parse_and_insert_rules(
    members: &[String],
    rule_parts: &[&str],
    line_number: usize,
    cpu_aliases: &HashMap<String, List<[u32; CONSUMER_CPUS]>>,
    result: &mut ConfigResult,
)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `members` | `&[String]` | 此规则适用的小写进程名称切片。对于单进程行，其中包含一个元素；对于 `{ }` 分组块，包含所有收集到的成员名称。 |
| `rule_parts` | `&[&str]` | 进程名称之后（或分组的 `}:` 之后）按冒号拆分的字段。预期的字段顺序为：`priority`、`affinity`、`cpuset`、`prime_cpus`、`io_priority`、`memory_priority`、`ideal_processor`、`grade`。至少需要 2 个字段。 |
| `line_number` | `usize` | 配置文件中从 1 开始的行号，用于错误和警告消息。 |
| `cpu_aliases` | `&HashMap<String, List<[u32; CONSUMER_CPUS]>>` | 由配置文件中先前出现的 `*name = cpu_spec` 行填充的别名查找表。 |
| `result` | `&mut ConfigResult` | 可变的配置结果累加器。解析后的条目被插入到 `result.process_level_configs` 和/或 `result.thread_level_configs` 中，任何错误或警告被追加到相应的向量中。 |

## 返回值

此函数不返回值。所有输出通过 `result` 参数累积。

## 备注

### 字段解析顺序

函数按位置解释 `rule_parts`。超出最少 2 个的字段是可选的；缺失的字段接收合理的默认值。

| 索引 | 字段 | 默认值 | 描述 |
|------|------|--------|------|
| 0 | `priority` | *（必需）* | 进程优先级类别字符串（例如 `"normal"`、`"high"`、`"none"`）。通过 `ProcessPriority::from_str` 解析。未知值被视为 `None` 并发出警告。 |
| 1 | `affinity` | *（必需）* | CPU 亲和性规格。通过 [`resolve_cpu_spec`](resolve_cpu_spec.md) 解析；支持别名引用（`*pcore`）和字面规格（`0-7`）。 |
| 2 | `cpuset` | `List::new()`（空） | CPU 集合规格。如果以 `@` 为前缀，则 `cpu_set_reset_ideal` 标志设为 `true`，并在解析前去除 `@`。 |
| 3 | `prime_cpus` | 空 CPU 列表，默认前缀 | 带有可选模块前缀过滤和跟踪指令的主线程 CPU 规格。参见下方**主线程解析**。 |
| 4 | `io_priority` | `IOPriority::None` | I/O 优先级字符串（例如 `"low"`、`"normal"`、`"high"`）。未知值产生警告并默认为 `None`。 |
| 5 | `memory_priority` | `MemoryPriority::None` | 内存优先级字符串（例如 `"very low"`、`"normal"`）。未知值产生警告并默认为 `None`。 |
| 6 | `ideal_processor` 或 `grade` | `Vec::new()` / `1` | 此字段具有多态性。如果以 `*` 开头或等于 `"0"`，则通过 [`parse_ideal_processor_spec`](parse_ideal_processor_spec.md) 解析为理想处理器规格；等级随后从索引 7 读取。如果可解析为整数，则直接解释为等级。 |
| 7 | `grade` | `1` | 规则应用频率。`1` = 每次循环，`N` = 每第 N 次循环。仅在字段 6 为理想处理器规格时使用。 |

### 主线程解析（字段 3）

主线程字段支持丰富的迷你语法：

- **`"0"`** — 不进行主线程调度。
- **`?N`** 前缀 — 跟踪前 N 个线程**并**进行主线程调度（正 `track_top_x_threads`）。
- **`??N`** 前缀 — 跟踪前 N 个线程但**不**进行主线程调度（负 `track_top_x_threads`）。
- **`*alias@prefix1;prefix2`** 段 — 带模块前缀过滤的每别名 CPU 集合。可以链接多个 `*alias@...` 段。每段产生一个或多个 [`PrimePrefix`](PrimePrefix.md) 条目。
- **`prefix!priority`** 后缀 — 特定前缀的可选线程优先级覆盖，通过 `ThreadPriority::from_str` 解析。
- **纯 `*alias`** — 不带前缀过滤的单个别名；所有线程均有资格。

当存在 `@` 时，解析器按 `*` 拆分以提取段，从 `cpu_aliases` 解析每个别名，并构建一个 `Vec<PrimePrefix>`。前缀字符串中的 `!` 将模块名称与可选的线程优先级覆盖分隔开。

### 拆分为进程级和线程级

解析所有字段后，函数检查哪些设置为非默认值：

- **进程级有效**：`priority != None`、`affinity_cpus` 非空、`cpu_set_cpus` 非空、`io_priority != None` 或 `memory_priority != None`。
- **线程级有效**：`prime_threads_cpus` 非空、`track_top_x_threads != 0` 或 `ideal_processor_rules` 非空。

对于 `members` 中的每个成员：

1. 如果进程级设置有效，则将 [`ProcessLevelConfig`](ProcessLevelConfig.md) 条目插入到 `result.process_level_configs` 中对应的等级键下。
2. 如果线程级设置有效，则将 [`ThreadLevelConfig`](ThreadLevelConfig.md) 条目插入到 `result.thread_level_configs` 中对应的等级键下，并递增 `thread_level_configs_count`。
3. 如果两者都无效，则发出警告，指出该进程没有有效规则。

### 重复规则检测

在插入之前，函数检查进程名称是否已存在于 `process_level_configs` 或 `thread_level_configs` 的任何等级桶中。如果是，则递增 `result.redundant_rules_count` 并推送一条警告，指出先前的定义将被覆盖。

### 错误条件

| 条件 | 严重性 | 消息格式 |
|------|--------|---------|
| 规则字段少于 2 个 | 错误 | `"Line {n}: Too few fields ({count}) - expected at least 2"` |
| 未知优先级字符串 | 警告 | `"Line {n}: Unknown priority '...' - will be treated as 'none'"` |
| 未定义的 CPU 别名 | 错误 | `"Line {n}: Undefined alias '*...' in {field} field"` |
| 未知 I/O 优先级 | 警告 | `"Line {n}: Unknown IO priority '...' - will be treated as 'none'"` |
| 未知内存优先级 | 警告 | `"Line {n}: Unknown memory priority '...' - will be treated as 'none'"` |
| 等级值为 0 | 警告 | `"Line {n}: Grade cannot be 0, using 1 instead"` |
| 无效的等级字符串 | 警告 | `"Line {n}: Invalid grade '...', using 1"` |
| 重复的进程级规则 | 警告 | `"Line {n}: Redundant process level rule - '...' already defined"` |
| 重复的线程级规则 | 警告 | `"Line {n}: Redundant thread level rule - '...' already defined"` |
| 所有字段均为 none/0 | 警告 | `"No valid rules(all none/0) for process '...'"` |

### 副作用

- 通过插入新条目修改 `result.process_level_configs` 和 `result.thread_level_configs`。
- 通过追加诊断消息修改 `result.errors` 和 `result.warnings`。
- 将 `result.process_rules_count` 递增 `members.len()`。
- 对每个检测到的重复项递增 `result.redundant_rules_count`。
- 对每个创建的线程级条目递增 `result.thread_level_configs_count`。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `config.rs` |
| 可见性 | 私有（`fn`，非 `pub fn`） |
| 调用方 | [`read_config`](read_config.md)、[`sort_and_group_config`](sort_and_group_config.md)（通过 `read_config` 间接调用） |
| 被调用方 | [`resolve_cpu_spec`](resolve_cpu_spec.md)、[`parse_ideal_processor_spec`](parse_ideal_processor_spec.md)、`ProcessPriority::from_str`、`IOPriority::from_str`、`MemoryPriority::from_str`、`ThreadPriority::from_str` |
| 依赖 | [`ProcessLevelConfig`](ProcessLevelConfig.md)、[`ThreadLevelConfig`](ThreadLevelConfig.md)、[`PrimePrefix`](PrimePrefix.md)、[`IdealProcessorRule`](IdealProcessorRule.md)、[`ConfigResult`](ConfigResult.md) |
| 权限 | 无 |

## 另请参阅

| 资源 | 链接 |
|------|------|
| read_config | [read_config](read_config.md) |
| resolve_cpu_spec | [resolve_cpu_spec](resolve_cpu_spec.md) |
| parse_ideal_processor_spec | [parse_ideal_processor_spec](parse_ideal_processor_spec.md) |
| collect_group_block | [collect_group_block](collect_group_block.md) |
| ProcessLevelConfig | [ProcessLevelConfig](ProcessLevelConfig.md) |
| ThreadLevelConfig | [ThreadLevelConfig](ThreadLevelConfig.md) |
| PrimePrefix | [PrimePrefix](PrimePrefix.md) |
| ConfigResult | [ConfigResult](ConfigResult.md) |
| config 模块概述 | [README](README.md) |

---
*Commit: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)*