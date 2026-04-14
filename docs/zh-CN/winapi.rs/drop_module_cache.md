# drop_module_cache 函数 (winapi.rs)

从全局 [MODULE_CACHE](MODULE_CACHE.md) 中移除指定进程的已缓存模块列表，释放内存并确保下一次对该进程调用 [resolve_address_to_module](resolve_address_to_module.md) 时会重新枚举其已加载的模块。

## 语法

```rust
pub fn drop_module_cache(pid: u32)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 需要移除已缓存模块列表的进程标识符。如果该 PID 不存在于缓存中，则调用为空操作。 |

## 返回值

无。

## 备注

### 用途

[MODULE_CACHE](MODULE_CACHE.md) 存储每个进程的模块枚举结果（每个已加载 DLL/EXE 的基地址、大小和名称），以便对同一进程重复调用 [resolve_address_to_module](resolve_address_to_module.md) 时无需每次都重新枚举模块。但在以下情况下必须使缓存失效：

- 进程退出时，防止过期条目累积。
- 新的主循环迭代开始时，以获取自上次扫描以来已加载或卸载的模块。
- 操作系统将该 PID 回收并分配给新进程时，新进程的模块布局完全不同。

`drop_module_cache` 通过移除给定 PID 的条目来处理所有这些情况。

### 实现

该函数获取 [MODULE_CACHE](MODULE_CACHE.md) 的互斥锁，并对 PID 键调用 `HashMap::remove`。如果该 PID 不在映射中，`remove` 返回 `None`，不执行任何操作。当 `MutexGuard` 在函数末尾离开作用域时，锁被释放。

### 线程安全

对 [MODULE_CACHE](MODULE_CACHE.md) 的访问通过 `std::sync::Mutex` 进行序列化。锁仅在 `remove` 调用期间持有，对于 `HashMap` 而言该操作的摊销时间复杂度为 O(1)。

### 与 resolve_address_to_module 的关系

缓存条目的典型生命周期如下：

1. 对一个尚未在缓存中的 PID 调用 [resolve_address_to_module](resolve_address_to_module.md)。
2. 该函数调用 [enumerate_process_modules](enumerate_process_modules.md) 并将结果存储在 [MODULE_CACHE](MODULE_CACHE.md) 中。
3. 后续对同一 PID 调用 `resolve_address_to_module` 时使用已缓存的数据。
4. 在循环迭代结束或进程退出时，`drop_module_cache` 移除该条目。
5. 如果进程在下一次迭代中仍在运行，则从步骤 1 开始使用新的模块数据重复执行。

### 调用位置

该函数在调度器模块中进程退出时（`PrimeThreadScheduler::drop_process_by_pid` 期间）以及主循环迭代之间被调用，以防止过期数据在轮询周期之间持续存在。

## 要求

| | |
|---|---|
| **模块** | `winapi` (`src/winapi.rs`) |
| **可见性** | `pub` |
| **调用者** | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md)（进程退出清理）、[`main.rs`](../main.rs/README.md) 中的主循环 |
| **被调用者** | `Mutex::lock`、`HashMap::remove`（标准库） |
| **依赖** | [MODULE_CACHE](MODULE_CACHE.md) |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块地址解析 | [resolve_address_to_module](resolve_address_to_module.md) |
| 全局模块缓存 | [MODULE_CACHE](MODULE_CACHE.md) |
| 模块枚举 | [enumerate_process_modules](enumerate_process_modules.md) |
| 主线程调度器（进程退出清理） | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| 线程起始地址查询 | [get_thread_start_address](get_thread_start_address.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd