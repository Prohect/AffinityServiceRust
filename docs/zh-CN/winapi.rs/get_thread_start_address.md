# get_thread_start_address 函数 (winapi.rs)

通过 `NtQueryInformationThread` 获取线程的起始地址。该地址标识线程开始执行的入口点函数，用于确定线程所属的模块，以实现基于模块的理想处理器分配。

## 语法

```rust
pub fn get_thread_start_address(thread_handle: HANDLE) -> usize
```

## 参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `thread_handle` | `HANDLE` | 至少具有 `THREAD_QUERY_LIMITED_INFORMATION` 访问权限的有效线程句柄。通常为 [`ThreadHandle`](ThreadHandle.md) 的 `r_limited_handle` 或 `r_handle` 字段。 |

## 返回值

以 `usize` 形式返回线程的起始地址。如果查询失败（即 `NtQueryInformationThread` 返回非成功的 `NTSTATUS`），则返回 `0`。

## 备注

- 此函数使用信息类 `9`（`ThreadQuerySetWin32StartAddress`）调用 `NtQueryInformationThread` 来获取线程的 Win32 起始地址。这是传递给 `CreateThread` 或类似线程创建 API 的地址，不一定是当前的指令指针。

- 返回的地址可传递给 [`resolve_address_to_module`](resolve_address_to_module.md) 以确定哪个已加载模块（DLL 或 EXE）拥有该地址，从而实现模块感知的线程到核心分配策略。

- 返回值为 `0` 表示查询失败或线程没有记录的起始地址。调用者应将 `0` 视为未知/无法解析的地址。

- 该函数在失败时**不会**记录错误。它静默返回 `0`，由调用者决定失败是否重要。

- `NtQueryInformationThread` 函数是一个未文档化（但稳定的）NTDLL 导出，通过 `winapi.rs` 顶部的 `#[link(name = "ntdll")]` extern 块链接。

### 平台说明

- **仅限 Windows。** `NtQueryInformationThread` 是 NT 原生 API，在其他平台上不可用。
- 起始地址是目标线程所在进程地址空间中的虚拟内存地址。只有与同一进程的模块基地址信息结合使用时才有意义。
- 对于由 CRT 或运行时库创建的线程，起始地址可能指向运行时包装器（例如 `KERNEL32!BaseThreadInitThunk`），而不是用户提供的线程函数。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `winapi.rs` |
| **调用者** | `apply.rs` — 理想处理器分配逻辑 |
| **被调用者** | `NtQueryInformationThread`（ntdll，信息类 `9`） |
| **API** | NT 原生 API — `NtQueryInformationThread` |
| **权限** | 需要具有查询访问权限的有效线程句柄。对于其他会话中的线程可能需要 `SeDebugPrivilege`。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| resolve_address_to_module | [resolve_address_to_module](resolve_address_to_module.md) |
| ThreadHandle 结构体 | [ThreadHandle](ThreadHandle.md) |
| get_thread_handle | [get_thread_handle](get_thread_handle.md) |
| set_thread_ideal_processor_ex | [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) |
| get_thread_ideal_processor_ex | [get_thread_ideal_processor_ex](get_thread_ideal_processor_ex.md) |
| MODULE_CACHE 静态变量 | [MODULE_CACHE](MODULE_CACHE.md) |

---
> Commit SHA: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
