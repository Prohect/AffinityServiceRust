# process_logs 函数 (main.rs)

扫描由 `-find` 模式生成的 `.find.log` 文件，以发现尚未被任何配置等级或黑名单条目覆盖的进程。对于每个未知进程，该函数尝试使用 [Everything 搜索](https://www.voidtools.com/) (`es.exe`) 在磁盘上定位可执行文件，并将汇总结果写入文本文件以供人工审查。

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

| 参数 | 类型 | 描述 |
|------|------|------|
| `configs` | `&ConfigResult` | 完整解析后的配置结果。用于确定哪些进程名称已被任何进程级别或线程级别的配置等级所覆盖。 |
| `blacklist` | `&[String]` | 应从结果中排除的小写进程名称切片（已知无关的进程）。 |
| `logs_path` | `Option<&str>` | 用于扫描 `.find.log` 文件的目录路径。当为 `None` 时默认使用 `"logs"`。 |
| `output_file` | `Option<&str>` | 写入发现结果的文件路径。当为 `None` 时默认使用 `"new_processes_results.txt"`。 |

## 返回值

此函数没有返回值。

## 备注

### 日志解析

该函数读取 `logs_path` 中所有以 `.find.log` 结尾的文件。对于每一行，它搜索子字符串 `"find "`，并提取紧随其后的标记（直到下一个空格）。仅收集以 `.exe` 结尾的标记。所有名称在比较前均转换为小写。

### 过滤

如果满足以下任一条件，已发现的进程名称将被**排除**：

- 它作为键出现在 `configs.process_level_configs` 的任何等级中。
- 它作为键出现在 `configs.thread_level_configs` 的任何等级中。
- 它存在于 `blacklist` 切片中。

### 可执行文件路径解析

对于每个剩余的（新发现的）进程，该函数通过 `es` 命令行工具（Everything 搜索）以参数 `-utf8-bom -r ^<转义后的名称>$` 执行外部命令。进程名称中的点号会为正则表达式进行转义。命令的标准输出使用与当前控制台输出代码页 (`GetConsoleOutputCP`) 匹配的编码进行解码；代码页 936 映射为 `"gbk"`，其他所有代码页映射为 `"windows-<cp>"`。如果存在 UTF-8 BOM 前缀 (`0xEF 0xBB 0xBF`)，则在解码前将其剥离。

### 输出格式

结果以纯文本形式写入。每个进程条目的格式如下：

```/dev/null/example.txt#L1-5
Process: <名称>
Found:
  <路径1>
  <路径2>
---
```

如果 `es.exe` 未返回任何输出，则打印 `"Not found, result empty"`。如果 `es.exe` 执行完全失败，则打印 `"Not found, es failed"`。

### 副作用

- 设置全局控制台日志标志 (`*get_use_console!() = true`)，使执行期间的所有 `log!` 调用都写入标准输出。
- 通过 `std::fs::write` 将输出文件写入磁盘。错误会被记录但不会导致 panic。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `main.rs` |
| 调用者 | [main](main.md)（当 `cli.process_logs_mode` 为 `true` 时） |
| 被调用者 | `std::fs::read_dir`、`std::fs::read_to_string`、`std::fs::write`、`std::process::Command` (`es`)、`GetConsoleOutputCP`、`encoding_rs::Encoding::for_label_no_replacement` |
| 外部工具 | `es.exe`（Everything 命令行界面）必须在 `PATH` 中 |
| 权限 | 除常规文件系统访问外无需其他权限；不需要管理员权限。 |

## 另请参阅

| 参考 | 链接 |
|------|------|
| process_find | [process_find](process_find.md) |
| main | [main](main.md) |
| main 模块概述 | [README](README.md) |
| config 模块 | [config](../config.rs/README.md) |

---
> Commit SHA: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
