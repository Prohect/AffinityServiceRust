# enumerate_process_modules 函数 (winapi.rs)

枚举目标进程中所有已加载的模块（DLL 和主可执行文件），返回每个模块的基地址、映像大小和基本名称。这是 [resolve_address_to_module](resolve_address_to_module.md) 用来填充 [MODULE_CACHE](MODULE_CACHE.md) 的底层数据采集函数。

## 语法

```rust
fn enumerate_process_modules(pid: u32) -> Vec<(usize, usize, String)>
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 要枚举已加载模块的目标进程的进程标识符。该函数在内部以 `PROCESS_QUERY_INFORMATION \| PROCESS_VM_READ` 访问权限打开此进程的句柄。 |

## 返回值

一个 `Vec<(usize, usize, String)>`，其中每个元组代表一个已加载的模块：

| 索引 | 类型 | 描述 |
|------|------|------|
| `.0` | `usize` | 模块在目标进程虚拟地址空间中的基地址（`MODULEINFO::lpBaseOfDll`）。 |
| `.1` | `usize` | 模块映像的大小（字节）（`MODULEINFO::SizeOfImage`）。 |
| `.2` | `String` | 模块的基本名称（例如 `"kernel32.dll"`、`"game.exe"`），通过 `GetModuleBaseNameW` 获取。 |

如果无法打开进程或模块枚举失败，则返回空向量。

## 备注

### 算法

1. **打开进程** — 以 `PROCESS_QUERY_INFORMATION | PROCESS_VM_READ` 权限调用 `OpenProcess`。如果调用失败或返回无效句柄，则立即返回空向量。

2. **枚举模块句柄** — 使用 `LIST_MODULES_ALL` 标志调用 `EnumProcessModulesEx`，将最多 1024 个 `HMODULE` 句柄检索到栈分配的数组中。`LIST_MODULES_ALL` 标志确保同时包含 32 位和 64 位模块（与 WoW64 场景相关）。如果枚举失败，关闭进程句柄并返回空向量。

3. **查询每个模块** — 对于 `module_count` 个模块中的每一个（由 `cb_needed / size_of::<HMODULE>()` 计算得出）：
   - 调用 `GetModuleInformation` 获取 `MODULEINFO` 结构体（基地址、映像大小、入口点）。
   - 调用 `GetModuleBaseNameW` 将模块的基本文件名检索到一个 260 字符的 `u16` 缓冲区中。
   - 如果任一调用失败或名称长度为零，则跳过该模块。
   - 否则，将 `(base, size, name)` 元组推入结果向量。

4. **清理** — 返回前通过 `CloseHandle` 关闭进程句柄。

### 模块数量限制

该函数使用固定大小的 1024 个 `HMODULE` 元素数组。如果进程加载了超过 1024 个模块，则只枚举前 1024 个。在实践中，即使是大型应用程序也很少超过此限制 — 典型进程加载 50–200 个模块。

### 名称缓冲区

模块名称被检索到一个 260 元素的 `u16` 缓冲区（`MAX_PATH`）中，对于所有标准 Windows 模块名称都足够了。`GetModuleBaseNameW` 只返回文件名部分（例如 `"ntdll.dll"`），而非完整路径。

### 进程访问要求

该函数需要对目标进程同时拥有 `PROCESS_QUERY_INFORMATION` 和 `PROCESS_VM_READ` 权限。这比 [get_process_handle](get_process_handle.md) 用于一般操作的受限信息权限要求更高。如果没有 `SeDebugPrivilege`，对受保护进程和系统进程的调用将失败。当枚举失败时，[resolve_address_to_module](resolve_address_to_module.md) 函数会优雅地回退为原始十六进制地址格式。

### 句柄管理

该函数在内部自行打开和关闭进程句柄，不使用 [ProcessHandle](ProcessHandle.md) RAII 包装器。这是因为模块枚举是一个不频繁的、自包含的操作，仅在缓存未命中时触发，将其与主 apply 循环的句柄生命周期混合会增加不必要的复杂性。

### 跨架构注意事项

传递给 `EnumProcessModulesEx` 的 `LIST_MODULES_ALL` 标志确保了当 AffinityServiceRust 进程（64 位）枚举 WoW64（32 位）目标进程中的模块时的正确行为。如果没有此标志，将只返回本机架构的模块。

### 可见性

此函数是模块私有的（`fn`，无 `pub`）。它仅由 [resolve_address_to_module](resolve_address_to_module.md) 在填充 [MODULE_CACHE](MODULE_CACHE.md) 时调用。

## 要求

| | |
|---|---|
| **模块** | `winapi`（`src/winapi.rs`） |
| **可见性** | 模块私有（`fn`，无 `pub`） |
| **调用方** | [resolve_address_to_module](resolve_address_to_module.md) |
| **被调用方** | `OpenProcess`、`EnumProcessModulesEx`、`GetModuleInformation`、`GetModuleBaseNameW`、`CloseHandle`（Win32 进程状态 API / 线程） |
| **API** | [`EnumProcessModulesEx`](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-enumprocessmodulesex)、[`GetModuleInformation`](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getmoduleinformation)、[`GetModuleBaseNameW`](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getmodulebasenamew) |
| **特权** | 对目标进程需要 `PROCESS_QUERY_INFORMATION \| PROCESS_VM_READ`；对受保护进程建议启用 `SeDebugPrivilege` |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 地址到模块的解析（调用方） | [resolve_address_to_module](resolve_address_to_module.md) |
| 每进程模块缓存 | [MODULE_CACHE](MODULE_CACHE.md) |
| 缓存清除 | [drop_module_cache](drop_module_cache.md) |
| 线程起始地址查询（提供待解析的地址） | [get_thread_start_address](get_thread_start_address.md) |
| 进程句柄容器 | [ProcessHandle](ProcessHandle.md) |
| 调试特权启用 | [enable_debug_privilege](enable_debug_privilege.md) |
| EnumProcessModulesEx (MSDN) | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-enumprocessmodulesex) |
| GetModuleInformation (MSDN) | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getmoduleinformation) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd