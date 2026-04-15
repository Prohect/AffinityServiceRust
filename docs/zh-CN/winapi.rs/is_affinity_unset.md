# is_affinity_unset 函数 (winapi.rs)

检查进程是否具有默认（全 CPU）亲和性掩码——即进程亲和性掩码是否等于系统亲和性掩码。此函数由 `-find` 模式使用，用于识别未显式配置 CPU 亲和性的进程。

## 语法

```rust
pub fn is_affinity_unset(pid: u32, process_name: &str) -> bool
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 目标进程的进程标识符。 |
| `process_name` | `&str` | 目标进程的名称，用于诊断日志记录以及在访问被拒绝错误时填充查找失败集合。 |

## 返回值

如果进程的当前亲和性掩码等于系统亲和性掩码（即未应用自定义亲和性），则返回 `true`。在所有其他情况下返回 `false`，包括：

- 无法打开进程句柄。
- 句柄已打开但无效。
- `GetProcessAffinityMask` 调用失败。
- 进程具有与系统掩码不同的自定义亲和性掩码。

## 备注

### 算法

1. 通过 `OpenProcess` 以 `PROCESS_SET_INFORMATION | PROCESS_QUERY_INFORMATION` 访问权限打开目标进程。
2. 如果打开失败，通过 [`log_to_find`](../logging.rs/log_to_find.md) 将错误记录到查找日志中。如果错误代码为 `5`（`ACCESS_DENIED`），则将进程名称添加到 `FINDS_FAIL_SET`，以便在后续查找迭代中排除该进程。
3. 如果返回的句柄无效，记录诊断信息并返回 `false`。
4. 调用 `GetProcessAffinityMask` 获取进程亲和性掩码（`current_mask`）和系统亲和性掩码（`system_mask`）。
5. 比较两个掩码。仅当它们相等时返回 `true`。
6. 返回前关闭进程句柄。

### 访问被拒绝处理

当 `OpenProcess` 或 `GetProcessAffinityMask` 返回错误代码 `5`（`ACCESS_DENIED`）时，进程名称会被插入全局 `FINDS_FAIL_SET` 集合。查找模式逻辑使用此集合来跳过已知不可访问的进程（例如受保护进程、反作弊服务），从而避免重复的失败尝试和日志噪音。

### 句柄生命周期

该函数在自身作用域内打开和关闭进程句柄。它**不**使用 [`ProcessHandle`](ProcessHandle.md) RAII 包装器，因为它只需要一个具有特定组合访问权限（`PROCESS_SET_INFORMATION | PROCESS_QUERY_INFORMATION`）的句柄。

### 平台说明

- **仅限 Windows。** 使用 Win32 API 中的 `OpenProcess`、`GetProcessAffinityMask`、`GetLastError` 和 `CloseHandle`。
- 系统亲和性掩码表示进程所在处理器组中所有可用的逻辑处理器。在具有超过 64 个逻辑处理器的系统上，此函数仅考虑主处理器组。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `winapi.rs` |
| **调用者** | `apply.rs` / `scheduler.rs` 中的查找模式逻辑 |
| **被调用者** | `OpenProcess`、`GetProcessAffinityMask`、`GetLastError`、`CloseHandle`（Win32）、[`log_to_find`](../logging.rs/log_to_find.md)、[`error_from_code_win32`](../error_codes.rs/error_from_code_win32.md) |
| **Win32 API** | [OpenProcess](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess)、[GetProcessAffinityMask](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getprocessaffinitymask) |
| **权限** | 建议具有 `SeDebugPrivilege` 以查询受保护/提升权限的进程。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| get_process_handle | [get_process_handle](get_process_handle.md) |
| log_to_find | [log_to_find](../logging.rs/log_to_find.md) |
| error_from_code_win32 | [error_from_code_win32](../error_codes.rs/error_from_code_win32.md) |
| FINDS_FAIL_SET 静态变量 | [statics](../logging.rs/statics.md#finds_fail_set) |
| logging 模块 | [logging.rs](../logging.rs/README.md) |

---
> Commit SHA: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
