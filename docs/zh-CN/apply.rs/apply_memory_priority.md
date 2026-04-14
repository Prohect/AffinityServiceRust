# apply_memory_priority 函数 (apply.rs)

通过 `SetProcessInformation` 并使用 `ProcessMemoryPriority` 信息类来设置进程的内存优先级。内存优先级影响内存管理器在内存压力下回收和重新分配进程物理页面的积极程度——优先级较低的页面会被优先回收。该函数通过 `GetProcessInformation` 读取当前内存优先级，将其与配置的目标值进行比较，仅在两者不同时才应用更改。

## 语法

```AffinityServiceRust/src/apply.rs#L508-515
pub fn apply_memory_priority(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process_handle: &ProcessHandle,
    apply_config_result: &mut ApplyConfigResult,
)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 目标进程的进程标识符。用于 [log_error_if_new](log_error_if_new.md) 中的错误去重以及格式化日志消息。 |
| `config` | `&`[ProcessConfig](../config.rs/ProcessConfig.md) | 匹配到此进程的已解析配置规则。`memory_priority` 字段（一个 [MemoryPriority](../priority.rs/MemoryPriority.md) 枚举）指定所需的内存优先级级别。当该值为 `MemoryPriority::None` 时，函数会立即返回，不进行任何查询或修改。 |
| `dry_run` | `bool` | 当为 `true` 时，函数将预期更改记录到 `apply_config_result` 中，但不调用 `SetProcessInformation`。 |
| `process_handle` | `&`[ProcessHandle](../winapi.rs/ProcessHandle.md) | 目标进程的 OS 句柄包装器。通过 [get_handles](get_handles.md) 提取最佳可用的读取句柄（用于 `GetProcessInformation`）和写入句柄（用于 `SetProcessInformation`）。 |
| `apply_config_result` | `&mut`[ApplyConfigResult](ApplyConfigResult.md) | 此调用过程中产生的更改描述和错误消息的累加器。 |

## 返回值

无 (`()`)。结果通过 `apply_config_result` 传达。

## 备注

### 控制流程

1. [get_handles](get_handles.md) 提取最佳可用的读取和写入 `HANDLE`。如果其中任何一个为 `None`，函数立即返回。

2. `config.memory_priority.as_win_const()` 将 [MemoryPriority](../priority.rs/MemoryPriority.md) 枚举转换为其 Win32 [MEMORY_PRIORITY_INFORMATION](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-memory_priority_information) 值（包装在 [MemoryPriorityInformation](../priority.rs/MemoryPriorityInformation.md) 中）。如果配置的优先级为 `None`，`as_win_const()` 返回 `None`，函数退出——不查询、不更改。

3. 使用 `ProcessMemoryPriority` 类调用 `GetProcessInformation`，将当前内存优先级读入本地 [MemoryPriorityInformation](../priority.rs/MemoryPriorityInformation.md) 结构。
   - 失败时，捕获 Win32 错误码并通过 [log_error_if_new](log_error_if_new.md) 以 `Operation::GetProcessInformation2ProcessMemoryPriority` 路由。函数返回，不再尝试设置。

4. 如果当前值已与目标匹配，函数静默退出（幂等）。

5. 在模拟运行模式下，记录摘要更改消息后函数返回。

6. 否则，使用目标值构造新的 `MemoryPriorityInformation` 并传递给 `SetProcessInformation`。成功时记录更改；失败时捕获错误并记录。

### 更改消息格式

成功设置时（非模拟运行）：

```/dev/null/example.txt#L1
Memory Priority: Normal -> VeryLow
```

消息显示新旧内存优先级级别的人类可读名称，分别通过 `MemoryPriority::from_win_const()` 和 `MemoryPriority::as_str()` 获取。

### 内存优先级级别

[MemoryPriority](../priority.rs/MemoryPriority.md) 枚举映射到以下 Windows 定义的值：

| 枚举变体 | Win32 值 | 效果 |
|----------|----------|------|
| `VeryLow` | 1 | 在内存压力下，页面最先被回收。 |
| `Low` | 2 | 页面在中优先级页面之前被回收。 |
| `Medium` | 3 | 后台进程的默认值。 |
| `BelowNormal` | 4 | 略优于中优先级。 |
| `Normal` | 5 | 前台进程的默认值。页面最后被回收。 |

### 错误处理

查询和设置阶段的错误都通过 [log_error_if_new](log_error_if_new.md) 路由。常见的失败场景：

| Win32 错误 | 典型原因 |
|------------|----------|
| `ERROR_ACCESS_DENIED` (5) | 进程句柄以不足的访问权限打开，或目标是受保护进程。 |
| `ERROR_INVALID_PARAMETER` (87) | 传递了无效的内存优先级值（使用枚举时不应发生）。 |

### 幂等性

该函数是幂等的：当前内存优先级已与配置目标匹配时，不会进行 Win32 调用，也不会记录更改。这避免了每个轮询周期中不必要的内核转换。

### 与 IO 优先级的关系

内存优先级和 IO 优先级（[apply_io_priority](apply_io_priority.md)）是独立的设置。它们在 [ProcessConfig](../config.rs/ProcessConfig.md) 中分别配置，由不同的函数应用。两者都影响操作系统从进程回收资源的积极程度，但它们针对不同的子系统（内存管理器 vs. IO 调度器）。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `apply` |
| 可见性 | `pub`（crate 公开） |
| 调用者 | [apply_config_process_level](../main.rs/apply_config_process_level.md) |
| 被调用者 | [get_handles](get_handles.md), [log_error_if_new](log_error_if_new.md) |
| Win32 API | [`GetProcessInformation`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessinformation), [`SetProcessInformation`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessinformation)（使用 `ProcessMemoryPriority` 类） |
| 权限 | `PROCESS_QUERY_LIMITED_INFORMATION`（通过 `GetProcessInformation` 读取），`PROCESS_SET_INFORMATION`（通过 `SetProcessInformation` 写入）。服务通常持有 `SeDebugPrivilege`，可同时满足这两项要求。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| apply 模块概述 | [apply](README.md) |
| IO 优先级设置 | [apply_io_priority](apply_io_priority.md) |
| 进程优先级类设置 | [apply_priority](apply_priority.md) |
| 内存优先级枚举 | [MemoryPriority](../priority.rs/MemoryPriority.md) |
| MemoryPriorityInformation 包装器 | [MemoryPriorityInformation](../priority.rs/MemoryPriorityInformation.md) |
| 配置模型 | [ProcessConfig](../config.rs/ProcessConfig.md) |
| 句柄提取辅助函数 | [get_handles](get_handles.md) |
| 错误去重 | [log_error_if_new](log_error_if_new.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd