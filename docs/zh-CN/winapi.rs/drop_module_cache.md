# drop_module_cache 函数 (winapi.rs)

从全局 [MODULE_CACHE](MODULE_CACHE.md) 静态变量中移除给定进程 ID 的缓存模块列表。这确保在进程退出或下次调用 [resolve_address_to_module](resolve_address_to_module.md) 需要全新的模块枚举时，过期的模块信息会被丢弃。

## 语法

```rust
pub fn drop_module_cache(pid: u32)
```

## 参数

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `pid` | `u32` | 应移除其缓存模块列表的进程标识符。 |

## 返回值

此函数不返回值。

## 备注

- 该函数获取 [MODULE_CACHE](MODULE_CACHE.md) `Mutex<HashMap<u32, Vec<(usize, usize, String)>>>` 的锁，并使用提供的 `pid` 键调用 `HashMap::remove`。

- 如果 `pid` 不在缓存中（例如该进程从未被查询过或已被移除），则调用为空操作。

- 此函数的设计意图是在主调度循环中，当进程不再被跟踪时调用，或者在新迭代开始时调用以强制执行全新的模块枚举。如果不显式移除，缓存将无限期保留已终止进程的条目，消耗内存并在新进程复用相同 PID 时可能返回不正确的结果。

- 该函数是轻量级的——它在互斥锁保护下仅执行一次哈希映射查找和移除操作，不涉及系统调用或 I/O。

### 典型调用模式

1. 调度器检测到进程已退出（通过快照比较或 ETW 停止事件）。
2. 调用 `drop_module_cache(pid)` 丢弃过期的模块列表。
3. 如果相同的 PID 之后被新进程复用，[resolve_address_to_module](resolve_address_to_module.md) 将按需重新枚举模块。

### 线程安全

该函数获取 `MODULE_CACHE` 互斥锁，因此可以安全地从多个线程并发调用。但是，调用者应注意同时持有其他锁（例如 `CPU_SET_INFORMATION`）可能会产生锁顺序方面的问题。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `winapi.rs` |
| **调用者** | `apply.rs`、`scheduler.rs` — 进程生命周期管理 |
| **被调用者** | `Mutex::lock`、`HashMap::remove`（标准库） |
| **Win32 API** | 无 |
| **权限** | 无需特殊权限 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| resolve_address_to_module | [resolve_address_to_module](resolve_address_to_module.md) |
| MODULE_CACHE 静态变量 | [MODULE_CACHE](MODULE_CACHE.md) |
| enumerate_process_modules | [enumerate_process_modules](enumerate_process_modules.md) |
| get_thread_start_address | [get_thread_start_address](get_thread_start_address.md) |
| winapi 模块概述 | [README](README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
