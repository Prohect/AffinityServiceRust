# apply_io_priority 函数 (apply.rs)

`apply_io_priority` 函数通过 `NtQueryInformationProcess` 读取进程当前的 I/O 优先级，如果与配置的目标值不同，则通过 `NtSetInformationProcess` 将其设置为期望的值。在试运行模式下，变更会被记录但不会实际执行设置调用。错误通过 [`log_error_if_new`](log_error_if_new.md) 进行去重，以避免对同一进程的重复失败在日志中产生大量重复条目。

## 语法

```AffinityServiceRust/src/apply.rs#L402-412
pub fn apply_io_priority(
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
| `config` | `&ProcessLevelConfig` | 进程级配置，包含期望的 `io_priority` 值（`IOPriority` 枚举）。如果 `config.io_priority` 无法映射到 Windows 常量（即 `as_win_const()` 返回 `None`），函数将立即返回，不执行任何操作。 |
| `dry_run` | `bool` | 为 `true` 时，函数将*预期*的变更记录到 `apply_config_result` 中，而不调用 `NtSetInformationProcess`。仍会查询当前 I/O 优先级，以便变更消息能显示变更前后的值。 |
| `process_handle` | `&ProcessHandle` | 提供对目标进程读写访问权限的句柄包装器。函数通过 [`get_handles`](get_handles.md) 提取读取句柄（用于 `NtQueryInformationProcess`）和写入句柄（用于 `NtSetInformationProcess`）。如果任一句柄不可用，函数将提前返回。 |
| `apply_config_result` | `&mut ApplyConfigResult` | 变更描述和错误消息的累加器。成功时（或试运行时），会追加格式为 `"IO Priority: <旧值> -> <新值>"` 的变更字符串。失败时，会追加错误字符串（受去重限制）。 |

## 返回值

此函数不返回值。所有结果通过 `apply_config_result` 参数传达。

## 备注

- 该函数使用未公开的 `NtQueryInformationProcess` 和 `NtSetInformationProcess` NT API，信息类常量为 `PROCESS_INFORMATION_IO_PRIORITY`（值为 `33`）。此常量在函数体内部局部定义。
- 当前 I/O 优先级通过向 `NtQueryInformationProcess` 传递 `u32` 大小的缓冲区来查询。检查 NTSTATUS 返回值：负值表示失败，错误通过 [`log_error_if_new`](log_error_if_new.md) 使用 `error_from_ntstatus` 将 NTSTATUS 解码为可读字符串进行记录。查询失败时函数直接返回，不再尝试设置。
- 如果查询成功且当前 I/O 优先级已与目标匹配，则不执行任何操作，也不记录任何内容。
- 设置 I/O 优先级时，使用写入句柄调用 `NtSetInformationProcess`。NTSTATUS 返回值为负表示失败；错误使用不同的 `Operation` 变体（`NtSetInformationProcess2ProcessInformationIOPriority`）记录，以区分查询错误。
- 变更消息格式为 `"IO Priority: <当前名称> -> <目标名称>"`，使用 `IOPriority::from_win_const` 和 `IOPriority::as_str` 获取可读名称。
- Windows I/O 优先级级别通常包括 Very Low、Low、Normal、High 和 Critical，分别表示为整数常量 `0` 到 `4`。
- 查询操作的错误去重使用 `Operation::NtQueryInformationProcess2ProcessInformationIOPriority`，设置操作使用 `Operation::NtSetInformationProcess2ProcessInformationIOPriority`。这些是不同的变体，允许对同一进程的查询与设置错误进行独立抑制。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `apply.rs` |
| Crate | `AffinityServiceRust` |
| NT API | `NtQueryInformationProcess`（信息类 33）、`NtSetInformationProcess`（信息类 33） |
| 调用者 | `scheduler.rs` / `main.rs` 中遍历匹配进程的编排代码 |
| 被调用者 | [`get_handles`](get_handles.md)、[`log_error_if_new`](log_error_if_new.md)、`IOPriority::as_win_const`、`IOPriority::from_win_const`、`IOPriority::as_str`、`error_from_ntstatus`（error_codes 模块） |
| 权限 | 需要具有 `PROCESS_QUERY_INFORMATION` 或 `PROCESS_QUERY_LIMITED_INFORMATION`（读取）以及 `PROCESS_SET_INFORMATION`（写入）的进程句柄。 |

## 另请参阅

| 参考 | 链接 |
|------|------|
| apply 模块概述 | [`README`](README.md) |
| ApplyConfigResult | [`ApplyConfigResult`](ApplyConfigResult.md) |
| get_handles | [`get_handles`](get_handles.md) |
| log_error_if_new | [`log_error_if_new`](log_error_if_new.md) |
| apply_priority | [`apply_priority`](apply_priority.md) |
| apply_memory_priority | [`apply_memory_priority`](apply_memory_priority.md) |
| apply_affinity | [`apply_affinity`](apply_affinity.md) |
| ProcessLevelConfig | [`config.rs/ProcessLevelConfig`](../config.rs/ProcessLevelConfig.md) |
| IOPriority | [`priority.rs/IOPriority`](../priority.rs/IOPriority.md) |

---
*Commit: 7221ea0694670265d4eb4975582d8ed2ae02439d*