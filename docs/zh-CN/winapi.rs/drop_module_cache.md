# drop_module_cache 函数 (winapi.rs)

从全局 [MODULE_CACHE](README.md) 静态变量中删除特定进程的缓存模块枚举数据。在进程退出时调用，以释放内存并确保不会在 OS 随后重用该 PID 时使用过时的模块数据。

## 语法

```rust
pub fn drop_module_cache(pid: u32)
```

## 参数

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `pid` | `u32` | 应删除其缓存模块数据的进程标识符。 |

## 返回值

此函数不返回值。

## 说明

- 函数获取 `MODULE_CACHE` 互斥锁的锁，调用 `HashMap::remove(&pid)`，然后释放锁。如果给定 PID 没有条目，调用即为空操作。
- `MODULE_CACHE` 是一个 `Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>>`，将每个 PID 映射到 `(基地址、大小、模块名称)` 元组列表。条目按需填充，通过 [resolve_address_to_module](resolve_address_to_module.md) 在首次请求给定进程的地址解析时填充。
- 此函数由 [PrimeThreadScheduler::drop_process_by_pid](../scheduler.rs/PrimeThreadScheduler.md) 在进程退出清理期间调用。在此时丢弃缓存可确保：
  1. 当进程不再被跟踪时，用于模块列表的内存会立即释放。
  2. 如果 OS 为新的进程重用相同的 PID，旧进程的过时模块列表将不会由 [resolve_address_to_module](resolve_address_to_module.md) 返回。
- 此函数**不**关闭任何句柄 — 模块枚举在 [enumerate_process_modules](enumerate_process_modules.md) 内打开并关闭临时句柄。

### 时机

在典型的生命周期中，`drop_module_cache` 在每个跟踪的进程退出时调用一次。在进程存活期间的正常操作中不调用它，因此缓存会在进程的整个跟踪期内保留，以避免冗余的 `EnumProcessModulesEx` 调用。

## 要求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用方** | [PrimeThreadScheduler::drop_process_by_pid](../scheduler.rs/PrimeThreadScheduler.md) |
| **被调用方** | 无（仅操作 `MODULE_CACHE` 静态变量） |
| **Win32 API** | 无 |
| **权限** | 无 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [winapi.rs](README.md) |
| 填充缓存的地址解析 | [resolve_address_to_module](resolve_address_to_module.md) |
| 填充缓存条目的模块枚举 | [enumerate_process_modules](enumerate_process_modules.md) |
| 调用此函数的进程清理 | [PrimeThreadScheduler::drop_process_by_pid](../scheduler.rs/PrimeThreadScheduler.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*