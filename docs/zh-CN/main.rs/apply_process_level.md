# apply_process_level 函数 (main.rs)

为给定的 PID 打开进程句柄，并在单次传递中应用所有进程级设置——优先级类、CPU 亲和性掩码、默认 CPU 集合、IO 优先级和内存优先级。这是进程级包装器，在进程首次匹配配置规则时调用一次（如果启用了持续应用则每次迭代都调用）。

## 语法

```rust
fn apply_process_level<'a>(
    pid: u32,
    config: &ProcessLevelConfig,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    dry_run: bool,
    apply_configs: &mut ApplyConfigResult,
)
```

## 参数

`pid: u32`

目标进程的进程标识符。

`config: &ProcessLevelConfig`

[`ProcessLevelConfig`](../config.rs/ProcessLevelConfig.md)，包含所需的进程级设置（优先级、亲和性 CPU、CPU 集合 CPU、IO 优先级、内存优先级）。`config.name` 字段在打开进程句柄时用于错误报告。

`threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>`

延迟求值的闭包，返回进程的线程映射（以线程 ID 为键）。通过调用方（[`apply_config`](apply_config.md)）中的 `OnceCell` 与线程级路径共享。线程映射被 [`apply_affinity`](../apply.rs/apply_affinity.md)（在亲和性更改后重置理想处理器）和 [`apply_process_default_cpuset`](../apply.rs/apply_process_default_cpuset.md) 所需要。

`dry_run: bool`

当为 **true** 时，所有下游 `apply_*` 函数记录*将会*更改的内容而不调用任何 Windows API。当为 **false** 时，更改将应用到活动进程。

`apply_configs: &mut ApplyConfigResult`

更改描述和错误消息的累加器。参见 [`ApplyConfigResult`](../apply.rs/ApplyConfigResult.md)。

## 返回值

此函数不返回值。所有结果（已应用的更改、遇到的错误）都记录在 `apply_configs` 中。

## 备注

该函数按固定顺序执行操作：

1. **打开句柄** — 调用 [`get_process_handle`](../winapi.rs/get_process_handle.md) 获取 [`ProcessHandle`](../winapi.rs/ProcessHandle.md)。如果无法打开句柄（例如访问被拒绝、进程已退出），函数立即返回，不产生任何效果且不记录错误。
2. **应用优先级** — 委托给 [`apply_priority`](../apply.rs/apply_priority.md)。
3. **应用亲和性** — 委托给 [`apply_affinity`](../apply.rs/apply_affinity.md)。传递局部变量 `current_mask` 以捕获进程当前的亲和性掩码供下游使用。
4. **应用 CPU 集合** — 委托给 [`apply_process_default_cpuset`](../apply.rs/apply_process_default_cpuset.md)。
5. **应用 IO 优先级** — 委托给 [`apply_io_priority`](../apply.rs/apply_io_priority.md)。
6. **应用内存优先级** — 委托给 [`apply_memory_priority`](../apply.rs/apply_memory_priority.md)。
7. **释放句柄** — 在所有操作完成后显式释放 `ProcessHandle`。

每个 `apply_*` 函数独立检查其对应的配置字段是否设置为 `None`，如果是则短路返回。这意味着仅指定优先级和亲和性的配置不会影响 IO 或内存优先级。

### 线程枚举开销

`threads` 闭包仅在 `apply_*` 函数实际需要线程信息时才被调用（例如 `apply_affinity` 重置理想处理器，或 `apply_process_default_cpuset` 重新分配线程）。底层的 `OnceCell` 确保在进程级和线程级应用过程中线程枚举最多只发生一次。

## 要求

| | |
|---|---|
| **模块** | [`src/main.rs`](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/main.rs) |
| **调用方** | [`apply_config`](apply_config.md) |
| **被调用方** | [`get_process_handle`](../winapi.rs/get_process_handle.md)、[`apply_priority`](../apply.rs/apply_priority.md)、[`apply_affinity`](../apply.rs/apply_affinity.md)、[`apply_process_default_cpuset`](../apply.rs/apply_process_default_cpuset.md)、[`apply_io_priority`](../apply.rs/apply_io_priority.md)、[`apply_memory_priority`](../apply.rs/apply_memory_priority.md) |
| **Win32 API** | 无（直接委托给被调用方） |
| **特权** | `SeDebugPrivilege`（用于打开提升权限进程的句柄） |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 线程级对应函数 | [`apply_thread_level`](apply_thread_level.md) |
| 组合调用方 | [`apply_config`](apply_config.md) |
| 应用引擎概述 | [apply.rs](../apply.rs/README.md) |
| 配置结构体 | [`ProcessLevelConfig`](../config.rs/ProcessLevelConfig.md) |
| 结果累加器 | [`ApplyConfigResult`](../apply.rs/ApplyConfigResult.md) |

*为提交 [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf) 记录*