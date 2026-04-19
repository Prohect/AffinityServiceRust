# convert 函数 (config.rs)

将 Process Lasso 配置文件转换为 AffinityServiceRust 原生格式。读取 UTF-16LE 编码的 Process Lasso INI 风格配置，生成带有 CPU 别名和每进程规则行的 UTF-8 配置文件。

## 语法

```rust
pub fn convert(in_file: Option<String>, out_file: Option<String>)
```

## 参数

`in_file: Option<String>`

输入 Process Lasso 配置文件的路径。文件必须是 UTF-16LE 编码（Process Lasso 使用的默认编码）。如果为 `None`，则函数记录错误并立即返回。

`out_file: Option<String>`

输出 AffinityServiceRust 配置文件的路径。文件将被创建或覆盖为 UTF-8 编码内容。如果为 `None`，则函数记录错误并立即返回。

## 返回值

此函数不返回值。成功和失败通过日志消息传达。

## 备注

### Process Lasso 配置格式

转换器识别 Process Lasso 配置文件中的三种 INI 风格键值对：

| 键 | 格式 | 描述 |
|-----|--------|-------------|
| `NamedAffinities` | `alias,cpus,alias,cpus,...` | 命名 CPU 亲和性别名（逗号分隔的名称和 CPU 规格对） |
| `DefaultPriorities` | `process,priority,process,priority,...` | 进程优先级分配（逗号分隔的进程名和数字/字符串优先级对） |
| `DefaultAffinitiesEx` | `process,mask,cpuset,process,mask,cpuset,...` | 进程 CPU 亲和性分配（逗号分隔的进程名、旧掩码和 CPU 范围三元组） |

### 转换流程

1. **读取输入：** 文件通过 [read_utf16le_file](read_utf16le_file.md) 以 UTF-16LE 读取。
2. **解析部分：** 每个识别的键被解析到中间哈希表（`priorities`、`affinities`）和命名亲和性列表。
3. **构建别名反向映射：** 构建 `spec_to_alias` 映射，使得当进程的 CPU 规格匹配已知命名亲和性时，输出使用 `*alias` 引用而非原始规格。
4. **生成头部：** 输出以 `get_config_help_lines()` 的帮助行（来自 cli 模块）和转换注释开始。
5. **发射 CPU 别名：** 每个 `NamedAffinities` 条目作为 `*name = cpu_spec` 别名行发射。
6. **发射进程规则：** 所有唯一的进程名（来自 priorities 和 affinities 映射）按字母顺序排序，并以 `name:priority:affinity:0:0:none:none` 格式发射为单行规则。
7. **写入输出：** 生成的行被写入输出文件。

### 优先级映射

Process Lasso 使用字符串和数字优先级标识符。转换器将它们映射到 AffinityServiceRust 优先级名称：

| Process Lasso 值 | AffinityServiceRust 值 |
|------------------|----------------------|
| `"idle"` 或 `"1"` | `idle` |
| `"below normal"` 或 `"2"` | `below normal` |
| `"normal"` 或 `"3"` | `normal` |
| `"above normal"` 或 `"4"` | `above normal` |
| `"high"` 或 `"5"` | `high` |
| `"realtime"`, `"real time"` 或 `"6"` | `real time` |
| 无法识别 | `none` |

### 限制

- 转换器仅处理进程级设置（亲和性和优先级）。线程级功能如 Prime 线程调度、理想处理器分配和 IO/内存优先级不存在于 Process Lasso 配置中，在输出中默认为 `0`/`none`。
- `DefaultAffinitiesEx` 中的 CPU 集合信息放置在亲和性字段中，而非 CPU 集合字段。输出格式使用 `name:priority:affinity:0:0:none:none`，其中第三个字段是亲和性，CPU 集合字段为 `0`（未更改）。
- `DefaultAffinitiesEx` 三元组中的旧掩码字段被忽略；仅使用 CPU 范围（每个三元组的第三个元素）。
- 命名亲和性别名匹配基于 CPU 规格的精确字符串比较。如果进程的亲和性规格不与命名亲和性字符串完全匹配，则发射原始规格而非别名引用。

### 错误处理

- 如果 `in_file` 或 `out_file` 为 `None`，记录错误并返回。
- 如果输入文件无法读取（通过 [read_utf16le_file](read_utf16le_file.md)），记录错误并返回。
- 如果输出文件无法创建或写入，记录错误并返回。
- 成功时，记录摘要行，显示已解析的别名、优先级和亲和性数量。

## 需求

| | |
|---|---|
| **模块** | [`src/config.rs`](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/config.rs) |
| **调用方** | CLI 分发（使用 `-convert` 标志时） |
| **被调用方** | [read_utf16le_file](read_utf16le_file.md), `get_config_help_lines`（来自 [cli.rs](../cli.rs/README.md)） |
| **依赖项** | [HashMap](../collections.rs/README.md), [HashSet](../collections.rs/README.md) |
| **权限** | 文件系统读写访问 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [config.rs](README.md) |
| UTF-16LE 文件读取器 | [read_utf16le_file](read_utf16le_file.md) |
| 自动分组实用工具 | [sort_and_group_config](sort_and_group_config.md) |
| CPU 规格解析器 | [parse_cpu_spec](parse_cpu_spec.md) |
| 配置文件读取器 | [read_config](read_config.md) |
| CLI 参数 | [CliArgs](../cli.rs/CliArgs.md) |

*文档针对 Commit：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*