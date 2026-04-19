# apply_affinity 函数 (apply.rs)

设置进程的 CPU 亲和性掩码，限制其仅在指定的逻辑处理器上运行。

## 语法

```AffinityServiceRust/src/apply.rs#L134-142
pub fn apply_affinity<'a>(
    pid: u32,
    config: &ProcessLevelConfig,
    dry_run: bool,
    current_mask: &mut usize,
    process_handle: &ProcessHandle,
    threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

`pid: u32`

目标进程的进程标识符。

`config: &ProcessLevelConfig`

进程级配置，包含 `affinity_cpus`，即进程应被限制到的 CPU 索引列表。如果 `affinity_cpus` 为空，函数将立即返回而不执行任何操作。

`dry_run: bool`

如果为 `true`，则将预期更改记录到 `apply_config_result` 中，而不调用任何 Windows API。

`current_mask: &mut usize`

**\[输出\]** 在成功查询后接收进程的当前亲和性掩码。成功设置后更新为新掩码。此值被下游函数（如 [apply_prime_threads_promote](apply_prime_threads_promote.md)）使用，后者用它将主 CPU 索引过滤为仅包含进程亲和性范围内的索引。

`process_handle: &ProcessHandle`

目标进程的句柄包装器。需要读和写句柄；如果任一不可用，函数将提前返回。

`threads: &impl Fn() -> &'a HashMap<u32, SYSTEM_THREAD_INFORMATION>`

目标进程线程映射的惰性访问器。当亲和性成功更改时，传递给 [reset_thread_ideal_processors](reset_thread_ideal_processors.md)。

`apply_config_result: &mut ApplyConfigResult`

操作中产生的更改消息和错误的累积器。

## 返回值

此函数不返回值。结果通过 `current_mask`（副作用）和 `apply_config_result` 通信。

## 备注

函数通过 `cpu_indices_to_mask` 将配置的 CPU 索引转换为位掩码。然后使用 **GetProcessAffinityMask** 查询当前进程亲和性掩码，并将其与目标进行比较。如果不同，则调用 **SetProcessAffinityMask** 应用新掩码。

### 副作用

- **填充 `current_mask`**：即使配置的亲和性已匹配，当前掩码也通过 `GetProcessAffinityMask` 输出参数写入到 `*current_mask`。此内容被后续的线程逻辑消费。
- **重置线程理想处理器**：在亲和性成功更改后，函数立即调用 [reset_thread_ideal_processors](reset_thread_ideal_processors.md)，传入 `config.affinity_cpus`。这会将线程理想处理器重新分配到新的 CPU 集合上，以防止过时的分配将线程聚集在不再属于亲和性掩码的 CPU 上。
- **更新 `current_mask` 为新值**：成功设置后，`*current_mask` 被新亲和性掩码覆盖。

### 错误处理

- 如果 **GetProcessAffinityMask** 失败，错误通过 [log_error_if_new](log_error_if_new.md) 针对每个唯一的 (pid, operation, error_code) 记录一次，函数在不尝试设置的情况下退出。
- 如果 **SetProcessAffinityMask** 失败，错误同样被记录，并且掩码不会更新。
- 两个错误路径在 `dry_run` 模式下被抑制（获取路径的错误完全被跳过）。

### 亲和性掩码格式

亲和性掩码是一个 `usize` 位掩码，其中位 *N* 代表逻辑处理器 *N*。例如，CPUs `[0, 2, 4]` 产生掩码 `0x15`。转换后的零掩码导致函数跳过设置操作，因为 Windows 拒绝零亲和性掩码。

### 更改消息格式

```/dev/null/example.txt#L1
Affinity: 0xFF -> 0x15
```

## 需求

| | |
|---|---|
| **模块** | `src/apply.rs` |
| **调用方** | 主轮询循环（通过进程级配置应用） |
| **被调用方** | [get_handles](get_handles.md), [log_error_if_new](log_error_if_new.md), [reset_thread_ideal_processors](reset_thread_ideal_processors.md), `cpu_indices_to_mask` |
| **Win32 API** | [GetProcessAffinityMask](https://learn.microsoft.com/zh-cn/windows/win32/api/winbase/nf-winbase-getprocessaffinitymask), [SetProcessAffinityMask](https://learn.microsoft.com/zh-cn/windows/win32/api/winbase/nf-winbase-setprocessaffinitymask) |
| **权限** | `PROCESS_QUERY_LIMITED_INFORMATION`（读）, `PROCESS_SET_INFORMATION`（写） |

## 参见

| 主题 | 描述 |
|---|---|
| [ApplyConfigResult](ApplyConfigResult.md) | 更改和错误的累积器 |
| [apply_process_default_cpuset](apply_process_default_cpuset.md) | 通过 CPU 集合进行的软 CPU 偏好（硬亲和性的替代） |
| [reset_thread_ideal_processors](reset_thread_ideal_processors.md) | 亲和性更改后重新分配线程理想处理器 |
| [apply_prime_threads_promote](apply_prime_threads_promote.md) | 使用 `current_mask` 过滤主 CPU 索引 |
| [ProcessLevelConfig](../config.rs/ProcessLevelConfig.md) | 包含 `affinity_cpus` 的配置结构 |
| [ProcessHandle](../winapi.rs/ProcessHandle.md) | 具有读/写访问级别的进程句柄包装器 |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*