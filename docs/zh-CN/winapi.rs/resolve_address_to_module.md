# resolve_address_to_module 函数 (winapi.rs)

将进程内的内存地址解析为 `"module.dll+0xABC"` 格式的人类可读字符串，通过在进程的已加载模块列表中查找该地址。

## 语法

```rust
pub fn resolve_address_to_module(pid: u32, address: usize) -> String
```

## 参数

`pid`

目标进程的进程标识符，将在其模块列表中进行搜索。

`address`

要解析的内存地址，通常是通过 [`get_thread_start_address`](get_thread_start_address.md) 获取的线程起始地址。

## 返回值

返回格式为 `"module.dll+0xABC"` 的 `String`，其中 `module.dll` 是包含该地址的模块名称，`0xABC` 是相对于模块基地址的十六进制偏移量。如果该地址不在任何已知模块的范围内，则返回原始地址的备用字符串表示。

## 备注

此函数用于识别线程的起始地址属于哪个模块（DLL 或 EXE）。此信息对理想处理器分配功能至关重要，该功能根据线程起始地址的模块前缀将线程匹配到 CPU 核心（参见 [`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md)）。

该函数使用 [`MODULE_CACHE`](MODULE_CACHE.md) 静态变量来避免每次调用时重复枚举进程的模块。解析流程如下：

1. 锁定 [`MODULE_CACHE`](MODULE_CACHE.md) 并检查给定 `pid` 是否已有缓存的模块列表。
2. 如果尚未缓存，调用 [`enumerate_process_modules`](enumerate_process_modules.md) 填充该进程的缓存条目。
3. 在缓存的模块列表中搜索基地址–结束地址范围包含目标 `address` 的条目。
4. 如果找到，计算偏移量（`address - base`）并将结果格式化为 `"module_name+0xOFFSET"`。
5. 如果没有任何模块范围包含该地址，返回原始地址的字符串表示。

模块缓存按进程存储，跨循环迭代持久化以提高性能。当进程退出时，应调用 [`drop_module_cache`](drop_module_cache.md) 释放陈旧条目。

### 输出格式

输出格式 `"module.dll+0xABC"` 被设计为既在日志文件中便于人类阅读，又可用于配置规则中的模式匹配。模块名称从完整路径中提取（例如使用 `ntdll.dll` 而非 `C:\Windows\System32\ntdll.dll`），偏移量使用带 `0x` 前缀的小写十六进制。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/winapi.rs |
| **源码行** | L682–L708 |
| **调用方** | [`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md)、[`prefetch_all_thread_cycles`](../apply.rs/prefetch_all_thread_cycles.md) |
| **调用** | [`enumerate_process_modules`](enumerate_process_modules.md) |
| **使用** | [`MODULE_CACHE`](MODULE_CACHE.md) |

## 另请参阅

- [MODULE_CACHE 静态变量](MODULE_CACHE.md)
- [enumerate_process_modules](enumerate_process_modules.md)
- [drop_module_cache](drop_module_cache.md)
- [get_thread_start_address](get_thread_start_address.md)
- [winapi.rs 模块概述](README.md)