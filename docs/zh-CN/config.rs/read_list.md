# read_list 函数 (config.rs)

读取简单的逐行列表文件（例如黑名单），返回经过过滤的小写字符串向量。

## 语法

```rust
pub fn read_list<P: AsRef<Path>>(path: P) -> Result<Vec<String>>
```

## 参数

`path`

列表文件的文件系统路径。接受任何实现 `AsRef<Path>` 的类型，例如 `&str`、`String` 或 `PathBuf`。

## 返回值

返回 `Result<Vec<String>>`。成功时返回一个 `Vec<String>`，包含文件中所有非空、非注释行，每行均经过去除首尾空白并转换为小写的处理。如果文件无法打开，则返回 I/O 错误。

## 备注

此函数使用带缓冲的读取器逐行读取纯文本文件。每一行被去除首尾空白后转换为小写。空行或以 `#` 开头的行（注释）将被排除在结果之外。

主要用途是加载黑名单文件，指定需要从配置中排除的进程名称。文件格式非常简单——每行一个条目：

```
# 需要忽略的进程
explorer.exe
taskmgr.exe

# 系统进程
svchost.exe
```

由于所有条目均被转换为小写，与返回列表的匹配操作按惯例不区分大小写。调用方可以直接将小写的进程名称与列表内容进行比较。

与 [read_config](read_config.md) 不同，此函数不解析任何特殊语法，如别名、常量、组或冒号分隔的规则字段。它是用于简单行式数据的轻量工具函数。

对于单行读取中的错误，通过 `map_while(Result::ok)` 静默跳过，因此对于部分损坏的文件，将返回在第一个行级 I/O 错误之前成功读取的所有行。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/config.rs |
| **行号** | L842–L851 |
| **可见性** | `pub` |
| **依赖** | `std::fs::File`、`std::io::{BufRead, BufReader, Result}`、`std::path::Path` |
| **调用方** | `main::main`（用于加载黑名单文件） |

## 另请参阅

- [read_config](read_config.md) — 完整的配置文件解析器
- [read_utf16le_file](read_utf16le_file.md) — 读取 UTF-16 LE 编码的文件