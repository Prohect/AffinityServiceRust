# get_thread_handle 函数 (winapi.rs)

通过线程 ID 打开线程，并返回包含多种访问级别 HANDLE 的 [`ThreadHandle`](ThreadHandle.md)。

## 语法

```rust
pub fn get_thread_handle(tid: u32, pid: u32, process_name: &str) -> Option<ThreadHandle>
```

## 参数

`tid`

目标线程的线程标识符。

`pid`

拥有目标线程的进程的进程标识符。仅用于错误日志记录上下文。

`process_name`

拥有该线程的进程的显示名称。用于错误日志记录上下文以及通过 [`is_new_error`](../logging.rs/is_new_error.md) 进行去重。

## 返回值

如果线程至少以受限读取访问权限成功打开，则返回 `Some(ThreadHandle)`。如果线程完全无法打开，则返回 `None`。

## 备注

此函数使用 [`try_open_thread`](try_open_thread.md) 以不同的访问级别打开同一线程的多个句柄：

1. **`r_limited_handle`** — 以 `THREAD_QUERY_LIMITED_INFORMATION` 打开。这是最低访问级别，对可访问的线程预期始终成功。
2. **`r_handle`** — 以 `THREAD_QUERY_INFORMATION` 打开。用于需要完全查询访问的操作，例如通过 `NtQueryInformationThread` 调用的 [`get_thread_start_address`](get_thread_start_address.md)。
3. **`w_limited_handle`** — 以 `THREAD_SET_LIMITED_INFORMATION` 打开。用于需要基本写入访问的操作。
4. **`w_handle`** — 以 `THREAD_SET_INFORMATION` 打开。用于 [`set_thread_ideal_processor_ex`](set_thread_ideal_processor_ex.md) 等操作。

如果受限读取句柄无法打开，函数返回 `None`，因为无法对该线程执行任何有用操作。完全访问句柄可能因受保护或受限线程而打开失败，但函数仍会返回有效的 [`ThreadHandle`](ThreadHandle.md)，包含已获得的访问权限。

与 [`ProcessHandle`](ProcessHandle.md) 不同，[`ThreadHandle`](ThreadHandle.md) 直接存储全部四个句柄（而非 `Option`），但当访问被拒绝时某些句柄可能为无效 HANDLE。

打开句柄过程中的错误通过 [`is_new_error`](../logging.rs/is_new_error.md) 及相应的 [`Operation`](../logging.rs/Operation.md) 变体记录，以防止重复日志条目。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/winapi.rs |
| **源码行** | L229–L271 |
| **调用方** | [`apply_ideal_processors`](../apply.rs/apply_ideal_processors.md)、[`prefetch_all_thread_cycles`](../apply.rs/prefetch_all_thread_cycles.md) |
| **调用** | [`try_open_thread`](try_open_thread.md)、[`is_new_error`](../logging.rs/is_new_error.md) |
| **Windows API** | [OpenThread](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openthread) |

## 另请参阅

- [ThreadHandle](ThreadHandle.md)
- [get_process_handle](get_process_handle.md)
- [try_open_thread](try_open_thread.md)