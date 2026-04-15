# MODULE_CACHE 静态变量 (winapi.rs)

按进程缓存的已加载模块信息（基地址、大小和名称），由 [`resolve_address_to_module`](resolve_address_to_module.md) 使用，将内存地址转换为人类可读的模块相对偏移量（例如 `kernel32.dll+0x345`）。

## 语法

```rust
static MODULE_CACHE: Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>> =
    Lazy::new(|| Mutex::new(HashMap::default()));
```

## 成员

| 组件 | 类型 | 描述 |
|------|------|------|
| 键 | `u32` | 已枚举模块的进程 ID (PID)。 |
| 值 | `Vec<(usize, usize, String)>` | 元组向量，每个元组包含：**基地址** (`usize`)、**模块大小** (`usize`) 和 **模块名称** (`String`)。 |

值向量中的每个元组代表目标进程地址空间中的一个已加载模块：

| 元组索引 | 类型 | 描述 |
|----------|------|------|
| `.0` | `usize` | 模块的基虚拟地址 (`MODULEINFO.lpBaseOfDll`)。 |
| `.1` | `usize` | 模块映像的字节大小 (`MODULEINFO.SizeOfImage`)。 |
| `.2` | `String` | 模块的基本名称（例如 `kernel32.dll`），通过 `GetModuleBaseNameW` 获取。 |

## 备注

- **延迟初始化。** 缓存在首次访问时通过 `once_cell::sync::Lazy` 初始化为空的 `HashMap`。在为特定 PID 调用 [`resolve_address_to_module`](resolve_address_to_module.md) 之前，不会进行任何系统调用。

- **填充策略。** 当为缓存中尚不存在的 PID 调用 `resolve_address_to_module` 时，它会调用私有函数 [`enumerate_process_modules`](enumerate_process_modules.md)，通过 `EnumProcessModulesEx`、`GetModuleInformation` 和 `GetModuleBaseNameW` 枚举该进程中所有已加载的模块。结果存储在缓存中，供后续对同一 PID 的查找复用。

- **缓存失效。** 通过调用 [`drop_module_cache`](drop_module_cache.md) 显式移除条目，该函数会删除给定 PID 的条目。通常在进程退出时或新调度循环迭代开始时调用，以确保过期数据不会持续存在。

- **线程安全。** 缓存被包装在 `Mutex` 中，确保安全的并发访问。所有访问都通过 `MODULE_CACHE.lock().unwrap()` 进行。

- **地址查找算法。** 在解析地址时，[`resolve_address_to_module`](resolve_address_to_module.md) 对给定 PID 的缓存模块列表执行线性扫描，搜索基地址范围（`base..base+size`）包含目标地址的模块。找到的第一个匹配模块用于生成 `"module_name+0xOFFSET"` 字符串。

- **内存考量。** 每个进程条目持有一个模块元组的 `Vec`。对于典型的拥有 50–200 个已加载 DLL 的应用程序，每个 PID 条目约占几千字节。缓存随着遇到新 PID 而增长，随着条目被显式删除而缩小。

- 此处使用的 `HashMap` 类型是项目中 [`collections`](../collections.rs/README.md) 模块的 `FxHashMap` 自定义别名，使用快速的非加密哈希函数。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `winapi.rs` |
| **类型** | `Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>>` |
| **初始化方式** | `once_cell::sync::Lazy`（首次访问时为空） |
| **填充方式** | [`resolve_address_to_module`](resolve_address_to_module.md) → [`enumerate_process_modules`](enumerate_process_modules.md) |
| **失效方式** | [`drop_module_cache`](drop_module_cache.md) |
| **Win32 API** | `EnumProcessModulesEx`、`GetModuleInformation`、`GetModuleBaseNameW`（通过 `enumerate_process_modules`） |
| **平台** | Windows |

## 另请参阅

| 主题 | 链接 |
|------|------|
| resolve_address_to_module | [resolve_address_to_module](resolve_address_to_module.md) |
| drop_module_cache | [drop_module_cache](drop_module_cache.md) |
| enumerate_process_modules | [enumerate_process_modules](enumerate_process_modules.md) |
| get_thread_start_address | [get_thread_start_address](get_thread_start_address.md) |
| CPU_SET_INFORMATION 静态变量 | [CPU_SET_INFORMATION](CPU_SET_INFORMATION.md) |
| collections 模块 | [collections.rs](../collections.rs/README.md) |

---
> Commit SHA: `7221ea0694670265d4eb4975582d8ed2ae02439d`
