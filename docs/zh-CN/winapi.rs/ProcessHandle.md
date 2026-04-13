# ProcessHandle 结构体 (winapi.rs)

RAII 容器，最多持有四个以不同访问级别打开的 Windows 进程句柄。当 `ProcessHandle` 被丢弃（drop）时，所有有效句柄会通过 `CloseHandle` 自动关闭。两个受限句柄（`r_limited_handle` 和 `w_limited_handle`）在结构体存在时始终有效；完全访问句柄（`r_handle` 和 `w_handle`）使用 `Option` 包装，如果调用方缺少足够的特权来以该访问级别打开进程，则可能为 `None`。

## 语法

```rust
pub struct ProcessHandle {
    pub r_limited_handle: HANDLE,
    pub r_handle: Option<HANDLE>,
    pub w_limited_handle: HANDLE,
    pub w_handle: Option<HANDLE>,
}
```

## 成员

| 成员 | 类型 | 描述 |
|------|------|------|
| `r_limited_handle` | `HANDLE` | 以 `PROCESS_QUERY_LIMITED_INFORMATION` 打开的句柄。在结构体存在时始终有效。用于轻量级查询，如读取 CPU 集合和进程时间。 |
| `r_handle` | `Option<HANDLE>` | 以 `PROCESS_QUERY_INFORMATION` 打开的句柄。当调用方有足够的访问权限时为 `Some`；否则为 `None`。用于 `GetProcessAffinityMask` 和较高信息类的 `NtQueryInformationProcess` 等操作。 |
| `w_limited_handle` | `HANDLE` | 以 `PROCESS_SET_LIMITED_INFORMATION` 打开的句柄。在结构体存在时始终有效。用于通过 `SetProcessDefaultCpuSets` 设置 CPU 集合。 |
| `w_handle` | `Option<HANDLE>` | 以 `PROCESS_SET_INFORMATION` 打开的句柄。当调用方有足够的访问权限时为 `Some`；否则为 `None`。用于 `SetPriorityClass`、`SetProcessAffinityMask` 和 `NtSetInformationProcess` 等操作。 |

## 备注

### 构造

`ProcessHandle` 实例仅由 [get_process_handle](get_process_handle.md) 创建。该函数按顺序打开所有四个访问级别；两个受限句柄是必需的（失败时函数返回 `None`），而两个完全访问句柄会优雅降级为 `None`。这种设计使 AffinityServiceRust 能够在当前特权级别允许的范围内尽可能多地应用设置，而不会因为访问受限而完全失败。

### Drop 行为

`Drop` 实现按以下顺序关闭句柄：

1. `r_handle`（如果为 `Some`）
2. `w_handle`（如果为 `Some`）
3. `r_limited_handle`（始终关闭）
4. `w_limited_handle`（始终关闭）

每个 `CloseHandle` 调用都包装在 `unsafe` 中，其返回值被有意忽略——如果关闭句柄失败，没有有意义的恢复操作。

### 句柄访问级别映射

| 句柄 | Win32 访问标志 | 典型操作 |
|------|----------------|----------|
| `r_limited_handle` | `PROCESS_QUERY_LIMITED_INFORMATION` | `GetProcessDefaultCpuSets`、`GetProcessInformation`（内存优先级） |
| `r_handle` | `PROCESS_QUERY_INFORMATION` | `GetProcessAffinityMask`、`NtQueryInformationProcess`（IO 优先级） |
| `w_limited_handle` | `PROCESS_SET_LIMITED_INFORMATION` | `SetProcessDefaultCpuSets` |
| `w_handle` | `PROCESS_SET_INFORMATION` | `SetPriorityClass`、`SetProcessAffinityMask`、`NtSetInformationProcess`（IO / 内存优先级） |

### 在 apply 模块中的使用

[apply 模块](../apply.rs/README.md)接收 `&ProcessHandle` 引用，并通过辅助函数 `get_handles` 为每个操作选择适当的句柄，该函数返回最佳可用的读写句柄（优先选择完全访问而非受限访问）。

## 要求

| | |
|---|---|
| **模块** | `winapi`（`src/winapi.rs`） |
| **构造方** | [get_process_handle](get_process_handle.md) |
| **使用方** | [apply_priority](../apply.rs/apply_priority.md)、[apply_affinity](../apply.rs/apply_affinity.md)、[apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md)、[apply_io_priority](../apply.rs/apply_io_priority.md)、[apply_memory_priority](../apply.rs/apply_memory_priority.md) |
| **Win32 API** | `OpenProcess`、`CloseHandle` |
| **特权** | 建议对受保护进程持有 `SeDebugPrivilege` 以获取完全访问句柄 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 打开并返回 ProcessHandle | [get_process_handle](get_process_handle.md) |
| 线程句柄 RAII 容器 | [ThreadHandle](ThreadHandle.md) |
| 规则应用入口点（进程级别） | [apply_config_process_level](../main.rs/apply_config_process_level.md) |
| 句柄失败的日志操作 | [Operation 枚举](../logging.rs/README.md) |
| 错误码格式化 | [error_codes 模块](../error_codes.rs/README.md) |