# resolve_address_to_module 函数 (winapi.rs)

将虚拟内存地址解析为人类可读的模块名称加偏移字符串（如 `"kernel32.dll+0x1A40"`）。使用按 PID 的模块缓存以避免每次调用时重新枚举已加载的模块。

## 语法

```rust
pub fn resolve_address_to_module(pid: u32, address: usize) -> String
```

## 参数

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `pid` | `u32` | 应搜索其模块列表的进程标识符。用作 `MODULE_CACHE` 静态变量的键。 |
| `address` | `usize` | 要解析的虚拟地址。通常是线程的 Win32 起始地址，如从 [get_thread_start_address](get_thread_start_address.md) 获取。 |

## 返回值

`String` — 根据解析结果有三种格式：

| 条件 | 格式 | 示例 |
|-----------|--------|---------|
| `address == 0` | `"0x0"` | `"0x0"` |
| 地址落在已知模块内 | `"{module_name}+0x{offset:X}"` | `"game.dll+0x1A40"` |
| 地址不匹配任何模块 | `"0x{address:X}"` | `"0x7FF612340000"` |

## 说明

### 模块缓存

函数由静态变量 `MODULE_CACHE: Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>>` 支持。对于给定 PID 的第一次调用，函数调用 [enumerate_process_modules](enumerate_process_modules.md) 来构建 `(基地址、大小、模块名称)` 元组列表，然后存储在缓存中。同一 PID 的后续调用重用缓存数据而无需重新枚举。

PID 的缓存条目在以下情况被移除：

- 进程退出并且 [drop_module_cache](drop_module_cache.md) 从 [PrimeThreadScheduler::drop_process_by_pid](../scheduler.rs/PrimeThreadScheduler.md) 调用。

### 地址匹配

函数在线性搜索缓存的模块列表，查找第一个满足 `base <= address < base + size` 的模块。如果找到匹配项，模块内的偏移量（`address - base`）附加到模块名称。如果没有模块跨越该地址，则返回原始十六进制地址。

### 性能

`MODULE_CACHE` 互斥锁在每次调用时获取一次。模块列表从缓存中克隆后即释放锁，随后再执行线性搜索和（可能较慢的）字符串格式化步骤。这最大限度地减少了当多个调用快速连续发生时互斥锁的争用。

### 零地址快捷方式

地址 `0` 立即作为字符串 `"0x0"` 返回，无需获取缓存锁。这是线程起始地址无法查询（例如由于句柄访问权限不足）时的常见情况。

### 使用场景

此函数从两个主要位置调用：

1. **ThreadStats 自定义 `Debug` 实现** — 为人类可读的调试输出解析 `start_address`。
2. **PrimeThreadScheduler::drop_process_by_pid** — 当配置了 `track_top_x_threads` 时，在退出报告中解析线程起始地址。

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用方** | [ThreadStats::fmt (Debug)](../scheduler.rs/ThreadStats.md)、[PrimeThreadScheduler::drop_process_by_pid](../scheduler.rs/PrimeThreadScheduler.md) |
| **被调用方** | [enumerate_process_modules](enumerate_process_modules.md)（在缓存未命中时） |
| **Win32 API** | 无直接调用；依赖 [enumerate_process_modules](enumerate_process_modules.md) 进行模块数据 |
| **特权** | 底层模块枚举需要 `PROCESS_QUERY_INFORMATION` 和 `PROCESS_VM_READ` |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [winapi.rs](README.md) |
| 模块缓存清理 | [drop_module_cache](drop_module_cache.md) |
| 模块枚举 | [enumerate_process_modules](enumerate_process_modules.md) |
| 线程起始地址检索 | [get_thread_start_address](get_thread_start_address.md) |
| 使用此进行 Debug 输出的线程统计 | [ThreadStats](../scheduler.rs/ThreadStats.md) |
| 进程退出报告 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |

*文档版本：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*