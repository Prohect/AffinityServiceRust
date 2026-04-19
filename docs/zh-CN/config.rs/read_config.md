# read_config 函数 (config.rs)

主配置文件读取器。打开指定的文件，逐行迭代，并根据前缀字符将每行分派给适当的子解析器。返回完全填充的 [ConfigResult](ConfigResult.md)，包含所有已解析的规则、别名、常量、组、错误和警告。

## 语法

```rust
pub fn read_config<P: AsRef<Path>>(path: P) -> ConfigResult
```

## 参数

`path: P`

配置文件的路径。接受任何实现 `AsRef<Path>` 的类型，包括 `&str`、`String` 和 `PathBuf`。文件应为 UTF-8 编码，每行一个指令。

## 返回值

[ConfigResult](ConfigResult.md) — 包含以下内容的结构体：

- `process_level_configs` 和 `thread_level_configs` 映射，按等级和进程名填充已解析的规则。
- 从 `@CONSTANT = value` 指令填充的 `constants`（如果未指定则使用默认值）。
- 别名、组、组成员、进程规则、冗余规则和线程级配置的计数器。
- `errors` — 致命解析错误，使配置无效。
- `warnings` — 非致命问题，会被记录但不会阻止配置被使用。

如果文件无法打开，函数返回带有单条错误消息且无规则的 `ConfigResult`。

## 备注

### 行分派

函数将所有行读入内存，然后使用基于索引的循环进行迭代（以支持多行组块，这些组块会将索引推进超过一行）。每行非空、非注释行根据其前导字符进行分派：

| 前缀 | 处理器 | 描述 |
|--------|---------|-------------|
| `#` | (跳过) | 注释行 — 完全忽略。 |
| `@` | [parse_constant](parse_constant.md) | 期望 `@NAME = value`。解析调优常量，如 `MIN_ACTIVE_STREAK`、`KEEP_THRESHOLD`、`ENTRY_THRESHOLD`。 |
| `*` | [parse_alias](parse_alias.md) | 期望 `*name = cpu_spec`。定义命名 CPU 别名，供后续规则行引用。 |
| 包含 `{` | 组块 | 行包含组定义。如果同一行上有 `}`，则为内联组；否则 [collect_group_block](collect_group_block.md) 读取后续行直到找到 `}`。 |
| (其他) | 单行规则 | 行按 `:` 分割，第一段是进程名。其余段转发给 [parse_and_insert_rules](parse_and_insert_rules.md)。 |

### 解析顺序

指令按文件顺序处理。这意味着：

1. **常量** (`@`) 应出现在依赖调优行为的任何规则之前，尽管它们可以出现在任何位置。
2. **别名** (`*`) 必须在引用它们的任何规则行之前定义。对未定义别名的前向引用会产生错误。
3. **组和规则** 可以以任何顺序出现；同一进程名的后面定义会覆盖前面的定义，并带有警告。

### 组处理

组块有两种形式：

**内联组**（单行）：
```
group_name { proc1.exe: proc2.exe }:normal:*ecore:0:0:low:none:0:1
```

**多行组**：
```
group_name {
    proc1.exe: proc2.exe
    proc3.exe
}:normal:*ecore:0:0:low:none:0:1
```

两种情况下，组名从 `{` 之前的文本中提取。如果名称为空，则从行号生成匿名标签 `"anonymous@L{N}"`。成员由 [collect_members](collect_members.md) 收集，`}:` 后的规则后缀被分割并转发给 [parse_and_insert_rules](parse_and_insert_rules.md)。未闭合的组（缺少 `}`）会产生致命错误。

### 单行规则处理

非组、非指令行应具有以下格式：

```
process.exe:priority:affinity:cpuset:prime:io:memory:ideal:grade
```

必需的字段最少为 3 个（名称、优先级、亲和性）。少于该数量的字段会产生致命错误。进程名（字段 0）转为小写，作为单元素成员列表传递给 [parse_and_insert_rules](parse_and_insert_rules.md)。

### 错误处理

- 如果配置文件无法打开（权限、文件缺失），添加单条错误并立即返回。
- 来自子解析器（常量、别名、规则）的解析错误累积在 `ConfigResult.errors` 和 `ConfigResult.warnings` 中。调用方应在使用结果之前检查 [is_valid](ConfigResult.md)。

### CPU 别名范围

别名存储在局限于 `read_config` 调用的本地 `HashMap<String, List<[u32; CONSUMER_CPUS]>>` 中。它们不会保留在返回的 `ConfigResult` 中——只保留 `aliases_count`。这意味着别名是解析时的概念，在存储到规则结构体之前完全解析为具体的 CPU 索引列表。

## 需求

| | |
|---|---|
| **模块** | [`src/config.rs`](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/config.rs) |
| **调用方** | [hotreload_config](hotreload_config.md), 主服务启动 |
| **被调用方** | [parse_constant](parse_constant.md), [parse_alias](parse_alias.md), [collect_members](collect_members.md), [collect_group_block](collect_group_block.md), [parse_and_insert_rules](parse_and_insert_rules.md) |
| **依赖项** | `std::fs::File`, `std::io::BufReader`, [ConfigResult](ConfigResult.md), [HashMap](../collections.rs/README.md) |
| **权限** | 对配置文件路径的文件系统读取访问 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [config.rs](README.md) |
| 聚合解析结果 | [ConfigResult](ConfigResult.md) |
| 规则插入逻辑 | [parse_and_insert_rules](parse_and_insert_rules.md) |
| 别名定义 | [parse_alias](parse_alias.md) |
| 常量定义 | [parse_constant](parse_constant.md) |
| 组块收集器 | [collect_group_block](collect_group_block.md) |
| 包装热重载 | [hotreload_config](hotreload_config.md) |
| Process Lasso 转换器 | [convert](convert.md) |

*文档针对 Commit：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*