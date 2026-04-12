# MODULE_CACHE 静态变量 (winapi.rs)

每进程的已枚举模块基地址、结束地址和模块名称缓存。由 [`resolve_address_to_module`](resolve_address_to_module.md) 使用，以避免每次地址解析时都重新枚举进程模块。

## 语法

```rust
static MODULE_CACHE: Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
```

## 成员

该静态变量在 `Mutex` 后持有一个 `HashMap<u32, Vec<(usize, usize, String)>>`：

- **外层键** (`u32`) — 进程标识符（PID）。
- **值** (`Vec<(usize, usize, String)>`) — 元组向量，每个元组表示一个已加载模块：
  - `.0` (`usize`) — 模块在进程虚拟地址空间中的基地址。
  - `.1` (`usize`) — 模块的结束地址（基地址 + 大小）。
  - `.2` (`String`) — 模块文件名（例如 `"ntdll.dll"`、`"kernel32.dll"`）。

## 备注

通过 [`enumerate_process_modules`](enumerate_process_modules.md) 进行模块枚举是一个相对昂贵的操作，需要以 `PROCESS_QUERY_INFORMATION | PROCESS_VM_READ` 访问权限打开目标进程，并对每个模块调用 `EnumProcessModulesEx` 和 `GetModuleFileNameExW`。`MODULE_CACHE` 通过按 PID 存储结果来避免重复执行此操作。

### 缓存生命周期

1. **填充** — 当 [`resolve_address_to_module`](resolve_address_to_module.md) 被调用时，若给定 PID 没有缓存条目，则调用 [`enumerate_process_modules`](enumerate_process_modules.md) 并将结果插入缓存。
2. **查找** — 对同一 PID 的后续 [`resolve_address_to_module`](resolve_address_to_module.md) 调用对缓存的模块列表执行简单的线性扫描，将地址与每个模块的 `[base, end)` 范围进行比较。
3. **驱逐** — [`drop_module_cache`](drop_module_cache.md) 从缓存中移除指定 PID 的条目。当进程终止时应调用此函数，以防止陈旧数据累积。

### 线程安全

所有对缓存的访问都通过 `Mutex` 同步。每次查找或插入时获取锁，操作完成后立即释放。

### 内存考虑

由于每个条目包含一个进程的完整模块列表（对于复杂应用程序可能包含数百个 DLL），确保在进程退出时调用 [`drop_module_cache`](drop_module_cache.md) 非常重要。否则缓存将随时间无限增长。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/winapi.rs |
| **源码行** | L680 |
| **填充方** | [`resolve_address_to_module`](resolve_address_to_module.md)（通过 [`enumerate_process_modules`](enumerate_process_modules.md)） |
| **驱逐方** | [`drop_module_cache`](drop_module_cache.md) |

## 另请参阅

- [resolve_address_to_module](resolve_address_to_module.md)
- [drop_module_cache](drop_module_cache.md)
- [enumerate_process_modules](enumerate_process_modules.md)
- [winapi.rs 模块概述](README.md)