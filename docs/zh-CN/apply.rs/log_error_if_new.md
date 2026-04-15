# log_error_if_new 函数 (apply.rs)

`log_error_if_new` 函数仅在给定的 pid/tid/操作/错误码组合之前未被记录过时，才将错误消息追加到 `ApplyConfigResult` 中。这可以防止同一重复错误在每个应用周期中淹没变更日志，同时确保每个不同的失败至少被报告一次。

## 语法

```AffinityServiceRust/src/apply.rs#L71-83
#[inline(always)]
fn log_error_if_new(
    pid: u32,
    tid: u32,
    process_name: &str,
    operation: Operation,
    error_code: u32,
    apply_config_result: &mut ApplyConfigResult,
    format_msg: impl FnOnce() -> String,
)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 与错误关联的进程 ID。与 `tid`、`process_name`、`operation` 和 `error_code` 一起组成去重键。 |
| `tid` | `u32` | 与错误关联的线程 ID。对于非线程特定的进程级操作，传入 `0`。 |
| `process_name` | `&str` | 进程或配置规则的人类可读名称，用作去重键的一部分。 |
| `operation` | `Operation` | `logging` 模块中的枚举变体，标识哪个 Windows API 调用失败（例如 `Operation::SetPriorityClass`、`Operation::SetThreadSelectedCpuSets`）。 |
| `error_code` | `u32` | 失败的 API 调用返回的原始 Windows 错误码（Win32 `GetLastError` 结果或转换为 `u32` 的 NTSTATUS）。 |
| `apply_config_result` | `&mut ApplyConfigResult` | 结果累加器，如果错误是新的，则将错误字符串追加到其中。 |
| `format_msg` | `impl FnOnce() -> String` | 一个延迟求值的闭包，用于生成格式化的错误消息字符串。该闭包仅在确定错误为新错误时才会被调用，从而避免了对重复错误进行字符串格式化的开销。 |

## 返回值

此函数不返回值。

## 备注

- 该函数将去重工作委托给 `logging::is_new_error`，后者维护一个持久化的已见错误键集合。如果 `is_new_error` 返回 `true`，则调用 `format_msg` 闭包并将其结果传递给 `ApplyConfigResult::add_error`。否则，错误将被静默抑制。
- `format_msg` 参数使用 `impl FnOnce() -> String` 而非预格式化的 `String`，以推迟 `format!()` 宏展开的开销。在高频应用循环中，当同一错误每个周期都重复出现时，这可以避免数千次不必要的堆分配。
- 该函数标记为 `#[inline(always)]`，因为它在整个模块的每个错误点都会被调用，且函数体很小（一个分支加一次闭包调用）。
- 这是一个模块私有函数（`fn`，而非 `pub fn`）。它仅被 `apply.rs` 内的其他函数使用。
- 按照惯例，当错误发生在进程级别时（例如 `apply_priority`、`apply_affinity`），`tid` 参数设置为 `0`。线程级调用者（例如 `apply_prime_threads_promote`、`apply_prime_threads_demote`、`apply_ideal_processors`）传递实际的线程 ID。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `apply.rs` |
| Crate | `AffinityServiceRust` |
| 可见性 | 私有（crate 内部） |
| 依赖 | `logging::is_new_error`、`logging::Operation` |
| 调用者 | `apply_priority`、`apply_affinity`、`reset_thread_ideal_processors`、`apply_process_default_cpuset`、`apply_io_priority`、`apply_memory_priority`、`prefetch_all_thread_cycles`、`apply_prime_threads_promote`、`apply_prime_threads_demote`、`apply_ideal_processors` |
| 平台 | Windows |

## 另请参阅

| 参考 | 链接 |
|------|------|
| apply 模块概述 | [`README`](README.md) |
| ApplyConfigResult | [`ApplyConfigResult`](ApplyConfigResult.md) |
| get_handles | [`get_handles`](get_handles.md) |
| logging 模块 | [`logging.rs`](../logging.rs/README.md) |

---
*Commit: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)*