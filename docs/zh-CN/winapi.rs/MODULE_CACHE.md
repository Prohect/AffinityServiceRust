# MODULE_CACHE 静态变量 (winapi.rs)

按进程缓存的已枚举模块地址范围和名称，供 [resolve_address_to_module](resolve_address_to_module.md) 使用，用于将线程起始地址映射为人类可读的模块名称及偏移量。缓存在首次对每个进程进行地址解析时惰性填充，并在进程退出或通过 [drop_module_cache](drop_module_cache.md) 调用时清除。

## 语法

```rust
#[allow(clippy::type_complexity)]
static MODULE_CACHE: Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
```

## 成员

| 成员 | 类型 | 描述 |
|------|------|------|
| 外部键 | `u32` | 进程 ID（PID），用于索引缓存。每个进程拥有独立的模块列表。 |
| 内部值 | `Vec<(usize, usize, String)>` | 该进程的模块条目向量。每个元组包含：**(1)** `usize` — 模块在目标进程虚拟地址空间中的基地址，**(2)** `usize` — 模块映像的字节大小，**(3)** `String` — 模块的基本名称（例如 `"kernel32.dll"`、`"ntdll.dll"`）。 |

## 备注

### 填充

缓存由 [resolve_address_to_module](resolve_address_to_module.md) 按需填充。当该函数被调用且缓存中尚无对应 PID 的条目时，它会调用 [enumerate_process_modules](enumerate_process_modules.md)，通过 `EnumProcessModulesEx`、`GetModuleInformation` 和 `GetModuleBaseNameW` 遍历目标进程的已加载模块列表。生成的 `(base, size, name)` 元组向量随后被插入缓存，同时返回供即时使用。

### 缓存失效

通过调用 [drop_module_cache](drop_module_cache.md) 可移除特定 PID 的缓存条目，该函数仅调用 `cache.remove(&pid)`。通常在以下情况下执行：

- 进程退出，其 [ProcessStats](../scheduler.rs/ProcessStats.md) 条目被调度器清理时。
- 主循环迭代到新周期，旧条目不应继续保留时。

未实现自动过期或 LRU 淘汰机制。如果进程的模块列表在运行时发生变化（例如通过 `LoadLibrary`），缓存数据将变得过时。对于主线程调度的使用场景，这是可以接受的，因为模块加载通常发生在进程生命周期的早期，而线程起始地址在线程创建时即已固定。

### 线程安全

`HashMap` 被包装在 `std::sync::Mutex` 中。[resolve_address_to_module](resolve_address_to_module.md) 和 [drop_module_cache](drop_module_cache.md) 在操作期间都会获取锁。由于缓存在主服务循环线程上进行读写，实际中的锁竞争很小。

### 内存使用

每个缓存的进程条目包含每个已加载模块一个元组。典型的 Windows 进程加载 50–200 个模块，因此每个条目的大小约为几千字节。缓存随已执行过地址解析的不同 PID 数量线性增长，并在条目被清除时收缩。

### Clippy 抑制

`#[allow(clippy::type_complexity)]` 属性抑制了 Clippy 关于深度嵌套泛型类型的警告。该复杂度是 `HashMap<u32, Vec<(usize, usize, String)>>` 结构所固有的，提取类型别名并不会在此上下文中提高可读性。

## 要求

| | |
|---|---|
| **模块** | `winapi`（`src/winapi.rs`） |
| **Crate 依赖** | `once_cell::sync::Lazy`、`std::sync::Mutex`、`std::collections::HashMap` |
| **填充者** | [resolve_address_to_module](resolve_address_to_module.md)（通过 [enumerate_process_modules](enumerate_process_modules.md)） |
| **失效者** | [drop_module_cache](drop_module_cache.md) |
| **特权** | 目标进程需要 `PROCESS_QUERY_INFORMATION | PROCESS_VM_READ` 权限（由 [enumerate_process_modules](enumerate_process_modules.md) 要求） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 地址到模块的解析 | [resolve_address_to_module](resolve_address_to_module.md) |
| 缓存淘汰 | [drop_module_cache](drop_module_cache.md) |
| 模块枚举实现 | [enumerate_process_modules](enumerate_process_modules.md) |
| 线程起始地址查询 | [get_thread_start_address](get_thread_start_address.md) |
| 主线程调度器（模块名称的消费者） | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| CPU 集合拓扑缓存（类似的惰性全局变量） | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) |
| EnumProcessModulesEx (MSDN) | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-enumprocessmodulesex) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd