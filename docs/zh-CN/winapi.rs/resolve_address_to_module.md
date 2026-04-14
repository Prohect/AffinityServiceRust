# resolve_address_to_module 函数 (winapi.rs)

将内存地址解析为包含模块名称和十六进制偏移量的人类可读字符串（例如 `"engine.dll+0x1A3F0"`）。此函数被主线程调度器和理想处理器分配逻辑用于识别线程起始地址所属的已加载模块，从而通过 [PrimePrefix](../config.rs/PrimePrefix.md) 规则实现基于模块的线程过滤。

## 语法

```rust
pub fn resolve_address_to_module(pid: u32, address: usize) -> String
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 拥有该地址空间的进程标识符。用作 [MODULE_CACHE](MODULE_CACHE.md) 的键，以查找或填充该进程的模块列表。 |
| `address` | `usize` | 要解析的虚拟内存地址。通常是通过 [get_thread_start_address](get_thread_start_address.md) 获取的线程起始地址。 |

## 返回值

一个 `String`，表示解析后的地址。格式取决于解析是否成功：

| 场景 | 返回格式 | 示例 |
|------|----------|------|
| 地址为 `0` | `"0x0"` | `"0x0"` |
| 地址落在已知模块范围内 | `"{module_name}+0x{offset:X}"` | `"engine.dll+0x1A3F0"` |
| 地址不匹配任何已加载模块 | `"0x{address:X}"` | `"0x7FF6A1230000"` |

## 备注

### 模块缓存

该函数使用 [MODULE_CACHE](MODULE_CACHE.md) 全局静态变量——一个 `Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>>`——来避免每次调用时都枚举进程的模块。缓存以 PID 为键，每个条目是一个 `(base_address, size, module_name)` 元组向量。

首次对给定 PID 调用时：

1. 获取缓存锁。
2. 如果该 PID 不存在条目，则调用 [enumerate_process_modules](enumerate_process_modules.md) 填充缓存。
3. 新枚举的模块列表被克隆并存储到缓存中。

后续对同一 PID 的调用将直接返回缓存的模块列表，无需重新枚举。

### 地址解析算法

获取模块列表（来自缓存或新枚举）后，函数对列表进行线性搜索，查找第一个地址范围 `[base, base + size)` 包含目标 `address` 的模块。如果找到，计算偏移量为 `address - base`，并将结果格式化为 `"{module_name}+0x{offset:X}"`。

如果没有模块范围包含该地址，则以十六进制格式返回原始地址（`"0x{address:X}"`）。这可能发生在以下情况：

- 线程的起始地址指向动态分配的（非模块）内存。
- 自缓存填充以来进程的模块列表已发生变化。
- 枚举失败（例如，访问权限不足）。

### 零地址快速路径

如果 `address` 为 `0`，函数立即返回 `"0x0"`，不访问模块缓存。零起始地址通常表示无法查询线程信息（参见 [get_thread_start_address](get_thread_start_address.md) 的失败行为）。

### 缓存生命周期

给定 PID 的模块缓存会持续存在，直到通过 [drop_module_cache](drop_module_cache.md) 显式清除，该函数在进程退出或每次主循环迭代开始时被调用。这确保了模块列表保持合理的时效性，同时避免了每次线程检查时重新枚举的开销。

### 缓存数据的克隆

该函数在执行地址搜索前会从互斥锁保护中克隆出缓存的模块向量。这样可以快速释放互斥锁，在多次调用为不同进程解析地址时将争用降至最低。代价是每次调用会为克隆的向量产生一次分配，但鉴于典型的模块数量（数十到低百个），这是可以接受的。

### 线程安全

[MODULE_CACHE](MODULE_CACHE.md) 互斥锁仅在缓存查找或插入期间持有。实际的地址解析（对模块列表的线性搜索）在锁之外对克隆的数据进行。

## 要求

| | |
|---|---|
| **模块** | `winapi`（`src/winapi.rs`） |
| **可见性** | `pub` |
| **调用方** | [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md)、[apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |
| **被调用方** | [enumerate_process_modules](enumerate_process_modules.md)（缓存未命中时） |
| **依赖** | [MODULE_CACHE](MODULE_CACHE.md)、[get_thread_start_address](get_thread_start_address.md)（提供地址输入） |
| **API** | 无直接调用（模块枚举委托给 [enumerate_process_modules](enumerate_process_modules.md)） |
| **特权** | 对目标进程需要 `PROCESS_QUERY_INFORMATION | PROCESS_VM_READ`（由缓存未命中时的 [enumerate_process_modules](enumerate_process_modules.md) 要求） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 每进程模块缓存 | [MODULE_CACHE](MODULE_CACHE.md) |
| 缓存清除函数 | [drop_module_cache](drop_module_cache.md) |
| 模块枚举实现 | [enumerate_process_modules](enumerate_process_modules.md) |
| 线程起始地址查询 | [get_thread_start_address](get_thread_start_address.md) |
| 主线程模块名称前缀过滤器 | [PrimePrefix](../config.rs/PrimePrefix.md) |
| 主线程提升（使用模块解析） | [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md) |
| 理想处理器分配（使用模块解析） | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |
| EnumProcessModulesEx (MSDN) | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-enumprocessmodulesex) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd