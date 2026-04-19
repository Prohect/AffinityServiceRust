# hotreload_blacklist 函数 (config.rs)

检查黑名单文件在磁盘上是否已被修改，如果修改时间戳已更改则重新加载。此函数每次轮询迭代时调用，以支持在不重启服务的情况下实时更新进程排除列表。

## 语法

```rust
pub fn hotreload_blacklist(
    cli: &CliArgs,
    blacklist: &mut Vec<String>,
    last_blacklist_mod_time: &mut Option<std::time::SystemTime>,
)
```

## 参数

`cli: &CliArgs`

对已解析命令行参数的引用。`blacklist_file_name` 字段（`Option<String>`）指定黑名单文件的路径。如果为 `None`，函数立即返回，不执行任何操作。

`blacklist: &mut Vec<String>`

**\[in, out\]** 内存中当前小写进程名的黑名单。成功重新加载时，此向量被替换为新解析的内容。如果黑名单文件变得不可访问（被删除、移动或权限改变），则清空该向量。

`last_blacklist_mod_time: &mut Option<std::time::SystemTime>`

**\[in, out\]** 跟踪黑名单文件的最后已知修改时间戳。用于检测更改：

- `None` 表示之前没有成功读取（初始状态或文件变得不可访问）。
- `Some(time)` 包含最近一次成功重新加载时的 `modified()` 时间戳。

函数将文件的当前修改时间与此存储值进行比较。如果不同，则触发重新加载并更新此值。如果文件变得不可访问，则重置为 `None`。

## 返回值

此函数不返回值。副作用通过 `blacklist` 和 `last_blacklist_mod_time` 输出参数传达。

## 备注

### 重新加载逻辑

函数遵循以下决策树：

1. **未配置黑名单文件：** 如果 `cli.blacklist_file_name` 为 `None`，则函数立即返回。
2. **文件不可访问：** 如果 `std::fs::metadata()` 对黑名单路径失败：
   - 如果之前记录了修改时间（`last_blacklist_mod_time.is_some()`），则清空黑名单并将时间戳重置为 `None`。
   - 如果之前没有时间戳，则不执行任何操作（避免在启动时当文件不存在时重复记录日志）。
3. **文件未更改：** 如果文件的 `modified()` 时间戳与 `*last_blacklist_mod_time` 匹配，则函数不重新加载而直接返回。
4. **文件已更改：** 如果时间戳不同，则通过 [read_bleack_list](read_bleack_list.md) 重新加载文件。成功时，`*blacklist` 被替换，`*last_blacklist_mod_time` 被更新。失败时，保留之前的黑名单和时间戳。

### 文件消失处理

当之前可访问的黑名单文件变得不可访问时（例如，被用户删除），函数主动清空内存中的黑名单。这确保之前被黑名单排除的进程不再被阻止。会记录一条日志消息：`"Blacklist file '{path}' no longer accessible, clearing blacklist."`。

### 日志记录

- 重新加载时：`"Blacklist file '{path}' changed, reloading..."`
- 重新加载后：`"Blacklist reload complete: {N} items loaded."`
- 文件消失时：`"Blacklist file '{path}' no longer accessible, clearing blacklist."`

### 轮询频率

此函数每次主服务轮询迭代时调用一次。开销极小——每次迭代进行一次 `metadata()` 系统调用（当文件未更改时）。除非修改时间戳不同，否则不会发生文件 I/O。

### 线程安全

此函数不是线程安全的。它设计为从单线程主轮询循环中调用。对 `blacklist` 和 `last_blacklist_mod_time` 的可变引用在编译时强制独占访问。

## 需求

| | |
|---|---|
| **模块** | [`src/config.rs`](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/config.rs) |
| **调用方** | `main.rs` 中的主轮询循环 |
| **被调用方** | [read_bleack_list](read_bleack_list.md), `std::fs::metadata` |
| **依赖项** | [CliArgs](../cli.rs/CliArgs.md) |
| **权限** | 黑名单文件的文件系统读取访问 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [config.rs](README.md) |
| 黑名单文件读取器 | [read_bleack_list](read_bleack_list.md) |
| 配置热重载对应项 | [hotreload_config](hotreload_config.md) |
| CLI 参数结构体 | [CliArgs](../cli.rs/CliArgs.md) |
| 主服务循环 | [main.rs](../main.rs/README.md) |

*文档针对 Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*