# apply_ideal_processors 函数 (apply.rs)

`apply_ideal_processors` 函数根据配置中定义的模块前缀匹配规则，为线程分配理想处理器。对于每条规则，该函数识别其启动模块与指定前缀匹配的线程，按周期计数增量选出前 N 个线程（其中 N 等于规则中 CPU 的数量），并通过 `SetThreadIdealProcessorEx` 将每个选中线程分配到一个专用 CPU。当线程在后续应用周期中不再处于前 N 名时，其理想处理器将恢复为之前的值。该函数使用与主线程管道相同的基于滞后的选择机制，以防止快速振荡。

## 语法

```AffinityServiceRust/src/apply.rs#L1047-1058
pub fn apply_ideal_processors<'a>(
    pid: u32,
    config: &ThreadLevelConfig,
    dry_run: bool,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    prime_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `pid` | `u32` | 目标进程的进程 ID。用于调度器查找、错误去重和日志消息。 |
| `config` | `&ThreadLevelConfig` | 线程级配置，包含 `ideal_processor_rules`——一个规则列表，每条规则指定一组 CPU 索引（`cpus`）和可选的模块前缀（`prefixes`）。如果 `ideal_processor_rules` 为空，函数将立即返回。 |
| `dry_run` | `bool` | 当为 `true` 时，记录描述将执行哪些理想处理器分配的合成变更消息，而不调用任何 Windows API。当为 `false` 时，执行实际的分配和恢复操作。 |
| `threads` | `&impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>` | 一个惰性闭包，返回线程 ID 到其 `SYSTEM_THREAD_INFORMATION` 快照的映射引用，数据来自最近一次系统进程信息查询。闭包按需调用以枚举候选线程，将线程枚举的开销推迟到实际需要时。 |
| `prime_scheduler` | `&mut PrimeThreadScheduler` | 可变的主线程调度器状态，跨应用周期跟踪每个线程的统计信息，包括缓存的周期数、启动地址、线程句柄和理想处理器分配状态。 |
| `apply_config_result` | `&mut ApplyConfigResult` | 用于累积执行期间产生的变更描述和错误消息。 |

## 返回值

此函数不返回值。所有结果通过对 `prime_scheduler` 线程统计信息的修改和 `apply_config_result` 中追加的条目来传达。

## 备注

### 提前退出

如果 `config.ideal_processor_rules` 为空，函数将立即返回，不执行任何操作。

### 试运行模式

在试运行模式下，对每条规则会记录如下格式的变更消息：
`"Ideal Processor: CPUs [<cpu_list>] for top <N> threads from [<prefixes>]"`
其中 `<N>` 是规则中 CPU 的数量，`<prefixes>` 是模块前缀的连接列表，如果未指定前缀则为 `"all modules"`。

### 算法（非试运行）

1. **解析模块名称**：对于所有 `cached_cycles > 0` 的线程，函数通过 `resolve_address_to_module` 将每个线程的启动地址解析为模块名称。模块名称收集到共享的 `Vec<String>` 中并建立索引，以避免重复解析。每个线程表示为 `(tid, delta_cycles, start_address, name_index)` 元组。

2. **逐规则处理**：对于 `config.ideal_processor_rules` 中的每条规则：

   a. **按前缀过滤**：如果规则有前缀，则仅包含启动模块（小写化后）以其中一个前缀开头的线程。如果规则没有前缀，则所有线程都是候选者。

   b. **基于滞后的选择**：函数调用 `PrimeThreadScheduler::select_top_threads_with_hysteresis`，以 `rule.cpus.len()` 作为目标数量，以 `|ts| ts.ideal_processor.is_assigned` 作为"当前是否被选中"的谓词。已分配理想处理器的线程享有更宽松的保留阈值；新候选线程必须超过更严格的进入阈值并满足最低活跃连续周期要求。

   c. **认领现有分配**：对于被选为主线程且已有 `ideal_processor.is_assigned == true` 的线程，其当前 CPU 编号被添加到 `claimed` 集合中。对于新选中的线程，调用 `GetThreadIdealProcessorEx` 以捕获线程当前的理想处理器作为 `previous_group`/`previous_number` 基线。如果线程当前的理想处理器恰好已经是规则指定的 CPU 之一，则立即标记为已分配并认领。

   d. **从空闲池分配**：规则中不在 `claimed` 集合中的 CPU 构成空闲池。每个尚未被分配的新选中线程通过 `set_thread_ideal_processor_ex`（组 0，目标 CPU）从空闲池获得下一个可用 CPU。成功后，线程的 `ideal_processor.current_group` 和 `current_number` 被更新，`is_assigned` 设为 `true`，并记录变更消息：
      `"Thread <tid> -> ideal CPU <cpu> (group 0) start=<module>"`

   e. **恢复未选中线程**：具有 `ideal_processor.is_assigned == true` 但不再在选中集合中的线程，通过 `set_thread_ideal_processor_ex` 恢复到其 `previous_group`/`previous_number`。成功后，线程的 `current_group`/`current_number` 更新为之前的值，`is_assigned` 清除为 `false`。记录变更消息：
      `"Thread <tid> -> restored ideal CPU <prev_number> (group <prev_group>) start=<module>"`

### 边界情况

- 如果规则的 `cpus` 列表为空，该规则将被完全跳过。
- 如果规则没有前缀，所有具有缓存周期数的线程都是该规则的合格候选者。
- 如果在认领阶段 `GetThreadIdealProcessorEx` 失败，错误通过 [`log_error_if_new`](log_error_if_new.md) 以 `Operation::GetThreadIdealProcessorEx` 记录，且该线程不会被分配。
- 如果在分配或恢复期间 `set_thread_ideal_processor_ex` 失败，错误通过 [`log_error_if_new`](log_error_if_new.md) 以 `Operation::SetThreadIdealProcessorEx` 记录。分配失败时线程的 `is_assigned` 状态保持不变，但恢复失败时会清除该状态以避免无限重试。
- `cached_cycles == 0` 的线程（即周期数未成功预取的线程）将被排除在所有规则处理之外。
- 线程句柄验证遵循与其他函数相同的模式：优先使用 `w_handle`，回退到 `w_limited_handle`。如果两者都无效，记录错误并跳过该线程。
- 恢复理想处理器时，仅在先前值与当前值不同时（`prev_group != cur_group || prev_number != cur_number`）才执行恢复，避免对已在目标 CPU 上的线程进行不必要的 API 调用。

### 每线程理想处理器状态

`ThreadStats` 中的 `ideal_processor` 字段跟踪三部分信息：
- **`previous_group` / `previous_number`**：线程首次被选中时的理想处理器。这是降级时将恢复的值。
- **`current_group` / `current_number`**：线程当前分配的理想处理器。每次成功调用 `set_thread_ideal_processor_ex` 后更新。
- **`is_assigned`**：一个布尔值，指示此函数是否对该线程有活跃的理想处理器分配。

## 要求

| 要求 | 值 |
|-------------|-------|
| 模块 | `apply.rs` |
| Crate | `AffinityServiceRust` |
| 可见性 | `pub` |
| Windows API | `SetThreadIdealProcessorEx`（通过 `winapi::set_thread_ideal_processor_ex`）、`GetThreadIdealProcessorEx`（通过 `winapi::get_thread_ideal_processor_ex`）、`GetLastError` |
| 调用方 | `scheduler.rs` / `main.rs` 中遍历匹配进程的编排代码 |
| 被调用方 | [`log_error_if_new`](log_error_if_new.md)、`winapi::resolve_address_to_module`、`winapi::get_thread_ideal_processor_ex`、`winapi::set_thread_ideal_processor_ex`、`config::format_cpu_indices`、`error_codes::error_from_code_win32`、`PrimeThreadScheduler::get_thread_stats`、`PrimeThreadScheduler::select_top_threads_with_hysteresis` |
| 权限 | 需要具有 `THREAD_SET_INFORMATION` 或 `THREAD_SET_LIMITED_INFORMATION`（写入）以及 `THREAD_QUERY_INFORMATION` 或 `THREAD_QUERY_LIMITED_INFORMATION`（读取，用于 `GetThreadIdealProcessorEx`）权限的线程句柄。 |

## 另请参阅

| 参考 | 链接 |
|-----------|------|
| apply 模块概述 | [`README`](README.md) |
| ApplyConfigResult | [`ApplyConfigResult`](ApplyConfigResult.md) |
| reset_thread_ideal_processors | [`reset_thread_ideal_processors`](reset_thread_ideal_processors.md) |
| apply_prime_threads | [`apply_prime_threads`](apply_prime_threads.md) |
| apply_prime_threads_select | [`apply_prime_threads_select`](apply_prime_threads_select.md) |
| prefetch_all_thread_cycles | [`prefetch_all_thread_cycles`](prefetch_all_thread_cycles.md) |
| update_thread_stats | [`update_thread_stats`](update_thread_stats.md) |
| log_error_if_new | [`log_error_if_new`](log_error_if_new.md) |
| ThreadLevelConfig | [`config.rs/ThreadLevelConfig`](../config.rs/ThreadLevelConfig.md) |
| PrimeThreadScheduler | [`scheduler.rs/PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md) |

---
*Commit: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)*