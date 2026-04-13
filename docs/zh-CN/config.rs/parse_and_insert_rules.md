# parse_and_insert_rules 函数 (config.rs)

解析配置行中以冒号分隔的规则字段，并为所提供列表中的每个成员插入一个完整构造的 [ProcessConfig](ProcessConfig.md) 条目到 [ConfigResult](ConfigResult.md) 规则映射中。这是核心的规则构造函数：它验证每个字段、解析 CPU 别名、解析主线程前缀规范，并处理基于等级的规则分区。

## 语法

```rust
fn parse_and_insert_rules(
    members: &[String],
    rule_parts: &[&str],
    line_number: usize,
    cpu_aliases: &HashMap<String, Vec<u32>>,
    result: &mut ConfigResult,
)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `members` | `&[String]` | 小写的进程名称（或组成员名称）切片，表示此规则适用的目标。每个成员都会获得解析后 [ProcessConfig](ProcessConfig.md) 的独立克隆。对于单进程规则行，这是一个单元素切片。对于组，它包含从 `{ ... }` 块中展开的所有成员名称。 |
| `rule_parts` | `&[&str]` | 从配置行的规则部分提取的冒号分隔字段字符串切片。索引映射：`[0]` = 优先级、`[1]` = 亲和性、`[2]` = CPU 集合、`[3]` = 主线程 CPU、`[4]` = IO 优先级、`[5]` = 内存优先级、`[6]` = 理想处理器或等级、`[7]` = 等级（当字段 6 为理想处理器时）。至少需要 2 个元素；省略的字段默认为 `None`/空/`0`/`1`。 |
| `line_number` | `usize` | 配置文件中定义规则的 1 基行号。用于解析过程中产生的所有错误和警告消息。 |
| `cpu_aliases` | `&HashMap<String, Vec<u32>>` | 由先前 [parse_alias](parse_alias.md) 调用构建的已定义 CPU 别名映射（以小写别名名称为键）。被 [resolve_cpu_spec](resolve_cpu_spec.md) 和内联主线程/理想处理器别名解析逻辑使用。 |
| `result` | `&mut ConfigResult` | 可变引用，指向正在运行的解析结果累加器。成功时，[ProcessConfig](ProcessConfig.md) 条目被插入到 `result.configs` 中。错误和警告分别被推送到 `result.errors` 和 `result.warnings`。`process_rules_count` 和 `redundant_rules_count` 计数器也会被更新。 |

## 返回值

此函数没有返回值。所有输出通过对 `result` 的修改来传达。

## 备注

### 规则字段格式

完整的规则格式为：

```text
priority:affinity:cpuset:prime_cpus:io_priority:memory_priority:ideal_processor:grade
```

字段按位置排列并以冒号分隔。前两个字段（进程优先级和亲和性）是必需的；所有后续字段都是可选的。

### 字段解析详情

| 索引 | 字段 | 解析器 | 默认值 | 说明 |
|------|------|--------|--------|------|
| 0 | `priority` | `ProcessPriority::from_str` | — | **必需。** 无法识别的值会产生警告并默认为 `ProcessPriority::None`。 |
| 1 | `affinity` | [resolve_cpu_spec](resolve_cpu_spec.md) | — | **必需。** 支持 `*alias` 引用以及 [parse_cpu_spec](parse_cpu_spec.md) 接受的所有格式。 |
| 2 | `cpuset` | [resolve_cpu_spec](resolve_cpu_spec.md) | `[]`（空） | 前导 `@` 启用 `cpu_set_reset_ideal`，在应用 CPU 集合后将线程理想处理器分布到 CPU 集合中。`@` 在规范解析前被剥离。 |
| 3 | `prime_cpus` | 复杂解析（见下文） | `[]` / 通配前缀 | 支持跟踪前缀（`?N`、`??N`）、别名限定的模块名前缀规范（`*alias@prefix!priority`）和普通 CPU 规范。 |
| 4 | `io_priority` | `IOPriority::from_str` | `IOPriority::None` | 无法识别的值会产生警告。 |
| 5 | `memory_priority` | `MemoryPriority::from_str` | `MemoryPriority::None` | 无法识别的值会产生警告。 |
| 6 | `ideal_processor` 或 `grade` | [parse_ideal_processor_spec](parse_ideal_processor_spec.md) 或 `u32::parse` | `[]` / `1` | 如果值以 `*` 开头或等于 `"0"`，则解析为理想处理器规范，等级从字段 7 读取。如果可以解析为 `u32`，则视为裸等级而没有理想处理器规则。否则，尝试作为理想处理器规范解析，等级默认为 `1`。 |
| 7 | `grade` | `u32::parse` | `1` | 仅当字段 6 是理想处理器规范时才存在。等级 `0` 会被更正为 `1` 并产生警告。非数字值产生警告并默认为 `1`。 |

### 主线程字段（字段 3）子语法

主线程字段支持丰富的子语法，用于控制哪些线程被跟踪以及如何被固定：

**跟踪前缀：**

| 前缀 | 含义 |
|------|------|
| `?N` | 按周期增量跟踪前 N 个线程**并**应用主线程调度。`track_top_x_threads` 设为正 *N*。 |
| `??N` | 跟踪前 N 个线程**但不**应用主线程调度（仅观察模式）。`track_top_x_threads` 设为负 *N*。 |

跟踪前缀首先被消耗。在数字和 CPU 规范之间可以有可选的 `x` 或 `X` 分隔符（例如 `?8x*p@engine.dll`）。

**模块前缀限定的规范：**

当字段包含 `@` 时，它被解析为一个或多个 `*alias@prefix1;prefix2!priority` 段：

- `*alias` 引用该段 CPU 集合的 CPU 别名。
- `@prefix1;prefix2` 提供以分号分隔的模块名前缀过滤器。
- `!priority` 可选地为特定前缀设置线程优先级（例如 `!highest`、`!above normal`）。

每个段产生一个或多个 [PrimePrefix](PrimePrefix.md) 条目，其 `cpus: Some(...)` 指向该段解析后的 CPU 集合。`prime_threads_cpus` 字段是所有段 CPU 集合的并集。

**普通 CPU 规范：**

当没有 `@` 时，字段通过 [resolve_cpu_spec](resolve_cpu_spec.md) 解析，并创建一个带有空前缀字符串和 `cpus: None` 的单个通配 [PrimePrefix](PrimePrefix.md)。

**值 `"0"`：**

产生空的主线程 CPU 列表和空的前缀列表，实际上为此规则禁用了主线程调度。

### 理想处理器/等级消歧义（字段 6）

字段 6 是有歧义的，因为它可能包含理想处理器规范或裸等级整数。解析器使用以下启发式规则：

1. 如果值以 `*` 开头或等于 `"0"` → 解析为理想处理器规范，从字段 7 读取等级。
2. 否则如果值可以解析为 `u32` → 视为等级（无理想处理器规则）。
3. 否则 → 尝试解析为理想处理器规范，等级默认为 `1`。

在所有代码路径中，等级 `0` 无效并会被更正为 `1`，同时产生警告。

### 冗余规则检测

在插入每个成员之前，函数检查进程名称是否已存在于 `result.configs` 中任何等级映射中。如果发现重复：

- `result.redundant_rules_count` 递增。
- 推送警告：`"Line {N}: Redundant rule - '{name}' already defined (previous definition will be overwritten)"`。
- 之前的条目被新插入覆盖。

### 基于等级的插入

每个 [ProcessConfig](ProcessConfig.md) 被插入到 `result.configs.entry(grade).or_default()` 中，这是一个两级 `HashMap<u32, HashMap<String, ProcessConfig>>`。等级（默认 `1`）决定条目被放入哪个子映射。服务主循环遍历等级以控制规则评估优先级。

### 错误条件

| 条件 | 严重性 | 消息模式 |
|------|--------|----------|
| `rule_parts` 元素少于 2 个 | 错误 | `"Line {N}: Too few fields ({count}) - expected at least 2 (priority,affinity)"` |
| 无法识别的优先级字符串 | 警告 | `"Line {N}: Unknown priority '{val}' - will be treated as 'none'"` |
| 亲和性/CPU 集合/主线程字段中未定义的 `*alias` | 错误 | （来自 [resolve_cpu_spec](resolve_cpu_spec.md)） |
| 主线程 `*alias@` 段中未知的 CPU 别名 | 错误 | `"Line {N}: Unknown CPU alias '*{alias}' in prime specification"` |
| `!priority` 后缀中无法识别的线程优先级 | 警告 | `"Line {N}: Unknown thread priority '{val}' in prefix - will be treated as 'none' (auto-boost)"` |
| 无法识别的 IO 优先级 | 警告 | `"Line {N}: Unknown IO priority '{val}' - will be treated as 'none'"` |
| 无法识别的内存优先级 | 警告 | `"Line {N}: Unknown memory priority '{val}' - will be treated as 'none'"` |
| 等级值为 `0` | 警告 | `"Line {N}: Grade cannot be 0, using 1 instead"` |
| 无效的等级字符串 | 警告 | `"Line {N}: Invalid grade '{val}', using 1"` |
| 冗余的进程名称 | 警告 | `"Line {N}: Redundant rule - '{name}' already defined (previous definition will be overwritten)"` |

### 规则行示例

```text
# 简单示例：将 game.exe 设为高优先级，绑定到 CPU 0-7
game.exe:high:0-7

# 包含所有字段的完整规则
game.exe:high:*perf:@*perf:?8x*p@engine.dll;render.dll!above normal*e@audio.dll:normal:normal:*p@engine.dll:2

# 共享规则的组
{ game.exe: helper.exe: launcher.exe }:above normal:*perf:0:0:none:none
```

## 要求

| | |
|---|---|
| **模块** | `config` (`src/config.rs`) |
| **可见性** | 包内私有 |
| **调用者** | [read_config](read_config.md) |
| **被调用者** | [resolve_cpu_spec](resolve_cpu_spec.md)、[parse_ideal_processor_spec](parse_ideal_processor_spec.md)、`ProcessPriority::from_str`、`IOPriority::from_str`、`MemoryPriority::from_str`、`ThreadPriority::from_str` |
| **产出** | [ProcessConfig](ProcessConfig.md) 条目（插入到 [ConfigResult](ConfigResult.md)`.configs` 中） |
| **API** | 纯函数 - 无 I/O，无 Windows API 调用 |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 单进程配置记录 | [ProcessConfig](ProcessConfig.md) |
| 主线程前缀过滤器 | [PrimePrefix](PrimePrefix.md) |
| 理想处理器规则 | [IdealProcessorRule](IdealProcessorRule.md) |
| 解析后的配置聚合 | [ConfigResult](ConfigResult.md) |
| 别名感知的 CPU 规范解析 | [resolve_cpu_spec](resolve_cpu_spec.md) |
| 理想处理器规范解析器 | [parse_ideal_processor_spec](parse_ideal_processor_spec.md) |
| 主配置文件读取器（调用者） | [read_config](read_config.md) |
| CPU 别名定义解析器 | [parse_alias](parse_alias.md) |
| 进程优先级枚举 | [ProcessPriority](../priority.rs/ProcessPriority.md) |
| I/O 优先级枚举 | [IOPriority](../priority.rs/IOPriority.md) |
| 内存优先级枚举 | [MemoryPriority](../priority.rs/MemoryPriority.md) |
| 线程优先级枚举 | [ThreadPriority](../priority.rs/ThreadPriority.md) |
| 配置模块概述 | [README](README.md) |