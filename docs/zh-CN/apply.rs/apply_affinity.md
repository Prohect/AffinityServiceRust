# apply_affinity 函数 (apply.rs)

通过 `SetProcessAffinityMask` 设置进程的硬 CPU 亲和性掩码。硬亲和性掩码将进程限制为仅在指定的逻辑处理器上运行。成功更改亲和性后，该函数会自动调用 [reset_thread_ideal_processors](reset_thread_ideal_processors.md) 在新的 CPU 集合上重新分配线程的理想处理器。

## 语法

```AffinityServiceRust/src/apply.rs#L132-141
pub fn apply_affinity(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    current_mask: &mut usize,
    process_handle: &ProcessHandle,
    process: &mut ProcessEntry,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 目标进程的进程标识符。用于错误日志记录，并传递给 [reset_thread_ideal_processors](reset_thread_ideal_processors.md)。 |
| `config` | `&`[ProcessConfig](../config.rs/ProcessConfig.md) | 该进程的已解析配置。`affinity_cpus` 字段包含要纳入掩码的 CPU 索引列表。`prime_threads_cpus` 字段也会被检查——如果任一字段非空，则会查询当前亲和性掩码。 |
| `dry_run` | `bool` | 为 `true` 时，更改会记录到 `apply_config_result` 中，但不会调用 `SetProcessAffinityMask`，OS 状态不会被修改。 |
| `current_mask` | `&mut usize` | 输入/输出参数，接收进程的当前亲和性掩码（通过 `GetProcessAffinityMask` 查询）。设置成功后，会更新为新的掩码值。该值会被下游的 [apply_prime_threads_promote](apply_prime_threads_promote.md) 使用，以根据有效亲和性过滤主力线程的 CPU 索引。 |
| `process_handle` | `&`[ProcessHandle](../winapi.rs/ProcessHandle.md) | 目标进程的 OS 句柄包装器。通过 [get_handles](get_handles.md) 提取读取句柄（用于 `GetProcessAffinityMask`）和写入句柄（用于 `SetProcessAffinityMask`）。 |
| `process` | `&mut`[ProcessEntry](../process.rs/ProcessEntry.md) | 进程的快照条目，提供线程枚举。当亲和性掩码发生变化时，传递给 [reset_thread_ideal_processors](reset_thread_ideal_processors.md)。 |
| `apply_config_result` | `&mut`[ApplyConfigResult](ApplyConfigResult.md) | 此操作期间产生的更改描述和错误的累加器。 |

## 返回值

无 (`()`)。结果通过 `apply_config_result` 和 `current_mask` 进行传达。

## 备注

### 算法

1. **提前退出。** 如果 `config.affinity_cpus` 和 `config.prime_threads_cpus` 都为空，函数立即返回——没有需要查询或设置的内容。

2. **查询当前掩码。** 调用 `GetProcessAffinityMask` 读取当前进程亲和性和系统亲和性掩码。当前掩码会写入 `*current_mask`，以便后续函数（特别是 [apply_prime_threads_promote](apply_prime_threads_promote.md)）可以根据进程的有效亲和性过滤其 CPU 索引。
   - 如果查询失败且 `dry_run` 为 `false`，则通过 [log_error_if_new](log_error_if_new.md) 以 `Operation::GetProcessAffinityMask` 记录错误，函数返回而不尝试设置。

3. **比较并设置。** 期望的掩码由 [cpu_indices_to_mask](../config.rs/cpu_indices_to_mask.md) 从 `config.affinity_cpus` 计算得出。以下情况会跳过设置：
   - `config.affinity_cpus` 为空（查询仅用于获取 `current_mask`）。
   - 计算出的掩码为 `0`（没有有效的 CPU 解析出来）。
   - 计算出的掩码已经等于 `*current_mask`（亲和性已经正确）。

4. **应用。** 在 `dry_run` 模式下，更改消息会被记录但不调用 API。否则，调用 `SetProcessAffinityMask`。成功后，`*current_mask` 更新为新掩码，并调用 [reset_thread_ideal_processors](reset_thread_ideal_processors.md) 使用 `config.affinity_cpus` 重新分配线程的理想处理器。

### 副作用

- **`current_mask` 在 `GetProcessAffinityMask` 成功后总是会被写入**，即使没有执行设置操作。这是有意为之——[apply_prime_threads](apply_prime_threads.md) 和 [apply_prime_threads_promote](apply_prime_threads_promote.md) 依赖于查询到的值。
- **亲和性更改成功后会重置线程理想处理器。** Windows 在亲和性掩码更改时可能会内部清除或重新分配理想处理器，因此 [reset_thread_ideal_processors](reset_thread_ideal_processors.md) 会使用随机偏移量确定性地重新分配它们，以避免 CPU 热点。

### 更改消息格式

```/dev/null/example.txt#L1
Affinity: 0xFF -> 0xF0
```

消息以十六进制显示之前的掩码和新掩码。

### 边界情况

- 如果进程已退出或句柄无效，[get_handles](get_handles.md) 返回 `None`，函数静默返回。
- 掩码为 `0`（例如，来自空的或无效的 CPU 规格，解析后没有任何位）被视为"未请求更改"，永远不会传递给 `SetProcessAffinityMask`。
- 当仅配置了 `prime_threads_cpus`（没有 `affinity_cpus`）时，函数会查询当前掩码但不会设置新掩码。这会为下游的主力线程管道填充 `current_mask`。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `apply` |
| 可见性 | `pub`（crate 内公开） |
| 调用方 | [apply_config_process_level](../main.rs/apply_config_process_level.md) |
| 被调用方 | [get_handles](get_handles.md)、[cpu_indices_to_mask](../config.rs/cpu_indices_to_mask.md)、[log_error_if_new](log_error_if_new.md)、[reset_thread_ideal_processors](reset_thread_ideal_processors.md) |
| Win32 API | [`GetProcessAffinityMask`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getprocessaffinitymask)、[`SetProcessAffinityMask`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-setprocessaffinitymask) |
| 权限 | `PROCESS_QUERY_LIMITED_INFORMATION`（读取）、`PROCESS_SET_INFORMATION`（写入）。服务通常持有 `SeDebugPrivilege`，可授予这两种权限。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| apply 模块概述 | [apply](README.md) |
| 软 CPU 集合（硬亲和性的替代方案） | [apply_process_default_cpuset](apply_process_default_cpuset.md) |
| 亲和性更改后的理想处理器重分配 | [reset_thread_ideal_processors](reset_thread_ideal_processors.md) |
| 主力线程 CPU 固定（使用 `current_mask`） | [apply_prime_threads_promote](apply_prime_threads_promote.md) |
| CPU 索引 ↔ 掩码转换 | [cpu_indices_to_mask](../config.rs/cpu_indices_to_mask.md)、[format_cpu_indices](../config.rs/format_cpu_indices.md) |
| 进程句柄获取 | [get_process_handle](../winapi.rs/get_process_handle.md) |