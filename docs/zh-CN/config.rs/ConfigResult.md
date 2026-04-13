# ConfigResult 结构体 (config.rs)

汇聚配置文件解析过程的完整输出。`ConfigResult` 持有已解析的进程规则映射（先按等级再按进程名索引）、已解析的调优常量、用于报告的统计计数器，以及解析过程中遇到的所有错误和警告。它是 [read_config](read_config.md) 的返回类型，并提供辅助方法来检查有效性、统计规则总数以及打印人类可读的报告。

## 语法

```rust
#[derive(Debug, Default)]
pub struct ConfigResult {
    pub configs: HashMap<u32, HashMap<String, ProcessConfig>>,
    pub constants: ConfigConstants,
    pub constants_count: usize,
    pub aliases_count: usize,
    pub groups_count: usize,
    pub group_members_count: usize,
    pub process_rules_count: usize,
    pub redundant_rules_count: usize,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}
```

## 成员

| 成员 | 类型 | 描述 |
|------|------|------|
| `configs` | `HashMap<u32, HashMap<String, ProcessConfig>>` | 两级映射，存放已解析的进程规则。外层键是**等级**（正整数 `u32`，默认为 `1`），内层键是小写的进程名。等级排序允许服务循环在低等级规则之前应用高等级规则，从而在多条规则可能匹配时控制评估优先级。 |
| `constants` | `ConfigConstants` | 从 `@CONSTANT = value` 行解析的滞回调优常量。初始化为 [ConfigConstants::default()](ConfigConstants.md)，在文件中遇到常量定义时被覆盖。 |
| `constants_count` | `usize` | 成功解析的 `@CONSTANT` 行数。 |
| `aliases_count` | `usize` | 成功解析的 `*alias = cpu_spec` 行数。 |
| `groups_count` | `usize` | 已解析的 `{ ... }` 进程组块数量。 |
| `group_members_count` | `usize` | 从所有组块展开的单个进程名总数。 |
| `process_rules_count` | `usize` | 插入的 [ProcessConfig](ProcessConfig.md) 条目总数（包括单独规则和组展开的成员）。 |
| `redundant_rules_count` | `usize` | 覆盖了同一进程名先前定义的规则数量。每条冗余规则都会生成一条警告。 |
| `errors` | `Vec<String>` | 致命的解析错误。当非空时，配置被视为无效且不得应用。每个字符串包含行号和人类可读的描述。 |
| `warnings` | `Vec<String>` | 非致命的解析警告（例如，未知优先级字符串默认为 `none`、冗余规则、未知常量）。警告不会阻止配置的应用。 |

## 方法

### `is_valid`

```rust
pub fn is_valid(&self) -> bool
```

当 `errors` 为空时返回 `true`，表示已解析的配置可以安全应用。

### `total_rules`

```rust
pub fn total_rules(&self) -> usize
```

返回所有等级中 [ProcessConfig](ProcessConfig.md) 条目的总和。此方法统计的是实际映射条目数而非 `process_rules_count` 计数器，因此它反映的是经过覆盖去重后的最终数量。

### `print_report`

```rust
pub fn print_report(&self)
```

记录解析结果摘要。成功时记录组和规则数量。失败时记录所有错误和警告，最后输出错误总数并提示用户修复。当 `redundant_rules_count > 0` 时始终打印冗余规则警告。

## 备注

`ConfigResult` 实现了 `Default` trait。默认值具有空的 `configs` 映射、默认的 [ConfigConstants](ConfigConstants.md)、所有计数器为零，以及空的错误/警告向量。该默认值在 [read_config](read_config.md) 中用作初始状态，当配置文件无法打开时也会返回此默认值（并在 `errors` 中添加一条错误）。

### 等级系统

`configs` 映射使用等级作为外层键。等级从规则行的可选第八个字段解析，等级 `1` 为默认值。服务主循环按等级顺序迭代，使高等级规则先于低等级规则被评估。这允许设置"优先通道"——例如，等级 `2` 的游戏进程在等级 `1` 的后台工具之前被应用。

### 热重载行为

在 [hotreload_config](hotreload_config.md) 过程中，会从修改后的文件解析出新的 `ConfigResult`。如果 `is_valid()` 返回 `true`，新的 `configs` 映射和 `constants` 将替换运行中的状态。如果存在错误，则保留先前的配置并记录错误日志。这确保服务永远不会使用无效的规则集运行。

### 错误格式

所有错误和警告字符串遵循 `"Line {N}: {description}"` 的模式，其中 `N` 是配置文件中基于 1 的行号。这使用户可以方便地定位和修复配置中的问题。

## 要求

| | |
|---|---|
| **模块** | `config` (`src/config.rs`) |
| **返回自** | [read_config](read_config.md) |
| **消费者** | [hotreload_config](hotreload_config.md)、[main](../main.rs/main.md) |
| **关键依赖** | [ProcessConfig](ProcessConfig.md)、[ConfigConstants](ConfigConstants.md) |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 主配置解析器 | [read_config](read_config.md) |
| 每进程规则记录 | [ProcessConfig](ProcessConfig.md) |
| 滞回调优常量 | [ConfigConstants](ConfigConstants.md) |
| 配置文件热重载 | [hotreload_config](hotreload_config.md) |
| 规则字段解析 | [parse_and_insert_rules](parse_and_insert_rules.md) |
| 配置模块概述 | [README](README.md) |