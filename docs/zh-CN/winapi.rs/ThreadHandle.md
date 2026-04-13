# ThreadHandle 结构体 (winapi.rs)

RAII 容器，持有最多四个以不同访问级别打开的 Windows 线程句柄。`r_limited_handle` 字段始终有效（在构造时必须成功获取）；其余三个句柄会尝试获取，但如果对应的 `OpenThread` 调用失败，则可能持有无效的哨兵值。当 `ThreadHandle` 被销毁时，所有有效句柄会通过 `CloseHandle` 自动关闭。

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
|------|------|---------|------|
| `r_limited_handle` | `HANDLE` | `THREAD_QUERY_LIMITED_INFORMATION` | 始终有效。必需句柄——如果此打开失败，[get_thread_handle](get_thread_handle.md) 将返回 `None` 而不是构造 `ThreadHandle`。足以用于轻量级查询，如 `QueryThreadCycleTime`。 |
| `r_handle` | `HANDLE` | `THREAD_QUERY_INFORMATION` | 有效或 `HANDLE::default()`（无效哨兵值）。用于 `NtQueryInformationThread` 调用，如 [get_thread_start_address](get_thread_start_address.md) 以及 [get_thread_ideal_processor_ex](get_thread_ideal_processor_ex.md)。 |
| `w_limited_handle` | `HANDLE` | `THREAD_SET_LIMITED_INFORMATION` | 有效或 `HANDLE::default()`（无效哨兵值）。用于通过 `SetThreadSelectedCpuSets` 将主线程固定到特定 CPU 集合。 |
| `w_handle` | `HANDLE` | `THREAD_SET_INFORMATION` | 有效或 `HANDLE::default()`（无效哨兵值）。用于 [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) 和 `SetThreadPriority`。 |

## 备注

### 句柄有效性模型

与使用 `Option<HANDLE>` 来处理非必需句柄的 [ProcessHandle](ProcessHandle.md) 不同，`ThreadHandle` 存储原始 `HANDLE` 值，并依赖 `HANDLE::is_invalid()` 来区分成功与失败。这种设计反映了线程句柄是批量创建的（每个进程的每个线程一个），因此更轻量的表示可以减少分配压力。

调用者在使用 `r_handle`、`w_limited_handle` 或 `w_handle` 之前应检查 `is_invalid()`：

```rust
if !thread_handle.w_handle.is_invalid() {
    set_thread_ideal_processor_ex(thread_handle.w_handle, group, number)?;
}
```

### Drop 行为

`Drop` 实现会逐个关闭每个句柄，跳过无效的句柄：

1. `r_limited_handle` — 始终关闭（始终有效）。
2. `r_handle` — 仅在 `!is_invalid()` 时关闭。
3. `w_limited_handle` — 仅在 `!is_invalid()` 时关闭。
4. `w_handle` — 仅在 `!is_invalid()` 时关闭。

每个 `CloseHandle` 调用都包装在 `unsafe` 中，其返回值被丢弃，这与 Windows 的惯例一致——关闭有效句柄不会产生有意义的失败。

### 典型生命周期

`ThreadHandle` 实例通常存储在 [ThreadStats](../scheduler.rs/ThreadStats.md) 中，贯穿进程的整个跟踪生命周期。它们在主线程调度器首次遇到某个线程时由 [get_thread_handle](get_thread_handle.md) 创建，并在线程退出或所属 `ProcessStats` 被移除时销毁。

### 访问级别设计原理

四种访问级别涵盖了 AffinityServiceRust 执行的全部线程操作：

| 访问权限 | 用途 |
|---------|------|
| `THREAD_QUERY_LIMITED_INFORMATION` | `QueryThreadCycleTime` |
| `THREAD_QUERY_INFORMATION` | `NtQueryInformationThread`（起始地址）、`GetThreadIdealProcessorEx` |
| `THREAD_SET_LIMITED_INFORMATION` | `SetThreadSelectedCpuSets` |
| `THREAD_SET_INFORMATION` | `SetThreadIdealProcessorEx`、`SetThreadPriority` |

受保护进程可能拒绝完全访问变体但仍授予受限访问，因此拆分句柄允许在提升权限不足以获得完全控制时仍能部分运行。

## 要求

| | |
|---|---|
| **模块** | `winapi` (`src/winapi.rs`) |
| **构造者** | [get_thread_handle](get_thread_handle.md) |
| **存储于** | [ThreadStats](../scheduler.rs/ThreadStats.md)（字段 `handle`） |
| **使用者** | [apply_prime_threads](../apply.rs/apply_prime_threads.md)、[apply_ideal_processors](../apply.rs/apply_ideal_processors.md)、[prefetch_all_thread_cycles](../apply.rs/prefetch_all_thread_cycles.md) |
| **API** | `OpenThread`、`CloseHandle`（Win32 Threading） |
| **特权** | 建议启用 `SeDebugPrivilege` 以进行跨进程线程访问 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 线程句柄构造函数 | [get_thread_handle](get_thread_handle.md) |
| 单访问权限线程打开辅助函数 | [try_open_thread](try_open_thread.md) |
| 进程句柄对应物 | [ProcessHandle](ProcessHandle.md) |
| 线程统计信息存储 | [ThreadStats](../scheduler.rs/ThreadStats.md) |
| 线程起始地址查询 | [get_thread_start_address](get_thread_start_address.md) |
| 理想处理器设置器 | [set_thread_ideal_processor_ex](set_thread_ideal_processor_ex.md) |
| OpenThread (MSDN) | [Microsoft Learn — OpenThread](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openthread) |