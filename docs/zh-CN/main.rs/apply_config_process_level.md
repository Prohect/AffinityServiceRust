# apply_config_process_level 函数 (main.rs)

为单个受管进程应用进程级设置。此函数在每个进程的生命周期内仅调用一次（一次性），负责处理优先级类别、CPU affinity、CPU set、IO 优先级和内存优先级。它会打开目标进程的句柄，并将具体操作委托给 `apply` 模块中的各个应用函数。

## 语法

```rust
fn apply_config_process_level(
    pid: u32,
    config: &ProcessConfig,
    process: &mut ProcessEntry,
    dry_run: bool,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

`pid`

目标进程的进程标识符 (PID)。

`config`

指向 [ProcessConfig](../config.rs/ProcessConfig.md) 的引用，包含此进程所需的设置。从该结构体中读取 `priority`、`affinity_cpus`、`cpu_set_cpus`、`io_priority` 和 `memory_priority` 等字段。

`process`

指向目标进程的 [ProcessEntry](../process.rs/ProcessEntry.md) 的可变引用。`apply_affinity` 和 `apply_process_default_cpuset` 会修改此引用以更新缓存的线程理想处理器状态和 CPU set 分配。

`dry_run`

当为 `true` 时，函数模拟变更并将预期操作记录在 `apply_config_result` 中，而不实际调用任何 Win32 API。当为 `false` 时，设置会应用到实际进程。

`apply_config_result`

指向 [ApplyConfigResult](../apply.rs/ApplyConfigResult.md) 累加器的可变引用。每个子函数会将其变更和错误追加到此结构体中。调用方在调用后检查该结果以记录变更和错误。

## 返回值

此函数不返回值。如果无法获取进程句柄（例如进程已退出或访问被拒绝），函数会提前返回，不应用任何设置。

## 备注

该函数按以下顺序调用应用函数：

1. [apply_priority](../apply.rs/apply_priority.md) — 设置进程优先级类别（例如高、高于正常）。
2. [apply_affinity](../apply.rs/apply_affinity.md) — 设置硬 CPU affinity 掩码，并在 affinity 发生变更时重置线程理想处理器。
3. [apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md) — 分配进程默认 CPU set（软 CPU 偏好）。
4. [apply_io_priority](../apply.rs/apply_io_priority.md) — 通过 `NtSetInformationProcess` 设置进程 IO 优先级。
5. [apply_memory_priority](../apply.rs/apply_memory_priority.md) — 通过 `SetProcessInformation` 设置进程内存优先级。

执行顺序很重要：affinity 在 CPU set 之前设置，因为 `apply_affinity` 会跟踪 `current_mask`，该值可能被下游引用。优先级最先设置，因为某些优先级变更需要特定的句柄访问权限，提前失败可以避免不必要的工作。

进程句柄通过 [get_process_handle](../winapi.rs/get_process_handle.md) 获取，该函数同时请求查询和设置访问权限。如果无法打开句柄（例如进程已退出，或调用方缺少 `SeDebugPrivilege`），函数会立即返回，不应用任何设置。

此函数被设计为每个进程仅调用一次。调用方（`main`）在名为 `process_level_applied` 的 `HashSet<u32>` 中跟踪已处理的 PID，并跳过对同一 PID 的后续调用。如果进程退出后创建了具有相同 PID 的新进程，ETW 监视器会将该 PID 从已应用集合中移除，允许重新应用。

在试运行模式（`-dryrun` CLI 标志）下，所有子函数会将其预期变更记录在 `apply_config_result.changes` 中，而不调用任何 Win32 API。这对于在部署前验证配置非常有用。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `main` |
| 调用方 | [main](main.md)（轮询循环、ETW 待处理队列） |
| 被调用方 | [get_process_handle](../winapi.rs/get_process_handle.md)、[apply_priority](../apply.rs/apply_priority.md)、[apply_affinity](../apply.rs/apply_affinity.md)、[apply_process_default_cpuset](../apply.rs/apply_process_default_cpuset.md)、[apply_io_priority](../apply.rs/apply_io_priority.md)、[apply_memory_priority](../apply.rs/apply_memory_priority.md) |
| API | `OpenProcess`（通过 `get_process_handle`）、`SetPriorityClass`、`SetProcessAffinityMask`、`SetProcessDefaultCpuSets`、`NtSetInformationProcess`、`SetProcessInformation` |
| 权限 | `SeDebugPrivilege`（推荐）、`SeIncreaseBasePriorityPrivilege`（用于高/实时优先级） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 线程级设置（每次迭代） | [apply_config_thread_level](apply_config_thread_level.md) |
| 进程配置结构体 | [ProcessConfig](../config.rs/ProcessConfig.md) |
| 应用结果累加器 | [ApplyConfigResult](../apply.rs/ApplyConfigResult.md) |
| 主入口点和轮询循环 | [main](main.md) |