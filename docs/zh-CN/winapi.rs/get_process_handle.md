# get_process_handle 函数 (winapi.rs)

通过 PID 以多种访问级别打开进程，并返回一个包含所有成功打开的句柄的 [ProcessHandle](ProcessHandle.md)。两个受限访问句柄（`PROCESS_QUERY_LIMITED_INFORMATION` 和 `PROCESS_SET_LIMITED_INFORMATION`）是必需的——如果任一失败，函数将返回 `None`。两个完全访问句柄（`PROCESS_QUERY_INFORMATION` 和 `PROCESS_SET_INFORMATION`）会尝试获取但非必需；如果调用方缺少足够的特权，它们将以 `Option<HANDLE>` 的形式存储为 `None`。

## 语法

```rust
pub fn get_process_handle(pid: u32, process_name: &str) -> Option<ProcessHandle>
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 目标进程的进程标识符。通过进程枚举获取（例如 `SYSTEM_PROCESS_INFORMATION.UniqueProcessId`）。 |
| `process_name` | `&str` | 目标进程的映像名称（例如 `"explorer.exe"`）。仅在句柄打开失败时用于诊断日志记录。 |

## 返回值

| 值 | 描述 |
|----|------|
| `Some(ProcessHandle)` | 一个至少 `r_limited_handle` 和 `w_limited_handle` 有效的 [ProcessHandle](ProcessHandle.md)。`r_handle` 和 `w_handle` 字段在较高特权打开失败时可能为 `None`。所有 `Some` 句柄均已确认有效（非无效）。 |
| `None` | `PROCESS_QUERY_LIMITED_INFORMATION` 或 `PROCESS_SET_LIMITED_INFORMATION` 无法打开，或者返回的句柄无效。失败时不会泄漏句柄——任何已部分打开的句柄在返回 `None` 之前会被关闭。 |

## 备注

### 访问级别

该函数以逐步提高的访问权限打开进程四次：

| 顺序 | 访问权限 | 必需 | 字段 |
|------|---------|------|------|
| 1 | `PROCESS_QUERY_LIMITED_INFORMATION` | 是 | `r_limited_handle` |
| 2 | `PROCESS_SET_LIMITED_INFORMATION` | 是 | `w_limited_handle` |
| 3 | `PROCESS_QUERY_INFORMATION` | 否 | `r_handle` |
| 4 | `PROCESS_SET_INFORMATION` | 否 | `w_handle` |

受限信息句柄足以满足大多数操作（读取优先级类、通过 CPU 集合设置亲和性、查询内存/IO 优先级）。完全信息句柄用于需要特定信息类的 `NtSetInformationProcess` 等操作。通过尽可能多地获取句柄，服务在未持有 `SeDebugPrivilege` 或目标进程具有受限访问时能够优雅降级。

### 错误日志记录

两个必需句柄的失败通过 `log_to_find` 记录，使用 `is_new_error` 去重机制——每个唯一的 `(pid, operation, error_code)` 组合仅记录一次。两个可选句柄的失败被静默忽略（源代码中的日志调用已被注释掉）。

### 内部错误码映射

该函数在使用 `Operation::InvalidHandle` 调用 `is_new_error` 时使用合成的 `internal_op_code` 值：

| 代码 | 含义 |
|------|------|
| `0` | `r_limited_handle` 无效 |
| `1` | `w_limited_handle` 无效 |
| `2` | `r_handle` 无效（当前未记录日志） |
| `3` | `w_handle` 无效（当前未记录日志） |

### 部分失败时的句柄清理

如果 `r_limited_handle` 成功打开但 `w_limited_handle` 失败，函数在返回 `None` 之前会关闭 `r_limited_handle`。这可以防止句柄泄漏。成功时，所有句柄清理工作延迟到 [ProcessHandle](ProcessHandle.md) 的 `Drop` 实现。

### 常见失败原因

- **访问被拒绝（错误 5）：** 目标进程是受保护进程（例如 `csrss.exe`、`System`），且调用方未持有 `SeDebugPrivilege`。
- **参数无效（错误 87）：** 该 PID 已不存在（快照与打开之间的竞态条件）。
- **进程已退出：** 进程在枚举和 `OpenProcess` 调用之间终止。

## 要求

| | |
|---|---|
| **模块** | `winapi` (`src/winapi.rs`) |
| **调用方** | [`apply_config_process_level`](../main.rs/apply_config_process_level.md)、[`apply_config_thread_level`](../main.rs/apply_config_thread_level.md)、[`main.rs`](../main.rs/README.md) 中的主循环 |
| **被调用方** | `OpenProcess` (Win32)、`CloseHandle` (Win32)、`GetLastError` (Win32)、[`is_new_error`](../logging.rs/is_new_error.md)、`log_to_find` |
| **API** | `OpenProcess` — [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess) |
| **特权** | 建议启用 `SeDebugPrivilege` 以获取受保护进程的完全信息句柄 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| RAII 进程句柄容器 | [ProcessHandle](ProcessHandle.md) |
| 线程句柄获取 | [get_thread_handle](get_thread_handle.md) |
| 调试特权启用 | [enable_debug_privilege](enable_debug_privilege.md) |
| 错误去重 | [`is_new_error`](../logging.rs/is_new_error.md) |
| 错误日志记录的 Operation 枚举 | [`Operation`](../logging.rs/Operation.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd