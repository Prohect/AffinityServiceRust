# apply_priority 函数 (apply.rs)

读取当前进程优先级类别，如果与配置的目标值不同，则将其设置为所需的值。在试运行模式下，变更会被记录但不会调用 Windows API。错误通过 `log_error_if_new` 进行去重，以确保同一进程/操作/错误码组合的重复失败不会生成重复的日志条目。

## 语法

```AffinityServiceRust/src/apply.rs#L85-131
pub fn apply_priority(
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
| `pid` | `u32` | 目标进程的进程标识符。用于错误去重和日志消息。 |
| `config` | `&ProcessLevelConfig` | 包含所需 `priority` 值（`ProcessPriority` 枚举）的进程级配置。如果 `config.priority` 未映射到 Windows 常量（即 `as_win_const()` 返回 `None`），则函数立即返回不执行任何操作。 |
| `dry_run` | `bool` | 当为 `true` 时，函数在 `apply_config_result` 中记录*将要*进行的变更，而不调用 `SetPriorityClass`。当为 `false` 时，调用 Windows API 应用变更。 |
| `process_handle` | `&ProcessHandle` | 提供目标进程读写访问权限的句柄包装器。函数通过 [`get_handles`](get_handles.md) 提取读取句柄（用于 `GetPriorityClass`）和写入句柄（用于 `SetPriorityClass`）。如果任一句柄不可用，函数立即返回。 |
| `apply_config_result` | `&mut ApplyConfigResult` | 变更描述和错误消息的累加器。成功（或试运行）时，会追加格式为 `"Priority: <旧值> -> <新值>"` 的变更字符串。失败时，会追加错误字符串（受去重机制控制）。 |

## 返回值

此函数不返回值。所有结果通过 `apply_config_result` 参数传递。

## 备注

- 函数首先使用读取句柄调用 `GetPriorityClass` 获取当前优先级类别。如果当前值已与配置目标匹配，则不执行任何操作且不记录任何内容。
- 当前优先级类别通过 `ProcessPriority::from_win_const` 解码回可读字符串，用于变更消息。
- 当 `SetPriorityClass` 失败时，通过 `GetLastError` 获取 Win32 错误码并传递给 [`log_error_if_new`](log_error_if_new.md)，该函数仅在此特定 `pid`/`Operation::SetPriorityClass`/错误码三元组之前未出现过时才记录错误。
- Windows 优先级类别常量是标准值，如 `IDLE_PRIORITY_CLASS`、`BELOW_NORMAL_PRIORITY_CLASS`、`NORMAL_PRIORITY_CLASS`、`ABOVE_NORMAL_PRIORITY_CLASS`、`HIGH_PRIORITY_CLASS` 和 `REALTIME_PRIORITY_CLASS`。
- 如果 `config.priority` 为 `ProcessPriority::None`（或任何 `as_win_const()` 返回 `None` 的变体），函数将直接退出，不查询或修改进程。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `apply.rs` |
| Crate | `AffinityServiceRust` |
| Windows API | `GetPriorityClass`、`SetPriorityClass`、`GetLastError` |
| 调用者 | `scheduler.rs` / `main.rs` 中迭代匹配进程的编排代码 |
| 被调用者 | [`get_handles`](get_handles.md)、[`log_error_if_new`](log_error_if_new.md)、`ProcessPriority::as_win_const`、`ProcessPriority::from_win_const`、`error_from_code_win32` |
| 权限 | 需要具有 `PROCESS_QUERY_INFORMATION` 或 `PROCESS_QUERY_LIMITED_INFORMATION`（读取）和 `PROCESS_SET_INFORMATION`（写入）的进程句柄。 |

## 另请参阅

| 参考 | 链接 |
|------|------|
| apply 模块概述 | [`README`](README.md) |
| ApplyConfigResult | [`ApplyConfigResult`](ApplyConfigResult.md) |
| get_handles | [`get_handles`](get_handles.md) |
| log_error_if_new | [`log_error_if_new`](log_error_if_new.md) |
| apply_affinity | [`apply_affinity`](apply_affinity.md) |
| apply_io_priority | [`apply_io_priority`](apply_io_priority.md) |
| apply_memory_priority | [`apply_memory_priority`](apply_memory_priority.md) |
| ProcessLevelConfig | [`config.rs/ProcessLevelConfig`](../config.rs/ProcessLevelConfig.md) |
| ProcessPriority | [`priority.rs/ProcessPriority`](../priority.rs/ProcessPriority.md) |

---
*Commit: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)*