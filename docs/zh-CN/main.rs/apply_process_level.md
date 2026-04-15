# apply_process_level 函数 (main.rs)

对由 PID 标识的单个 Windows 进程应用一次性的进程级设置。此函数获取进程句柄，然后将各设置类别分别委托给专用的 apply 辅助函数处理：优先级类别、处理器亲和性（含线程理想处理器重置）、默认 CPU 集合、I/O 优先级和内存优先级。除非通过 CLI 标志启用了持续进程级应用，否则每个进程仅调用一次。

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

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 目标进程的 Windows 进程标识符。 |
| `config` | `&ProcessLevelConfig` | 进程级配置块，描述此进程所需的优先级类别、亲和性掩码、CPU 集合 ID、I/O 优先级和内存优先级。 |
| `threads` | `&impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>` | 一个延迟闭包，返回线程 ID 到其 `SYSTEM_THREAD_INFORMATION` 快照的映射引用。该闭包在调用方（`apply_config`）中由 `OnceCell` 支持，因此线程映射仅在首次访问时实际生成，之后复用。被 `apply_affinity` 和 `apply_process_default_cpuset` 用于在进程级变更后重置每线程的理想处理器。 |
| `dry_run` | `bool` | 当为 `true` 时，函数仅记录其*将要执行*的操作，但不会调用任何 Win32 API 来修改进程状态。 |
| `apply_configs` | `&mut ApplyConfigResult` | 应用过程中产生的变更和错误的累加器。由各子函数填充，之后由 [`log_apply_results`](log_apply_results.md) 消费。 |

## 返回值

此函数不返回值。如果无法获取进程句柄（例如权限不足或进程已退出），函数将提前返回且不应用任何设置。所有结果——成功和失败——均记录在 `apply_configs` 累加器中。

## 备注

- 函数首先调用 `get_process_handle`。如果返回 `None`（访问被拒绝、进程已退出等），则整个函数不执行任何操作。
- 一个局部变量 `current_mask` 初始化为 `0` 并传递给 `apply_affinity`，后者在请求亲和性变更时将其填充为当前亲和性掩码。此掩码在亲和性辅助函数内部用于确定是否需要重置理想处理器。
- `threads` 参数是延迟闭包而非直接引用。这避免了在没有子函数实际需要线程映射时（例如仅配置了优先级或 I/O/内存优先级变更时）枚举线程的开销。调用方中由 `OnceCell` 支持的闭包确保即使多个子函数对其解引用，线程快照也最多只获取一次。
- 应用顺序是确定性的：优先级 → 亲和性 → CPU 集合 → I/O 优先级 → 内存优先级。此顺序确保在亲和性变更的线程级副作用发生之前，进程优先级类别已被设置。
- 每个子函数（`apply_priority`、`apply_affinity`、`apply_process_default_cpuset`、`apply_io_priority`、`apply_memory_priority`）独立检查其对应的配置字段是否设置为 `None` 哨兵值，在不需要变更时跳过自身。
- 默认情况下，此函数**不会**在每次轮询迭代中被调用。一旦某个 PID 出现在 `process_level_applied` 中，除非激活了 `-continuousProcessLevelApply` CLI 标志，否则将被跳过。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `main.rs` |
| 调用方 | [`apply_config`](apply_config.md) |
| 被调用方 | `winapi::get_process_handle`、`apply::apply_priority`、`apply::apply_affinity`、`apply::apply_process_default_cpuset`、`apply::apply_io_priority`、`apply::apply_memory_priority` |
| Win32 API | 间接调用——委托给 `apply` 模块中调用 `SetPriorityClass`、`SetProcessAffinityMask`、`SetProcessDefaultCpuSets`、`NtSetInformationProcess` 的函数 |
| 权限 | `SeDebugPrivilege`（用于打开提升权限/系统进程的句柄） |

## 另请参阅

| 参考 | 链接 |
|------|------|
| apply_thread_level | [apply_thread_level](apply_thread_level.md) |
| apply_config | [apply_config](apply_config.md) |
| log_apply_results | [log_apply_results](log_apply_results.md) |
| ProcessLevelConfig | [config 模块](../config.rs/README.md) |
| apply 模块 | [apply 模块](../apply.rs/README.md) |

---
> Commit SHA: `b0df9da35213b050501fab02c3020ad4dbd6c4e0`
