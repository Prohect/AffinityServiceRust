# get_handles 函数 (apply.rs)

从 `ProcessHandle` 中提取读写句柄，优先使用完整访问句柄而非受限访问句柄。

## 语法

```rust
fn get_handles(process_handle: &ProcessHandle) -> (Option<HANDLE>, Option<HANDLE>)
```

## 参数

`process_handle`

对 [`ProcessHandle`](../winapi.rs/ProcessHandle.md) 的引用，其中包含最多四个句柄（读/写 × 完整/受限访问）。

## 返回值

返回 `(read_handle, write_handle)` 元组，每个值均包装在 `Option<HANDLE>` 中。

- **读句柄**：优先使用 `r_handle`（完整访问），若不可用则回退到 `r_limited_handle`（受限访问）。
- **写句柄**：优先使用 `w_handle`（完整访问），若不可用则回退到 `w_limited_handle`（受限访问）。

由于有效的 `ProcessHandle` 中受限句柄始终存在，因此两个返回值实际上总是 `Some(...)`。

## 备注

这是一个内联辅助函数，几乎在每个 `apply_*` 函数的开头使用。调用方通过 `let-else` 守卫对返回值进行解构：

```rust
let (Some(r_handle), Some(w_handle)) = get_handles(process_handle) else {
    return;
};
```

完整访问句柄（`r_handle`、`w_handle`）被优先使用，因为它们携带更广泛的访问权限（`PROCESS_QUERY_INFORMATION`、`PROCESS_SET_INFORMATION`）。受限句柄（`r_limited_handle`、`w_limited_handle`）仅携带 `PROCESS_QUERY_LIMITED_INFORMATION` 和 `PROCESS_SET_LIMITED_INFORMATION` 权限，对于某些操作足够，但并非全部。

该函数标记为 `#[inline(always)]` 以消除调用开销。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | `src/apply.rs`（L58–L65） |
| **可见性** | 私有（`fn`，非 `pub fn`） |
| **调用者** | [apply_priority](apply_priority.md)、[apply_affinity](apply_affinity.md)、[apply_process_default_cpuset](apply_process_default_cpuset.md)、[apply_io_priority](apply_io_priority.md)、[apply_memory_priority](apply_memory_priority.md) |
| **依赖** | [`ProcessHandle`](../winapi.rs/ProcessHandle.md)、`HANDLE`（Windows crate） |