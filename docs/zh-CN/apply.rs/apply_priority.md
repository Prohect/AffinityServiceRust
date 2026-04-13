# apply_priority 函数 (apply.rs)

将进程优先级类设置为配置中指定的值。该函数通过 `GetPriorityClass` 读取当前优先级，将其与配置的目标值进行比较，当两者不同时调用 `SetPriorityClass` 应用更改。在模拟运行模式下，更改会被记录但不会执行。

## 语法

```AffinityServiceRust/src/apply.rs#L83-129
pub fn apply_priority(
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
| `pid` | `u32` | 目标进程的进程标识符。用于 [log_error_if_new](log_error_if_new.md) 中的错误去重和日志消息。 |
| `config` | `&ProcessConfig` | 该进程的已解析 [ProcessConfig](../config.rs/ProcessConfig.md)。`priority` 字段（一个 [ProcessPriority](../priority.rs/ProcessPriority.md) 枚举）决定了期望的优先级类。当 `priority` 为 `ProcessPriority::None` 时，函数立即返回，不进行任何查询或修改。 |
| `dry_run` | `bool` | 当为 `true` 时，函数将预期更改记录到 `apply_config_result` 中，但不调用 `SetPriorityClass`。 |
| `process_handle` | `&ProcessHandle` | 为目标进程打开的 [ProcessHandle](../winapi.rs/ProcessHandle.md)。通过 [get_handles](get_handles.md) 提取用于读取（`GetPriorityClass`）和写入（`SetPriorityClass`）的句柄。 |
| `apply_config_result` | `&mut ApplyConfigResult` | 更改和错误的累加器。参见 [ApplyConfigResult](ApplyConfigResult.md)。 |

## 返回值

无（`()`）。结果通过 `apply_config_result` 传递。

## 备注

### 控制流

1. [get_handles](get_handles.md) 提取最佳可用的读写 `HANDLE`。如果其中任一为 `None`（对于有效的 `ProcessHandle` 不应发生），函数立即返回。
2. `config.priority.as_win_const()` 将 [ProcessPriority](../priority.rs/ProcessPriority.md) 枚举转换为其 Win32 `PROCESS_CREATION_FLAGS` 常量。如果配置的优先级为 `None`，`as_win_const()` 返回 `None`，函数退出——不查询，不更改。
3. `GetPriorityClass` 从操作系统读取当前优先级。
4. 如果当前优先级已与目标一致，函数静默退出。
5. 如果 `dry_run` 为 `true`，记录更改消息后函数退出。
6. 否则，调用 `SetPriorityClass`。成功时记录更改；失败时通过 `GetLastError` 捕获 Win32 错误码并传递给 [log_error_if_new](log_error_if_new.md)。

### 更改消息格式

```/dev/null/example.txt#L1
Priority: Normal -> High
```

消息显示新旧优先级类的可读名称，分别通过 `ProcessPriority::from_win_const()` 和 `ProcessPriority::as_str()` 获取。

### 错误处理

`SetPriorityClass` 的错误通过 [log_error_if_new](log_error_if_new.md) 以 `Operation::SetPriorityClass` 路由。常见的失败原因包括：

| Win32 错误 | 典型原因 |
|------------|----------|
| `ERROR_ACCESS_DENIED` (5) | 服务对目标进程缺少 `PROCESS_SET_INFORMATION` 访问权限（例如受保护的进程）。 |
| `ERROR_INVALID_PARAMETER` (87) | 请求的优先级类对目标进程无效（例如在没有 `SeIncreaseBasePriorityPrivilege` 的情况下设置 `Realtime`）。 |

由于错误会被去重，持续拒绝访问的进程在所有轮询周期中只会生成一条日志条目，直到去重映射被清除（参见 [purge_fail_map](../logging.rs/purge_fail_map.md)）。

### 幂等性

该函数是幂等的：当当前优先级已与目标一致时，不会进行 Win32 调用，也不会记录更改。这避免了每个轮询周期中不必要的内核态切换。

### 调用上下文

`apply_priority` 在进程级应用阶段由 [apply_config_process_level](../main.rs/apply_config_process_level.md) 调用，每个配置周期每个进程运行一次。它有意与线程级操作（[apply_prime_threads](apply_prime_threads.md)、[apply_ideal_processors](apply_ideal_processors.md)）分离，后者以不同的节奏运行。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `apply` |
| 可见性 | `pub`（crate 公开） |
| 调用者 | [apply_config_process_level](../main.rs/apply_config_process_level.md) |
| 被调用者 | [get_handles](get_handles.md)、[log_error_if_new](log_error_if_new.md) |
| Win32 API | [`GetPriorityClass`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getpriorityclass)、[`SetPriorityClass`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setpriorityclass)、[`GetLastError`](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror) |
| 权限 | `PROCESS_QUERY_LIMITED_INFORMATION`（读取）、`PROCESS_SET_INFORMATION` 或 `PROCESS_SET_LIMITED_INFORMATION`（写入）。在其他用户拥有的进程上设置 `Realtime` 或 `High` 优先级需要 `SeIncreaseBasePriorityPrivilege`。 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 优先级枚举和 Win32 常量映射 | [ProcessPriority](../priority.rs/ProcessPriority.md) |
| 进程级应用编排 | [apply_config_process_level](../main.rs/apply_config_process_level.md) |
| 配置模型 | [ProcessConfig](../config.rs/ProcessConfig.md) |
| 句柄提取辅助函数 | [get_handles](get_handles.md) |
| 错误去重 | [log_error_if_new](log_error_if_new.md) |
| apply 模块概览 | [apply](README.md) |