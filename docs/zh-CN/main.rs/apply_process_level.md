# apply_process_level 函数 (main.rs)

将一次性进程级设置应用到由 PID 标识的单个 Windows 进程上。此函数获取进程句柄，然后委托给针对每个设置类别的专用 apply 辅助函数：优先级类别、处理器亲和性（含线程理想处理器重置）、默认 CPU 集合、I/O 优先级和内存优先级。除非通过 CLI 标志启用了持续进程级应用，否则此函数对每个进程仅调用一次。

## 语法

```rust
fn apply_process_level(
    pid: u32,
    config: &ProcessLevelConfig,
    threads: &HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    dry_run: bool,
    apply_configs: &mut ApplyConfigResult,
)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 目标进程的 Windows 进程标识符。 |
| `config` | `&ProcessLevelConfig` | 进程级配置块，描述该进程所需的优先级类别、亲和性掩码、CPU Set ID、I/O 优先级和内存优先级。 |
| `threads` | `&HashMap<u32, SYSTEM_THREAD_INFORMATION>` | 线程 ID 到其 `SYSTEM_THREAD_INFORMATION` 快照的映射。供亲和性和 CPU 集合辅助函数在进程级亲和性变更后重置每线程理想处理器时使用。 |
| `dry_run` | `bool` | 为 `true` 时，函数记录其*将要*执行的操作但不调用任何 Win32 API 来修改进程状态。 |
| `apply_configs` | `&mut ApplyConfigResult` | 应用过程中产生的变更和错误的累加器。由各个子函数填充，后续由 [`log_apply_results`](log_apply_results.md) 消费。 |

## 返回值

此函数不返回值。如果无法获取进程句柄（例如权限不足或进程已退出），函数将提前返回且不应用任何设置。所有结果——成功和失败——都记录在 `apply_configs` 累加器中。

## 备注

- 函数首先调用 `get_process_handle`。如果返回 `None`（访问被拒绝、进程已退出等），整个函数不执行任何操作。
- 一个局部变量 `current_mask` 被初始化为 `0` 并传递给 `apply_affinity`，后者在请求亲和性变更时会将其填充为当前亲和性掩码。此掩码在内部由亲和性辅助函数使用，以确定是否需要进行理想处理器重置。
- 应用顺序是确定性的：优先级 → 亲和性 → CPU 集合 → I/O 优先级 → 内存优先级。此顺序确保在亲和性变更的任何线程级副作用发生之前，进程优先级类别已被设置。
- 每个子函数（`apply_priority`、`apply_affinity`、`apply_process_default_cpuset`、`apply_io_priority`、`apply_memory_priority`）会独立检查其对应的配置字段是否设置为 `None` 哨兵值，如果未请求变更则自行跳过。
- 默认情况下，此函数**不会**在每次轮询迭代中调用。一旦某个 PID 出现在 `process_level_applied` 中，除非激活了 `-continuousProcessLevelApply` CLI 标志，否则会被跳过。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `main.rs` |
| 调用者 | [`apply_config`](apply_config.md) |
| 被调用者 | `winapi::get_process_handle`、`apply::apply_priority`、`apply::apply_affinity`、`apply::apply_process_default_cpuset`、`apply::apply_io_priority`、`apply::apply_memory_priority` |
| Win32 API | 间接调用——委托给 `apply` 模块函数，它们调用 `SetPriorityClass`、`SetProcessAffinityMask`、`SetProcessDefaultCpuSets`、`NtSetInformationProcess` |
| 权限 | `SeDebugPrivilege`（用于打开提升/系统进程的句柄） |

## 另请参阅

| 参考 | 链接 |
|------|------|
| apply_thread_level | [apply_thread_level](apply_thread_level.md) |
| apply_config | [apply_config](apply_config.md) |
| log_apply_results | [log_apply_results](log_apply_results.md) |
| ProcessLevelConfig | [config 模块](../config.rs/README.md) |
| apply 模块 | [apply 模块](../apply.rs/README.md) |

---
> Commit SHA: `7221ea0694670265d4eb4975582d8ed2ae02439d`
