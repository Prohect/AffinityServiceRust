# get_thread_start_address 函数 (winapi.rs)

通过 `NtQueryInformationThread` 查询线程的起始地址，返回线程入口点函数所在的内存地址。

## 语法

```rust
pub fn get_thread_start_address(thread_handle: HANDLE) -> usize
```

## 参数

`thread_handle`

有效的线程 `HANDLE`，需至少以 `THREAD_QUERY_INFORMATION` 访问权限打开。通常对应 [`ThreadHandle`](ThreadHandle.md) 的 `r_handle` 字段。

## 返回值

返回 `usize`，表示线程起始例程的虚拟内存地址。如果查询失败或线程句柄无效，则返回 `0`。

## 备注

此函数调用 NT 层 API `NtQueryInformationThread`，使用 `ThreadQuerySetWin32StartAddress` 信息类来获取创建线程时传递给 `CreateThread`（或等效函数）的函数地址。

返回的地址被理想处理器分配系统在 [`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md) 中使用，以确定线程源自哪个模块。该地址随后被传递给 [`resolve_address_to_module`](resolve_address_to_module.md)，将其映射为 `"module.dll+0xABC"` 格式的字符串。然后将此字符串与 [`IdealProcessorRule`](../config.rs/IdealProcessorRule.md) 中定义的前缀规则进行匹配，以根据线程的来源模块将其分配到特定处理器。

起始地址在每个线程的生命周期内仅查询一次，并缓存在 [`ThreadStats.start_address`](../scheduler.rs/ThreadStats.md) 中，以避免在后续循环迭代中进行冗余的系统调用。

### 限制

- 起始地址可能并不总是反映线程的当前指令指针——它是*原始*入口点，而非当前执行位置。
- 对于由运行时创建的线程（例如线程池线程），起始地址通常指向通用的运行时存根，而非用户代码。
- 如果线程句柄缺少 `THREAD_QUERY_INFORMATION` 访问权限，函数返回 `0`。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/winapi.rs |
| **源码行** | L637–L656 |
| **调用方** | [`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md)、[`prefetch_all_thread_cycles`](../apply.rs/prefetch_all_thread_cycles.md) |
| **调用** | `NtQueryInformationThread` |
| **Windows API** | [NtQueryInformationThread](https://learn.microsoft.com/en-us/windows/win32/api/winternl/nf-winternl-ntqueryinformationthread) |

## 另请参阅

- [resolve_address_to_module](resolve_address_to_module.md)
- [ThreadHandle 结构体](ThreadHandle.md)
- [apply_ideal_processors](../apply.rs/apply_ideal_processors.md)
- [winapi.rs 模块概述](README.md)