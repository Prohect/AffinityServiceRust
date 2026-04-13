# hotreload_blacklist 函数 (config.rs)

检查黑名单文件是否自上次检查以来被修改，如果是则重新加载。

## 语法

```rust
pub fn hotreload_blacklist(
    cli: &CliArgs,
    blacklist: &mut Vec<String>,
    last_blacklist_mod_time: &mut Option<std::time::SystemTime>,
)
```

## 参数

`cli`

指向 [`CliArgs`](../cli.rs/CliArgs.md) 的引用，包含黑名单文件路径。

`blacklist`

当前黑名单向量的可变引用。文件变更时就地更新。

`last_blacklist_mod_time`

跟踪最后已知的修改时间。文件重新加载时更新。如果文件不可访问，设为 `None` 并同时清空黑名单。

## 备注

每次循环迭代中在休眠后由 [`main`](../main.rs/main.md) 调用。处理三种情况：

1. **文件不可访问** — 如果文件无法读取且之前可访问，清空黑名单并重置修改时间。
2. **文件已修改** — 如果修改时间已变更，通过 [`read_list`](read_list.md) 重新加载黑名单。
3. **无变化** — 不执行任何操作。

此函数之前内联在 `main()` 中，现已提取为独立函数以提高可读性。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/config.rs |
| **调用方** | [`main`](../main.rs/main.md) |

## 另请参阅

- [hotreload_config](hotreload_config.md)
- [read_list](read_list.md)
- [config.rs 模块概述](README.md)