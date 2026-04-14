# log_error_if_new 函数 (apply.rs)

仅当相同的 pid / tid / 操作 / 错误码组合之前未被记录过时，才将错误消息记录到 [ApplyConfigResult](ApplyConfigResult.md) 累加器中。这可以防止重复的失败（例如进程在每个轮询周期都拒绝访问时的常见情况）用大量相同的条目淹没日志。

## 语法

```AffinityServiceRust/src/apply.rs#L69-81
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
| `pid` | `u32` | 错误所属的进程标识符。与 `tid`、`operation` 和 `error_code` 一起组成去重键。 |
| `tid` | `u32` | 线程标识符。对于不针对特定线程的进程级操作（例如 `SetPriorityClass`、`SetProcessAffinityMask`），传入 `0`。 |
| `process_name` | `&str` | 进程的显示名称，转发给 [is_new_error](../logging.rs/is_new_error.md) 用于去重映射，并包含在格式化的消息中。 |
| `operation` | [Operation](../logging.rs/Operation.md) | 标识失败的 Windows API 调用的枚举变体（例如 `Operation::SetPriorityClass`、`Operation::NtSetInformationProcess2ProcessInformationIOPriority`）。 |
| `error_code` | `u32` | 原始 Win32 错误码（`GetLastError().0`）或 NTSTATUS 值的无符号转换结果。 |
| `apply_config_result` | `&mut` [ApplyConfigResult](ApplyConfigResult.md) | 当错误为新错误时，通过 `add_error` 接收格式化错误字符串的累加器。 |
| `format_msg` | `impl FnOnce() -> String` | 延迟格式化闭包。仅在错误*确实是*新错误时才会被求值，避免了对被抑制的重复错误执行 `format!()` 的开销。 |

## 返回值

无（`()`）。

## 备注

该函数将去重逻辑委托给 [is_new_error](../logging.rs/is_new_error.md)，后者维护一个以 pid 为键的全局 `HashMap<u32, HashMap<ApplyFailEntry, bool>>`。如果 `is_new_error` 返回 `true`，则调用 `format_msg` 闭包，并将生成的 `String` 推入 `apply_config_result.errors`。如果返回 `false`，则既不调用闭包也不调用 `add_error`。

由于 `format_msg` 是 `FnOnce`，格式化工作（通常涉及调用 [error_from_code_win32](../error_codes.rs/error_from_code_win32.md) 或 [error_from_ntstatus](../error_codes.rs/error_from_ntstatus.md)）会被推迟到确认消息确实需要被记录时才执行。在进程持续拒绝访问的稳态循环中，这避免了每个周期数千次无用的内存分配。

该函数标记为 `#[inline(always)]`，因为它位于每个 `apply_*` 函数错误分支的热路径上，且仅包含一个条件调用。

[apply](README.md) 模块中的所有 `apply_*` 函数都通过 `log_error_if_new` 路由其错误处理，而不是直接调用 `add_error`。这使得去重策略统一且集中化。

### 错误消息约定

调用方按以下模式格式化消息：

`"fn_name: [OPERATION][error_description] pid-tid-process_name"`

例如：

`"apply_priority: [SET_PRIORITY_CLASS][Access is denied. (0x5)] 1234-chrome"`

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `apply` |
| 可见性 | crate 私有（`fn`） |
| 调用方 | [apply_priority](apply_priority.md)、[apply_affinity](apply_affinity.md)、[reset_thread_ideal_processors](reset_thread_ideal_processors.md)、[apply_process_default_cpuset](apply_process_default_cpuset.md)、[apply_io_priority](apply_io_priority.md)、[apply_memory_priority](apply_memory_priority.md)、[prefetch_all_thread_cycles](prefetch_all_thread_cycles.md)、[apply_prime_threads_promote](apply_prime_threads_promote.md)、[apply_prime_threads_demote](apply_prime_threads_demote.md)、[apply_ideal_processors](apply_ideal_processors.md) |
| 被调用方 | [is_new_error](../logging.rs/is_new_error.md)、[ApplyConfigResult::add_error](ApplyConfigResult.md) |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 去重映射与清除 | [is_new_error](../logging.rs/is_new_error.md)、[purge_fail_map](../logging.rs/purge_fail_map.md) |
| 操作枚举 | [Operation](../logging.rs/Operation.md) |
| 错误结果累加器 | [ApplyConfigResult](ApplyConfigResult.md) |
| Win32 错误格式化 | [error_from_code_win32](../error_codes.rs/error_from_code_win32.md) |
| NTSTATUS 错误格式化 | [error_from_ntstatus](../error_codes.rs/error_from_ntstatus.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd