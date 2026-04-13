# reset_thread_ideal_processors 函数 (apply.rs)

在亲和性掩码或 CPU 集合变更后，将线程的理想处理器重新分配到指定的 CPU 集合上。当 Windows 更改进程亲和性时，可能会将线程的理想处理器重置为不再合理的值。此函数通过按 CPU 时间降序排列线程，并以随机偏移量进行轮询分配的方式，将理想处理器重新分配到目标 CPU 上，从而避免将 CPU 占用最高的线程始终集中在同一个核心上。

## 语法

```AffinityServiceRust/src/apply.rs#L219-313
pub fn reset_thread_ideal_processors(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    cpus: &[u32],
    process: &mut ProcessEntry,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 目标进程的进程标识符。用于错误日志记录和线程句柄获取。 |
| `config` | `&ProcessConfig` | 该进程的已解析 [ProcessConfig](../config.rs/ProcessConfig.md)。`name` 字段用于错误消息。 |
| `dry_run` | `bool` | 当为 `true` 时，在 `apply_config_result` 中记录*将要*执行的变更，但不调用任何 Windows API。 |
| `cpus` | `&[u32]` | 用于分配线程理想处理器的 CPU 索引集合。在亲和性变更后，调用方传入 `&config.affinity_cpus`；在 CPU 集合变更后（当 `cpu_set_reset_ideal` 启用时），调用方传入 `&config.cpu_set_cpus`。 |
| `process` | `&mut ProcessEntry` | 目标进程的 [ProcessEntry](../process.rs/ProcessEntry.md)。提供对线程列表及最近进程快照中每个线程的内核/用户时间数据的访问。 |
| `apply_config_result` | `&mut` [ApplyConfigResult](ApplyConfigResult.md) | 变更描述和错误消息的累加器。 |

## 返回值

无 (`()`)。

## 备注

### 算法

1. **提前退出** — 如果 `cpus` 为空，函数立即返回。在模拟运行模式下，记录一条摘要变更后函数返回。

2. **收集并排序线程** — 从 `process.get_threads()` 获取所有线程，收集到一个 `Vec<(tid, total_cpu_time)>` 中，其中 `total_cpu_time = KernelTime + UserTime`（以 100 纳秒为单位）。按 CPU 时间降序排序，确保最繁忙的线程最先被分配。

3. **打开线程句柄** — 按排序顺序，为每个线程 ID 通过 [get_thread_handle](../winapi.rs/get_thread_handle.md) 获取一个 [ThreadHandle](../winapi.rs/ThreadHandle.md)。为每个线程选择写句柄（`w_handle`，回退到 `w_limited_handle`）。无法打开句柄的线程将被跳过。

4. **带随机偏移的轮询分配** — 通过 `rand::random::<u8>()` 生成一个随机 `u8` 偏移量。对于排序索引为 `i` 的线程，目标 CPU 为 `cpus[(i + random_shift) % cpus.len()]`。随机偏移确保在连续调用中分配不是确定性的，避免 CPU 时间最高的线程总是落在 CPU 0 上的病态集中问题。

5. **设置理想处理器** — 使用处理器组 `0` 和目标 CPU 编号调用 [set_thread_ideal_processor_ex](../winapi.rs/set_thread_ideal_processor_ex.md)。失败通过 [log_error_if_new](log_error_if_new.md) 记录；成功则递增计数器。

6. **报告** — 记录一条变更条目 `"reset ideal processor for N threads"`，其中包含成功分配的数量。

### 调用时机

- **亲和性变更后** — [apply_affinity](apply_affinity.md) 在成功调用 `SetProcessAffinityMask` 后，立即使用 `&config.affinity_cpus` 调用此函数。这是为了补偿 Windows 在亲和性掩码变更时重置理想处理器的行为。

- **CPU 集合变更后** — [apply_process_default_cpuset](apply_process_default_cpuset.md) 在 `config.cpu_set_reset_ideal` 为 `true` 时，在应用 `SetProcessDefaultCpuSets` 之前使用 `&config.cpu_set_cpus` 调用此函数。CPU 集合是一种软偏好，不会强制重置理想处理器，因此这一可选行为让用户自行决定是否重新分配。

### 线程句柄生命周期

线程句柄被打开并存储在一个局部的 `Vec<(u32, Option<ThreadHandle>)>` 中。当该向量在函数退出时超出作用域，[ThreadHandle](../winapi.rs/ThreadHandle.md) 的 `Drop` 实现会自动关闭所有 OS 句柄。

### 处理器组限制

此函数始终向 `set_thread_ideal_processor_ex` 传递处理器组 `0`。这对于拥有最多 64 个逻辑处理器（单个处理器组）的系统是正确的。跨多个处理器组（超过 64 个逻辑处理器）的系统目前不受此函数支持。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `apply` |
| 可见性 | `pub`（crate 内公开） |
| 调用方 | [apply_affinity](apply_affinity.md)、[apply_process_default_cpuset](apply_process_default_cpuset.md) |
| 被调用方 | [get_thread_handle](../winapi.rs/get_thread_handle.md)、[set_thread_ideal_processor_ex](../winapi.rs/set_thread_ideal_processor_ex.md)、[log_error_if_new](log_error_if_new.md) |
| API | [SetThreadIdealProcessorEx](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadidealprocessorex)（Win32） |
| 权限 | 需要 `SeDebugPrivilege` 以进行跨进程线程句柄访问（在服务启动时通过 [enable_debug_privilege](../winapi.rs/enable_debug_privilege.md) 获取） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 硬 CPU 亲和性掩码应用 | [apply_affinity](apply_affinity.md) |
| 软 CPU 集合应用 | [apply_process_default_cpuset](apply_process_default_cpuset.md) |
| 线程句柄获取 | [get_thread_handle](../winapi.rs/get_thread_handle.md) |
| 理想处理器 Win32 封装 | [set_thread_ideal_processor_ex](../winapi.rs/set_thread_ideal_processor_ex.md) |
| 基于规则的理想处理器分配 | [apply_ideal_processors](apply_ideal_processors.md) |
| 进程线程枚举 | [ProcessEntry](../process.rs/ProcessEntry.md) |