# process_logs 函数 (main.rs)

处理由 `-find` 模式生成的 `.find.log` 文件，以发现新的未管理进程。该函数扫描日志目录中的 find-log 条目，过滤掉已被加载的配置或黑名单覆盖的进程，并使用 Everything 搜索引擎命令行工具 (`es.exe`) 解析可执行文件路径。结果写入输出文本文件，供手动审查并可能纳入配置。

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

对按等级键控的配置映射的引用。外层 `HashMap<u32, ...>` 以等级值为键，内层 `HashMap<String, ProcessConfig>` 以小写进程名为键。用于过滤掉在任何等级层级中已有配置规则的进程。

`blacklist`

一个小写进程名的切片，应从结果中排除。出现在黑名单中的进程被视为有意不管理，不会作为新发现报告。

`logs_path`

包含 `.find.log` 文件的目录的可选路径。当为 `None` 时，默认为 `"logs"`（相对于当前工作目录）。对应 `-in` 命令行参数。

`output_file`

写入结果的输出文件的可选路径。当为 `None` 时，默认为 `"new_processes_results.txt"`。对应 `-out` 命令行参数。

## 返回值

此函数不返回值。结果写入指定的输出文件。如果文件写入失败，会在控制台记录错误消息。

## 备注

### 日志文件格式

该函数读取日志目录中所有文件名以 `.find.log` 结尾的文件。每行应包含子字符串 `"find "`，后跟进程名。解析器提取 `"find "` 之后到下一个空格（或行尾）之间的标记，并仅保留以 `.exe` 结尾的条目。所有进程名都转为小写以进行不区分大小写的比较。

典型的 `.find.log` 行格式如下：

```
[12:34:56]find notepad.exe default affinity
```

### 过滤流程

1. **收集** — 从日志目录中每个 `.find.log` 文件收集所有唯一的 `.exe` 进程名到一个 `HashSet` 中。
2. **按配置过滤** — 移除出现在 `configs` 任何等级层级中的进程（已被管理）。
3. **按黑名单过滤** — 移除出现在黑名单中的进程（已被有意排除）。
4. **解析路径** — 对每个剩余进程，调用 `es.exe`（Everything 命令行界面）进行正则搜索（`-r` 标志和 `-utf8-bom` 输出），查找磁盘上的完整文件路径。

### Everything 搜索集成

该函数使用以下参数调用 `es.exe`：

- `-utf8-bom` — 请求带 BOM 的 UTF-8 输出。
- `-r` — 启用正则模式。
- `^<escaped_name>$` — 锚定的正则表达式，匹配精确的可执行文件名（点号被转义为字面 `.`）。

`es.exe` 的标准输出使用系统当前的控制台输出代码页（通过 `GetConsoleOutputCP` 获取）进行解码。对于代码页 936 (GBK)，使用 `"gbk"` 编码标签；否则，构造 `"windows-<codepage>"` 标签。如果编码不被识别，使用 UTF-8 作为后备方案。如果存在 BOM 前缀（`0xEF 0xBB 0xBF`），在解码前会被去除。

如果 `es.exe` 未安装或执行失败，输出文件会为该进程记录 `"Not found, es failed"`。如果搜索成功但未返回结果，则记录 `"Not found, result empty"`。

### 输出格式

输出文件为每个发现的进程包含一个块：

```
Process: example.exe
Found:
  C:\Program Files\Example\example.exe
  D:\Games\Example\example.exe
---
```

或者，如果未找到可执行文件：

```
Process: unknown.exe
Not found, es failed
---
```

### 控制台模式

此函数强制控制台输出（`*get_use_console!() = true`），因为它用于通过 `-processlogs` 命令行模式进行交互式使用。程序在此函数返回后立即退出。

### 典型命令行调用

```
AffinityServiceRust.exe -processlogs -config config.ini -blacklist blacklist.txt -in logs -out new_processes_results.txt
```

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `main` |
| 调用者 | [main](main.md)（当 `cli.process_logs_mode` 为 `true` 时） |
| 被调用者 | `std::fs::read_dir`、`std::fs::read_to_string`、`std::fs::write`、`std::process::Command`（`es.exe`）、`GetConsoleOutputCP` |
| 外部工具 | [Everything CLI (es.exe)](https://www.voidtools.com/support/everything/command_line_interface/) 必须在 `PATH` 中才能进行路径解析 |
| 权限 | 无（不打开进程句柄） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 查找模式（运行时进程发现） | [process_find](process_find.md) |
| 命令行参数与模式标志 | [CliArgs](../cli.rs/CliArgs.md) |
| 配置加载 | [read_config](../config.rs/read_config.md) |
| 黑名单加载 | [read_list](../config.rs/read_list.md) |
| 主入口点 | [main](main.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd