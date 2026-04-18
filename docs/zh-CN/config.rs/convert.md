# convert 函数 (config.rs)

将 Process Lasso 配置文件（UTF-16 LE 编码的 INI 格式）转换为 AffinityServiceRust 的原生配置格式。该函数从输入文件中读取命名亲和性、默认优先级和默认亲和性，将它们映射为等效的 AffinityServiceRust 规则语法，并将结果写入指定的输出文件，同时在文件头部附加完整的配置参考信息。

## 语法

```AffinityServiceRust/src/config.rs#L908-1063
pub fn convert(in_file: Option<String>, out_file: Option<String>)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `in_file` | `Option<String>` | 输入 Process Lasso 配置文件的路径。该文件必须为 UTF-16 LE 编码（Process Lasso 使用的默认编码）。如果为 `None`，函数记录错误并立即返回。对应 `-in <file>` CLI 参数。 |
| `out_file` | `Option<String>` | 转换后的 AffinityServiceRust 配置将写入的输出文件路径。如果为 `None`，函数记录错误并立即返回。对应 `-out <file>` CLI 参数。 |

## 返回值

此函数不返回值。输出写入 `out_file` 指定的文件。诊断消息通过 `log!` 宏发出。

## 备注

### 解析的 Process Lasso INI 键

该函数逐行扫描输入文件，从三个特定的 INI 键中提取数据：

| 键 | 格式 | 描述 |
|----|------|------|
| `NamedAffinities=` | `alias,cpus,alias,cpus,...` | 以逗号分隔的别名和 CPU 规格配对。每对转换为输出中的 `*alias = cpus` 别名定义。 |
| `DefaultPriorities=` | `process,priority,process,priority,...` | 以逗号分隔的进程名称（小写）和优先级值配对。优先级值可以是字符串名称（`"idle"`、`"normal"` 等）或数字代码（`1`–`6`）。 |
| `DefaultAffinitiesEx=` | `process,mask,cpuset,process,mask,cpuset,...` | 以逗号分隔的进程名称、旧式位掩码（通常为 `0`，被忽略）和 CPU 集合规格三元组。仅使用 CPU 集合值；CPU 集合为 `"0"` 或为空的条目将被跳过。 |

### 优先级映射

Process Lasso 数字优先级代码映射到 AffinityServiceRust 的字符串名称：

| 数字代码 | 字符串等效 | AffinityServiceRust 值 |
|---------|-----------|----------------------|
| `1` | `idle` | `idle` |
| `2` | `below normal` | `below normal` |
| `3` | `normal` | `normal` |
| `4` | `above normal` | `above normal` |
| `5` | `high` | `high` |
| `6` | `realtime` / `real time` | `real time` |
| 其他 | — | `none` |

### 输出结构

生成的输出文件按以下顺序包含三个部分：

1. **配置参考头部** — 来自 [`get_config_help_lines`](../cli.rs/get_config_help_lines.md) 的完整配置文件帮助模板，为用户提供语法文档。
2. **CPU 别名** — 每个 `NamedAffinities` 条目对应一行 `*alias = cpu_spec`，位于标题 `# CPU Aliases (from Process Lasso NamedAffinities)` 下。
3. **进程规则** — 在 `DefaultPriorities` 和 `DefaultAffinitiesEx` 中找到的每个唯一进程对应一行规则，按字母顺序排列。每行遵循格式：`name:priority:affinity:0:0:none:none`。在可能的情况下，字面 CPU 规格会使用命名亲和性的反向查找替换为 `*alias` 引用。

### 别名到规格的反向查找

该函数构建一个 `spec_to_alias` 映射，将原始 CPU 规格字符串映射回其 `*alias` 名称。写入进程规则时，如果进程的亲和性规格与已知的命名亲和性匹配，则使用 `*alias` 形式代替原始规格。这会生成更清晰、更易读的输出。

### 文件编码

输入文件通过 [`read_utf16le_file`](read_utf16le_file.md) 读取，该函数使用有损转换将 UTF-16 LE 字节对解码为 Rust `String`。输出文件以 UTF-8 编码写入。

### 错误处理

| 条件 | 行为 |
|------|------|
| `in_file` 为 `None` | 记录 `"Error: -in <file> is required for -convert"` 并返回。 |
| `out_file` 为 `None` | 记录 `"Error: -out <file> is required for -convert"` 并返回。 |
| 无法读取输入文件 | 记录 `"Failed to read {path}: {error}"` 并返回。 |
| 无法创建输出文件 | 记录 `"Failed to create {path}: {error}"` 并返回。 |
| 输出期间写入失败 | 记录 `"Failed to write to {path}"` 并返回。 |

成功时，函数记录摘要：`"Parsed {n} aliases, {n} priorities, {n} affinities"`，随后 `"Converted {in_path} to {out_path}"`。

### CLI 用法

```/dev/null/example.sh#L1
AffinityServiceRust.exe -convert -in ProcessLassoConfig.ini -out config.ini
```

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `config.rs` |
| 可见性 | `pub` |
| 调用方 | `main.rs`（当 `cli.convert_mode` 为 `true` 时） |
| 被调用方 | [`read_utf16le_file`](read_utf16le_file.md)、[`get_config_help_lines`](../cli.rs/get_config_help_lines.md)、`File::create`、`writeln!`、`log!` |
| 依赖 | [`collections.rs`](../collections.rs/README.md) 中的 `HashMap`、`HashSet`；`std::fs::File`、`std::io::Write` |
| 权限 | 无（需要文件系统读写访问权限） |

## 另请参阅

| 资源 | 链接 |
|------|------|
| sort_and_group_config | [sort_and_group_config](sort_and_group_config.md) |
| read_utf16le_file | [read_utf16le_file](read_utf16le_file.md) |
| get_config_help_lines | [get_config_help_lines](../cli.rs/get_config_help_lines.md) |
| read_config | [read_config](read_config.md) |
| CliArgs | [CliArgs](../cli.rs/CliArgs.md) |
| config 模块概述 | [README](README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*