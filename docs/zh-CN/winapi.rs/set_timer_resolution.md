# set_timer_resolution 函数 (winapi.rs)

通过调用 `ntdll.dll` 中未公开的 `NtSetTimerResolution` 函数，将系统级定时器分辨率设置为调用方指定的值。较低的定时器分辨率值会使 Windows 调度器更频繁地执行时钟中断，从而降低时间敏感型工作负载的调度延迟，但代价是略微增加功耗和 CPU 开销。

## 语法

```rust
pub fn set_timer_resolution(cli: &CliArgs)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `cli` | `&CliArgs` | 对已解析的 CLI 参数的引用。该函数读取 `cli.time_resolution`，这是一个以 100 纳秒为单位的 `u32` 值（例如 `5000` = 0.5 ms，`10000` = 1.0 ms）。值为 `0` 是默认值，通常表示"不更改定时器分辨率"。 |

## 返回值

无。该函数在内部记录成功或失败信息，不返回结果。

## 备注

### 定时器分辨率单位

`time_resolution` 值使用 Windows 内核的原生时间单位——100 纳秒间隔（也称为"百纳秒"）。常见值：

| `time_resolution` | 等效周期 | 描述 |
|-------------------|----------|------|
| `5000` | 0.5000 ms | 大多数硬件支持的最高分辨率 |
| `10000` | 1.0000 ms | 常见的高分辨率设置 |
| `156250` | 15.6250 ms | Windows 默认定时器分辨率 |

### NtSetTimerResolution API

该函数使用以下参数调用 `ntdll.dll` 中未公开的 `NtSetTimerResolution`：

- `desired_resolution` — 以 100 ns 为单位的请求分辨率（`cli.time_resolution`）。
- `set_resolution` — `true` 表示请求新的分辨率。
- `p_current_resolution` — 指向 `u32` 的指针，接收先前（旧的）定时器分辨率。

返回值为 `NTSTATUS`：

| NTSTATUS 范围 | 含义 |
|---------------|------|
| `>= 0`（非负） | 成功。定时器分辨率已更改（或已处于请求的值）。 |
| `< 0`（负值） | 失败。请求的分辨率可能超出支持范围或调用方缺少权限。 |

### 日志记录

| 结果 | 日志消息 |
|------|----------|
| 成功 | `"Succeed to set timer resolution: {value}ms"`（格式化为 4 位小数），后跟 `"elder timer resolution: {previous_value}"`（原始 100 ns 刻度） |
| 失败 | `"Failed to set timer resolution: 0x{ntstatus:08X}"` |

### 系统级影响

通过 `NtSetTimerResolution`（以及已公开的 `timeBeginPeriod`）进行的定时器分辨率更改是系统全局的：Windows 内核使用所有运行进程请求的最小（最频繁）分辨率。当调用进程退出时，其分辨率请求会自动移除，系统恢复到下一个最小的活跃请求。

### 典型用法

如果用户指定了 `--time_resolution` CLI 参数，AffinityServiceRust 会在启动期间调用一次 `set_timer_resolution`。默认的 `time_resolution` 值为 `0`（或未指定标志）意味着该函数仍然会被调用，但请求 0 刻度的分辨率，`NtSetTimerResolution` 会拒绝（返回负 NTSTATUS），实际上使该调用成为空操作。

### 安全性

函数体包装在 `unsafe` 块中，因为 `NtSetTimerResolution` 是在模块的 `extern "system"` 块中声明的 FFI 调用。唯一涉及的可变状态是一个栈局部 `u32`（`current_resolution`），用作输出参数。

### 与进程调度的关系

更高频率的定时器中断降低了最小休眠粒度和调度量子，这有利于服务所管理的实时应用程序（游戏、音频引擎）。但是，它也会略微增加系统级中断开销。用户应选择一个在延迟需求和效率之间取得平衡的值。

## 要求

| | |
|---|---|
| **模块** | `winapi` (`src/winapi.rs`) |
| **可见性** | `pub` |
| **调用方** | [`main`](../main.rs/README.md)（启动期间） |
| **被调用方** | `NtSetTimerResolution`（ntdll.dll FFI） |
| **API** | `NtSetTimerResolution` — 未公开的 ntdll 函数；已公开的等效函数：[`timeBeginPeriod`](https://learn.microsoft.com/en-us/windows/win32/api/timeapi/nf-timeapi-timebeginperiod) |
| **特权** | 无需超出正常进程特权的额外权限 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| CLI 参数解析（time_resolution 标志） | [cli 模块](../cli.rs/README.md) |
| 服务主入口点 | [main 模块](../main.rs/README.md) |
| timeBeginPeriod（已公开替代） | [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/api/timeapi/nf-timeapi-timebeginperiod) |
| 定时器分辨率深入探讨 | [Microsoft Learn — Timer Resolution](https://learn.microsoft.com/en-us/windows/win32/sysinfo/acquiring-high-resolution-time-stamps) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd