# read_bleack_list 函数 (config.rs)

读取黑名单文件，包含要排除在配置执行之外的进程名称。文件中的每一行指定一个进程名称；以 `#` 开头的行被视为注释并被忽略。

> **注意：** 函数名保留原始拼写 `read_bleack_list`，因为它在源代码中出现的方式。

## 语法

```rust
pub fn read_bleack_list<P: AsRef<Path>>(path: P) -> Result<Vec<String>>
```

## 参数

`path: P`

黑名单文件的路径。接受任何实现 `AsRef<Path>` 的类型，如 `&str`、`String` 或 `PathBuf`。该文件应为 UTF-8 文本文件，每行一个进程名。

## 返回值

`Result<Vec<String>>` — 成功时，返回小写进程名的 `Vec<String>`。失败时（例如文件未找到、权限被拒绝），返回底层的 `std::io::Error`。

## 备注

### 文件格式

黑名单文件使用简单的行导向格式：

```
# 这是一个注释
svchost.exe
explorer.exe
# 另一个注释
taskmgr.exe
```

### 解析规则

1. 使用缓冲读取器打开文件并逐行读取。
2. 修剪每行的首尾空白。
3. 将行转换为小写，以便与运行中的进程名进行不区分大小写的匹配。
4. 丢弃空行和以 `#` 开头的行。
5. 所有保留的行收集到 `Vec<String>` 中。

成功加载后，函数通过 `log!` 宏记录加载的黑名单项数（例如，`"12 blacklist items loaded"`）。

### 与热重载的关系

当黑名单文件的修改时间戳更改时，[hotreload_blacklist](hotreload_blacklist.md) 会调用此函数。在热重载期间读取失败时，之前的黑名单通过调用位置的 `unwrap_or_default()` 保留。

### 对服务行为的影响

小写名称出现在黑名单中的进程在 apply 循环中被完全跳过。即使配置文件中存在匹配规则，也不会对黑名单中的进程应用任何进程级或线程级配置。

### 错误处理

如果文件无法打开，函数会传播来自 `File::open` 的 `std::io::Error`。读取错误通过 `map_while(Result::ok)` 导致迭代器在第一次失败时停止，但成功读取的行仍包含在结果中。

## 需求

| | |
|---|---|
| **模块** | [`src/config.rs`](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/config.rs) |
| **调用方** | [hotreload_blacklist](hotreload_blacklist.md), `main.rs` 中的主初始化 |
| **被调用方** | `std::fs::File::open`, `std::io::BufReader`, `log!` 宏 |
| **权限** | 黑名单路径的文件读取访问 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [config.rs](README.md) |
| 黑名单的热重载机制 | [hotreload_blacklist](hotreload_blacklist.md) |
| 配置文件读取器（对应项） | [read_config](read_config.md) |
| 指定黑名单路径的 CLI 参数 | [CliArgs](../cli.rs/CliArgs.md) |

*文档针对 Commit：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*