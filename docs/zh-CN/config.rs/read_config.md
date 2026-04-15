# read_config 函数 (config.rs)

读取并解析整个 AffinityServiceRust 配置文件，返回完整填充的 [`ConfigResult`](ConfigResult.md)，其中包含所有进程级规则、线程级规则、常量、别名、分组展开，以及解析过程中产生的所有错误和警告。这是从磁盘加载配置的主要入口点。

## 语法

```AffinityServiceRust/src/config.rs#L743-875
pub fn read_config<P: AsRef<Path>>(path: P) -> ConfigResult
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `path` | `P: AsRef<Path>` | 要读取的配置文件的文件系统路径（例如 `"config.ini"`）。接受任何实现 `AsRef<Path>` 的类型，包括 `&str`、`String` 和 `PathBuf`。 |

## 返回值

类型：[`ConfigResult`](ConfigResult.md)

完整填充的配置结果结构体，包含：

- `process_level_configs` — 按等级组织的所有已解析的进程级规则。
- `thread_level_configs` — 按等级组织的所有已解析的线程级规则。
- `constants` — 从 `@NAME = value` 行解析的可调调度器常量（或默认值）。
- 别名、分组和规则计数，用于诊断报告。
- `errors` — 致命错误列表。当非空时，配置被视为无效（`is_valid()` 返回 `false`）。
- `warnings` — 非致命警告列表。

如果文件无法打开，函数返回的 `ConfigResult` 的 `errors` 向量中包含一条错误消息，其他所有字段为默认值。

## 备注

### 文件格式概述

配置文件是面向行的文本格式。每一行是以下之一：

| 行类型 | 前缀 | 处理程序 | 描述 |
|--------|------|---------|------|
| 注释 | `#` | 跳过 | 注释和空行被忽略。 |
| 常量 | `@` | [`parse_constant`](parse_constant.md) | 调度器调优常量（例如 `@MIN_ACTIVE_STREAK = 3`）。 |
| 别名 | `*` | [`parse_alias`](parse_alias.md) | 命名 CPU 规格（例如 `*pcore = 0-7`）。 |
| 分组块 | `{` | [`collect_group_block`](collect_group_block.md) + [`parse_and_insert_rules`](parse_and_insert_rules.md) | 用花括号括起来的多进程分组规则。 |
| 进程规则 | *（其他）* | [`parse_and_insert_rules`](parse_and_insert_rules.md) | 单独的 `name:priority:affinity:...` 规则行。 |

### 解析算法

1. 打开文件并读入缓冲读取器。所有行被收集到 `Vec<String>` 中。
2. 初始化一个本地 `cpu_aliases` 哈希映射，用于存储解析过程中遇到的别名定义。
3. 解析器使用索引变量 `i` 顺序遍历各行：
   - **空行和注释**（以 `#` 为前缀）被跳过。
   - **常量**（以 `@` 为前缀）按 `=` 拆分并分派给 [`parse_constant`](parse_constant.md)。
   - **别名**（以 `*` 为前缀）按 `=` 拆分并分派给 [`parse_alias`](parse_alias.md)。解析后的别名存储在 `cpu_aliases` 中，供后续规则行使用。
   - **分组块**（包含 `{` 的行）有两种子情况处理：
     - *单行分组*：`{` 和 `}` 都出现在同一行。成员在行内提取。
     - *多行分组*：`{` 在本行但 `}` 不在。解析器调用 [`collect_group_block`](collect_group_block.md) 向前扫描后续行直到找到 `}`，将 `i` 推进到右花括号之后。
     - 在两种情况下，分组名称（`{` 之前的文本）被捕获用于诊断。空名称会生成标签 `"anonymous@L{line_number}"`。
     - 收集到的成员和规则后缀被传递给 [`parse_and_insert_rules`](parse_and_insert_rules.md)。
   - **单独规则**：不匹配以上任何模式的行按 `:` 拆分，第一个元素作为进程名称，其余作为规则字段。至少需要 3 个冒号分隔的部分（名称、优先级、亲和性）。名称被转为小写并传递给 [`parse_and_insert_rules`](parse_and_insert_rules.md)。
4. 处理完所有行后，返回填充好的 `ConfigResult`。

### 错误处理策略

解析器被设计为**弹性的**——它收集所有错误和警告，而不是在第一个问题时中止。这允许用户在一次验证遍历中看到配置文件中的所有问题。致命错误（例如，未闭合的分组、字段太少、无效的常量语法）追加到 `result.errors`。非致命问题（例如，未知优先级字符串、空分组、重复规则）追加到 `result.warnings`。

### 关键错误条件

| 条件 | 严重性 | 消息格式 |
|------|--------|---------|
| 文件无法打开 | 错误 | `"Cannot open config file: {io_error}"` |
| 常量行缺少 `=` | 错误 | `"Line {n}: Invalid constant - expected '@NAME = value'"` |
| 别名行缺少 `=` | 错误 | `"Line {n}: Invalid alias - expected '*name = cpu_spec'"` |
| 未闭合的分组块 | 错误 | `"Line {n}: Unclosed group '{label}' - missing }"` |
| 没有成员的分组 | 警告 | `"Line {n}: Group '{label}' has no members"` |
| 分组没有规则后缀 | 错误 | `"Line {n}: Group '{label}' missing rule - use }:priority:affinity,..."` |
| 单独行的字段少于 3 个 | 错误 | `"Line {n}: Too few fields - expected name:priority:affinity,..."` |
| 空进程名称 | 错误 | `"Line {n}: Empty process name"` |

### 行号

错误和警告消息中的行号从 1 开始（`line_number = i + 1`），与用户在文本编辑器中看到的一致。

### 顺序约束

- **别名必须在使用前定义。**解析器顺序处理各行，因此如果对应的 `*alias = cpu_spec` 定义出现在文件的更后面位置，规则中的 `*alias` 引用将无法解析。
- **常量**可以出现在任何位置并立即应用；它们不影响规则解析。
- **规则**可以以任何顺序出现。重复定义会覆盖先前的定义并发出警告。

### 热重载使用

`read_config` 在启动时和 [`hotreload_config`](hotreload_config.md) 进行热重载时都会被调用。在热重载期间，函数将修改后的文件解析为新的 `ConfigResult`。如果 `is_valid()` 为 `true`，则新配置替换活跃配置；否则保留先前的配置并记录错误。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `config.rs` |
| 可见性 | `pub` |
| 调用方 | `main.rs`（启动时）、[`hotreload_config`](hotreload_config.md)（运行时重载） |
| 被调用方 | [`parse_constant`](parse_constant.md)、[`parse_alias`](parse_alias.md)、[`collect_group_block`](collect_group_block.md)、[`collect_members`](collect_members.md)、[`parse_and_insert_rules`](parse_and_insert_rules.md) |
| 依赖 | [`ConfigResult`](ConfigResult.md)、`HashMap`、`List`、[`collections.rs`](../collections.rs/README.md) 中的 `CONSUMER_CPUS` |
| I/O | 通过 `std::fs::File` 和 `std::io::BufReader` 读取文件 |
| 权限 | 配置文件路径的文件系统读取权限 |

## 另请参阅

| 资源 | 链接 |
|------|------|
| ConfigResult | [ConfigResult](ConfigResult.md) |
| parse_constant | [parse_constant](parse_constant.md) |
| parse_alias | [parse_alias](parse_alias.md) |
| collect_group_block | [collect_group_block](collect_group_block.md) |
| parse_and_insert_rules | [parse_and_insert_rules](parse_and_insert_rules.md) |
| hotreload_config | [hotreload_config](hotreload_config.md) |
| convert | [convert](convert.md) |
| cli 模块 | [cli.rs 概述](../cli.rs/README.md) |
| config 模块概述 | [README](README.md) |

---
*Commit: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)*