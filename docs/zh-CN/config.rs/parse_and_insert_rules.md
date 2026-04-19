# parse_and_insert_rules 函数 (config.rs)

主要的规则字段解析器和插入函数。接收进程名称数组（单个名称或组块成员）和冒号分割的规则字段数组，验证每个字段，构造 [ProcessLevelConfig](ProcessLevelConfig.md) 和/或 [ThreadLevelConfig](ThreadLevelConfig.md) 实例，并将它们插入到 [ConfigResult](ConfigResult.md) 中适当的等级键控映射里。

## 语法

```rust
fn parse_and_insert_rules(
    members: &[String],
    rule_parts: &[&str],
    line_number: usize,
    cpu_aliases: &HashMap<String, List<[u32; CONSUMER_CPUS]>>,
    result: &mut ConfigResult,
)
```

## 参数

`members: &[String]`

要应用已解析规则的小写进程名称切片。对于单行规则，它包含一个元素（例如 `["game.exe"]`）。对于组规则，它包含从 `{ }` 块中提取的所有成员名称。

`rule_parts: &[&str]`

从配置行规则部分分割的字段字符串切片。期望的字段位置：

| 索引 | 字段 | 示例 | 描述 |
|-------|-------|---------|-------------|
| 0 | priority | `"normal"` | 进程优先级类 |
| 1 | affinity | `"*pcore"` 或 `"0-7"` | CPU 亲和性规格 |
| 2 | cpuset | `"@*ecore"` 或 `"0"` | CPU 集合规格；`@` 前缀启用理想处理器重置 |
| 3 | prime | `"?8x*p@engine.dll"` | Prime 线程规格（CPU、前缀、跟踪计数） |
| 4 | io_priority | `"low"` | IO 优先级级别 |
| 5 | memory_priority | `"none"` | 内存优先级级别 |
| 6 | ideal / grade | `"*p@render.dll"` 或 `"2"` | 理想处理器规格，或等级（如果为数字） |
| 7 | grade | `"1"` | 应用频率等级（仅在字段 6 为理想处理器规格时） |

至少需要 2 个字段（优先级和亲和性）；所有后续字段都是可选的，默认值为"不更改"。

`line_number: usize`

此规则所在的配置文件中的 1-based 行号。用于用户诊断中的所有错误和警告消息。

`cpu_aliases: &HashMap<String, List<[u32; CONSUMER_CPUS]>>`

已定义的 CPU 别名映射，由之前的 [parse_alias](parse_alias.md) 调用填充。传递给 [resolve_cpu_spec](resolve_cpu_spec.md) 以进行别名感知的 CPU 字段解析。

`result: &mut ConfigResult`

**\[in, out\]** 累积的解析结果。此函数读取并写入 `errors`、`warnings`、`process_level_configs`、`thread_level_configs`、`process_rules_count`、`redundant_rules_count` 和 `thread_level_configs_count`。

## 返回值

此函数不返回值。所有输出通过 `result` 的变异进行通信。

## 备注

### 字段解析流程

每个字段按顺序独立解析：

1. **优先级（字段 0）：** 通过 `ProcessPriority::from_str` 解析。未知值产生警告并默认为 `ProcessPriority::None`。

2. **亲和性（字段 1）：** 通过 [resolve_cpu_spec](resolve_cpu_spec.md) 解析，该函数处理 `*alias` 引用和原始 CPU 规格。

3. **CPU 集合（字段 2）：** 如果字段以 `@` 开头，则去除 `@` 并设置 `cpu_set_reset_ideal` 为 `true`；其余部分作为 CPU 规格解析。此标志会在应用 CPU 集合后重新分布线程理想处理器。

4. **Prime 规格（字段 3）：** 最复杂的字段。支持几种子格式：
   - `"0"` — 无 Prime 线程调度。
   - `"*alias"` — 将 Prime 线程固定到别名的 CPU，匹配所有线程。
   - `"?Nx*alias"` — 按 CPU 周期跟踪前 N 个线程并将其 Prime 到别名 CPU。`?` 前缀加数字设置正的 `track_top_x_threads`。
   - `"??N"` — 跟踪前 N 个线程，无 Prime 固定。`??` 前缀设置负的 `track_top_x_threads`（仅跟踪模式）。
   - `"*alias@prefix1;prefix2!priority"` — 每前缀 CPU 路由，带可选的线程优先级提升。每个 `*alias@` 段生成 [PrimePrefix](PrimePrefix.md) 条目。`!` 分隔符在 prefix 内设置 [ThreadPriority](../priority.rs/ThreadPriority.md) 覆盖。

5. **IO 优先级（字段 4）：** 通过 `IOPriority::from_str` 解析。未知值产生警告。

6. **内存优先级（字段 5）：** 通过 `MemoryPriority::from_str` 解析。未知值产生警告。

7. **理想处理器 / 等级（字段 6）：** 模糊字段——如果以 `*` 开头或为 `"0"`，则通过 [parse_ideal_processor_spec](parse_ideal_processor_spec.md) 解析为理想处理器规格。如果解析为纯整数，则视为等级。否则尝试作为理想处理器规格，等级默认为 1。

8. **等级（字段 7）：** 如果字段 6 是理想处理器规格且字段 7 存在，则解析为等级。等级必须 ≥ 1；值为 0 产生警告并默认为 1。

### 配置插入逻辑

对于每个成员名称，函数执行两个独立的验证检查：

- **进程级有效：** `priority`、`affinity_cpus`、`cpu_set_cpus`、`io_priority` 或 `memory_priority` 中至少有一个非默认值。如果有效，创建 [ProcessLevelConfig](ProcessLevelConfig.md) 并插入到适当等级下的 `result.process_level_configs`。

- **线程级有效：** `prime_threads_cpus`（非空）、`track_top_x_threads`（非零）或 `ideal_processor_rules`（非空）中至少有一个激活。如果有效，创建 [ThreadLevelConfig](ThreadLevelConfig.md) 并插入到 `result.thread_level_configs`。

如果两者都不通过，则发出警告，表明该进程没有有效规则。

### 冗余检测

插入之前，函数检查所有现有等级映射中是否有具有相同进程名的条目。如果找到，则 `redundant_rules_count` 递增并发出警告。新条目覆盖旧条目——最后定义生效。

### Prime 前缀构造

当 prime 规格包含 `@` 段时，解析器构建 `Vec<PrimePrefix>`，具有每段 CPU 列表和可选的线程优先级覆盖。`prime_threads_cpus` 字段设置为所有段 CPU 集合的并集。当没有 `@` 段时，创建单个默认 [PrimePrefix](PrimePrefix.md)，其 prefix 为空（匹配所有线程），cpus 为 `None`（继承自 `prime_threads_cpus`），`thread_priority` 为 `ThreadPriority::None`。

### 无前缀规格的默认 PrimePrefix

即使 prime 规格是简单的 `*alias` 而没有任何 `@` 前缀过滤器，函数仍然创建一个包含一个默认条目的 `Vec<PrimePrefix>`。这确保下游 [apply_prime_threads](../apply.rs/apply_prime_threads.md) 函数总是至少有一个前缀条目要迭代。

## 需求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **可见性** | 私有 (`fn`) |
| **调用方** | [read_config](read_config.md)（对于单行规则和组规则） |
| **被调用方** | [resolve_cpu_spec](resolve_cpu_spec.md), [parse_ideal_processor_spec](parse_ideal_processor_spec.md), `ProcessPriority::from_str`, `IOPriority::from_str`, `MemoryPriority::from_str`, `ThreadPriority::from_str` |
| **写入到** | [ConfigResult](ConfigResult.md) (`.process_level_configs`, `.thread_level_configs`, `.errors`, `.warnings`, 计数器) |
| **权限** | 无 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [config.rs](README.md) |
| 进程级配置结构体 | [ProcessLevelConfig](ProcessLevelConfig.md) |
| 线程级配置结构体 | [ThreadLevelConfig](ThreadLevelConfig.md) |
| Prime 前缀结构体 | [PrimePrefix](PrimePrefix.md) |
| 理想处理器规则结构体 | [IdealProcessorRule](IdealProcessorRule.md) |
| CPU 别名解析 | [resolve_cpu_spec](resolve_cpu_spec.md) |
| 理想处理器规格解析器 | [parse_ideal_processor_spec](parse_ideal_processor_spec.md) |
| 配置文件读取器 | [read_config](read_config.md) |
| 配置结果容器 | [ConfigResult](ConfigResult.md) |
| 优先级枚举 | [ProcessPriority](../priority.rs/ProcessPriority.md), [IOPriority](../priority.rs/IOPriority.md), [MemoryPriority](../priority.rs/MemoryPriority.md), [ThreadPriority](../priority.rs/ThreadPriority.md) |

*文档针对 Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*