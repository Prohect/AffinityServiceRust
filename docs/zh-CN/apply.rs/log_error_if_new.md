# log_error_if_new 函数 (apply.rs)

仅当针对给定的进程/线程/操作组合尚未记录过同一错误时，才记录错误消息。这可以防止当同一操作在每次轮询周期都失败时，日志中出现重复的错误消息。

## 语法

```AffinityServiceRust/src/apply.rs#L71-80
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

`pid: u32`

与错误关联的进程 ID。

`tid: u32`

与错误关联的线程 ID。对于非特定线程的进程级操作，请传递 `0`。

`process_name: &str`

匹配的配置规则中的进程显示名称。用作去重键的一部分并包含在日志输出中。

`operation: Operation`

标识哪个 Windows API 调用失败的 [Operation](../logging.rs/Operation.md) 枚举变体。构成去重的唯一性键的一部分。

`error_code: u32`

失败的 API 调用返回的 Win32 错误代码或 NTSTATUS 值。构成唯一性键的一部分 — 同一操作以不同的错误代码失败被视为新的、不同的错误。

`apply_config_result: &mut ApplyConfigResult`

[ApplyConfigResult](ApplyConfigResult.md) 累加器，通过 `add_error()` 将格式化的错误消息追加到其中，但仅当错误是新的时才追加。

`format_msg: impl FnOnce() -> String`

惰性求值的闭包，用于生成格式化的错误消息字符串。仅当确定错误是新的时才调用闭包，从而避免为被抑制的重复项分配字符串格式化的成本。

## 返回值

此函数不返回值。

## 备注

此函数是 [`logging::is_new_error`](../logging.rs/is_new_error.md) 的简单包装。它将去重委托给 `is_new_error(pid, tid, process_name, operation, error_code)`，后者维护每个 PID 的 `ApplyFailEntry` 记录集。如果之前没有具有相同 `(tid, process_name, operation, error_code)` 元组的条目，则认为错误是"新的"。

此函数标记为 `#[inline(always)]` 以消除调用开销，因为它在 `apply` 模块的每个错误位置都会被调用。

惰性 `format_msg` 闭包模式对于性能很重要。在稳定状态下，大多数错误都是先前失败重复项（例如，对受保护进程的访问被拒绝）。通过在去重检查之后延迟字符串格式化，热路径完全避免了分配。

### 错误消息格式约定

调用方遵循一致的闭包输出格式：

```/dev/null/error_format.txt#L1
fn_name: [OPERATION_NAME][error_description] pid-tid-process_name
```

例如：
```/dev/null/error_example.txt#L1
apply_affinity: [SET_PROCESS_AFFINITY_MASK][Access is denied. (0x5)] 1234-my_process
```

### 典型用法

模块中的每个 `apply_*` 函数在 Windows API 调用失败后都会调用 `log_error_if_new`，传递相关的 `Operation` 变体和 Win32/NTSTATUS 错误代码。这集中了去重 - 然后记录的逻辑，并使各个 apply 函数专注于其核心逻辑。

## 要求

| | |
|---|---|
| **模块** | `apply` |
| **调用方** | [apply_priority](apply_priority.md)、[apply_affinity](apply_affinity.md)、[reset_thread_ideal_processors](reset_thread_ideal_processors.md)、[apply_process_default_cpuset](apply_process_default_cpuset.md)、[apply_io_priority](apply_io_priority.md)、[apply_memory_priority](apply_memory_priority.md)、[prefetch_all_thread_cycles](prefetch_all_thread_cycles.md)、[apply_prime_threads_promote](apply_prime_threads_promote.md)、[apply_prime_threads_demote](apply_prime_threads_demote.md)、[apply_ideal_processors](apply_ideal_processors.md) |
| **被调用方** | [`logging::is_new_error`](../logging.rs/is_new_error.md)、[ApplyConfigResult::add_error](ApplyConfigResult.md) |
| **可见性** | `fn`（crate 私有） |

## 参见

| | |
|---|---|
| [ApplyConfigResult](ApplyConfigResult.md) | 变化和错误的累加器 |
| [Operation](../logging.rs/Operation.md) | Windows API 操作标识符枚举 |
| [is_new_error](../logging.rs/is_new_error.md) | 底层去重逻辑 |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*