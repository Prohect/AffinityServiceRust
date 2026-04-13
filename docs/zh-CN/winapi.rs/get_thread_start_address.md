# get_thread_start_address 函数 (winapi.rs)

通过以信息类 `ThreadQuerySetWin32StartAddress` (9) 调用 `NtQueryInformationThread` 来查询线程的起始地址。起始地址标识了线程被创建来执行的函数——进而标识了所属模块——从而实现基于模块的线程分类，用于理想处理器和主线程调度。

## 语法

```rust
pub fn get_thread_start_address(thread_handle: HANDLE) -> usize
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `thread_handle` | `HANDLE` | 至少以 `THREAD_QUERY_INFORMATION` 访问权限打开的有效线程句柄。通常为 [ThreadHandle](ThreadHandle.md) 中的 `r_handle` 字段。使用 `r_limited_handle`（`THREAD_QUERY_LIMITED_INFORMATION`）**不**够——信息类 9 的 `NtQueryInformationThread` 需要完整查询权限。 |

## 返回值

| 值 | 含义 |
|----|------|
| 非零 `usize` | 线程入口点函数所在的虚拟内存地址。这是在创建线程时传递给 `CreateThread` / `CreateRemoteThread`（或等效函数）的地址。 |
| `0` | 查询失败。可能原因包括句柄缺少 `THREAD_QUERY_INFORMATION` 访问权限、线程已退出，或 `NtQueryInformationThread` 调用返回了失败的 `NTSTATUS`。 |

## 备注

### 信息类

该函数使用信息类 `9`，对应 `ThreadQuerySetWin32StartAddress`。这是一个未公开但稳定的信息类，在所有现代 Windows 版本（Windows 7 及更高版本）上均受支持。它返回线程的 Win32 起始地址，即用户提供的线程过程的地址（例如传递给 `CreateThread` 的函数指针）。

### 输出格式

输出是一个原始的指针大小的值（`usize`）。在 64 位 Windows 上为 8 字节地址，在 32 位 Windows 上为 4 字节地址。该函数将 `size_of::<usize>()` 作为输出长度传递给 `NtQueryInformationThread`，确保跨指针宽度的可移植性。

### 模块解析

返回的地址通常传递给 [resolve_address_to_module](resolve_address_to_module.md)，该函数将其映射为模块名加偏移量的字符串，例如 `"engine.dll+0x1A30"`。然后，此模块名称被主线程调度器和理想处理器规则用于将线程与 [PrimePrefix](../config.rs/PrimePrefix.md) 和 [IdealProcessorRule](../config.rs/IdealProcessorRule.md) 配置进行匹配。

### 失败处理

该函数在失败时静默返回 `0`。调用方应将 `0` 视为"未知起始地址"。[resolve_address_to_module](resolve_address_to_module.md) 函数处理 `0` 地址时会返回字符串 `"0x0"`，因此失败会通过模块解析管道优雅传播。

### 典型调用序列

```
thread_handle = get_thread_handle(tid, pid, process_name)
start_address = get_thread_start_address(thread_handle.r_handle)
module_name = resolve_address_to_module(pid, start_address)
// 使用 module_name 进行前缀匹配
```

### NtQueryInformationThread 内部细节

该函数执行一个单独的 unsafe FFI 调用：

```rust
NtQueryInformationThread(
    thread_handle,
    9,                                          // ThreadQuerySetWin32StartAddress
    &mut start_address as *mut _ as *mut c_void,
    size_of::<usize>() as u32,
    &mut return_len,
)
```

`return_len` 输出参数接收写入的字节数，但不进行检查——仅检查 `NTSTATUS` 返回值。如果状态为成功（非负值），则返回 `start_address` 值；否则返回 `0`。

### 线程起始地址与指令指针

起始地址**不是**线程当前的指令指针（`RIP`/`EIP`）。它是创建线程时所用函数的地址。此值在线程的整个生命周期内保持不变，即使线程在不同的函数和模块中执行也是如此。这种稳定性使其适合作为用于分类目的的持久线程标识符。

## 要求

| | |
|---|---|
| **模块** | `winapi` (`src/winapi.rs`) |
| **可见性** | `pub` |
| **调用方** | [prefetch_all_thread_cycles](../apply.rs/prefetch_all_thread_cycles.md)、[apply_ideal_processors](../apply.rs/apply_ideal_processors.md)、[ThreadStats](../scheduler.rs/ThreadStats.md) 初始化 |
| **被调用方** | `NtQueryInformationThread`（ntdll.dll FFI） |
| **API** | `NtQueryInformationThread`，使用 `ThreadQuerySetWin32StartAddress`（信息类 9） |
| **特权** | 线程句柄需要 `THREAD_QUERY_INFORMATION` 访问权限；跨进程线程可能需要 `SeDebugPrivilege` |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 模块地址解析 | [resolve_address_to_module](resolve_address_to_module.md) |
| 模块缓存管理 | [MODULE_CACHE](MODULE_CACHE.md) |
| 线程句柄容器 | [ThreadHandle](ThreadHandle.md) |
| 线程句柄获取 | [get_thread_handle](get_thread_handle.md) |
| 主线程前缀匹配 | [PrimePrefix](../config.rs/PrimePrefix.md) |
| 理想处理器规则匹配 | [IdealProcessorRule](../config.rs/IdealProcessorRule.md) |
| 线程统计信息（存储起始地址） | [ThreadStats](../scheduler.rs/ThreadStats.md) |
| NtQueryInformationThread（非官方文档） | [ntdll 未公开函数](https://learn.microsoft.com/en-us/windows/win32/api/winternl/) |