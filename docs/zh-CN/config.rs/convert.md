# convert 函数 (config.rs)

将 Process Lasso 配置文件转换为 AffinityServiceRust 原生配置格式。

## 语法

```rust
pub fn convert(in_file: Option<String>, out_file: Option<String>)
```

## 参数

`in_file`

输入 Process Lasso 配置文件的路径（UTF-16 LE 编码的 INI 格式）。此参数为必需项；若为 `None`，则记录错误并立即返回。

`out_file`

输出文件的路径，转换后的 AffinityServiceRust 配置将写入此文件。此参数为必需项；若为 `None`，则记录错误并立即返回。

## 返回值

此函数无返回值。结果直接写入输出文件，诊断信息通过日志系统输出到控制台。

## 备注

此函数读取 Process Lasso 的 INI 风格配置文件（UTF-16 LE 编码），并生成等效的 AffinityServiceRust 配置文件。它从 Process Lasso 格式中解析以下三个关键节：

- **NamedAffinities** — 以逗号分隔的 `别名,cpu_spec` 对，转换为 `*alias = cpu_spec` CPU 别名行。
- **DefaultPriorities** — 以逗号分隔的 `进程名,优先级` 对，映射到输出规则中的优先级字段。数字优先级代码（`1`–`6`）被翻译为命名优先级（`idle`、`below normal`、`normal`、`above normal`、`high`、`real time`）。
- **DefaultAffinitiesEx** — 以逗号分隔的 `进程名,掩码,cpuset` 三元组，其中 cpuset 值用作亲和性字段。如果 cpuset 匹配某个命名亲和性，则使用别名引用（例如 `*alias`）而非原始 CPU 规格。

输出文件包含：

1. 配置帮助行（来自 [get_config_help_lines](../cli.rs/get_config_help_lines.md)）作为文件头注释。
2. 从 `NamedAffinities` 派生的 CPU 别名定义。
3. 以 `name:priority:affinity:0:0:none:none` 格式输出的进程规则，按进程名字母排序。

同时出现在 `DefaultPriorities` 和 `DefaultAffinitiesEx` 中的进程将被合并为单条输出规则。缺失字段默认为：优先级使用 `none`，亲和性使用 `0`。

输入文件通过 [read_utf16le_file](read_utf16le_file.md) 读取，该函数处理 Process Lasso 配置导出文件典型的 UTF-16 LE 编码。

此函数通过 `-convert` CLI 标志配合 `-in` 和 `-out` 参数调用。

### 使用示例

```text
AffinityService.exe -convert -in "C:\ProgramData\Process Lasso\processgovernor.ini" -out config.txt
```

### 输出示例

```text
# Converted from Process Lasso config

# CPU Aliases (from Process Lasso NamedAffinities)
*perf = 0-7
*eff = 8-15

chrome.exe:normal:*perf:0:0:none:none
game.exe:high:*perf:0:0:none:none
obs64.exe:above normal:*eff:0:0:none:none
```

## 要求

| 要求 | 值 |
| --- | --- |
| **所属模块** | src/config.rs |
| **可见性** | `pub` |
| **调用方** | [main](../main.rs/main.md)（当设置 `-convert` CLI 标志时） |
| **依赖** | [read_utf16le_file](read_utf16le_file.md)、[get_config_help_lines](../cli.rs/get_config_help_lines.md) |

## 另请参阅

- [sort_and_group_config](sort_and_group_config.md) — 转换后自动分组具有相同规则的进程
- [read_config](read_config.md) — 解析 `convert` 生成的原生配置格式
- [parse_cpu_spec](parse_cpu_spec.md) — 输出规则中使用的 CPU 规格格式