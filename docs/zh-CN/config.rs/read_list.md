# read_list 函数 (config.rs)

读取文本文件，将其中非空、非注释的行作为小写字符串向量返回。此实用函数用于加载简单的面向行的列表文件，例如查找模式中使用的黑名单文件。

## 语法

```AffinityServiceRust/src/config.rs#L877-886
pub fn read_list<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .map_while(Result::ok)
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty() && !s.starts_with('#'))
        .collect())
}
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `path` | `P: AsRef<Path>` | 要读取的文本文件的文件系统路径。接受任何可以转换为 `Path` 引用的类型，包括 `&str`、`String` 和 `PathBuf`。 |

## 返回值

类型：`std::io::Result<Vec<String>>`

成功时返回 `Ok`，其中包含一个字符串向量，每个元素是文件中已修剪、已转为小写、非空且不以 `#` 开头的行。失败时返回来自 `File::open` 的 `std::io::Error`（例如，文件未找到、权限被拒绝）。

### 示例

给定包含以下内容的文件：

```/dev/null/example_blacklist.txt#L1-6
# Processes to ignore during find mode
svchost.exe
System
  Explorer.EXE

# End of list
```

返回的向量为：

```/dev/null/example_result.txt#L1-3
["svchost.exe", "system", "explorer.exe"]
```

## 备注

### 处理管道

1. 文件通过 `File::open` 打开，并包装在 `BufReader` 中进行行缓冲读取。
2. 通过 `lines()` 迭代器惰性读取行。`map_while(Result::ok)` 组合子在成功打开后遇到第一个 I/O 错误时停止读取（在这种情况下不会返回部分结果——迭代器只是提前终止并返回已收集的内容）。
3. 每一行去除前后空白并转换为小写，以便在运行时进行不区分大小写的匹配。
4. 修剪后为空的行以及以 `#` 开头的行（注释）被过滤掉。
5. 剩余的行被收集到 `Vec<String>` 中。

### 大小写规范化

所有条目都被转为小写，以便与 Windows 进程名称进行不区分大小写的比较，因为 Windows 进程名称本身就是不区分大小写的。

### 注释语法

修剪后以 `#` 开头的行被视为注释并从结果中排除。不支持行内注释（例如 `svchost.exe # system process`）——整行会按原样被包含（已转为小写并修剪），因为它不以 `#` 开头。

### 错误传播

只有 `File::open` 调用可以产生通过 `?` 操作符传播给调用方的错误。后续的行读取错误被 `map_while(Result::ok)` 静默吸收，它在第一个失败的行读取时终止迭代，而不是传播错误。

### 编码

该函数以 UTF-8 格式读取文件（`BufReader::lines` 的默认编码）。对于以其他格式编码的文件（例如 UTF-16 LE），请改用 [`read_utf16le_file`](read_utf16le_file.md)。

### 用途

`read_list` 主要用于：
- 主循环加载黑名单文件（`-blacklist <file>`），用于查找模式。
- [`hotreload_blacklist`](hotreload_blacklist.md) 在黑名单文件在磁盘上发生变更时重新加载。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `config.rs` |
| 可见性 | `pub` |
| 调用方 | `main.rs`（黑名单加载）、[`hotreload_blacklist`](hotreload_blacklist.md) |
| 被调用方 | `File::open`、`BufReader::new`、`BufRead::lines` |
| API | `std::io::Result`、`std::fs::File`、`std::io::BufReader`、`std::path::Path` |
| 权限 | 对指定路径的文件系统读取权限 |

## 另请参阅

| 资源 | 链接 |
|------|------|
| read_utf16le_file | [read_utf16le_file](read_utf16le_file.md) |
| read_config | [read_config](read_config.md) |
| hotreload_blacklist | [hotreload_blacklist](hotreload_blacklist.md) |
| config 模块概述 | [README](README.md) |

---
*Commit: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)*