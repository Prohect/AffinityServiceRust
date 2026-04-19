# ThreadHandle 结构体（winapi.rs）

RAII 包装器，用于在多个访问级别上打开 Windows 线程句柄集。在丢弃时自动关闭所有有效的句柄。`r_limited_handle` 在结构体存在时始终有效；其他句柄可能无效，如果相应的 `OpenThread` 调用因权限不足而失败。

## 语法

```rust
#[derive(Debug)]
pub struct ThreadHandle {
    pub r_limited_handle: HANDLE,
    pub r_handle: HANDLE,
    pub w_limited_handle: HANDLE,
    pub w_handle: HANDLE,
}
```

## 成员

| 成员 | 类型 | 访问权限 | 描述 |
|--------|------|-------------|-------------|
| `r_limited_handle` | `HANDLE` | `THREAD_QUERY_LIMITED_INFORMATION` | 始终有效的读取句柄，用于基本线程查询，如周期时间检索。这是最低访问级别，对于结构体构造是必需的。 |
| `r_handle` | `HANDLE` | `THREAD_QUERY_INFORMATION` | 用于高级查询的完整读取句柄，如 [get_thread_start_address](get_thread_start_address.md)（通过 `NtQueryInformationThread`）。如果访问被拒绝，可能是无效句柄 (`HANDLE::default()`)。使用前请用 `is_invalid()` 检查。 |
| `w_limited_handle` | `HANDLE` | `THREAD_SET_LIMITED_INFORMATION` | 用于设置线程 CPU 集合分配等操作的限制写入句柄。如果访问被拒绝，可能无效。 |
| `w_handle` | `HANDLE` | `THREAD_SET_INFORMATION` | 用于设置线程理想处理器 [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) 和线程优先级更改等操作的全写入句柄。如果访问被拒绝，可能无效。 |

## Drop

```rust
impl Drop for ThreadHandle {
    fn drop(&mut self);
}
```

关闭结构体持有的所有句柄。`r_limited_handle` 始终无条件关闭（保证有效）。其余三个句柄（`r_handle`、`w_limited_handle`、`w_handle`）仅在它们不是无效句柄时才关闭，这通过 `HANDLE::is_invalid()` 确定。

### 句柄关闭顺序

1. `r_limited_handle` — 始终关闭
2. `r_handle` — 如果不是无效则关闭
3. `w_limited_handle` — 如果不是无效则关闭
4. `w_handle` — 如果不是无效则关闭

每次关闭调用 `CloseHandle` 并丢弃结果。

## 说明

### 与 ProcessHandle 的区别

与使用 `Option<HANDLE>` 作为可选句柄的 [ProcessHandle](ProcessHandle.md) 不同，`ThreadHandle` 使用原始 `HANDLE` 值并通过 `is_invalid()` 区分有效和失败的句柄。这种设计差异是因为线程句柄在循环中被更频繁地访问（每个线程、每次迭代），避免 `Option` 解包可以减少开销。

### 访问级别策略

四个句柄代表读取/写入 × 限制/完整访问的矩阵：

|  | 限制 | 完整 |
|--|---------|------|
| **读取** | `r_limited_handle` | `r_handle` |
| **写入** | `w_limited_handle` | `w_handle` |

受限访问权限的成功率更高（例如，对于受保护进程），但支持的操作较少。调用方应优先使用受限句柄，并在必要时在完整句柄上使用 `is_invalid()` 检查结果。

### ThreadStats 中的缓存

`ThreadHandle` 实例缓存在 [ThreadStats](../scheduler.rs/ThreadStats.md)`.handle` 中，以避免每次轮询迭代时重新打开句柄。句柄在线程生命周期内或直到父进程退出且 [PrimeThreadScheduler::drop_process_by_pid](../scheduler.rs/PrimeThreadScheduler.md) 丢弃统计条目时持续存在。

### Send 安全性

`ThreadHandle` 是 `Send` 的，因为 `HANDLE` 是围绕指针大小值的薄包装，Windows 句柄可以在同一进程内的任何线程中使用。

## 要求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用方** | [get_thread_handle](get_thread_handle.md)、[ThreadStats](../scheduler.rs/ThreadStats.md)、[prefetch_all_thread_cycles](../apply.rs/prefetch_all_thread_cycles.md)、[apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md)、[apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |
| **被调用方** | `CloseHandle`（Win32） |
| **Win32 API** | [CloseHandle](https://learn.microsoft.com/zh-cn/windows/win32/api/handleapi/nf-handleapi-closehandle) |
| **权限** | Drop 时不需要；创建需要适当的线程访问权限（参见 [get_thread_handle](get_thread_handle.md)） |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [winapi.rs](README.md) |
| 线程句柄工厂 | [get_thread_handle](get_thread_handle.md) |
| 低级线程打开器 | [try_open_thread](try_open_thread.md) |
| 进程句柄对应物 | [ProcessHandle](ProcessHandle.md) |
| 线程统计缓存 | [ThreadStats](../scheduler.rs/ThreadStats.md) |

*文档化于提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*