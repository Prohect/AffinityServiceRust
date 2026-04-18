# hotreload_blacklist 函数 (config.rs)

通过将黑名单文件的文件系统修改时间戳与缓存值进行比较来监视文件修改，并在检测到更改时重新加载文件内容。此函数在每次轮询迭代中被调用，以支持在不重启服务的情况下实时更新黑名单。

## 语法

```AffinityServiceRust/src/config.rs#L1279-1301
pub fn hotreload_blacklist(
    cli: &CliArgs,
    blacklist: &mut Vec<String>,
    last_blacklist_mod_time: &mut Option<std::time::SystemTime>,
)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `cli` | `&CliArgs` | 已解析的命令行参数的引用。函数读取 `cli.blacklist_file_name` 来确定黑名单文件的路径。如果 `blacklist_file_name` 为 `None`，函数立即返回且不执行任何操作。 |
| `blacklist` | `&mut Vec<String>` | 内存中黑名单向量的可变引用。当黑名单文件被重新加载时，此向量将被替换为新内容。当黑名单文件变得不可访问时，此向量将被清空。 |
| `last_blacklist_mod_time` | `&mut Option<std::time::SystemTime>` | 上次成功检查时黑名单文件的缓存修改时间戳的可变引用。用于检测更改。初始时以及文件变得不可访问时设为 `None`；成功重新加载后设为 `Some(mod_time)`。 |

## 返回值

此函数不返回值。所有结果通过对 `blacklist` 和 `last_blacklist_mod_time` 的修改来传递。

## 备注

### 变更检测算法

1. 如果 `cli.blacklist_file_name` 为 `None`，函数立即返回——未配置黑名单文件。
2. 函数对黑名单文件路径调用 `std::fs::metadata`：
   - **元数据调用失败**（文件被删除、权限变更等）：如果 `last_blacklist_mod_time` 之前为 `Some`（表示文件至少被加载过一次），则清空黑名单，将 `last_blacklist_mod_time` 重置为 `None`，并发出日志消息：`"Blacklist file '{path}' no longer accessible, clearing blacklist."`。如果它已经是 `None`，则不执行任何操作（避免重复的日志输出）。
   - **元数据调用成功**：将文件的修改时间与 `last_blacklist_mod_time` 进行比较。如果时间戳不同（或 `last_blacklist_mod_time` 为 `None`，表示首次加载），则重新加载文件。

3. 重新加载时，函数调用 [`read_bleack_list`](read_bleack_list.md) 将文件内容解析为小写、非空、非注释字符串的向量。如果 `read_bleack_list` 返回错误，则通过 `unwrap_or_default()` 使用空向量作为后备。
4. 缓存的时间戳更新为文件的当前修改时间。
5. 发出日志消息：
   - `"Blacklist file '{path}' changed, reloading..."` — 重新加载之前。
   - `"Blacklist reload complete: {n} items loaded."` — 重新加载之后。

### 文件消失处理

当先前可访问的黑名单文件被删除或变得不可访问时：

- 内存中的黑名单被**清空**（移除所有条目）。
- `last_blacklist_mod_time` 被设为 `None`。
- 发出一条日志消息。只要文件保持不可访问状态，后续轮询迭代不会再次记录日志（因为 `last_blacklist_mod_time` 已经是 `None`）。

如果文件稍后重新出现，下一次元数据检查将成功，检测到时间戳不匹配（因为缓存时间为 `None`），并触发全新的重新加载。

### 首次加载行为

在 `last_blacklist_mod_time` 设为 `None` 且存在有效黑名单文件的首次调用时，函数检测到时间戳不匹配（`Some(mod_time) != None`）并加载文件。这统一处理了初始启动加载和消失后恢复的情况。

### 轮询频率

此函数设计为在每次轮询循环迭代中调用一次。当文件存在且未更改时，每次调用的开销仅为一次 `std::fs::metadata` 系统调用，使其适合频繁调用。

### 线程安全性

此函数不是线程安全的，必须从单个线程（主轮询循环）调用。对 `blacklist` 和 `last_blacklist_mod_time` 的可变引用通过 Rust 的借用检查器在语言层面强制了这一点。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `config.rs` |
| 可见性 | `pub` |
| 调用方 | `main.rs`（主轮询循环） |
| 被调用方 | [`read_bleack_list`](read_bleack_list.md)、`std::fs::metadata`、`log!` 宏 |
| 依赖 | [`CliArgs`](../cli.rs/CliArgs.md)（用于 `blacklist_file_name`）、`std::time::SystemTime` |
| I/O | 文件系统元数据查询 + 通过 [`read_bleack_list`](read_bleack_list.md) 的可选文件读取 |
| 权限 | 对黑名单文件路径的文件系统读取权限 |

## 另请参阅

| 资源 | 链接 |
|------|------|
| hotreload_config | [hotreload_config](hotreload_config.md) |
| read_bleack_list | [read_bleack_list](read_bleack_list.md) |
| CliArgs | [CliArgs](../cli.rs/CliArgs.md) |
| read_config | [read_config](read_config.md) |
| config 模块概述 | [README](README.md) |

---
*提交：[29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*