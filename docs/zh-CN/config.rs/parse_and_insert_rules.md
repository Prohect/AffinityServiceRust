# parse_and_insert_rules 函数 (config.rs)

解析配置行中的规则字段，并为所有组成员插入 [ProcessConfig](ProcessConfig.md) 条目到 [ConfigResult](ConfigResult.md) 中。这是 [read_config](read_config.md) 针对单个进程行和组块调用的核心规则构建函数。

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

`members`

此规则适用的进程名称字符串切片（已小写化）。对于单个进程行，包含一个元素；对于组块，包含由 [collect_members](collect_members.md) / [collect_group_block](collect_group_block.md) 收集的所有成员。

`rule_parts`

由配置行的规则部分按 `:` 分割得到的字段字符串切片。期望的字段顺序如下：

| 索引 | 字段 | 类型 | 默认值 |
| --- | --- | --- | --- |
| 0 | priority | `ProcessPriority` | `None` |
| 1 | affinity | CPU 规格或别名 | 空 |
| 2 | cpuset | CPU 规格或别名（前缀 `@` 表示重置理想处理器） | 空 |
| 3 | prime specification | 跟踪 + CPU 规格 + 前缀 | 空 |
| 4 | io_priority | `IOPriority` | `None` |
| 5 | memory_priority | `MemoryPriority` | `None` |
| 6 | ideal processor / grade | 理想处理器规格或等级编号 | 空 / 1 |
| 7 | grade | `u32`（≥1） | 1 |

至少需要 2 个字段（priority 和 affinity）；其余字段为可选。

`line_number`

配置文件中的行号（从 1 开始），用于错误和警告消息。

`cpu_aliases`

别名名称到 CPU 索引向量的映射表，在配置解析期间由 [parse_alias](parse_alias.md) 填充。在 CPU 规格字段中使用 `*` 前缀引用别名。

`result`

可变的 [ConfigResult](ConfigResult.md) 累加器。已解析的配置将按等级和进程名称作为键插入到 `result.configs` 中。错误、警告和计数器会就地更新。

## 返回值

此函数没有返回值。结果通过修改 `result` 参数写入。

## 备注

### 字段解析详情

**优先级（字段 0）：** 通过 `ProcessPriority::from_str` 解析。未知值会生成警告并默认为 `None`。

**亲和性（字段 1）：** 通过 [resolve_cpu_spec](resolve_cpu_spec.md) 解析，支持字面 CPU 规格和 `*alias` 引用。

**CPU 集（字段 2）：** 同样通过 [resolve_cpu_spec](resolve_cpu_spec.md) 解析。如果字段值以 `@` 为前缀，则 `cpu_set_reset_ideal` 标志将被设置为 `true`，这会在应用 CPU 集后将线程理想处理器重新分配到该 CPU 集上。

**Prime 规格（字段 3）：** 最复杂的字段，支持多种子格式：

- `0` — 不进行 prime 线程调度。
- `?Nx*alias` — 跟踪前 N 个线程并将其 prime 调度到指定别名的 CPU。`x` 分隔符在计数和别名之间是必需的。
- `??Nx*alias` — 跟踪前 N 个线程但不进行 prime 调度（负值 `track_top_x_threads`）。
- `*alias@module1;module2!priority` — 基于模块过滤的 prime 调度。每个 `*alias@prefixes` 段定义一个 [PrimePrefix](PrimePrefix.md) 规则。前缀上的 `!priority` 后缀设置线程优先级提升。
- `*alias` — 将所有线程进行简单 prime 调度到别名 CPU。

**IO 优先级（字段 4）：** 通过 `IOPriority::from_str` 解析。未知值会生成警告。

**内存优先级（字段 5）：** 通过 `MemoryPriority::from_str` 解析。未知值会生成警告。

**理想处理器 / 等级（字段 6–7）：** 字段 6 具有双重含义——如果以 `*` 开头或等于 `0`，则通过 [parse_ideal_processor_spec](parse_ideal_processor_spec.md) 解析为理想处理器规格，字段 7 用作等级。如果字段 6 是纯数字，则直接作为等级处理。等级必须 ≥1；值为 0 时会修正为 1 并发出警告。

### 冗余规则检测

如果 `result.configs` 中任意等级已存在同名进程，则发出警告并递增 `redundant_rules_count`。新规则会覆盖先前的定义。

### 配置插入

对于每个成员，使用所有已解析字段构造一个 [ProcessConfig](ProcessConfig.md) 结构体（克隆共享向量），并将其插入到 `result.configs` 中对应的等级键下。`process_rules_count` 按成员数量递增。

### 配置行示例

单个进程：
```
chrome.exe:high:*p:0:0:none:none:*e:1
```

带 prime 线程调度的组：
```
games { game1.exe: game2.exe }:high:*p:@*c:?8x*p@engine.dll;render.dll:none:none:*e
```

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/config.rs |
| **可见性** | 私有（`fn`） |
| **调用方** | [read_config](read_config.md) |
| **调用** | [resolve_cpu_spec](resolve_cpu_spec.md)、[parse_ideal_processor_spec](parse_ideal_processor_spec.md) |
| **相关类型** | [ProcessConfig](ProcessConfig.md)、[PrimePrefix](PrimePrefix.md)、[ConfigResult](ConfigResult.md)、[IdealProcessorRule](IdealProcessorRule.md) |