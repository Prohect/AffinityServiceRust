# process_logs 函数 (main.rs)

处理 find 模式的日志文件以发现新的未管理进程。从日志目录读取所有 `.find.log` 文件，提取唯一的进程名称，过滤掉已存在于配置或黑名单中的进程，并使用 [Everything 搜索](https://www.voidtools.com/)（`es.exe`）在磁盘上定位可执行文件路径。结果写入文本文件供手动审查。

## 语法

```rust
fn process_logs(
    configs: &ConfigResult,
    blacklist: &[String],
    logs_path: Option<&str>,
    output_file: Option<&str>,
)
```

## 参数

`configs: &ConfigResult`

已解析的 [ConfigResult](../config.rs/ConfigResult.md)，包含所有进程级和线程级配置映射。用于确定哪些进程已被管理。

`blacklist: &[String]`

小写进程名称列表，除 `configs` 中已有的进程外，这些进程也应从结果中排除。

`logs_path: Option<&str>`

包含 `.find.log` 文件的目录路径。当为 `None` 时默认为 `"logs"`。

`output_file: Option<&str>`

写入结果的输出文本文件路径。当为 `None` 时默认为 `"new_processes_results.txt"`。

## 返回值

此函数不返回值。输出写入 `output_file` 指定的文件。

## 备注

该函数实现了一个多阶段管道：

1. **日志扫描** — 遍历 `logs_path` 中所有文件名以 `.find.log` 结尾的文件。对于每个文件，解析行并查找 `"find <process_name>"` 模式，提取以 `.exe` 结尾的进程名称。所有名称转为小写并收集到 `HashSet` 中进行去重。

2. **过滤** — 移除出现在任何等级的 `configs.process_level_configs` 或 `configs.thread_level_configs` 中的进程名称，以及出现在 `blacklist` 中的名称。

3. **可执行文件定位** — 对于每个剩余的进程名称，调用 Everything 命令行接口（`es.exe`），使用 `-utf8-bom -r ^<escaped_name>$` 执行正则表达式搜索以精确匹配文件名。进程名称中的 `.` 字符被转义为 `\.` 以确保正则匹配正确。

4. **编码处理** — 通过 `GetConsoleOutputCP` 检测控制台输出代码页，并选择适当的编码（代码页 936 使用 GBK，否则使用 `windows-<codepage>`）。`es.exe` 的输出使用此编码解码。如果存在 UTF-8 BOM 前缀（`0xEF 0xBB 0xBF`），则会被剥离。

5. **输出格式化** — 每个进程以块格式写入：
   - `Process: <name>` 标题
   - `Found:` 部分包含缩进的路径，或 `Not found, result empty` / `Not found, es failed`
   - `---` 分隔符

### 副作用

- 将全局 `use_console` 标志设置为 `true`（强制控制台输出模式）。
- 生成外部 `es.exe` 进程。如果 Everything 搜索未安装或 `es.exe` 不在 `PATH` 中，所有查找将报告失败。

### 编码边界情况

该函数通过使用 `encoding_rs` crate 处理 `es.exe` 输出编码与 Rust UTF-8 字符串之间的不匹配。在中文区域设置系统（代码页 936）上使用 GBK 解码；在其他区域设置上选择相应的 Windows 代码页编码。

## 要求

| | |
|---|---|
| **模块** | `src/main.rs` |
| **调用方** | [main](main.md)（当 `cli.process_logs_mode` 为 `true` 时） |
| **被调用方** | `std::fs::read_dir`、`std::fs::read_to_string`、`std::fs::write`、`std::process::Command`（`es.exe`）、`GetConsoleOutputCP`、`encoding_rs::Encoding::for_label_no_replacement` |
| **外部工具** | [Everything CLI (`es.exe`)](https://www.voidtools.com/support/everything/command_line_interface/) |
| **权限** | 仅需文件系统读写访问权限 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| Find 模式运行时发现 | [process_find](process_find.md) |
| 配置类型 | [ConfigResult](../config.rs/ConfigResult.md) |
| 入口点和模式分发 | [main](main.md) |
| 日志基础设施 | [logging.rs](../logging.rs/README.md) |

*为提交 [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf) 记录*