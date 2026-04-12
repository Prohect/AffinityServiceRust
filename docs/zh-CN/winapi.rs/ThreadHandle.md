# ThreadHandle 结构体 (winapi.rs)

持有线程的读写 `HANDLE`，包含受限和完全访问两种级别。用于查询和修改线程属性，如理想处理器、周期时间和 CPU 集分配。

## 语法

```rust
pub struct ThreadHandle {
    pub r_limited_handle: HANDLE,
    pub r_handle: HANDLE,
    pub w_limited_handle: HANDLE,
    pub w_handle: HANDLE,
}
```

## 成员

`r_limited_handle`

以 `THREAD_QUERY_LIMITED_INFORMATION` 访问权限打开的读取句柄。当 `ThreadHandle` 构造完成时此句柄始终有效。用于线程周期时间等轻量级查询。

`r_handle`

以 `THREAD_QUERY_INFORMATION` 访问权限打开的读取句柄。用于需要完全信息访问的查询，例如通过 `NtQueryInformationThread` 调用的 [`get_thread_start_address`](get_thread_start_address.md)。

`w_limited_handle`

以 `THREAD_SET_LIMITED_INFORMATION` 访问权限打开的写入句柄。用于接受受限访问句柄的操作。

`w_handle`

以 `THREAD_SET_INFORMATION` 访问权限打开的写入句柄。用于需要完全写入访问的操作，例如 [`set_thread_ideal_processor_ex`](set_thread_ideal_processor_ex.md) 和 `SetThreadSelectedCpuSets`。

## 备注

与 [`ProcessHandle`](ProcessHandle.md) 不同——后者的完全句柄（`r_handle`、`w_handle`）为 `Option<HANDLE>`（因为受保护进程可能拒绝提升访问），`ThreadHandle` 中的全部四个句柄均为非可选的 `HANDLE` 值。线程句柄通常比进程句柄更容易访问，因此预期所有访问级别都会成功。

`ThreadHandle` 由 [`get_thread_handle`](get_thread_handle.md) 返回，并存储在 [`ThreadStats`](../scheduler.rs/ThreadStats.md) 中以便在循环迭代之间复用。句柄在首次遇到线程时打开，并在后续迭代中缓存使用。

各个线程句柄由 [`try_open_thread`](try_open_thread.md) 打开，该函数在内部处理错误日志记录和去重。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/winapi.rs |
| **行号** | L197–L204 |
| **返回方** | [`get_thread_handle`](get_thread_handle.md) |
| **存储于** | 调度器模块中的 [`ThreadStats`](../scheduler.rs/ThreadStats.md) |

## 另请参阅

- [ProcessHandle](ProcessHandle.md)
- [get_thread_handle](get_thread_handle.md)
- [try_open_thread](try_open_thread.md)
- [winapi.rs 模块概述](README.md)