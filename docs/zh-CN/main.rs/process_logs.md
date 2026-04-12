# process_logs 函数 (main.rs)

处理来自 `-find` 模式的日志文件，发现新进程并通过 Everything 搜索定位可执行文件路径。

## 语法

```rust
fn process_logs(
    configs: &HashMap<u32, HashMap<String, ProcessConfig>>,
    blacklist: &[String],
    logs_path: Option<&str>,
    output_file: Option<&str>,
)
```

## 参数

`configs`

指向当前已加载配置的引用。键为 grade 级别（`u32`），值为进程名到 [ProcessConfig](../config.rs/ProcessConfig.md) 的映射。用于过滤已有配置规则的进程。

`blacklist`

黑名单进程名称切片。匹配的进程将被排除，不出现在输出结果中。

`logs_path`

日志目录路径。若为 `None`，默认使用 `"logs"`。函数扫描此目录下所有 `.find.log` 文件。

`output_file`

输出文件路径。若为 `None`，默认使用 `"new_processes_results.txt"`。

## 返回值

此函数不返回值。结果直接写入 `output_file` 指定的文件。

## 备注

此函数实现了 `-processlogs` 模式的核心逻辑，用于分析 `-find` 模式产生的日志，自动发现尚未纳入配置管理的新进程。

### 算法流程

1. **扫描日志文件** — 遍历 `logs_path` 目录下所有以 `.find.log` 结尾的文件。
2. **提取进程名** — 解析每行日志，查找 `"find "` 关键字后的进程名（必须以 `.exe` 结尾），收集到 `HashSet` 中自动去重。
3. **过滤已知进程** — 排除以下两类进程：
   - 已存在于任意 grade 配置中的进程（`configs` 的所有 grade 层级均检查）
   - 出现在 `blacklist` 中的进程
4. **搜索可执行文件路径** — 对每个新发现的进程，通过 `es.exe`（[Everything](https://www.voidtools.com/) 命令行工具）执行正则搜索 `^{process_name}$`，定位文件系统中的完整路径。使用 `-utf8-bom` 参数请求 UTF-8 输出。
5. **编码处理** — 根据当前控制台代码页（`GetConsoleOutputCP`）选择解码方式：代码页 936 使用 GBK，其他使用对应的 `windows-{codepage}` 编码，回退到 UTF-8。自动跳过 BOM 头（`0xEF 0xBB 0xBF`）。
6. **写入结果** — 将所有结果写入输出文件，每个进程一段，格式如下：

```
Process: {name}
Found:
  {path1}
  {path2}
---
```

若 `es.exe` 未返回结果或执行失败，则输出 `"Not found, result empty"` 或 `"Not found, es failed"`。

### 控制台模式

函数入口处强制启用控制台输出（`*get_use_console!() = true`），因为此模式为交互式工具用途。

### 典型使用方式

```
AffinityServiceRust.exe -processlogs -config config.ini -blacklist blacklist.txt -in logs -out results.txt
```

### 外部依赖

此函数依赖 [Everything](https://www.voidtools.com/) 的命令行工具 `es.exe`。如果系统未安装 Everything 或 `es.exe` 不在 PATH 中，所有搜索将报告失败。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/main.rs |
| **源码行** | L111–L194 |
| **调用者** | [main](main.md)（当 `cli.process_logs_mode` 为 `true` 时） |
| **调用** | [read_config](../config.rs/read_config.md)、[read_list](../config.rs/read_list.md) |
| **Windows API** | [GetConsoleOutputCP](https://learn.microsoft.com/en-us/windows/console/getconsoleoutputcp) |
| **外部工具** | `es.exe`（Everything 命令行搜索） |
| **Crate 依赖** | `encoding_rs`（字符编码转换） |

## 另请参阅

- [main.rs 模块概述](README.md)
- [main](main.md) — 入口点，调度 `-processlogs` 模式
- [CliArgs](../cli.rs/CliArgs.md) — `process_logs_mode`、`in_file_name`、`out_file_name` 字段
- [read_config](../config.rs/read_config.md) — 加载配置文件
- [read_list](../config.rs/read_list.md) — 加载黑名单文件