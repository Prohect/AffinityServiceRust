# apply_ideal_processors 函数 (apply.rs)

根据可配置的 [`IdealProcessorRule`](../config.rs/IdealProcessorRule.md) 条目为线程分配理想处理器提示。每条规则指定一组 CPU 和可选的模块名前缀；该函数通过起始地址的模块名称匹配线程，使用迟滞机制选择前 *N* 个线程（其中 *N* = 规则中的 CPU 数量），并通过轮转方式将每个选定的线程分配到一个专用的理想 CPU。未进入前 *N* 名的线程会将其理想处理器恢复到分配前观察到的值。

## 语法

```AffinityServiceRust/src/apply.rs#L1048-1057
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
| `pid` | `u32` | 目标进程的进程 ID。 |
| `config` | `&ThreadLevelConfig` | 包含 `ideal_processor_rules` 的线程级别配置。 |
| `dry_run` | `bool` | 当为 `true` 时，记录将会进行的更改，但不调用任何 Windows API。 |
| `threads` | `&impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>` | 用于获取进程线程映射的延迟访问器。 |
| `prime_scheduler` | `&mut PrimeThreadScheduler` | 拥有线程统计信息（周期缓存、理想处理器跟踪状态）的调度器。 |
| `apply_config_result` | `&mut ApplyConfigResult` | 用于累积更改和错误信息的容器。 |

## 返回值

此函数不返回任何值。所有结果都记录在 `apply_config_result` 中。

## 说明

### 算法

对于 `config.ideal_processor_rules` 中的每条 [`IdealProcessorRule`](../config.rs/IdealProcessorRule.md)：

1. **模块匹配** — 起始地址模块与规则的前缀列表之一匹配（不区分大小写）的每个线程都被视为候选者。如果 `prefixes` 为空，则所有线程都是候选者。

2. **迟滞选择** — 候选者被送入 [`PrimeThreadScheduler::select_top_threads_with_hysteresis`](../scheduler.rs/PrimeThreadScheduler.md)，槽数量等于 `rule.cpus.len()`。`is_currently_assigned` 谓词检查 `thread_stats.ideal_processor.is_assigned`，这稳定了轮询间隔之间的线程选择。

3. **保留现有分配** — 从前一次迭代中已经分配了理想 CPU 的线程，如果仍被选中，则保留其 CPU 槽位。其 CPU 会被添加到 `claimed` 集合中以避免重复分配。

4. **新分配** — 新选中的尚未分配的线程通过轮转方式从空闲池（不在 `claimed` 中的规则 CPU）获得 CPU。通过处理器组 0 和目标 CPU 编号调用 API `SetThreadIdealProcessorEx`。

5. **恢复** — 之前被分配但不再被选中的线程，其理想处理器会恢复到第一次分配之前捕获的值（`previous_group`，`previous_number`）。`is_assigned` 标志被清除。

### 理想处理器状态跟踪

每个线程的理想处理器状态在 `ThreadStats.ideal_processor` 中跟踪：

| 字段 | 用途 |
|-------|---------|
| `is_assigned` | 此函数是否当前拥有该线程的理想处理器。 |
| `previous_group` / `previous_number` | 第一次分配之前的理想处理器，用于恢复。 |
| `current_group` / `current_number` | 此函数最近设置的理想处理器。 |

在首次选中时，调用 `GetThreadIdealProcessorEx` 来捕获基线。如果线程的当前理想处理器已经在 `rule.cpus` 范围内，则无需冗余 `Set` 调用即可保留。

### 试运行行为

当 `dry_run` 为 `true` 时，函数记录每条规则的一个摘要更改，指示 CPU 集合和前缀过滤器，然后返回而不打开任何线程句柄或调用 Windows API。

### 错误处理

`GetThreadIdealProcessorEx` 和 `SetThreadIdealProcessorEx` 的错误通过 [`log_error_if_new`](log_error_if_new.md) 报告，后者通过 `(pid, tid, operation, error_code)` 进行去重。无效的线程句柄会被记录一次并跳过该线程。

### 平台说明

- 理想处理器是向 Windows 调度器发出的*提示*，而非硬性约束。操作系统仍可能在其他 CPU 上调度该线程。
- 仅支持处理器组 0 进行分配。如果线程的基线理想处理器在组 0 且位于 `rule.cpus` 内，则无需 `Set` 调用即可占用。

## 要求

| | |
|---|---|
| **模块** | `apply` (`src/apply.rs`) |
| **调用方** | 服务轮询循环（通过顶层 apply 编排器） |
| **被调用方** | [`PrimeThreadScheduler::select_top_threads_with_hysteresis`](../scheduler.rs/PrimeThreadScheduler.md), `resolve_address_to_module` (`winapi`), `get_thread_ideal_processor_ex` / `set_thread_ideal_processor_ex` (`winapi`), [`log_error_if_new`](log_error_if_new.md) |
| **Win32 API** | `SetThreadIdealProcessorEx`, `GetThreadIdealProcessorEx`（通过 `winapi` 封装） |
| **权限** | `THREAD_SET_INFORMATION`（写入句柄）, `THREAD_QUERY_INFORMATION` 或 `THREAD_QUERY_LIMITED_INFORMATION`（读取句柄） |

## 参见

| 主题 | 链接 |
|-------|------|
| 结果累加器 | [ApplyConfigResult](ApplyConfigResult.md) |
| 线程级别配置 | [ThreadLevelConfig](../config.rs/ThreadLevelConfig.md) |
| 理想处理器规则定义 | [IdealProcessorRule](../config.rs/IdealProcessorRule.md) |
| Prime 线程调度器 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| 周期预取（前置条件） | [prefetch_all_thread_cycles](prefetch_all_thread_cycles.md) |
| 线程统计快照 | [update_thread_stats](update_thread_stats.md) |
| 模块概览 | [apply.rs](README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*