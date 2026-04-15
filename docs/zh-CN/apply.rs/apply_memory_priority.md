# apply_memory_priority 函数 (apply.rs)

`apply_memory_priority` 函数通过 `GetProcessInformation` 使用 `ProcessMemoryPriority` 信息类读取当前进程内存优先级，如果与配置的目标值不同，则通过 `SetProcessInformation` 设置新值。在试运行模式下，仅记录预期的更改而不调用任何修改状态的 API。

## 语法

```AffinityServiceRust/src/apply.rs#L490-498
pub fn apply_memory_priority(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 目标进程的进程标识符。用于错误去重键和日志消息。 |
| `config` | `&ProcessLevelConfig` | 包含所需 `memory_priority` 值（`MemoryPriority` 枚举）的进程级配置。如果 `config.memory_priority.as_win_const()` 返回 `None`（例如 `MemoryPriority::None`），函数将立即返回，不查询也不修改进程。 |
| `dry_run` | `bool` | 为 `true` 时，函数在 `apply_config_result` 中记录*将要*进行的更改，但不调用 `SetProcessInformation`。为 `false` 时，调用 Windows API 来应用更改。 |
| `process_handle` | `&ProcessHandle` | 提供对目标进程读写访问的句柄包装器。函数通过 [`get_handles`](get_handles.md) 提取读取句柄（用于 `GetProcessInformation`）和写入句柄（用于 `SetProcessInformation`）。如果任一句柄不可用，函数将提前返回。 |
| `apply_config_result` | `&mut ApplyConfigResult` | 执行过程中产生的更改描述和错误消息的累加器。 |

## 返回值

此函数不返回值。所有结果通过 `apply_config_result` 参数传达。

## 备注

- 函数首先使用 `ProcessMemoryPriority` 信息类和 `MemoryPriorityInformation` 结构体调用 `GetProcessInformation` 来获取当前内存优先级。如果查询失败，通过 `GetLastError` 获取 Win32 错误代码，并通过 [`log_error_if_new`](log_error_if_new.md) 以 `Operation::GetProcessInformation2ProcessMemoryPriority` 记录。查询失败后不会进行进一步操作。
- 如果当前内存优先级已与目标匹配，则不记录任何更改，函数静默返回。
- 应用更改时（非试运行），`SetProcessInformation` 使用包含目标值的新 `MemoryPriorityInformation` 结构体调用。失败时，通过 [`log_error_if_new`](log_error_if_new.md) 以 `Operation::SetProcessInformation2ProcessMemoryPriority` 记录错误。
- 更改消息格式为 `"Memory Priority: <current> -> <target>"`，使用 `MemoryPriority::from_win_const` 和 `config.memory_priority.as_str()` 获取人类可读的字符串表示。
- `MemoryPriorityInformation` 包装器类型是在 `priority` 模块中定义的 `u32` 新类型。它与 Windows `MEMORY_PRIORITY_INFORMATION` 结构体的布局匹配。
- Windows 内存优先级值范围从 0（最低/极低）到 5（正常）。`priority` 模块中的 `MemoryPriority` 枚举将用户可见的名称映射到这些数值常量。
- **注意：** 在试运行路径中，更改消息文本引用了 `config.io_priority.as_str()` 而不是 `config.memory_priority.as_str()`。这是源代码中的一个已知不一致，试运行消息显示的是 I/O 优先级标签而非内存优先级标签。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `apply.rs` |
| Crate | `AffinityServiceRust` |
| Windows API | `GetProcessInformation`（`ProcessMemoryPriority`）、`SetProcessInformation`（`ProcessMemoryPriority`）、`GetLastError` |
| 调用方 | `scheduler.rs` / `main.rs` 中遍历匹配进程的编排代码 |
| 被调用方 | [`get_handles`](get_handles.md)、[`log_error_if_new`](log_error_if_new.md)、`MemoryPriority::as_win_const`、`MemoryPriority::from_win_const`、`MemoryPriority::as_str`、`error_from_code_win32` |
| 权限 | 需要具有 `PROCESS_QUERY_INFORMATION` 或 `PROCESS_QUERY_LIMITED_INFORMATION`（读取）和 `PROCESS_SET_INFORMATION`（写入）的进程句柄。 |

## 另请参阅

| 参考 | 链接 |
|------|------|
| apply 模块概述 | [`README`](README.md) |
| ApplyConfigResult | [`ApplyConfigResult`](ApplyConfigResult.md) |
| get_handles | [`get_handles`](get_handles.md) |
| log_error_if_new | [`log_error_if_new`](log_error_if_new.md) |
| apply_priority | [`apply_priority`](apply_priority.md) |
| apply_io_priority | [`apply_io_priority`](apply_io_priority.md) |
| apply_affinity | [`apply_affinity`](apply_affinity.md) |
| ProcessLevelConfig | [`config.rs/ProcessLevelConfig`](../config.rs/ProcessLevelConfig.md) |
| MemoryPriority | [`priority.rs/MemoryPriority`](../priority.rs/MemoryPriority.md) |

---
*Commit: 7221ea0694670265d4eb4975582d8ed2ae02439d*