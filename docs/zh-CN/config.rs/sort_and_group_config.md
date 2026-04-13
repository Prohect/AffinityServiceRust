# sort_and_group_config 函数 (config.rs)

读取现有的配置文件，识别共享相同规则设置的进程，并写入一个新的配置文件，将这些进程合并为命名的 `{ ... }` 组块。这减少了重复内容，提高了包含大量相同优先级、亲和性和调度设置的进程的大型配置文件的可维护性。

## 语法

```rust
pub fn sort_and_group_config(in_file: Option<String>, out_file: Option<String>)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `in_file` | `Option<String>` | 要读取和分析的输入配置文件路径。如果为 `None`，则记录错误并立即返回。 |
| `out_file` | `Option<String>` | 要写入的输出配置文件路径。如果为 `None`，则记录错误并立即返回。输出文件将被创建或覆盖。 |

## 返回值

此函数没有返回值。结果写入输出文件，摘要统计信息通过日志输出。

## 备注

### 用途

随着时间推移，配置文件会积累许多共享相同设置的独立进程规则。例如，十个后台工具可能都共享 `below normal:8-15:0:0:none:none`。`sort_and_group_config` 检测这些重复项并使用紧凑的组表示法重写文件，减少视觉混乱并使批量更改更加容易。

### 算法

1. **读取并分类行** — 将输入文件读入内存。行被分为两类：
   - **前言行** — 注释 (`#`)、空行、常量 (`@`) 和别名 (`*`) 按原样收集，并逐字保留到输出中。
   - **规则行** — 单独的进程规则 (`name:rule`) 和组块 (`{ ... }:rule`) 被分解为成员名称和规则字符串。对于单独规则，规则字符串是第一个 `:` 之后的所有内容；对于组，是 `}:` 之后的后缀。

2. **组块解析** — 当遇到 `{` 时，解析器使用 [collect_group_block](collect_group_block.md)（多行块）或内联解析（单行块）收集成员名称，然后将所有成员与规则后缀字符串关联。

3. **构建规则到成员的映射** — 一个 `HashMap<String, Vec<String>>` 将每个唯一的规则字符串映射到共享该规则的进程名称列表。一个单独的 `Vec<String>` (`rule_order`) 保留不同规则首次遇到的顺序，确保输出顺序稳定。

4. **排序和去重** — 对于每个唯一的规则字符串，成员列表按字母顺序排序并去重。

5. **生成输出** — 首先写入前言行（到最后一个非空前言行为止）。然后，对于每个唯一规则：
   - **单个成员** — 写为普通的单独规则：`name:rule`。
   - **多个成员** — 写为命名组。组名自动生成为 `grp_0`、`grp_1` 等。输出格式取决于行长度：
     - **短组**（总计 < 128 个字符）— 写为单行组：`grp_N { member1: member2: member3 }:rule`。
     - **长组**（≥ 128 个字符）— 写为多行组，使用 4 空格缩进，每行在 128 个字符处换行：

       ```text
       grp_N {
           member1: member2: member3
           member4: member5
       }:rule
       ```

6. **写入并记录日志** — 输出写入 `out_file`。记录摘要：`"Auto-grouped: {total} total process rules → {singles} individual + {grouped} processes merged into {groups} groups"`。

### 前言保留

第一个规则行之前的所有内容（注释、空行、常量定义、别名定义）都以原始形式保留，并在任何规则之前写入输出文件。这确保了 `@CONSTANT` 和 `*alias` 定义保持完整，可供后续分组规则使用。

### 行长度启发式

单行与多行组格式的 128 字符阈值由 `INDENT` 常量（`"    "`，4 个空格）定义。当组的单行表示超过 128 个字符时，函数切换为多行格式。在多行格式中，每个缩进行上的成员名称也在 128 个字符处换行。

### 规则字符串一致性

如果两个进程的完整规则字符串（原始行中第一个 `:` 之后的所有内容）在修剪后完全相同，则认为它们具有"相同的规则"。这是字符串比较，而非语义比较——即使 `high:0-7` 和 `high:0-7:0` 可能产生等效的 [ProcessConfig](ProcessConfig.md) 条目，它们也被视为不同的规则。

### 组命名

自动生成的组名遵循 `grp_0`、`grp_1`、`grp_2` 等模式，每个包含两个或更多成员的组递增。这些名称仅用于文档/可读性——[read_config](read_config.md) 中的配置解析器不使用组名进行匹配。单成员规则不分配组名。

### 错误处理

- 如果 `in_file` 为 `None`，记录 `"Error: -in <file> is required for -autogroup"` 并返回。
- 如果 `out_file` 为 `None`，记录 `"Error: -out <file> is required for -autogroup"` 并返回。
- 如果无法读取输入文件，记录错误并返回。
- 如果无法创建或写入输出文件，记录错误并返回。

不返回 `Result` 或错误类型——所有错误都通过日志记录，函数优雅退出。

### CLI 集成

当用户传递 `-autogroup` CLI 标志以及 `-in <input_config>` 和 `-out <output_config>` 时，会调用此函数。这是一个独立的离线工具，不需要服务处于运行状态。

### 示例

**输入 (`config.txt`)：**

```text
# CPU 别名
*perf = 0-7

game1.exe:high:*perf
game2.exe:high:*perf
game3.exe:high:*perf
helper.exe:normal:0
updater.exe:below normal:8-15
telemetry.exe:below normal:8-15
```

**输出 (`config_grouped.txt`)：**

```text
# CPU 别名
*perf = 0-7

grp_0 { game1.exe: game2.exe: game3.exe }:high:*perf

helper.exe:normal:0

grp_1 { telemetry.exe: updater.exe }:below normal:8-15
```

**日志输出：**

```text
Auto-grouped: 6 total process rules → 1 individual + 5 processes merged into 2 groups
Written to config_grouped.txt
```

### 局限性

- 该函数不会重新解析规则语义——它纯粹基于规则字符串相等性进行分组。文本表示不同但语义等效的规则不会被合并。
- 前言中的别名定义会被保留但不会展开。如果两个规则引用相同的别名但一个使用别名而另一个使用解析后的值，它们会被视为不同的规则。
- 输出文件始终使用组表示法，即使输入使用了不同的格式风格。在输入中穿插在规则行之间的注释不会保留在输出的规则部分中（仅保留前言注释）。

## 要求

| | |
|---|---|
| **模块** | `config` (`src/config.rs`) |
| **可见性** | `pub` |
| **调用者** | [main](../main.rs/main.md)（通过 `-autogroup` CLI 标志） |
| **被调用者** | [collect_members](collect_members.md)、[collect_group_block](collect_group_block.md)、`std::fs::read_to_string`、`std::fs::File::create`、`std::io::Write::writeln` |
| **API** | 仅标准库文件 I/O——无 Windows API 调用 |
| **权限** | 输入文件的读取权限，输出文件的写入权限 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| Process Lasso 配置转换器 | [convert](convert.md) |
| 主配置文件读取器 | [read_config](read_config.md) |
| 成员名称分词器 | [collect_members](collect_members.md) |
| 多行组块收集器 | [collect_group_block](collect_group_block.md) |
| 每进程配置记录 | [ProcessConfig](ProcessConfig.md) |
| CLI 参数解析 | [cli 模块](../cli.rs/README.md) |
| 模块概述 | [config 模块](README.md) |