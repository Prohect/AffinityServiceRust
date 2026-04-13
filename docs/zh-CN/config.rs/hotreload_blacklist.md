# hotreload_blacklist 函数 (config.rs)

检查黑名单文件的最后修改时间戳，如果文件自上次检查后已被修改，则将其重新加载到内存中。如果文件变得不可访问（例如被删除或锁定），则清空内存中的黑名单。此函数在每次服务循环迭代中调用一次，以支持在不重启服务的情况下实时更新黑名单。

## 语法

```rust
pub fn hotreload_blacklist(
    cli: &CliArgs,
    blacklist: &mut Vec<String>,
    last_blacklist_mod_time: &mut Option<std::time::SystemTime>,
)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `cli` | `&CliArgs` | 已解析的命令行参数的引用。`blacklist_file_name` 字段（`Option<String>`）提供要监控的文件路径。如果为 `None`，函数立即返回而不执行任何操作。 |
| `blacklist` | `&mut Vec<String>` | 服务循环维护的活动黑名单向量的可变引用。重新加载时，此向量将替换为由 [read_list](read_list.md) 新解析的条目。当文件不可访问时，向量将被清空。 |
| `last_blacklist_mod_time` | `&mut Option<std::time::SystemTime>` | 上次检查时缓存的修改时间戳的可变引用。成功重新加载后更新为 `Some(mod_time)`，当文件变得不可访问时重置为 `None`。用于检测文件自上次轮询以来是否已更改。 |

## 返回值

此函数没有返回值。所有输出通过对 `blacklist` 和 `last_blacklist_mod_time` 的修改来传递。

## 备注

### 重新加载算法

1. **未配置黑名单** — 如果 `cli.blacklist_file_name` 为 `None`，函数立即返回。不进行文件监控。
2. **文件元数据检查** — 对黑名单文件路径调用 `std::fs::metadata`。
   - **元数据错误（文件不可访问）：** 如果 `last_blacklist_mod_time` 之前为 `Some(...)`，则清空黑名单，将 `last_blacklist_mod_time` 设置为 `None`，并输出日志消息：`"Blacklist file '{path}' no longer accessible, clearing blacklist."`。如果 `last_blacklist_mod_time` 已经是 `None`，则不采取任何操作（避免在每次循环迭代中重复输出日志消息）。
   - **元数据成功：** 将文件的修改时间与 `*last_blacklist_mod_time` 进行比较。
3. **修改时间比较** — 如果 `Some(mod_time) != *last_blacklist_mod_time`（即文件是新创建的或已被修改），则触发重新加载：
   a. 将 `last_blacklist_mod_time` 更新为 `Some(mod_time)`。
   b. 输出日志消息：`"Blacklist file '{path}' changed, reloading..."`。
   c. 调用 [read_list](read_list.md) 解析文件。出错时，`unwrap_or_default()` 产生一个空向量。
   d. 结果替换 `*blacklist` 的内容。
   e. 输出完成日志消息：`"Blacklist reload complete: {N} items loaded."`。
4. **无变更** — 如果修改时间与缓存值匹配，函数直接返回而不执行任何操作。

### 首次调用行为

首次调用时，`last_blacklist_mod_time` 通常为 `None`（由调用方初始化）。只要文件存在，`Some(mod_time) != None` 求值为 `true`，从而触发初始加载。这意味着该函数通过同一代码路径处理初始加载和后续重新加载。

### 文件删除和重新创建

如果黑名单文件被删除，下一次调用将清空黑名单并将 `last_blacklist_mod_time` 重置为 `None`。如果文件后来被重新创建，后续调用将检测到新的修改时间（`Some(mod_time) != None`）并重新加载它。这允许用户通过删除文件临时禁用黑名单，并通过重新创建文件重新启用它。

### 错误容忍

如果 [read_list](read_list.md) 失败（例如由于瞬态 I/O 错误或编码问题），`unwrap_or_default()` 将静默产生一个空黑名单。修改时间仍会被更新，因此在文件再次修改之前不会重试该错误。随后的文件修改（即使是无操作的保存）将触发另一次重新加载尝试。

### 轮询频率

此函数不管理自己的计时器。它由服务主循环在每次迭代中调用（由 `--interval` 命令行参数控制，默认为几秒钟）。因此，文件系统元数据检查的频率由服务循环间隔决定。

### 日志记录

所有日志输出使用 `log!` 宏，该宏写入服务的日志文件并可选地输出到控制台。函数输出：

- 当文件变得不可访问时输出一条消息。
- 成功重新加载时输出两条消息（开始和完成，包含条目数量）。
- 当文件未更改时不输出任何消息。

### 线程安全性

此函数不是线程安全的。它通过独占可变引用修改 `blacklist` 和 `last_blacklist_mod_time`。调用方（[main](../main.rs/main.md) 中的服务主循环）以单线程运行并独占这些值的所有权。

### 与 hotreload_config 的比较

[hotreload_config](hotreload_config.md) 对主配置文件执行类似的热重载，但包含额外的验证：仅在新的解析结果没有错误时才替换活动配置。`hotreload_blacklist` 更简单 — 它始终用 [read_list](read_list.md) 返回的内容替换黑名单，因为黑名单文件格式没有复杂的验证要求。

## 要求

| | |
|---|---|
| **模块** | `config` (`src/config.rs`) |
| **可见性** | `pub` |
| **调用方** | [main](../main.rs/main.md)（服务循环） |
| **被调用方** | [read_list](read_list.md)、`std::fs::metadata`、`log!` 宏 |
| **API** | `std::fs::metadata` 用于文件系统轮询 |
| **权限** | 对黑名单文件路径的读取权限 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 黑名单文件读取器 | [read_list](read_list.md) |
| 配置文件热重载（类似功能） | [hotreload_config](hotreload_config.md) |
| 命令行参数（blacklist_file_name） | [CliArgs](../cli.rs/CliArgs.md) |
| 服务主循环 | [main](../main.rs/main.md) |
| 配置模块概述 | [README](README.md) |