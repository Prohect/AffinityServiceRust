# enumerate_process_modules 函数 (winapi.rs)

枚举目标进程的所有已加载模块，返回基地址、结束地址和模块名称的元组向量。

## 语法

```rust
pub fn enumerate_process_modules(pid: u32) -> Vec<(usize, usize, String)>
```

## 参数

`pid`

要枚举其已加载模块的目标进程的进程标识符。

## 返回值

返回 `Vec<(usize, usize, String)>`，其中每个元组表示一个已加载模块：

- `.0` (`usize`) — 模块在进程虚拟地址空间中的基地址。
- `.1` (`usize`) — 模块的结束地址（基地址 + 模块大小）。
- `.2` (`String`) — 模块的文件名（例如 `"ntdll.dll"`、`"game.exe"`）。仅为文件名，而非完整路径。

如果进程无法打开、模块枚举失败或进程已退出，则返回空向量。

## 备注

此函数执行以下步骤：

1. 通过 `OpenProcess` 以 `PROCESS_QUERY_INFORMATION | PROCESS_VM_READ` 访问权限打开目标进程。
2. 使用 `LIST_MODULES_ALL` 调用 `EnumProcessModulesEx`，获取所有已加载模块（包括 32 位和 64 位）的句柄。
3. 对每个模块句柄，调用 `GetModuleInformation` 获取基地址和大小，调用 `GetModuleFileNameExW` 获取模块名称。
4. 从完整模块路径中提取文件名部分。
5. 将结果收集为 `(base, base + size, name)` 元组向量。

返回的数据通常由 [`resolve_address_to_module`](resolve_address_to_module.md) 缓存到 [`MODULE_CACHE`](MODULE_CACHE.md) 中，以避免在每次地址解析调用时重复执行此昂贵的枚举操作。此函数的直接调用方应注意，它对每个模块执行多次 Windows API 调用，对于加载了大量 DLL 的进程可能较慢。

### 错误处理

如果 `OpenProcess` 失败（例如受保护进程的访问被拒绝，或进程已退出），函数返回空向量且不记录错误。模块枚举被视为尽力而为——调用方（通常是 [`resolve_address_to_module`](resolve_address_to_module.md)）通过回退到原始地址格式化来优雅地处理缺失的模块数据。

如果 `EnumProcessModulesEx` 或单个模块查询失败，函数返回到该时刻为止已成功枚举的模块。

### 访问要求

该函数需要对目标进程的 `PROCESS_QUERY_INFORMATION | PROCESS_VM_READ` 访问权限。这些是相对高权限的访问权限——如果未启用 [`SeDebugPrivilege`](enable_debug_privilege.md)，对其他用户拥有的进程或在更高完整性级别运行的进程的枚举将失败。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/winapi.rs |
| **源码行** | L767–L820 |
| **调用方** | [`resolve_address_to_module`](resolve_address_to_module.md) |
| **填充** | [`MODULE_CACHE`](MODULE_CACHE.md)（间接，通过调用方） |
| **Windows API** | [OpenProcess](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess)、[EnumProcessModulesEx](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-enumprocessmodulesex)、[GetModuleInformation](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getmoduleinformation)、[GetModuleFileNameExW](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getmodulefilenameexw) |

## 另请参阅

- [MODULE_CACHE 静态变量](MODULE_CACHE.md)
- [resolve_address_to_module](resolve_address_to_module.md)
- [drop_module_cache](drop_module_cache.md)
- [get_thread_start_address](get_thread_start_address.md)
- [winapi.rs 模块概述](README.md)