# try_open_thread 函数 (winapi.rs)

尝试以单一指定访问权限打开一个线程句柄。失败时返回无效的 `HANDLE` 而非 `None`，允许调用方将结果直接存储在 [ThreadHandle](ThreadHandle.md) 结构体中，其中非必需的句柄可能为无效值。

## 语法

```rust
#[inline(always)]
#[allow(unused_variables)]
fn try_open_thread(
    pid: u32,
    tid: u32,
    process_name: &str,
    access: THREAD_ACCESS_RIGHTS,
    internal_op_code: u32,
) -> HANDLE
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 拥有该线程的进程 ID。用于错误日志上下文。 |
| `tid` | `u32` | 要打开的线程 ID。传递给 `OpenThread`。 |
| `process_name` | `&str` | 拥有该线程的进程的显示名称。用于错误日志上下文。 |
| `access` | `THREAD_ACCESS_RIGHTS` | 请求的访问权限。典型值为 `THREAD_QUERY_INFORMATION`、`THREAD_SET_LIMITED_INFORMATION` 或 `THREAD_SET_INFORMATION`。 |
| `internal_op_code` | `u32` | 标识正在打开哪个句柄槽位的数值代码，由内部 `error_detail` 辅助函数和 `is_new_error` 去重系统使用。代码：`1` = `r_handle`，`2` = `w_limited_handle`，`3` = `w_handle`。 |

## 返回值

| 值 | 含义 |
|----|------|
| 有效的 `HANDLE` | 线程已成功以请求的访问权限打开。 |
| `HANDLE::default()` | 打开尝试失败（`OpenThread` 返回错误，或返回的句柄无效）。这是一个无效句柄哨兵值。 |

## 备注

这是一个模块私有辅助函数，由 [get_thread_handle](get_thread_handle.md) 调用，用于三个非必需的句柄槽位（`r_handle`、`w_limited_handle`、`w_handle`）。必需的 `r_limited_handle` 直接在 `get_thread_handle` 中打开，因为它的失败会中止整个操作。

### 设计原理

与使用 `Option<HANDLE>` 来处理可选句柄的 [get_process_handle](get_process_handle.md) 不同，[ThreadHandle](ThreadHandle.md) 将所有四个句柄存储为裸 `HANDLE` 值。`try_open_thread` 函数在失败时返回 `HANDLE::default()`（无效句柄），`ThreadHandle::Drop` 实现会在调用 `CloseHandle` 之前检查该值。这避免了将每个线程句柄包装在 `Option` 中，同时仍能提供安全的清理。

### 错误日志

该函数包含已注释掉的 `is_new_error` 和 `log_to_find` 调用。这些调用保留在源代码中供诊断用途，但在生产环境中被禁用，以减少线程在枚举和句柄打开之间退出时产生的大量非关键句柄失败日志噪声。

内部 `error_detail` 辅助函数将 `internal_op_code` 值映射为人类可读的句柄名称：

| `internal_op_code` | 句柄名称 |
|--------------------|----------|
| `1` | `r_handle` |
| `2` | `w_limited_handle` |
| `3` | `w_handle` |

### 失败行为

当 `OpenThread` 失败或返回无效句柄时，函数静默返回 `HANDLE::default()`。调用方（[get_thread_handle](get_thread_handle.md)）仍然会成功——它将无效句柄存储在 [ThreadHandle](ThreadHandle.md) 中，后续代码在使用该特定句柄之前必须检查 `is_invalid()`。

### 内联

该函数标记为 `#[inline(always)]`，因为它是在线程枚举热路径中调用的单个 Windows API 调用的薄包装器。

## 要求

| | |
|---|---|
| **模块** | `winapi` (`src/winapi.rs`) |
| **可见性** | 模块私有（`fn`，无 `pub`） |
| **调用方** | [get_thread_handle](get_thread_handle.md) |
| **API** | `OpenThread`（`kernel32.dll` / `windows` crate `Win32::System::Threading`） |
| **特权** | 建议启用 `SeDebugPrivilege` 以进行跨进程线程访问 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 线程句柄容器 | [ThreadHandle](ThreadHandle.md) |
| 完整线程句柄获取 | [get_thread_handle](get_thread_handle.md) |
| 进程句柄获取（类似模式） | [get_process_handle](get_process_handle.md) |
| 错误去重 | [is_new_error](../logging.rs/README.md) |
| OpenThread (Microsoft Learn) | [OpenThread 函数](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openthread) |