# log_error_if_new 函数 (apply.rs)

仅当对同一进程/线程/操作组合首次出现错误时才记录日志。此去重机制可防止在多次轮询迭代中重复失败的操作产生大量重复错误日志。

## 语法

```rust
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

`pid`

与错误关联的进程 ID。

`tid`

与错误关联的线程 ID。对于非线程级别的进程操作，使用 `0`。

`process_name`

进程名称，用作去重键的一部分，同时包含在错误消息中。

`operation`

[Operation](../logging.rs/Operation.md) 枚举变体，标识失败的 Windows API 调用或逻辑操作。

`error_code`

失败操作返回的 Win32 错误码或 NTSTATUS 值。与 `pid`、`tid`、`process_name` 和 `operation` 组合形成去重键。

`apply_config_result`

[ApplyConfigResult](ApplyConfigResult.md) 的可变引用，用于收集当前应用周期中的错误。若错误为新出现的，格式化后的消息将通过 `add_error()` 添加。

`format_msg`

一个闭包（`FnOnce() -> String`），用于生成错误消息字符串。该闭包延迟求值——仅在判定错误为新错误时才调用，避免在错误已被记录过的情况下产生不必要的格式化开销。

## 返回值

此函数无返回值。

## 备注

这是一个辅助函数，在 `apply.rs` 模块中被所有与 Windows API 交互的函数广泛使用。它封装了 logging 模块中的 [is_new_error](../logging.rs/is_new_error.md)。

去重逻辑的工作流程如下：

1. 调用 `is_new_error(pid, tid, process_name, operation, error_code)` 检查该组合是否曾经出现过。
2. 如果错误是**新的**，则调用 `format_msg()` 并将生成的字符串通过 `add_error()` 添加到 `apply_config_result.errors`。
3. 如果错误**已被记录过**，函数立即返回，不对 `format_msg` 求值，节省字符串格式化的开销。

使用 `impl FnOnce() -> String` 的延迟求值模式对性能至关重要，因为错误格式化涉及 `format!()` 调用以及 Win32 错误码查找，对于已知的重复错误来说是不必要的开销。

**错误消息约定：** 调用方通常按以下格式构造消息：

`"fn_name: [OPERATION_NAME][error_description] pid-tid-process_name"`

例如：

`"apply_priority: [SET_PRIORITY_CLASS][Access is denied (5)] 1234-game.exe"`

该函数标记为 `#[inline(always)]`，以消除调用开销，因为它在模块中每个错误路径上都会被调用。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/apply.rs |
| **源码行** | L67–L81 |
| **可见性** | 私有（`fn`） |
| **调用者** | [apply_priority](apply_priority.md)、[apply_affinity](apply_affinity.md)、[reset_thread_ideal_processors](reset_thread_ideal_processors.md)、[apply_process_default_cpuset](apply_process_default_cpuset.md)、[apply_io_priority](apply_io_priority.md)、[apply_memory_priority](apply_memory_priority.md)、[prefetch_all_thread_cycles](prefetch_all_thread_cycles.md)、[apply_prime_threads_promote](apply_prime_threads_promote.md)、[apply_prime_threads_demote](apply_prime_threads_demote.md)、[apply_ideal_processors](apply_ideal_processors.md) |
| **依赖项** | [is_new_error](../logging.rs/is_new_error.md)、[ApplyConfigResult](ApplyConfigResult.md) |