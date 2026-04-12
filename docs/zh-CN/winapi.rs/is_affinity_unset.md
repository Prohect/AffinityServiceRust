# is_affinity_unset 函数 (winapi.rs)

检查进程是否具有默认（全核心）亲和性掩码，即尚未应用任何自定义亲和性。

## 语法

```rust
pub fn is_affinity_unset(pid: u32, process_name: &str) -> bool
```

## 参数

`pid`

要查询的目标进程的进程标识符。

`process_name`

目标进程的名称。用于错误日志记录上下文，以及通过 [`get_process_handle`](get_process_handle.md) 打开进程句柄。

## 返回值

如果进程的当前亲和性掩码与系统亲和性掩码一致（即所有逻辑处理器均已启用），则返回 `true`。如果进程已设置自定义亲和性掩码，或查询失败，则返回 `false`。

## 备注

此函数通过 [`get_process_handle`](get_process_handle.md) 打开一个临时 [`ProcessHandle`](ProcessHandle.md)，然后调用 `GetProcessAffinityMask` 同时获取进程亲和性掩码和系统亲和性掩码。如果两个掩码相等，则说明进程的亲和性未被限制，函数返回 `true`。

比较逻辑非常直接："未设置"的亲和性意味着进程可以在系统允许的所有处理器上运行。任何掩码与系统掩码不同的进程都已被显式约束，无论是由本应用程序还是由其他工具设置的。

此检查在配置应用期间用于判断是否需要进行亲和性更改。如果进程已有与配置值不同的自定义亲和性，这可能表明另一个工具或进程本身已设置了亲和性。该函数使 apply 逻辑能够区分"从未修改"和"已被修改"两种状态。

如果 `get_process_handle` 返回 `None`（例如进程已退出或访问被拒绝），函数返回 `false` 作为保守的默认值，这将导致调用方尝试应用亲和性并通过正常路径处理任何产生的错误。

如果 `GetProcessAffinityMask` 失败，错误将通过 [`is_new_error`](../logging.rs/is_new_error.md) 以去重方式记录，函数返回 `false`。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/winapi.rs |
| **源码行** | L583–L635 |
| **调用方** | [`apply_affinity`](../apply.rs/apply_affinity.md)、[`apply_config`](../main.rs/apply_config.md) |
| **调用** | [`get_process_handle`](get_process_handle.md)、[`is_new_error`](../logging.rs/is_new_error.md)、[`error_from_code_win32`](../error_codes.rs/error_from_code_win32.md) |
| **Windows API** | [GetProcessAffinityMask](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getprocessaffinitymask) |

## 另请参阅

- [ProcessHandle](ProcessHandle.md)
- [apply_affinity](../apply.rs/apply_affinity.md)
- [filter_indices_by_mask](filter_indices_by_mask.md)
- [winapi.rs 模块概述](README.md)