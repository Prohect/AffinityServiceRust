# set_thread_ideal_processor_ex 函数 (winapi.rs)

设置线程的理想处理器，同时指定处理器组和该组内的处理器编号。理想处理器是向 Windows 内核发出的调度提示，指示该线程倾向于在哪个逻辑处理器上运行。

## 语法

```rust
pub fn set_thread_ideal_processor_ex(
    thread_handle: HANDLE,
    group: u16,
    number: u8,
) -> Result<PROCESSOR_NUMBER, Error>
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `thread_handle` | `HANDLE` | 以 `THREAD_SET_INFORMATION` 访问权限打开的有效线程句柄。通常为 [`ThreadHandle`](ThreadHandle.md) 的 `w_handle` 字段。 |
| `group` | `u16` | 处理器组编号（从零开始）。在逻辑处理器数量不超过 64 个的系统上，该值始终为 `0`。 |
| `number` | `u8` | 指定组内的处理器编号（从零开始）。这是相对于该组的逻辑处理器索引。 |

## 返回值

成功时，返回 `Ok(PROCESSOR_NUMBER)`，其中包含该线程**先前**的理想处理器设置。返回的 `PROCESSOR_NUMBER` 包括 `Group`、`Number` 和 `Reserved` 字段。

失败时，返回 `Err(Error)`，其中包含 `SetThreadIdealProcessorEx` 返回的 Windows 错误。

## 备注

- 该函数使用提供的 `group` 和 `number` 值（`Reserved` 设为 `0`）构造一个 `PROCESSOR_NUMBER` 结构体，然后调用 Win32 API 中的 `SetThreadIdealProcessorEx`。

- 理想处理器是一个**提示**，而非硬性约束。如果理想处理器繁忙，Windows 调度程序仍可能将线程调度到其他处理器上运行。若需硬性亲和约束，请改用 CPU 集合 API（`SetProcessDefaultCpuSets`、`SetThreadSelectedCpuSets`）或亲和掩码。

- 返回先前的理想处理器设置，以便调用方稍后恢复或记录更改。该值由 Win32 `SetThreadIdealProcessorEx` API 通过其输出参数直接提供。

- 此函数要求线程句柄具有 `THREAD_SET_INFORMATION` 访问权限。如果 [`ThreadHandle`](ThreadHandle.md) 结构体的 `w_handle` 字段无效（打开调用失败），调用方不得将其传递给此函数。调用前请检查 `HANDLE::is_invalid()`。

- 与旧版 `SetThreadIdealProcessor` API 不同，`SetThreadIdealProcessorEx` 支持处理器组，使其适用于逻辑处理器超过 64 个的系统。

### 平台说明

- **仅限 Windows。** 使用 `windows::Win32::System::Threading` 中的 `SetThreadIdealProcessorEx`。
- 需要 Windows 7 / Windows Server 2008 R2 或更高版本。
- 在单组系统（≤ 64 个逻辑处理器）上，`group` 应始终为 `0`。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `winapi.rs` |
| **调用方** | `apply.rs` — 用于模块感知线程放置的理想处理器分配逻辑 |
| **被调用方** | [`SetThreadIdealProcessorEx`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadidealprocessorex)（Win32 API） |
| **Win32 API** | `kernel32.dll` — `SetThreadIdealProcessorEx` |
| **权限** | 需要具有 `THREAD_SET_INFORMATION` 访问权限的线程句柄。对于其他用户拥有的线程，可能需要 `SeDebugPrivilege`。 |
| **平台** | Windows 7+ / Windows Server 2008 R2+ |

## 另请参阅

| 主题 | 链接 |
|------|------|
| get_thread_ideal_processor_ex | [get_thread_ideal_processor_ex](get_thread_ideal_processor_ex.md) |
| get_thread_start_address | [get_thread_start_address](get_thread_start_address.md) |
| resolve_address_to_module | [resolve_address_to_module](resolve_address_to_module.md) |
| ThreadHandle 结构体 | [ThreadHandle](ThreadHandle.md) |
| get_thread_handle | [get_thread_handle](get_thread_handle.md) |
| winapi 模块概述 | [README](README.md) |

---
*提交：[29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
