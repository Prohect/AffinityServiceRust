# read_list 函数 (config.rs)

读取一个简单的逐行列表文件，返回经过修剪、小写化且非空、非注释的字符串向量。此函数用于加载黑名单文件，该文件包含服务在应用规则时应跳过的进程名称。

## 语法

```rust
pub fn read_list<P: AsRef<Path>>(path: P) -> Result<Vec<String>>
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `path` | `P: AsRef<Path>` | 列表文件的文件系统路径。接受任何实现了 `AsRef<Path>` 的类型，包括 `&str`、`String` 和 `PathBuf`。文件通过 `File::open` 打开，并通过缓冲读取器逐行读取。 |

## 返回值

返回 `Result<Vec<String>>`：

- **`Ok(Vec<String>)`** — 从文件中提取的小写化、修剪后的进程名称向量。向量按文件中条目出现的顺序排列，空行和注释行已被移除。
- **`Err(io::Error)`** — 当文件无法打开时（例如文件未找到、权限被拒绝），从 `File::open` 传播的错误。

## 备注

### 解析规则

对于文件中的每一行：

1. 去除行首尾的空白字符。
2. 将修剪后的行通过 `to_lowercase()` 转换为小写。
3. 如果满足以下条件则**跳过**该行：
   - 修剪后为空。
   - 以 `#` 开头（视为注释）。
4. 所有剩余的行被收集到返回的向量中。

### 文件格式

列表文件是一个纯文本文件，每行一个条目。支持注释和空行以提高可读性：

```text
# 要跳过的进程
system
svchost.exe
csrss.exe

# 开发工具
devenv.exe
code.exe
```

### 大小写规范化

所有条目都被转换为小写，以确保服务在将运行中的进程名称与黑名单进行比较时执行大小写不敏感的匹配。Windows 进程名称不区分大小写，因此此规范化与配置系统的其余部分保持一致。

### 编码

文件使用 Rust 默认的 UTF-8 文件 I/O 进行读取。包含无效 UTF-8 的行会被 `map_while(Result::ok)` 迭代器适配器静默丢弃——不会为单个格式错误的行引发错误。

### 与 read_config 的区别

与处理复杂多格式配置语言的 [read_config](read_config.md) 不同，`read_list` 执行简单的逐行提取，不涉及别名、常量、组或规则字段的特殊语法。每一行要么是进程名称，要么是注释。

### 黑名单用法

`read_list` 的主要使用者是服务主循环，它在启动时以及通过 [hotreload_blacklist](hotreload_blacklist.md) 加载黑名单。在应用阶段，小写化后的进程名称出现在黑名单向量中的进程将被跳过，无论配置中是否存在匹配的规则。

### 错误传播

`File::open` 上的 `?` 运算符意味着文件未找到或权限错误会作为 `io::Error` 传播给调用者。调用者负责处理此错误——例如，[hotreload_blacklist](hotreload_blacklist.md) 使用 `unwrap_or_default()` 在出错时回退到空黑名单。

## 要求

| | |
|---|---|
| **模块** | `config` (`src/config.rs`) |
| **可见性** | `pub` |
| **调用者** | [main](../main.rs/main.md)、[hotreload_blacklist](hotreload_blacklist.md) |
| **被调用者** | `std::fs::File::open`、`std::io::BufReader`、`std::io::BufRead::lines` |
| **API** | 仅使用标准库文件 I/O——无 Windows API 调用 |
| **权限** | 对列表文件路径的读取权限 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 主配置文件读取器 | [read_config](read_config.md) |
| 黑名单热重载 | [hotreload_blacklist](hotreload_blacklist.md) |
| UTF-16 LE 文件读取器 | [read_utf16le_file](read_utf16le_file.md) |
| 模块概述 | [config 模块](README.md) |