# drop_module_cache 函数 (winapi.rs)

从全局 [`MODULE_CACHE`](MODULE_CACHE.md) 中移除指定进程的已缓存模块枚举数据，释放内存并确保陈旧的模块信息不会被重复使用。

## 语法

```rust
pub fn drop_module_cache(pid: u32)
```

## 参数

`pid`

要从 [`MODULE_CACHE`](MODULE_CACHE.md) 中移除其缓存模块数据的进程标识符。

## 返回值

此函数不返回值。

## 备注

[`MODULE_CACHE`](MODULE_CACHE.md) 存储每进程的模块枚举结果（基地址、结束地址、模块名称元组），以避免在每次循环迭代中对同一进程重复调用 [`enumerate_process_modules`](enumerate_process_modules.md)。当进程终止或不再被跟踪时，应调用此函数移除其缓存条目。

该函数获取 [`MODULE_CACHE`](MODULE_CACHE.md) 互斥锁并移除以 `pid` 为键的条目。如果给定 PID 不存在对应条目，则该函数为空操作。

如果未对已终止的进程调用此函数，缓存将随着不断遇到新进程而旧条目永远不被清理而无限增长。调用方（通常是 [`main`](../main.rs/main.md) 中的主循环）负责在被跟踪的进程不再存活时调用此函数。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/winapi.rs |
| **源码行** | L710–L713 |
| **调用方** | [`main`](../main.rs/main.md) 循环清理 |
| **修改** | [`MODULE_CACHE`](MODULE_CACHE.md) |

## 另请参阅

- [MODULE_CACHE 静态变量](MODULE_CACHE.md)
- [resolve_address_to_module](resolve_address_to_module.md)
- [enumerate_process_modules](enumerate_process_modules.md)
- [winapi.rs 模块概述](README.md)