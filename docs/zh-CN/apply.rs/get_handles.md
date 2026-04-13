# get_handles 函数 (apply.rs)

从 [ProcessHandle](../winapi.rs/ProcessHandle.md) 中提取最佳可用的读取和写入 `HANDLE`，优先使用完全访问句柄而非受限访问句柄。这是一个小型内联辅助函数，用于每个需要与 Windows 进程 API 交互的 `apply_*` 函数的开头。

## 语法

```AffinityServiceRust/src/apply.rs#L61-65
#[inline(always)]
fn get_handles(process_handle: &ProcessHandle) -> (Option<HANDLE>, Option<HANDLE>) {
    let r = process_handle.r_handle.or(Some(process_handle.r_limited_handle));
    let w = process_handle.w_handle.or(Some(process_handle.w_limited_handle));
    (r, w)
}
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `process_handle` | `&ProcessHandle` | 对为目标进程打开的 [ProcessHandle](../winapi.rs/ProcessHandle.md) 的引用。最多包含四个句柄：受限读取、完全读取、受限写入和完全写入。 |

## 返回值

返回 `(Option<HANDLE>, Option<HANDLE>)` —— 一个 (读取句柄, 写入句柄) 的元组。

- **第一个**元素是读取句柄，如果 `r_handle` 为 `Some` 则使用它，否则使用 `r_limited_handle`。
- **第二个**元素是写入句柄，如果 `w_handle` 为 `Some` 则使用它，否则使用 `w_limited_handle`。

由于 [ProcessHandle](../winapi.rs/ProcessHandle.md) 存在时 `r_limited_handle` 和 `w_limited_handle` 总是会被填充，返回的 `Option` 实际上始终为 `Some`。尽管如此，调用方仍然使用 `let (Some(r), Some(w)) = get_handles(...) else { return; }` 进行模式匹配作为防御性保护。

## 备注

[ProcessHandle](../winapi.rs/ProcessHandle.md) 为读取和写入分别存储两级访问权限：

| 字段 | 访问权限 | 可用性 |
|------|---------|--------|
| `r_limited_handle` | `PROCESS_QUERY_LIMITED_INFORMATION` | 始终存在 |
| `r_handle` | `PROCESS_QUERY_INFORMATION` | 仅当服务具有足够权限时存在 |
| `w_limited_handle` | `PROCESS_SET_LIMITED_INFORMATION` | 始终存在 |
| `w_handle` | `PROCESS_SET_INFORMATION` | 仅当服务具有足够权限时存在 |

完全访问句柄（`r_handle`、`w_handle`）为 `Option<HANDLE>`。当服务以 `SeDebugPrivilege` 和提升的权限运行时，通常对所有进程都可用。当权限受限时（例如 `--no-debug-priv`），只会打开受限句柄。`get_handles` 抽象了这一层级选择逻辑，使每个 `apply_*` 函数无需重复回退逻辑。

该函数标记为 `#[inline(always)]`，因为它不进行任何分配，编译后仅为两个条件移动指令。它在每个应用周期中为每个匹配的进程调用，因此消除调用开销是值得的。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `apply` |
| 可见性 | `fn`（crate 私有） |
| 调用方 | [apply_priority](apply_priority.md)、[apply_affinity](apply_affinity.md)、[apply_process_default_cpuset](apply_process_default_cpuset.md)、[apply_io_priority](apply_io_priority.md)、[apply_memory_priority](apply_memory_priority.md) |
| 被调用方 | 无 |
| API | 无 —— 纯 Rust 逻辑，操作已有句柄 |
| 权限 | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 句柄包装结构体 | [ProcessHandle](../winapi.rs/ProcessHandle.md) |
| 句柄获取 | [get_process_handle](../winapi.rs/get_process_handle.md) |
| apply 模块概述 | [apply](README.md) |