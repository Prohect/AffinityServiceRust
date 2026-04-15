# get_handles 函数 (apply.rs)

从 `ProcessHandle` 中提取读取和写入 `HANDLE` 值，优先使用完全访问句柄而非受限句柄。这是一个私有辅助函数，供进程级 apply 函数在调用查询或修改进程属性的 Windows API 之前获取适当的句柄对。

## 语法

```AffinityServiceRust/src/apply.rs#L63-67
fn get_handles(process_handle: &ProcessHandle) -> (Option<HANDLE>, Option<HANDLE>) {
    let r = process_handle.r_handle.or(Some(process_handle.r_limited_handle));
    let w = process_handle.w_handle.or(Some(process_handle.w_limited_handle));
    (r, w)
}
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `process_handle` | `&ProcessHandle` | 对 `ProcessHandle` 结构体的引用，该结构体为目标进程持有最多四个 OS 句柄：完全访问读取、受限读取、完全访问写入和受限写入。 |

## 返回值

返回一个元组 `(Option<HANDLE>, Option<HANDLE>)`，其中：

| 位置 | 含义 |
|------|------|
| `.0` | 读取句柄。如果 `process_handle.r_handle` 为 `Some` 则使用该值；否则回退到 `Some(process_handle.r_limited_handle)`。 |
| `.1` | 写入句柄。如果 `process_handle.w_handle` 为 `Some` 则使用该值；否则回退到 `Some(process_handle.w_limited_handle)`。 |

仅当首选的 `Option` 字段为 `None` **且**受限句柄也恰好为 `None` 等价值时，结果才会为 `(None, _)` 或 `(_, None)`，但在实际使用中不会出现此情况，因为 `ProcessHandle` 始终会填充受限字段。

## 备注

- 该函数标记为 `#[inline(always)]` 以消除调用开销，因为它在每个进程级 apply 函数的开头都会被调用。
- 调用方使用 `let … else` 模式解构返回值：
  ```AffinityServiceRust/src/apply.rs#L92-94
  let (Some(r_handle), Some(w_handle)) = get_handles(process_handle) else {
      return;
  };
  ```
  如果任一句柄为 `None`，调用的 apply 函数将提前返回，不会尝试任何 Windows API 调用。
- "完全访问"与"受限"的区别在于，服务可能根据目标进程的保护级别被授予不同的访问权限。完全访问句柄（例如 `PROCESS_SET_INFORMATION | PROCESS_QUERY_INFORMATION`）是首选，因为它们允许同时查询和设置属性。受限句柄可能仅支持部分操作。
- 此函数**不会**检查底层 `HANDLE` 值是否无效（例如 `INVALID_HANDLE_VALUE`）；该责任由调用方或 Windows API 自身承担，后者会返回错误代码。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `apply.rs` |
| 可见性 | 私有（`fn`，非 `pub fn`） |
| 调用方 | [`apply_priority`](apply_priority.md)、[`apply_affinity`](apply_affinity.md)、[`apply_process_default_cpuset`](apply_process_default_cpuset.md)、[`apply_io_priority`](apply_io_priority.md)、[`apply_memory_priority`](apply_memory_priority.md) |
| 被调用方 | 无（仅读取结构体字段） |
| API | 无 |
| 权限 | 无 |

## 另请参阅

| 参考 | 链接 |
|------|------|
| apply 模块概述 | [`README`](README.md) |
| ProcessHandle | [`winapi.rs/ProcessHandle`](../winapi.rs/ProcessHandle.md) |
| apply_priority | [`apply_priority`](apply_priority.md) |
| apply_affinity | [`apply_affinity`](apply_affinity.md) |
| apply_process_default_cpuset | [`apply_process_default_cpuset`](apply_process_default_cpuset.md) |
| apply_io_priority | [`apply_io_priority`](apply_io_priority.md) |
| apply_memory_priority | [`apply_memory_priority`](apply_memory_priority.md) |

---
*Commit: 7221ea0694670265d4eb4975582d8ed2ae02439d*