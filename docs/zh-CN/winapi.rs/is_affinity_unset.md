# is_affinity_unset 函数 (winapi.rs)

检查进程的 CPU 亲和性掩码是否等于系统默认值（所有逻辑处理器均已启用）。此函数用于 `-find` 模式中识别尚未应用自定义亲和性的进程，帮助用户发现哪些进程仍在使用默认设置运行。

## 语法

```rust
pub fn is_affinity_unset(pid: u32, process_name: &str) -> bool
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 目标进程的进程标识符。 |
| `process_name` | `&str` | 目标进程的映像名称（例如 `"game.exe"`）。用于诊断日志记录，以及在访问被拒绝时填充 find-fail 集合。 |

## 返回值

| 值 | 含义 |
|------|------|
| `true` | 进程的当前亲和性掩码等于系统亲和性掩码——即所有 CPU 均已启用，未设置自定义亲和性。 |
| `false` | 进程具有自定义亲和性掩码（CPU 的子集），**或者**无法打开进程，**或者**亲和性查询失败。 |

## 备注

### 算法

1. 通过 `OpenProcess` 以 `PROCESS_SET_INFORMATION | PROCESS_QUERY_INFORMATION` 访问权限打开进程。
2. 如果打开失败，通过 `log_to_find` 记录错误。如果错误码为 `5`（访问被拒绝），则将 `process_name` 插入 `FINDS_FAIL_SET`，以便 find 模式报告可以标注此进程不可访问。
3. 调用 `GetProcessAffinityMask` 获取进程亲和性掩码（`current_mask`）和系统亲和性掩码（`system_mask`）。
4. 当且仅当 `current_mask == system_mask` 时返回 `true`。
5. 返回前关闭进程句柄。

### 句柄管理

与此模块中的大多数其他函数不同，`is_affinity_unset` **不**使用 [ProcessHandle](ProcessHandle.md) RAII 包装器。它通过 `OpenProcess` 直接打开单个组合访问权限的句柄，并在函数末尾通过 `CloseHandle` 手动关闭。这是因为该函数是仅在 `-find` 模式下使用的独立查询，不参与主 apply 循环的句柄生命周期管理。

### 错误行为

函数在遇到任何错误时返回 `false`，将无法访问或查询失败的进程视为"已配置"，以避免在 find 模式输出中产生误报。具体错误处理如下：

| 场景 | 行为 |
|------|------|
| `OpenProcess` 失败 | 通过 `log_to_find` 记录错误；如果错误码为 5（访问被拒绝），添加到 fail 集合；返回 `false` |
| `OpenProcess` 返回无效句柄 | 通过 `log_to_find` 记录 `[INVALID_HANDLE]`；返回 `false` |
| `GetProcessAffinityMask` 失败 | 通过 `log_to_find` 记录错误；如果错误码为 5，添加到 fail 集合；返回 `false` |

### 访问被拒绝跟踪

当错误码为 `5`（`ERROR_ACCESS_DENIED`）时，进程名称会被插入全局 `FINDS_FAIL_SET`（通过 `get_fail_find_set!()` 宏访问）。此集合由 find 模式报告使用，用于列出无法检查的进程，通常是因为未持有 `SeDebugPrivilege` 或进程受到保护。

### 系统亲和性掩码

`GetProcessAffinityMask` 返回的系统亲和性掩码反映了调用进程可用的所有逻辑处理器集合。在单处理器组系统（≤ 64 个 CPU）上，这通常为 `(1 << cpu_count) - 1`。当进程未调用过 `SetProcessAffinityMask` 时，其进程掩码等于系统掩码。

### 与 apply 模块中亲和性的比较

[apply_affinity](../apply.rs/apply_affinity.md) 函数在主服务循环期间使用 [ProcessHandle](ProcessHandle.md) 来获取和设置亲和性。`is_affinity_unset` 独立运行，仅在 `-find` 模式发现过程中被调用，而非在 apply 周期中。

## 要求

| | |
|---|---|
| **模块** | `winapi` (`src/winapi.rs`) |
| **可见性** | `pub` |
| **调用者** | [process_find](../main.rs/process_find.md)（通过 `-find` CLI 模式） |
| **被调用者** | `OpenProcess`、`GetProcessAffinityMask`、`GetLastError`、`CloseHandle`（Win32）；`log_to_find`、`get_fail_find_set!()` |
| **API** | [`GetProcessAffinityMask`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getprocessaffinitymask)、[`OpenProcess`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess) |
| **特权** | 建议启用 `SeDebugPrivilege`；没有该特权时，受保护进程将返回访问被拒绝 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 亲和性应用逻辑 | [apply_affinity](../apply.rs/apply_affinity.md) |
| Find 模式入口点 | [process_find](../main.rs/process_find.md) |
| 进程句柄 RAII 包装器 | [ProcessHandle](ProcessHandle.md) |
| 调试特权启用 | [enable_debug_privilege](enable_debug_privilege.md) |
| CPU 索引转位掩码工具 | [cpu_indices_to_mask](../config.rs/cpu_indices_to_mask.md) |
| 错误码格式化 | [error_codes 模块](../error_codes.rs/README.md) |
| GetProcessAffinityMask (MSDN) | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getprocessaffinitymask) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd