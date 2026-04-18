# sort_and_group_config 函数 (config.rs)

自动将共享相同规则设置的进程分组为命名的组块，以减少配置文件中的重复内容。该函数读取现有配置文件，识别规则字段（优先级、亲和性、cpuset 等）完全相同的进程，将它们合并为 `{ }` 组块并生成组名，然后输出一个紧凑、去重的配置文件。

## 语法

```AffinityServiceRust/src/config.rs#L1065-1066
pub fn sort_and_group_config(in_file: Option<String>, out_file: Option<String>)
```

## 参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `in_file` | `Option<String>` | 要读取和分析的输入配置文件路径。文件必须是 UTF-8 编码的有效 AffinityServiceRust 配置。如果为 `None`，函数记录错误并立即返回。对应 `-in <file>` CLI 参数。 |
| `out_file` | `Option<String>` | 分组后的配置将写入的输出文件路径。如果为 `None`，函数记录错误并立即返回。对应 `-out <file>` CLI 参数。 |

## 返回值

此函数不返回值。输出写入 `out_file` 指定的文件。诊断消息和汇总统计通过 `log!` 宏发出。

## 备注

### 算法概述

1. **读取并划分行** — 逐行读取输入文件并划分为两个部分：
   - **前言区**：注释行（`#` 前缀）、空行、常量定义（`@` 前缀）和别名定义（`*` 前缀）。这些内容在输出中原样保留。
   - **规则区**：其余所有内容 — 单独的进程规则行和 `{ }` 组块。

2. **提取规则键** — 对于每条进程规则（无论是独立的还是位于组块内），将进程名称与规则字符串（第一个 `:` 之后的全部内容）分离。规则字符串成为分组键。

3. **按规则分组** — 使用 `HashMap<String, Vec<String>>` 将每个唯一规则字符串映射到共享该规则的进程名称列表。另有一个 `rule_order` 向量保留插入顺序，确保输出按首次出现的顺序保持稳定排列。

4. **生成分组输出** — 对于每个唯一规则字符串（按插入顺序）：
   - 如果只有**一个**进程使用该规则，则输出为单行规则：`name:rule_string`。
   - 如果**多个**进程共享该规则，则输出为命名组块。组名自动生成为 `grp_0`、`grp_1` 等。

5. **格式化组** — 对于多进程组：
   - 如果单行表示（`grp_N { a: b: c }:rule`）短于 128 个字符，则在一行内输出。
   - 否则使用多行格式，4 空格缩进，128 字符处换行：
     ```/dev/null/example.ini#L1-4
     grp_0 {
         process1.exe: process2.exe: process3.exe
         process4.exe: process5.exe
     }:priority:affinity:cpuset:prime:io:mem:ideal:grade
     ```

6. **去重** — 在每个组内，成员名称在输出前通过 `sort()` 和 `dedup()` 进行字母排序和去重。

### 前言区保留

所有非规则行（注释、空行、常量、别名）被收集到前言区中，在所有规则之前写入输出。前言区末尾的多余空行被修剪为单个分隔行。这确保别名定义对规则区中的 `*alias` 引用仍然可用。

### 组块重新解析

当输入文件已包含 `{ }` 组块时，函数使用 [`collect_group_block`](collect_group_block.md) 和 [`collect_members`](collect_members.md) 重新解析它们，提取成员名称和规则后缀。现有组名被丢弃 — 输出中所有组均接收全新自动生成的名称（`grp_0`、`grp_1`、…）。

### 行长度阈值

常量 `128` 用作单行组输出的最大行长度。超过此阈值的组将格式化为多行块，使用 `const INDENT: &str = "    "`（4 个空格）以提高可读性。

### 输出统计

完成后，函数记录一条汇总消息：

```/dev/null/example.txt#L1-2
Auto-grouped: {total} total process rules → {single} individual + {grouped} processes merged into {groups} groups
Written to {out_path}
```

其中：
- `total` = `single_count + grouped_member_count`
- `single` = 具有唯一规则（无需分组）的进程数
- `grouped` = 被合并到组中的进程总数
- `groups` = 创建的组块数量

### 错误处理

| 条件 | 行为 |
|------|------|
| `in_file` 为 `None` | 记录 `"Error: -in <file> is required for -autogroup"` 并返回。 |
| `out_file` 为 `None` | 记录 `"Error: -out <file> is required for -autogroup"` 并返回。 |
| 无法读取输入文件 | 记录 `"Failed to read {path}: {error}"` 并返回。 |
| 无法创建输出文件 | 记录 `"Failed to create {path}: {error}"` 并返回。 |
| 写入过程中失败 | 记录 `"Failed to write to {path}"` 并返回。 |

### CLI 用法

```/dev/null/example.sh#L1
AffinityServiceRust.exe -autogroup -in config.ini -out config_grouped.ini
```

### 幂等性

对自身输出再次运行 `sort_and_group_config` 会产生等效文件（组名和格式可能不同），因为函数会重新解析所有现有组。但每次运行时组名都会从 `grp_0` 重新编号。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `config.rs` |
| 可见性 | `pub` |
| 调用者 | `main.rs`（当 `cli.autogroup_mode` 为 `true` 时） |
| 被调用者 | [`collect_group_block`](collect_group_block.md)、[`collect_members`](collect_members.md)、`std::fs::read_to_string`、`File::create`、`writeln!`、`log!` |
| 依赖 | [`collections.rs`](../collections.rs/README.md) 中的 `HashMap`；`std::fs::File`、`std::io::Write` |
| 权限 | 对指定路径的文件系统读写访问权限 |

## 另请参阅

| 资源 | 链接 |
|------|------|
| convert | [convert](convert.md) |
| read_config | [read_config](read_config.md) |
| collect_group_block | [collect_group_block](collect_group_block.md) |
| collect_members | [collect_members](collect_members.md) |
| CliArgs | [CliArgs](../cli.rs/CliArgs.md) |
| config 模块概述 | [README](README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*