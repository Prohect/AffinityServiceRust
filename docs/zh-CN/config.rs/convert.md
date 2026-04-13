# convert 函数 (config.rs)

将 Process Lasso 配置文件（UTF-16 LE 编码的 INI 格式）转换为 AffinityServiceRust 原生配置格式。该函数从 Process Lasso 文件中读取命名亲和性、默认优先级和默认亲和性，将其映射为等效的 AffinityServiceRust 规则语法，并以 UTF-8 编码将结果写入输出文件，包含 CPU 别名和每进程规则行。

## 语法

```rust
pub fn convert(in_file: Option<String>, out_file: Option<String>)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `in_file` | `Option<String>` | 输入 Process Lasso 配置文件的路径（UTF-16 LE 编码）。如果为 `None`，则记录错误消息并立即返回。传递给 [read_utf16le_file](read_utf16le_file.md) 进行解码。 |
| `out_file` | `Option<String>` | 转换后的 AffinityServiceRust 配置将写入的输出文件路径。如果为 `None`，则记录错误消息并立即返回。使用 `File::create` 创建（或覆盖）文件。 |

## 返回值

此函数不返回值。结果写入输出文件，进度/错误消息通过 `log!` 宏输出。

## 备注

### Process Lasso INI 格式

该函数从 Process Lasso 配置中解析三个特定的 INI 键：

| 键 | 格式 | 描述 |
|----|------|------|
| `NamedAffinities=` | `alias1,cpuspec1,alias2,cpuspec2,...` | 以逗号分隔的别名和 CPU 规格对。每对被转换为输出中的 `*alias = cpu_spec` 行。 |
| `DefaultPriorities=` | `process1,priority1,process2,priority2,...` | 以逗号分隔的进程名称和优先级值对。优先级值可以是可读字符串（`"idle"`、`"high"`）或数字代码（`1`–`6`）。 |
| `DefaultAffinitiesEx=` | `process1,mask1,cpuset1,process2,mask2,cpuset2,...` | 以逗号分隔的进程名称、旧版亲和性掩码（通常为 `0`，忽略）和 CPU 集规格的三元组。仅包含 CPU 集非空且不为 `"0"` 的条目。 |

### 转换算法

1. **读取输入** — 通过 [read_utf16le_file](read_utf16le_file.md) 读取输入文件，处理 UTF-16 LE 到 UTF-8 的转换。
2. **解析 INI 键** — 函数遍历各行，将三个已识别键的值提取到中间数据结构中：
   - `named_affinities: Vec<(String, String)>` — 别名/CPU 规格对。
   - `priorities: HashMap<String, String>` — 进程名称 → 优先级字符串。
   - `affinities: HashMap<String, String>` — 进程名称 → CPU 集字符串。
3. **构建别名反向映射** — 从 `named_affinities` 构建 `spec_to_alias` 映射，允许在输出中用 `*alias` 引用替换亲和性值以提高可读性。
4. **生成输出** — 输出文件在内存中以 `Vec<String>` 形式组装：
   a. 配置帮助行（来自 `get_config_help_lines()`）作为注释添加在前面。
   b. 添加 `"# Converted from Process Lasso config"` 标题。
   c. 为每个命名亲和性写入 CPU 别名定义（`*alias = cpu_spec`）。
   d. 对于每个唯一的进程名称（`priorities` 和 `affinities` 键的并集，按字母排序），以 `name:priority:affinity:0:0:none:none` 格式输出一行规则。
5. **写入输出** — 使用 `writeln!` 将组装好的行逐行写入输出文件。

### 优先级映射

Process Lasso 优先级值映射为 AffinityServiceRust 优先级字符串：

| Process Lasso 值 | AffinityServiceRust 等效值 |
|------------------|---------------------------|
| `idle` 或 `1` | `idle` |
| `below normal` 或 `2` | `below normal` |
| `normal` 或 `3` | `normal` |
| `above normal` 或 `4` | `above normal` |
| `high` 或 `5` | `high` |
| `realtime` / `real time` 或 `6` | `real time` |
| *（其他任何值）* | `none` |

字符串匹配不区分大小写（比较前先转为小写）。

### 亲和性别名替换

当进程的亲和性 CPU 规格与已知命名亲和性的 CPU 规格完全匹配时，输出使用 `*alias` 引用而非原始规格。这使转换后的文件更简洁，并且通过编辑单行别名即可更轻松地调整 CPU 分配。

### 输出格式

生成的输出是有效的 AffinityServiceRust 配置文件。每条进程规则使用完整的 7 字段格式，未使用的字段设置为显式默认值：

```text
process.exe:priority:affinity:0:0:none:none
```

其中：
- 字段 3–4（`cpuset`、`prime_cpus`）设置为 `0`（禁用）。
- 字段 5–6（`io_priority`、`memory_priority`）设置为 `none`。

这为用户提供了一个干净的起点，之后可以自定义 cpuset、prime-thread 和其他高级字段。

### 错误处理

| 条件 | 行为 |
|------|------|
| `in_file` 为 `None` | 记录 `"Error: -in <file> is required for -convert"` 并返回。 |
| `out_file` 为 `None` | 记录 `"Error: -out <file> is required for -convert"` 并返回。 |
| 无法读取输入文件 | 记录 `"Failed to read {path}: {error}"` 并返回。 |
| 无法创建输出文件 | 记录 `"Failed to create {path}: {error}"` 并返回。 |
| 写入失败 | 记录 `"Failed to write to {path}"` 并返回。 |

不会向调用方传播错误 — 所有失败均通过 `log!` 报告，函数优雅地返回。

### 日志记录

成功时，函数记录：

```text
Parsed {N} aliases, {N} priorities, {N} affinities
Converted {in_path} to {out_path}
```

### CLI 集成

当用户在命令行传递 `-convert` 标志时调用此函数。`-in` 和 `-out` 参数分别提供 `in_file` 和 `out_file` 参数。详见 [cli 模块](../cli.rs/README.md) 了解参数解析。

### 限制

- 仅解析三个已识别的 INI 键。其他 Process Lasso 设置（例如 I/O 优先级、电源计划、看门狗规则）不会被转换。
- 函数不会尝试合并或去重可能对同一进程存在冲突优先级和亲和性的规则 — 它只是输出源文件中找到的内容。
- CPU 规格从 Process Lasso 文件中按原样传递，不进行重新规范化。如果 Process Lasso 文件使用了不常见的格式，可能需要手动清理输出。
- 输入文件必须为 UTF-16 LE 编码。UTF-8 或其他编码将产生乱码输出，且不会有明确的错误提示。

## 要求

| | |
|---|---|
| **模块** | `config` (`src/config.rs`) |
| **可见性** | `pub` |
| **调用方** | [main](../main.rs/main.md)（通过 `-convert` CLI 标志） |
| **被调用方** | [read_utf16le_file](read_utf16le_file.md), `get_config_help_lines`（来自 [cli 模块](../cli.rs/README.md)）, `File::create`, `writeln!` |
| **API** | 输出使用 `std::fs::File::create`，输入使用 [read_utf16le_file](read_utf16le_file.md) |
| **权限** | 需要输入文件的读取权限，输出文件路径的写入权限 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| UTF-16 LE 文件读取器 | [read_utf16le_file](read_utf16le_file.md) |
| 转换输出的自动分组 | [sort_and_group_config](sort_and_group_config.md) |
| 原生配置文件读取器 | [read_config](read_config.md) |
| CPU 规格解析器 | [parse_cpu_spec](parse_cpu_spec.md) |
| CLI 参数解析 | [cli 模块](../cli.rs/README.md) |
| 模块概述 | [README](README.md) |