# apply_priority 函数 (apply.rs)

读取当前进程优先级类，如果与配置值不同，则将其设置为所需的级别。变更和错误会记录在提供的 [`ApplyConfigResult`](ApplyConfigResult.md) 中。

## 语法

```AffinityServiceRust/src/apply.rs#L85-91
pub fn apply_priority(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

`pid: u32`

目标进程的进程标识符。

`config: &ProcessLevelConfig`

包含所需 [`ProcessPriority`](../priority.rs/ProcessPriority.md) 值的 [`ProcessLevelConfig`](../config.rs/ProcessLevelConfig.md)。如果 `config.priority` 未映射到有效的 Windows 常量（`as_win_const()` 返回 `None`），则函数立即返回且不产生任何效果。

`dry_run: bool`

当为 **true** 时，函数在 `apply_config_result` 中记录*将要*发生的变更，但不调用任何 Windows API。当为 **false** 时，变更将应用于运行中的进程。

`process_handle: &ProcessHandle`

目标进程的 [`ProcessHandle`](../winapi.rs/ProcessHandle.md)。通过 [`get_handles`](get_handles.md) 提取读取句柄（用于查询当前优先级）和写入句柄（用于设置新优先级）。

`apply_config_result: &mut ApplyConfigResult`

变更描述和错误消息的累加器。参见 [`ApplyConfigResult`](ApplyConfigResult.md)。

## 返回值

此函数不返回值。结果通过 `apply_config_result` 传达。

## 备注

该函数遵循读取-比较-写入模式：

1. **提取句柄** — 调用 [`get_handles`](get_handles.md) 获取读取和写入 `HANDLE` 值。如果任一为 `None`，函数静默返回（进程句柄未成功打开）。
2. **检查配置** — 如果 `config.priority` 为 `ProcessPriority::None`（即 `as_win_const()` 返回 `None`），则不采取任何操作。
3. **读取当前值** — 使用读取句柄调用 `GetPriorityClass` 获取当前优先级类。
4. **比较** — 如果当前值已与配置值匹配，则无需变更。
5. **写入新值** — 在试运行模式下，变更消息被记录但不调用任何 API。否则，使用写入句柄调用 `SetPriorityClass`。
6. **记录结果** — 成功时，添加格式为 `"Priority: <old> -> <new>"` 的变更消息。失败时，[`log_error_if_new`](log_error_if_new.md) 仅在此特定 (pid, operation, error_code) 组合尚未被记录过时才记录错误。

### 错误处理

`SetPriorityClass` 的错误通过 `GetLastError` 捕获，并以 `Operation::SetPriorityClass` 操作传递给 [`log_error_if_new`](log_error_if_new.md)。相同进程和错误代码的重复错误会被抑制以避免日志泛滥。

### 平台说明

- `GetPriorityClass` 和 `SetPriorityClass` 是 Win32 线程 API 函数。写入句柄通常需要 `PROCESS_SET_INFORMATION` 访问权限。
- 优先级类值由 Windows 定义（例如 `IDLE_PRIORITY_CLASS`、`NORMAL_PRIORITY_CLASS`、`HIGH_PRIORITY_CLASS`、`REALTIME_PRIORITY_CLASS` 等）。

## 要求

| | |
|---|---|
| **模块** | `src/apply.rs` |
| **调用方** | 主执行循环（通过 `src/main.rs`） |
| **被调用方** | [`get_handles`](get_handles.md)、[`log_error_if_new`](log_error_if_new.md)、`GetPriorityClass`、`SetPriorityClass`、`GetLastError` |
| **Win32 API** | [`GetPriorityClass`](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-getpriorityclass)、[`SetPriorityClass`](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-setpriorityclass) |
| **权限** | `PROCESS_QUERY_LIMITED_INFORMATION`（读取）、`PROCESS_SET_INFORMATION`（写入） |

## 另请参阅

| 主题 | 描述 |
|---|---|
| [`ApplyConfigResult`](ApplyConfigResult.md) | 变更和错误的累加器 |
| [`ProcessLevelConfig`](../config.rs/ProcessLevelConfig.md) | 包含目标优先级的配置结构体 |
| [`ProcessPriority`](../priority.rs/ProcessPriority.md) | 将友好名称映射到 Windows 优先级类常量的枚举 |
| [`apply_affinity`](apply_affinity.md) | 应用 CPU 亲和性掩码的伴随函数 |
| [`apply_io_priority`](apply_io_priority.md) | 应用 IO 优先级的伴随函数 |
| [`apply_memory_priority`](apply_memory_priority.md) | 应用内存优先级的伴随函数 |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*