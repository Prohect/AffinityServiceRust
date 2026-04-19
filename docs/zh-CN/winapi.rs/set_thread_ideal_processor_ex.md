# set_thread_ideal_processor_ex 函数 (winapi.rs)

Windows `SetThreadIdealProcessorEx` API 的封装，设置线程的理想处理器提示，指定处理器组和组内的逻辑处理器编号。返回之前设置的理想处理器分配。

## 语法

```rust
pub fn set_thread_ideal_processor_ex(
    thread_handle: HANDLE,
    group: u16,
    number: u8,
) -> Result<PROCESSOR_NUMBER, Error>
```

## 参数

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `thread_handle` | `HANDLE` | 有效的线程句柄，具有 `THREAD_SET_INFORMATION` 访问权限。通常来自 [ThreadHandle](ThreadHandle.md)`.w_handle`。调用方必须在调用此函数前验证句柄不是无效的。 |
| `group` | `u16` | 理想处理器的处理器组号。在单组系统（≤64 个逻辑处理器）上，此值始终为 `0`。 |
| `number` | `u8` | 在指定的 `group` 中设置为线程理想处理器的逻辑处理器编号。例如，`0` 指向组中的第一个处理器。 |

## 返回值

`Result<PROCESSOR_NUMBER, Error>` — 成功时，返回线程的**之前**理想处理器，作为包含 `Group`、`Number` 和 `Reserved` 字段的 `PROCESSOR_NUMBER` 结构。失败时，返回带有底层 Win32 错误代码的 Windows `Error`。

## 说明

- 函数从 `group` 和 `number` 参数构造 `PROCESSOR_NUMBER` 结构（`Reserved` 设置为 `0`），并将其传递给 `SetThreadIdealProcessorEx`。可变的 `previous` 输出参数捕获 API 返回的先前理想处理器分配。
- 理想处理器是一个**调度提示**，而不是硬性约束。Windows 调度程序倾向于在指示的处理器上调度线程，但如果理想处理器繁忙，也可能将线程放置在线程亲和性掩码内的任何处理器上。对于硬 CPU 绑定，请使用 `SetThreadSelectedCpuSets` 通过 CPU 集合。
- 此函数由 [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) 和 [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md) 调用，以引导调度器将延迟敏感线程定向到特定核心。
- 此函数返回的先前理想处理器由 [IdealProcessorState](../scheduler.rs/IdealProcessorState.md) 用于跨轮询迭代跟踪分配更改。

### 错误场景

| 条件 | 行为 |
|-----------|----------|
| 无效的线程句柄 | 返回 `Err(Error)`，带有 `ERROR_INVALID_HANDLE` |
| 句柄缺少 `THREAD_SET_INFORMATION` 访问权限 | 返回 `Err(Error)`，带有 `ERROR_ACCESS_DENIED` |
| 无效组或编号（超出系统拓扑） | 返回 `Err(Error)` — 行为取决于操作系统版本 |

### 与 get_thread_ideal_processor_ex 的关系

此函数是 [get_thread_ideal_processor_ex](get_thread_ideal_processor_ex.md) 的写入对应项。它们一起形成用于管理线程理想处理器提示的读/写对。服务通常会读取当前分配以检测外部工具的更改，然后根据配置指示时写入新的分配。

## 需求

| | |
|---|---|
| **模块** | `src/winapi.rs` |
| **调用方** | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md)、[apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md)、[reset_thread_ideal_processors](../apply.rs/reset_thread_ideal_processors.md) |
| **被调用方** | `SetThreadIdealProcessorEx` (Win32) |
| **Win32 API** | [SetThreadIdealProcessorEx](https://learn.microsoft.com/zh-cn/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadidealprocessorex) |
| **特权** | 需要线程句柄上的 `THREAD_SET_INFORMATION` 访问权限 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [winapi.rs](README.md) |
| 读取对应项 | [get_thread_ideal_processor_ex](get_thread_ideal_processor_ex.md) |
| 理想处理器状态跟踪 | [IdealProcessorState](../scheduler.rs/IdealProcessorState.md) |
| 提供写入访问的线程句柄 | [ThreadHandle](ThreadHandle.md) |
| 设置理想处理器的应用函数 | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |

*文档版本：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*