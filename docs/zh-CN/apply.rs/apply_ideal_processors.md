# apply_ideal_processors 函数 (apply.rs)

根据可配置的规则为线程分配理想处理器，这些规则将线程启动模块前缀匹配到专用的 CPU 集合。对于每条规则，该函数识别启动模块与规则前缀之一匹配的线程，使用与主力线程选择相同的滞后算法选出 CPU 周期增量最高的前 *N* 个线程（其中 *N* 等于规则中的 CPU 数量），并通过 `SetThreadIdealProcessorEx` 将每个选中的线程固定到专用 CPU。当线程从前 *N* 名中掉出时，其理想处理器将恢复为分配前的值。

## 语法

```AffinityServiceRust/src/apply.rs#L1061-1072
pub fn apply_ideal_processors(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process: &mut ProcessEntry,
    prime_scheduler: &mut PrimeThreadScheduler,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `pid` | `u32` | 目标进程的进程标识符。用作 [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) 状态映射的键以及日志记录。 |
| `config` | `&`[ProcessConfig](../config.rs/ProcessConfig.md) | 此进程的已解析配置。`ideal_processor_rules` 字段（`Vec<`[IdealProcessorRule](../config.rs/IdealProcessorRule.md)`>`）包含零条或多条规则，每条规则指定一组 CPU 索引和一组模块名前缀。 |
| `dry_run` | `bool` | 为 `true` 时，函数在 `apply_config_result` 中记录每条规则*将会*执行的操作摘要，但不调用任何 Windows API 也不修改调度器状态。 |
| `process` | `&mut`[ProcessEntry](../process.rs/ProcessEntry.md) | 目标进程的快照条目。提供用于枚举候选线程的线程列表（线程 ID）。 |
| `prime_scheduler` | `&mut`[PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) | 持久化的调度器状态，保存每个线程的 [ThreadStats](../scheduler.rs/ThreadStats.md)，包括缓存的周期计数、线程句柄、起始地址和 [IdealProcessorState](../scheduler.rs/IdealProcessorState.md)。该函数读取 `cached_cycles`、`last_cycles`、`start_address` 和 `ideal_processor` 字段，并写入 `ideal_processor.current_group`、`ideal_processor.current_number`、`ideal_processor.previous_group`、`ideal_processor.previous_number` 和 `ideal_processor.is_assigned`。 |
| `apply_config_result` | `&mut`[ApplyConfigResult](ApplyConfigResult.md) | 此操作期间产生的变更描述和错误消息的累加器。 |

## 返回值

无（`()`）。结果通过 `apply_config_result` 和对 `prime_scheduler` 的副作用来传递。

## 备注

### 算法

该函数独立处理每条 [IdealProcessorRule](../config.rs/IdealProcessorRule.md)。对于每条规则：

**步骤 1 — 收集线程信息。**
从调度器中收集所有 `cached_cycles` 非零的线程，构建 `Vec<(tid, delta_cycles, start_address, start_module)>`。启动模块通过 [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md) 从线程的起始地址解析。此集合仅计算一次，并在所有规则间共享。

**步骤 2 — 按前缀过滤。**
对线程列表进行过滤，仅保留启动模块（转为小写）以规则的某个前缀（同样转为小写）开头的线程。如果规则的 `prefixes` 列表为空，则*所有*线程都匹配——这允许一条"兜底"规则在不进行模块过滤的情况下跨整个进程分配理想处理器。

**步骤 3 — 通过滞后算法选择前 N 个。**
从过滤后的线程构建 `Vec<(tid, delta_cycles, is_selected)>`，并传递给 `prime_scheduler.select_top_threads_with_hysteresis()`。选择使用与 [apply_prime_threads_select](apply_prime_threads_select.md) 相同的[滞后算法](../scheduler.rs/PrimeThreadScheduler.md#hysteresis-algorithm)，但"当前已分配"谓词检查的是 `thread_stats.ideal_processor.is_assigned` 而非 `pinned_cpu_set_ids`。*N* 等于 `rule.cpus.len()`——此规则中可用于理想处理器分配的 CPU 数量。

**步骤 4 — 认领已持有的 CPU。**
对于每个已有 `is_assigned == true` 的选中线程，将其当前持有的 CPU 加入"已认领"集合。对于新选中的线程（尚未分配），函数通过 [get_thread_ideal_processor_ex](../winapi.rs/get_thread_ideal_processor_ex.md) 读取线程当前的理想处理器，并保存到 `ideal_processor.previous_group` 和 `ideal_processor.previous_number`。如果线程当前的理想处理器恰好已在 `rule.cpus` 中，则就地认领，无需重新分配。

**步骤 5 — 从空闲池中分配。**
`rule.cpus` 中不在已认领集合中的 CPU 构成"空闲池"。尚未分配的新选中线程按顺序从此池中分配 CPU。调用 [set_thread_ideal_processor_ex](../winapi.rs/set_thread_ideal_processor_ex.md)，传入组 `0` 和目标 CPU 编号。成功后，更新 `ideal_processor.current_group`、`ideal_processor.current_number` 和 `ideal_processor.is_assigned`。记录变更消息：

`"Thread 1234 -> ideal CPU 5 (group 0) start=game.dll!WorkerThread"`

如果空闲池在所有选中线程分配完之前耗尽，则跳过剩余线程。

**步骤 6 — 恢复未选中的线程。**
对于每个之前已分配（`is_assigned == true`）但不再在选中集合中的线程，如果 `(previous_group, previous_number)` 与 `(current_group, current_number)` 不同，则将理想处理器恢复为 `(previous_group, previous_number)`。调用 [set_thread_ideal_processor_ex](../winapi.rs/set_thread_ideal_processor_ex.md) 传入之前的值，并将 `is_assigned` 设为 `false`。记录变更消息：

`"Thread 1234 -> restored ideal CPU 3 (group 0) start=game.dll!WorkerThread"`

### 模拟运行行为

在模拟运行模式下，函数为每条规则记录一条变更消息，概述意图：

`"Ideal Processor: CPUs [4,5,6] for top 3 threads from [game.dll; render.dll]"`

当规则的前缀列表为空时，消息显示为 `"from [all modules]"`。

不会打开线程句柄，不会执行 Win32 调用，也不会修改调度器状态。

### 多规则

每条规则独立运作。如果单个线程的启动模块匹配了多条规则中的前缀，则该线程可能被多条规则匹配。但是，`is_assigned` 标志在规则间共享，因此一旦线程被较早的规则分配，后续规则会将其视为已分配，可能会认领其 CPU 或跳过它。因此，当规则具有重叠的前缀集合时，配置中规则的顺序很重要。

### 处理器组限制

与 [reset_thread_ideal_processors](reset_thread_ideal_processors.md) 类似，此函数始终在处理器组 `0` 内操作。具有超过 64 个逻辑处理器且跨多个处理器组的系统未完全支持；仅组 0 的 CPU 可分配。

### 与主力线程调度的关系

`apply_ideal_processors` 和 [apply_prime_threads](apply_prime_threads.md) 针对不同的使用场景：

| 方面 | 主力线程 | 理想处理器 |
|--------|--------------|------------------|
| 机制 | 每线程 CPU 集合（`SetThreadSelectedCpuSets`） | 理想处理器提示（`SetThreadIdealProcessorEx`） |
| 强度 | 硬约束——线程*不能*在其他地方运行 | 软提示——调度器*优先*使用指定的 CPU |
| 优先级提升 | 是（可配置或自动 +1 级） | 否 |
| 模块过滤 | 通过 `prime_threads_prefixes` | 通过 `ideal_processor_rules[].prefixes` |

两个功能共享 [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) 状态和相同的滞后选择算法，但它们写入 [ThreadStats](../scheduler.rs/ThreadStats.md) 中的不同字段（`pinned_cpu_set_ids` 与 `ideal_processor`）。

### 错误处理

句柄解析和 Win32 API 调用失败通过 [log_error_if_new](log_error_if_new.md) 记录。记录的操作包括：

| 操作 | 触发时机 |
|-----------|------|
| `Operation::OpenThread` | 线程句柄无效（`w_handle` 和 `w_limited_handle` 均无效）。 |
| `Operation::GetThreadIdealProcessorEx` | 为新选中的线程读取当前理想处理器失败。 |
| `Operation::SetThreadIdealProcessorEx` | 设置或恢复理想处理器失败。 |

## 要求

| 要求 | 值 |
|-------------|-------|
| 模块 | `apply` |
| 可见性 | `pub`（crate 公开） |
| 调用者 | [apply_config_thread_level](../main.rs/apply_config_thread_level.md) |
| 被调用者 | [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md)、[set_thread_ideal_processor_ex](../winapi.rs/set_thread_ideal_processor_ex.md)、[get_thread_ideal_processor_ex](../winapi.rs/get_thread_ideal_processor_ex.md)、[format_cpu_indices](../config.rs/format_cpu_indices.md)、[log_error_if_new](log_error_if_new.md)、[PrimeThreadScheduler::select_top_threads_with_hysteresis](../scheduler.rs/PrimeThreadScheduler.md)、[PrimeThreadScheduler::get_thread_stats](../scheduler.rs/PrimeThreadScheduler.md) |
| Win32 API | [`SetThreadIdealProcessorEx`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadidealprocessorex)、[`GetThreadIdealProcessorEx`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadidealprocessorex) |
| 权限 | `THREAD_SET_INFORMATION`（写入）、`THREAD_QUERY_INFORMATION`（读取理想处理器）。`SeDebugPrivilege` 提供跨进程线程访问权限。 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| apply 模块概览 | [apply](README.md) |
| 理想处理器规则配置 | [IdealProcessorRule](../config.rs/IdealProcessorRule.md) |
| 亲和性变更后的理想处理器重分配 | [reset_thread_ideal_processors](reset_thread_ideal_processors.md) |
| 主力线程调度（硬 CPU 集合固定） | [apply_prime_threads](apply_prime_threads.md) |
| 滞后选择算法 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| 每线程理想处理器状态 | [IdealProcessorState](../scheduler.rs/IdealProcessorState.md) |
| 线程启动模块解析 | [resolve_address_to_module](../winapi.rs/resolve_address_to_module.md) |
| Win32 理想处理器封装 | [set_thread_ideal_processor_ex](../winapi.rs/set_thread_ideal_processor_ex.md)、[get_thread_ideal_processor_ex](../winapi.rs/get_thread_ideal_processor_ex.md) |
| 线程级应用编排 | [apply_config_thread_level](../main.rs/apply_config_thread_level.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd