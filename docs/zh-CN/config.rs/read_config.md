# read_config 函数 (config.rs)

读取并解析配置文件，生成一个完全解析的 [ConfigResult](ConfigResult.md)，其中包含进程规则、CPU 别名、调优常量，以及解析过程中遇到的所有错误和警告。这是加载 AffinityServiceRust 配置的主要入口点，在启动时和热重载期间被调用。

## 语法

```rust
pub fn read_config<P: AsRef<Path>>(path: P) -> ConfigResult
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `path` | `P: AsRef<Path>` | 配置文件的文件系统路径。接受任何实现了 `AsRef<Path>` 的类型，包括 `&str`、`String` 和 `PathBuf`。文件通过 `File::open` 打开，并通过缓冲读取器逐行读取。 |

## 返回值

返回一个 [ConfigResult](ConfigResult.md)，包含：

- **`configs`** — 一个 `HashMap<u32, HashMap<String, ProcessConfig>>`，按等级和小写进程名索引。每个叶节点值是一个完全解析的 [ProcessConfig](ProcessConfig.md)。
- **`constants`** — 一个 [ConfigConstants](ConfigConstants.md)，初始化为默认值，并被文件中的任何 `@CONSTANT = value` 行覆盖。
- **统计计数器** — `constants_count`、`aliases_count`、`groups_count`、`group_members_count`、`process_rules_count`、`redundant_rules_count`。
- **`errors`** — 致命解析错误。当非空时，不应应用该配置。
- **`warnings`** — 非致命解析警告（未知的优先级字符串、冗余规则等）。

如果文件无法打开，则会立即返回一个 `errors` 向量中包含单个错误的 `ConfigResult`。

## 备注

### 文件格式概述

配置文件使用类 INI 的、面向行的格式，包含五种顶级行：

| 行前缀 | 含义 | 处理函数 |
|--------|------|----------|
| `#` | 注释 — 完全忽略。 | `read_config`（跳过） |
| `@NAME = value` | 常量定义 — 设置调度器调优参数。 | [parse_constant](parse_constant.md) |
| `*name = cpu_spec` | CPU 别名定义 — 创建一个命名的 CPU 集合，供规则字段使用。 | [parse_alias](parse_alias.md) |
| `name { ... }:rule` | 进程组 — 定义共享同一规则的多个进程。 | [collect_group_block](collect_group_block.md)、[parse_and_insert_rules](parse_and_insert_rules.md) |
| `name:priority:affinity:...` | 单进程规则 — 为单个进程定义设置。 | [parse_and_insert_rules](parse_and_insert_rules.md) |

空行将被忽略。

### 解析算法

1. **打开并缓冲** — 打开文件并将所有行收集到一个 `Vec<String>` 中以便随机访问迭代（多行组块需要此能力）。
2. **初始化状态** — 创建一个默认的 [ConfigResult](ConfigResult.md) 和一个空的 `cpu_aliases` 映射。
3. **逐行分派** — 索引 `i` 遍历行向量。每行经过裁剪后，根据第一个非空白字符确定分派路径：
   - 空行或 `#` → 跳过，推进 `i`。
   - `@` → 按 `=` 分割，委托给 [parse_constant](parse_constant.md)，推进 `i`。
   - `*` → 按 `=` 分割，委托给 [parse_alias](parse_alias.md)，推进 `i`。
   - 包含 `{` → 开始组解析（见下文）。
   - 其他 → 按 `:` 分割，从 `parts[0]` 提取进程名，将 `parts[1..]` 委托给 [parse_and_insert_rules](parse_and_insert_rules.md)，推进 `i`。
4. **返回** — 返回完成的 `ConfigResult`。

### 组块解析

当某行包含 `{` 时，解析器进入组模式：

1. **组名** — `{` 前的文本用作诊断标签。如果为空，则标签为 `"anonymous@L{line_number}"`。
2. **单行组** — 如果 `}` 与 `{` 出现在同一行，则通过 [collect_members](collect_members.md) 内联解析大括号之间的文本，`}:` 之后的文本为规则后缀。
3. **多行组** — 如果同一行没有 `}`，则 [collect_group_block](collect_group_block.md) 扫描后续行直到找到闭合的 `}`，同时累积成员名。索引 `i` 被推进到块之后。
4. **未闭合的组** — 如果文件结束时未找到 `}`，则记录一个错误并跳过该组。
5. **空组** — 如果大括号之间没有找到成员，则记录一个警告并跳过该组。
6. **规则应用** — 如果在闭合大括号之后存在规则后缀（即 `}:priority:affinity:...`），则以收集到的成员列表调用 [parse_and_insert_rules](parse_and_insert_rules.md)。如果未找到规则后缀，则记录一个错误。

### 单进程规则解析

对于非组行，按 `:` 分割该行并验证各部分：

- **少于 3 个部分** — 记录一个错误（`"Too few fields — expected name:priority:affinity,..."`）。
- **空进程名** — 记录一个错误。
- **有效行** — `parts[0]`（小写化）成为成员名，`parts[1..]` 传递给 [parse_and_insert_rules](parse_and_insert_rules.md)。

### 顺序依赖

- **别名必须在规则之前** — CPU 别名定义（`*name = spec`）按从上到下的顺序处理，并存储在 `cpu_aliases` 映射中。通过 `*name` 引用别名的规则行，如果别名尚未在前面的行中定义，将会因"未定义的别名"错误而失败。
- **常量可以出现在任何位置** — `@CONSTANT` 行立即更新 `result.constants`，不依赖于相对于规则的解析顺序。
- **组不能嵌套** — 不支持嵌套的 `{ { } }` 结构；在开放组块内部的 `{` 被视为普通文本。

### 错误累积

`read_config` 使用**遇错继续**策略。当遇到解析错误时，将描述性消息（包含基于 1 的行号）推送到 `result.errors`，然后继续解析下一行。这允许用户在一次解析中看到所有错误，而不是逐个修复。

### 配置文件示例

```text
# CPU 别名
*perf = 0-7
*eff = 8-15

# 调度器调优
@MIN_ACTIVE_STREAK = 3
@KEEP_THRESHOLD = 0.70

# 单进程规则
game.exe:high:*perf:*perf:?8x*perf@engine.dll:none:none:*perf

# 组规则
background_apps {
    updater.exe: telemetry.exe
    cloud_sync.exe
}:below normal:*eff:0:0:none:none
```

### 线程安全

`read_config` 不是为并发访问设计的。它从文件系统读取数据并写入本地 `ConfigResult`。调用者（[main](../main.rs/main.md) 或 [hotreload_config](hotreload_config.md)）在用解析结果替换实时配置时负责同步。

### 性能

在解析开始之前，所有行都被读入内存。这是可以接受的，因为配置文件通常很小（最多几百行）。两阶段架构（收集行，然后解析）是必要的，以支持多行组块（解析器必须从 `{` 向前查找到 `}`）。

## 要求

| | |
|---|---|
| **模块** | `config` (`src/config.rs`) |
| **可见性** | `pub` |
| **调用者** | [main](../main.rs/main.md)、[hotreload_config](hotreload_config.md) |
| **被调用函数** | [parse_constant](parse_constant.md)、[parse_alias](parse_alias.md)、[collect_members](collect_members.md)、[collect_group_block](collect_group_block.md)、[parse_and_insert_rules](parse_and_insert_rules.md) |
| **API** | `std::fs::File::open`、`std::io::BufReader`、`std::io::BufRead::lines` |
| **权限** | 配置文件路径的读取权限 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 解析配置输出 | [ConfigResult](ConfigResult.md) |
| 每进程规则结构体 | [ProcessConfig](ProcessConfig.md) |
| 调度器调优常量 | [ConfigConstants](ConfigConstants.md) |
| 规则字段解析器 | [parse_and_insert_rules](parse_and_insert_rules.md) |
| 常量行解析器 | [parse_constant](parse_constant.md) |
| 别名行解析器 | [parse_alias](parse_alias.md) |
| 组块收集器 | [collect_group_block](collect_group_block.md) |
| 成员名分词器 | [collect_members](collect_members.md) |
| 配置热重载 | [hotreload_config](hotreload_config.md) |
| 黑名单文件读取器 | [read_list](read_list.md) |
| 配置模块概述 | [README](README.md) |