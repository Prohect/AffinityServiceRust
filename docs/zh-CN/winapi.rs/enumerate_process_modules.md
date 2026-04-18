# enumerate_process_modules 函数 (winapi.rs)

枚举给定进程的所有已加载模块（DLL 和主可执行文件），返回每个模块的基地址、大小和名称。这是一个内部辅助函数，由 [`resolve_address_to_module`](resolve_address_to_module.md) 使用，用于构建缓存在 [`MODULE_CACHE`](MODULE_CACHE.md) 中的每进程模块映射。

## 语法

```rust
fn enumerate_process_modules(pid: u32) -> Vec<(usize, usize, String)>
```

## 参数

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `pid` | `u32` | 要枚举已加载模块的目标进程的进程标识符。 |

## 返回值

返回一个 `Vec<(usize, usize, String)>`，其中每个元组元素表示：

| 索引 | 类型 | 描述 |
|-------|------|-------------|
| `.0` | `usize` | 模块在目标进程虚拟地址空间中的基地址 (`MODULEINFO.lpBaseOfDll`)。 |
| `.1` | `usize` | 模块映像的字节大小 (`MODULEINFO.SizeOfImage`)。 |
| `.2` | `String` | 模块的基本名称（例如 `kernel32.dll`），通过 `GetModuleBaseNameW` 获取。 |

在以下情况下返回空 `Vec`：

- 无法以所需的访问权限打开进程。
- 打开的句柄无效。
- `EnumProcessModulesEx` 失败（例如进程受保护或 32 位/64 位不匹配）。

## 备注

### 算法

1. **打开进程** — 使用 `PROCESS_QUERY_INFORMATION | PROCESS_VM_READ` 访问权限调用 `OpenProcess`。两种权限都是必需的：`PROCESS_QUERY_INFORMATION` 用于模块枚举，`PROCESS_VM_READ` 用于从目标进程内存中读取模块名称和信息。

2. **枚举模块** — 使用 `LIST_MODULES_ALL` 调用 `EnumProcessModulesEx`，以检索所有已加载模块的句柄（HMODULE）（包括 32 位和 64 位模块）。该函数使用一个固定大小的 1024 个 `HMODULE` 条目的数组作为输出缓冲区，这对于几乎所有实际进程都足够了。

3. **收集模块信息** — 对于每个返回的 `HMODULE`：
   - 调用 `GetModuleInformation` 获取 `MODULEINFO` 结构体（基地址和映像大小）。
   - 调用 `GetModuleBaseNameW` 获取模块的文件名作为 UTF-16 字符串，然后通过 `String::from_utf16_lossy` 将其转换为 Rust `String`。
   - 如果对给定模块的任一调用失败，该模块将被静默跳过。

4. **清理** — 无论成功与否，在返回之前通过 `CloseHandle` 关闭进程句柄。

### 容量限制

该函数分配一个包含 1024 个 `HMODULE` 条目的栈数组。如果一个进程有超过 1024 个已加载模块，则只枚举前 1024 个。实际上，即使是非常大的应用程序（例如网页浏览器、游戏引擎）也很少超过几百个模块。

### 模块名称缓冲区

模块名称被读入一个固定的 `[u16; 260]` 缓冲区（与 `MAX_PATH` 匹配）。超过 260 个字符的模块名称将被截断。

### 错误处理

此函数不记录错误日志。如果无法打开进程或模块枚举失败，它会静默返回空向量。错误报告留给调用者（[`resolve_address_to_module`](resolve_address_to_module.md)），调用者会回退到将原始地址格式化为十六进制字符串。

### 平台说明

- **仅限 Windows。** 使用来自 `psapi` 的 `EnumProcessModulesEx`、`GetModuleInformation` 和 `GetModuleBaseNameW`（通过 `windows` crate 的 `Win32::System::ProcessStatus` 模块链接）。
- `LIST_MODULES_ALL` 确保结果中包含 32 位和 64 位模块，这在从 64 位宿主检查 WoW64 进程时是相关的。
- 该函数每次调用时都会打开一个**新的**进程句柄。结果由 [`MODULE_CACHE`](MODULE_CACHE.md) 在外部缓存，以避免冗余的句柄打开和模块枚举。

## 要求

| 要求 | 值 |
|-------------|-------|
| **模块** | `winapi.rs` |
| **可见性** | 私有（模块内部，无 `pub`） |
| **调用者** | [`resolve_address_to_module`](resolve_address_to_module.md)（通过 [`MODULE_CACHE`](MODULE_CACHE.md) 填充） |
| **被调用者** | `OpenProcess`、`EnumProcessModulesEx`、`GetModuleInformation`、`GetModuleBaseNameW`、`CloseHandle`（Win32 API） |
| **Win32 API** | [EnumProcessModulesEx](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-enumprocessmodulesexw)、[GetModuleInformation](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getmoduleinformation)、[GetModuleBaseNameW](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getmodulebasenamew) |
| **访问权限** | `PROCESS_QUERY_INFORMATION \| PROCESS_VM_READ` |
| **权限** | 建议使用 `SeDebugPrivilege` 以打开受保护或提升的进程。 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| resolve_address_to_module | [resolve_address_to_module](resolve_address_to_module.md) |
| drop_module_cache | [drop_module_cache](drop_module_cache.md) |
| MODULE_CACHE 静态变量 | [MODULE_CACHE](MODULE_CACHE.md) |
| get_thread_start_address | [get_thread_start_address](get_thread_start_address.md) |
| get_process_handle | [get_process_handle](get_process_handle.md) |
| winapi 模块概述 | [README](README.md) |

---
*提交：[29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
