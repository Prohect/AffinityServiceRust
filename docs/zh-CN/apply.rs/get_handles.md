# get_handles 函数 (apply.rs)

从 [ProcessHandle](../winapi.rs/ProcessHandle.md) 中提取读、写句柄，优先使用完全访问权限的句柄而非受限访问权限的句柄。这是所有需要查询或修改进程状态的每进程 apply 函数的通用入口点。

## 语法

```AffinityServiceRust/src/apply.rs#L63-64
fn get_handles(process_handle: &ProcessHandle) -> (Option<HANDLE>, Option<HANDLE>)
```

## 参数

`process_handle: &ProcessHandle`

对 [ProcessHandle](../winapi.rs/ProcessHandle.md) 的引用，包含最多四个句柄：读和写的完全访问及受限访问变体。

## 返回值

返回一个元组 `(Option<HANDLE>, Option<HANDLE>)`，其中：

| 索引 | 说明 |
|-------|-------------|
| `.0` | 读句柄 — 如果存在 `r_handle` (`Some`)，否则为 `Some(r_limited_handle)`。 |
| `.1` | 写句柄 — 如果存在 `w_handle` (`Some`)，否则为 `Some(w_limited_handle)`。 |

当 `ProcessHandle` 成功打开时，两个元素始终为 `Some`，因为 `r_limited_handle` 和 `w_limited_handle` 会被无条件填充。`Option` 包装器允许调用方使用 `let (Some(r), Some(w)) = get_handles(...) else { return; }` 进行模式匹配，以便在句柄无效时退出。

## 说明

- 该函数标记为 `#[inline(always)]`，因为它在每个进程级别的 apply 函数（`apply_priority`、`apply_affinity`、`apply_io_priority`、`apply_memory_priority`、`apply_process_default_cpuset`）的顶部被调用。
- 完全访问权限的句柄 (`r_handle` / `w_handle`) 优先，因为它们使用更广泛的访问权限打开 (`PROCESS_QUERY_INFORMATION` / `PROCESS_SET_INFORMATION`)。受限句柄 (`r_limited_handle` / `w_limited_handle`) 使用 `PROCESS_QUERY_LIMITED_INFORMATION` / `PROCESS_SET_LIMITED_INFORMATION`，这可能不足以用于所有操作（例如，IO 优先级的 `NtQueryInformationProcess` 需要完全查询访问）。
- 该函数**不**验证返回的句柄是否是有效的 Win32 句柄；调用方依赖 Win32 API 调用来报告句柄过时或不足的错误。

## 要求

| | |
|---|---|
| **模块** | `src/apply.rs` |
| **可见性** | `fn` (crate-private) |
| **调用方** | [apply_priority](apply_priority.md)、[apply_affinity](apply_affinity.md)、[apply_process_default_cpuset](apply_process_default_cpuset.md)、[apply_io_priority](apply_io_priority.md)、[apply_memory_priority](apply_memory_priority.md) |
| **被调用方** | 无 |
| **API / OS** | 无 |
| **权限** | 无（句柄打开在其他地方完成） |

## 参见

| 主题 | 链接 |
|-------|------|
| ProcessHandle 结构体 | [ProcessHandle](../winapi.rs/ProcessHandle.md) |
| 进程优先级应用 | [apply_priority](apply_priority.md) |
| 进程亲和性应用 | [apply_affinity](apply_affinity.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*