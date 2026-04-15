# reset_thread_ideal_processors 函数 (apply.rs)

在亲和性掩码或 CPU 集合更改后，将线程的理想处理器重新分配到指定的一组 CPU 上。该函数按累计 CPU 时间（内核 + 用户）降序排列所有线程，并使用带随机起始偏移的轮转分配将每个线程分配到目标 CPU 列表中的一个理想处理器。这可以防止 Windows 在进程级别的亲和性或 CPU 集合更改后将所有线程集中到同一个 CPU 上。

## 语法

```AffinityServiceRust/src/apply.rs#L219-231
pub fn reset_thread_ideal_processors(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    cpus: &[u32],
    threads: &HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 目标进程的进程 ID。用于日志消息中的错误去重。 |
| `config` | `&ProcessLevelConfig` | 进程级配置。`name` 字段用于错误日志消息，并传递给 `get_thread_handle` 用于句柄获取。 |
| `dry_run` | `bool` | 为 `true` 时，记录一条描述将要重新分配多少个线程的合成变更消息，而不打开任何线程句柄或调用 `SetThreadIdealProcessorEx`。为 `false` 时，执行实际的重新分配。 |
| `cpus` | `&[u32]` | 用于分配线程理想处理器的 CPU 索引集合。亲和性掩码更改后，调用方传入 `&config.affinity_cpus`；CPU 集合更改时（当 `cpu_set_reset_ideal` 启用时），传入 `&config.cpu_set_cpus`。如果为空，函数立即返回。 |
| `threads` | `&HashMap<u32, SYSTEM_THREAD_INFORMATION>` | 线程 ID 到其 `SYSTEM_THREAD_INFORMATION` 快照的映射，来自最近的系统进程信息查询。`KernelTime` 和 `UserTime` 字段之和用于确定每个线程的总 CPU 时间以进行排序。 |
| `apply_config_result` | `&mut ApplyConfigResult` | 执行期间产生的变更描述和错误消息的累加器。 |

## 返回值

此函数不返回值。所有结果通过 `apply_config_result` 参数传达。

## 备注

### 算法

1. **提前退出**：如果 `cpus` 为空，函数立即返回。如果 `dry_run` 为 `true`，记录一条变更消息后函数返回，不执行任何 API 调用。
2. **收集 CPU 时间**：对于 `threads` 映射中的每个线程，计算总 CPU 时间为 `KernelTime + UserTime`（均以 100 纳秒为单位）。结果收集到一个固定容量列表（`List<[(u32, i64); TIDS_FULL]>`）中。
3. **排序**：线程列表使用 `sort_unstable_by_key` 配合 `Reverse` 按总 CPU 时间降序排序。这确保 CPU 活动最高的线程最先被分配理想处理器。
4. **随机偏移**：通过 `rand::random::<u8>()` 生成一个随机 `u8` 值，用作 CPU 数组的起始偏移。这避免了总是将第一个线程分配到列表中的第一个 CPU，在不同应用周期间提供一定程度的负载均衡。
5. **轮转分配**：每个线程通过遍历 `cpus` 数组分配理想处理器：`target_cpu = cpus[(success_count + random_shift) % cpus.len()]`。函数使用组 `0` 和计算的 CPU 索引调用 `set_thread_ideal_processor_ex`。
6. **句柄解析**：对于每个线程，通过 `get_thread_handle` 获取句柄。函数优先使用 `w_limited_handle`，其次使用 `w_handle`（使用任何非无效的句柄）。无法获取句柄的线程被静默跳过。
7. **结果记录**：完成后，追加一条格式为 `"reset ideal processor for N threads"` 的变更消息，其中 N 是成功重新分配的线程数。

### 边界情况

- 如果所有 `set_thread_ideal_processor_ex` 调用都失败，成功计数器保持为零，变更消息报告 `"reset ideal processor for 0 threads"`。
- 在快照和句柄打开尝试之间已退出的线程被静默跳过；`get_thread_handle` 返回 `None`，函数继续处理下一个线程。
- `random_shift` 是 `u8` 转换为 `usize`，因此值以 256 为模环绕，这没有问题因为模数始终是 `cpus.len()`。

### 调用方

此函数从两个位置调用：
- [`apply_affinity`](apply_affinity.md) — 在成功执行 `SetProcessAffinityMask` 后立即调用，传入 `&config.affinity_cpus`。
- [`apply_process_default_cpuset`](apply_process_default_cpuset.md) — 在 `config.cpu_set_reset_ideal` 为 `true` 时，在 `SetProcessDefaultCpuSets` 之前立即调用，传入 `&config.cpu_set_cpus`。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `apply.rs` |
| Crate | `AffinityServiceRust` |
| 可见性 | `pub` |
| Windows API | `SetThreadIdealProcessorEx`（通过 `winapi::set_thread_ideal_processor_ex`）、`GetLastError` |
| 调用者 | [`apply_affinity`](apply_affinity.md)、[`apply_process_default_cpuset`](apply_process_default_cpuset.md) |
| 被调用者 | [`log_error_if_new`](log_error_if_new.md)、`winapi::get_thread_handle`、`winapi::set_thread_ideal_processor_ex`、`error_codes::error_from_code_win32`、`rand::random` |
| 权限 | 需要具有 `THREAD_SET_INFORMATION` 或 `THREAD_SET_LIMITED_INFORMATION` 访问权限的线程句柄。 |

## 另请参阅

| 参考 | 链接 |
|------|------|
| apply 模块概述 | [`README`](README.md) |
| ApplyConfigResult | [`ApplyConfigResult`](ApplyConfigResult.md) |
| apply_affinity | [`apply_affinity`](apply_affinity.md) |
| apply_process_default_cpuset | [`apply_process_default_cpuset`](apply_process_default_cpuset.md) |
| apply_ideal_processors | [`apply_ideal_processors`](apply_ideal_processors.md) |
| log_error_if_new | [`log_error_if_new`](log_error_if_new.md) |
| ProcessLevelConfig | [`config.rs/ProcessLevelConfig`](../config.rs/ProcessLevelConfig.md) |

---
*Commit: 7221ea0694670265d4eb4975582d8ed2ae02439d*
