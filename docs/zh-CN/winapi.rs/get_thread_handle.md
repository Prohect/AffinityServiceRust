# get_thread_handle 函数 (winapi.rs)

为给定线程 ID 打开多个访问级别的 Windows 线程句柄集。返回一个 [ThreadHandle](ThreadHandle.md) RAII 包装器，在析构时自动关闭所有打开的句柄。函数需要 `THREAD_QUERY_LIMITED_INFORMATION` 作为最低权限；其他访问级别会被尝试但允许优雅地失败。

## 语法

```rust
pub fn get_thread_handle(tid: u32, pid: u32, process_name: &str) -> Option<ThreadHandle>
```

## 参数

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `tid` | `u32` | 目标线程的线程标识符。 |
| `pid` | `u32` | 拥有该线程的进程标识符。仅用于错误日志记录和通过 [is_new_error](../logging.rs/is_new_error.md) 进行去重。 |
| `process_name` | `&str` | 拥有进程的名称。仅用于错误日志上下文。 |

## 返回值

`Option<ThreadHandle>` — 如果成功打开了必需的 `r_limited_handle`，返回 `Some(ThreadHandle)`。如果无法获取所需的最小句柄，返回 `None`。

当返回 `Some` 时，[ThreadHandle](ThreadHandle.md) 包含：

| 句柄字段 | 访问权限 | 必需 | 失败时的行为 |
|---|---|---|---|
| `r_limited_handle` | `THREAD_QUERY_LIMITED_INFORMATION` | **是** | 函数返回 `None` |
| `r_handle` | `THREAD_QUERY_INFORMATION` | 否 | 设置为无效的 `HANDLE` |
| `w_limited_handle` | `THREAD_SET_LIMITED_INFORMATION` | 否 | 设置为无效的 `HANDLE` |
| `w_handle` | `THREAD_SET_INFORMATION` | 否 | 设置为无效的 `HANDLE` |

## 说明

函数遵循分层句柄打开策略：

1. **必需句柄** — 首先通过 `OpenThread` 打开 `THREAD_QUERY_LIMITED_INFORMATION`。如果此调用失败或返回无效句柄，错误将被记录（通过 [is_new_error](../logging.rs/is_new_error.md) 对每个唯一的 pid/tid/操作/错误组合仅记录一次），函数立即返回 `None`。

2. **可选句柄** — 通过 [try_open_thread](try_open_thread.md) 分别尝试打开 `THREAD_QUERY_INFORMATION`、`THREAD_SET_LIMITED_INFORMATION` 和 `THREAD_SET_INFORMATION`。这些句柄的失败会被静默吸收（源代码中的错误日志被注释掉），相应的字段设置为 `HANDLE::default()`（无效句柄）。调用方在使用这些句柄前必须检查 `is_invalid()`。

### 错误码映射

每个句柄打开尝试被分配一个内部操作码用于错误去重：

| 代码 | 句柄 | 访问权限 |
|------|--------|-------------|
| `0` | `r_limited_handle` | `THREAD_QUERY_LIMITED_INFORMATION` |
| `1` | `r_handle` | `THREAD_QUERY_INFORMATION` |
| `2` | `w_limited_handle` | `THREAD_SET_LIMITED_INFORMATION` |
| `3` | `w_handle` | `THREAD_SET_INFORMATION` |

### 句柄生命周期

所有返回的句柄由 [ThreadHandle](ThreadHandle.md) 结构体拥有。当 `ThreadHandle` 被析构时，它们通过 `CloseHandle` 自动关闭。调用方不应手动关闭这些句柄。

### 典型用法

线程句柄通常只打开一次，并缓存在 [ThreadStats::handle](../scheduler.rs/ThreadStats.md) 中以在各轮询迭代中重用。这避免了每个周期都调用 `OpenThread` 的开销。句柄缓存会在拥有进程退出时清除。

### 访问被拒绝场景

在没有 [SeDebugPrivilege](enable_debug_privilege.md) 的情况下，属于提升权限或受保护进程的线程可能会拒绝 `THREAD_QUERY_LIMITED_INFORMATION` 请求，返回 `ERROR_ACCESS_DENIED` (5)，导致函数返回 `None`。在启动时启用 SeDebugPrivilege 可以解决大多数进程的这个问题。

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用方** | [get_handles](../apply.rs/get_handles.md)、[prefetch_all_thread_cycles](../apply.rs/prefetch_all_thread_cycles.md) |
| **被调用方** | [try_open_thread](try_open_thread.md)、[is_new_error](../logging.rs/is_new_error.md) |
| **Win32 API** | [OpenThread](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-openthread) |
| **权限** | 不需要；建议 [SeDebugPrivilege](enable_debug_privilege.md) 以获得完全访问权限 |

## 参见

| 主题 | 链接 |
|-------|------|
| 线程句柄 RAII 封装 | [ThreadHandle](ThreadHandle.md) |
| 低级线程打开辅助函数 | [try_open_thread](try_open_thread.md) |
| 进程句柄等价物 | [get_process_handle](get_process_handle.md) |
| 缓存句柄的线程统计 | [ThreadStats](../scheduler.rs/ThreadStats.md) |
| 模块概述 | [winapi.rs](README.md) |

*文档版本：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*