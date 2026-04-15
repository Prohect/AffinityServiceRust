# reset_thread_ideal_processors 函数 (apply.rs)

在亲和性掩码或 CPU 集更改后，将线程理想处理器重新分配到指定的 CPU 集合上。该函数按累计 CPU 时间（内核 + 用户）降序排列所有线程，并使用带随机起始偏移的轮询分配方式，从目标 CPU 列表中为每个线程分配一个理想处理器。这可以防止 Windows 在进程级亲和性或 CPU 集更改后将所有线程集中到同一个 CPU 上。

## 语法

```AffinityServiceRust/src/apply.rs#L219-231
pub fn reset_thread_ideal_processors<'a>(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    cpus: &[u32],
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 目标进程的进程 ID。用于日志消息中的错误去重。 |
| `config` | `&ProcessLevelConfig` | 进程级配置。`name` 字段用于错误日志消息，并传递给 `get_thread_handle` 以获取句柄。 |
| `dry_run` | `bool` | 当为 `true` 时，记录一条描述将重新分配多少线程的合成变更消息，而不打开任何线程句柄或调用 `SetThreadIdealProcessorEx`。当为 `false` 时，执行实际的重新分配。 |
| `cpus` | `&[u32]` | 用于分配线程理想处理器的 CPU 索引集合。调用方在亲和性掩码更改后传递 `&config.affinity_cpus`，或在 CPU 集更改时（当 `cpu_set_reset_ideal` 启用时）传递 `&config.cpu_set_cpus`。如果为空，函数立即返回。 |
| `threads` | `&impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>` | 一个延迟闭包，返回线程 ID 到其 `SYSTEM_THREAD_INFORMATION` 快照（来自最近的系统进程信息查询）的映射引用。该闭包仅在函数通过提前退出检查后才会被调用。`KernelTime` 和 `UserTime` 字段之和用于确定每个线程的总 CPU 时间以进行排序。 |
| `apply_config_result` | `&mut ApplyConfigResult` | 执行过程中产生的变更描述和错误消息的累加器。 |

## 返回值

此函数不返回值。所有结果通过 `apply_config_result` 参数传达。

## 备注

### 算法

1. **提前退出**：如果 `cpus` 为空，函数立即返回。如果 `dry_run` 为 `true`，则记录一条变更消息后返回，不执行任何 API 调用。
2. **收集 CPU 时间**：调用 `threads` 闭包获取线程映射。对于返回映射中的每个线程，计算总 CPU 时间为 `KernelTime + UserTime`（均以 100 纳秒为单位）。结果收集到一个固定容量列表（`List<[(u32, i64); TIDS_FULL]>`）中。
3. **排序**：使用 `sort_unstable_by_key` 配合 `Reverse` 按总 CPU 时间降序排列线程列表。这确保 CPU 活跃度最高的线程最先被分配理想处理器。
4. **随机偏移**：通过 `rand::random::<u8>()` 生成一个随机 `u8` 值作为 CPU 数组的起始偏移。这避免了总是将第一个线程分配到列表中的第一个 CPU，在多次应用周期间提供一定程度的负载均衡。
5. **轮询分配**：通过循环遍历 `cpus` 数组为每个线程分配一个理想处理器：`target_cpu = cpus[(success_count + random_shift) % cpus.len()]`。函数使用 group `0` 和计算出的 CPU 索引调用 `set_thread_ideal_processor_ex`。
6. **句柄解析**：对于每个线程，通过 `get_thread_handle` 获取句柄。函数优先使用 `w_handle`，仅在 `w_handle` 无效时回退到 `w_limited_handle`。无法获取句柄的线程将被静默跳过。
7. **结果记录**：完成后，附加一条格式为 `"reset ideal processor for N threads"` 的变更消息，其中 N 是成功重新分配的线程数。

### 边界情况

- 如果所有 `set_thread_ideal_processor_ex` 调用都失败，成功计数器保持为零，变更消息将报告 `"reset ideal processor for 0 threads"`。
- 在快照和句柄打开尝试之间已退出的线程会被静默跳过；`get_thread_handle` 返回 `None`，函数继续处理下一个线程。
- `random_shift` 是从 `u8` 转换为 `usize` 的值，因此值在模 256 处回绕，这没有问题，因为取模运算始终使用 `cpus.len()`。

### 调用方

此函数从两处调用：
- [`apply_affinity`](apply_affinity.md) — 在成功调用 `SetProcessAffinityMask` 后立即调用，传递 `&config.affinity_cpus`。
- [`apply_process_default_cpuset`](apply_process_default_cpuset.md) — 当 `config.cpu_set_reset_ideal` 为 `true` 时，在 `SetProcessDefaultCpuSets` 之前立即调用，传递 `&config.cpu_set_cpus`。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `apply.rs` |
| Crate | `AffinityServiceRust` |
| 可见性 | `pub` |
| Windows API | `SetThreadIdealProcessorEx`（通过 `winapi::set_thread_ideal_processor_ex`）、`GetLastError` |
| 调用方 | [`apply_affinity`](apply_affinity.md)、[`apply_process_default_cpuset`](apply_process_default_cpuset.md) |
| 被调用方 | [`log_error_if_new`](log_error_if_new.md)、`winapi::get_thread_handle`、`winapi::set_thread_ideal_processor_ex`、`error_codes::error_from_code_win32`、`rand::random` |
| 权限 | 需要具有 `THREAD_SET_INFORMATION` 或 `THREAD_SET_LIMITED_INFORMATION` 访问权限的线程句柄。 |

## 另请参阅

| 参考 | 链接 |
|------|------|
| apply 模块概览 | [`README`](README.md) |
| ApplyConfigResult | [`ApplyConfigResult`](ApplyConfigResult.md) |
| apply_affinity | [`apply_affinity`](apply_affinity.md) |
| apply_process_default_cpuset | [`apply_process_default_cpuset`](apply_process_default_cpuset.md) |
| apply_ideal_processors | [`apply_ideal_processors`](apply_ideal_processors.md) |
| log_error_if_new | [`log_error_if_new`](log_error_if_new.md) |
| ProcessLevelConfig | [`config.rs/ProcessLevelConfig`](../config.rs/ProcessLevelConfig.md) |

---
*Commit: b0df9da35213b050501fab02c3020ad4dbd6c4e0*