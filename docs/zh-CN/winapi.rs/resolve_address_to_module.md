# resolve_address_to_module 函数 (winapi.rs)

将虚拟内存地址解析为人类可读的模块名称加偏移量格式（例如 `kernel32.dll+0x345`）。使用按 PID 缓存的已加载模块信息，避免每次调用时重复枚举进程模块。

## 语法

```AffinityServiceRust/src/winapi.rs#L688-688
pub fn resolve_address_to_module(pid: u32, address: usize) -> String
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 包含目标地址的地址空间所属的进程标识符。用作模块枚举结果的缓存键。 |
| `address` | `usize` | 要解析的虚拟内存地址。通常是通过 [get_thread_start_address](get_thread_start_address.md) 获取的线程起始地址。 |

## 返回值

返回一个 `String`，格式为以下三种之一：

| 条件 | 格式 | 示例 |
|------|------|------|
| `address` 为 `0` | `"0x0"` | `0x0` |
| 地址落在已知模块的范围内 | `"<模块名>+0x<偏移量>"` | `kernel32.dll+0x1A345` |
| 地址不匹配任何已加载的模块 | `"0x<地址>"` | `0x7FF8A1230000` |

## 备注

### 缓存策略

该函数在 [MODULE_CACHE](MODULE_CACHE.md) 静态变量（`Lazy<Mutex<HashMap<u32, Vec<(usize, usize, String)>>>>`）中维护按 PID 索引的模块缓存。首次为给定 PID 调用时，会调用 [enumerate_process_modules](enumerate_process_modules.md) 来填充缓存，生成 `(base_address, size, module_name)` 元组列表。后续对同一 PID 的调用将重用缓存数据，无需重新枚举。

应通过 [drop_module_cache](drop_module_cache.md) 定期清除缓存——当进程退出时或在每次主循环迭代开始时——以防止过期数据积累。

### 地址解析算法

1. 如果 `address` 为 `0`，立即返回 `"0x0"`（针对未知/空地址的快速路径）。
2. 获取 [MODULE_CACHE](MODULE_CACHE.md) 互斥锁。
3. 如果缓存中包含 `pid` 的条目，使用缓存的模块列表。否则，调用 [enumerate_process_modules](enumerate_process_modules.md) 构建列表，将其插入缓存并使用。
4. 在模块列表中搜索第一个满足 `base <= address < base + size` 的条目。
5. 如果找到匹配的模块，计算 `offset = address - base` 并返回 `"<模块名>+0x<偏移量>"`。
6. 如果未找到匹配项，以大写十六进制格式返回 `"0x<地址>"`。

### 重要副作用

- **首次为每个 PID 调用会触发模块枚举。** 这涉及以 `PROCESS_QUERY_INFORMATION | PROCESS_VM_READ` 权限打开目标进程，并调用 `EnumProcessModulesEx`、`GetModuleInformation` 和 `GetModuleBaseNameW`。对于加载了大量模块的进程，这可能会比较慢。
- **如果从不调用 [drop_module_cache](drop_module_cache.md)，缓存会无限增长。** 调用者负责清除已终止进程的条目。
- 该函数会获取 `MODULE_CACHE` 互斥锁，因此并发调用将在锁上串行化。

### 平台说明

- **仅限 Windows。** 模块枚举依赖于 Win32 进程状态 API（`psapi`）函数。
- 该函数在执行搜索之前会从互斥锁中克隆缓存的模块向量，以最小化锁的持有时间。
- 对于 64 位进程，地址和偏移量使用 `usize`（在 x86-64 上为 8 字节）进行格式化。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `winapi.rs` |
| **调用者** | `apply.rs` — 理想处理器分配逻辑，用于记录线程起始地址所属的模块。 |
| **被调用者** | [enumerate_process_modules](enumerate_process_modules.md)（缓存未命中时） |
| **静态变量** | [MODULE_CACHE](MODULE_CACHE.md) |
| **Win32 API** | 间接使用：`OpenProcess`、`EnumProcessModulesEx`、`GetModuleInformation`、`GetModuleBaseNameW`、`CloseHandle`（通过 `enumerate_process_modules`） |
| **权限** | 目标进程需要 `PROCESS_QUERY_INFORMATION \| PROCESS_VM_READ` 权限。建议启用 `SeDebugPrivilege` 以进行跨会话访问。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| drop_module_cache | [drop_module_cache](drop_module_cache.md) |
| enumerate_process_modules | [enumerate_process_modules](enumerate_process_modules.md) |
| MODULE_CACHE static | [MODULE_CACHE](MODULE_CACHE.md) |
| get_thread_start_address | [get_thread_start_address](get_thread_start_address.md) |
| set_thread_ideal_processor_ex | [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) |
| winapi module overview | [README](README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
