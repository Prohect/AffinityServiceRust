# format_100ns 函数 (scheduler.rs)

将以 100 纳秒为单位的时间间隔值格式化为具有毫秒精度的人类可读秒数字符串。

## 语法

```rust
fn format_100ns(time: i64) -> String
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `time` | `i64` | 以 100 纳秒为单位的时间间隔（Windows 内核时间字段的原生单位，例如 `SYSTEM_THREAD_INFORMATION` 中的 `KernelTime` 和 `UserTime`）。 |

## 返回值

返回格式为 `"{秒数}.{毫秒数} s"` 的 `String`，其中毫秒部分用零填充至三位数字。

**输出示例：**

| 输入（100 纳秒刻度） | 输出 |
|----------------------|--------|
| `0` | `"0.000 s"` |
| `10_000_000` | `"1.000 s"` |
| `156_250_000` | `"15.625 s"` |
| `45_678` | `"0.004 s"` |

## 备注

这是一个模块私有辅助函数，用于在 [`PrimeThreadScheduler::drop_process_by_pid`](PrimeThreadScheduler.md#drop_process_by_pid) 记录线程诊断信息时使用。它将 `SYSTEM_THREAD_INFORMATION` 中的原始内核/用户时间值转换为可读格式，以便在进程退出线程报告中使用。

Windows 将内核时间和用户时间表示为 100 纳秒间隔（1000 万刻度 = 1 秒）。该函数通过整数除法提取秒数和截断的毫秒数：

- **秒数：** `time / 10_000_000`
- **毫秒数：** `(time % 10_000_000) / 10_000`

毫秒值是截断的，而非四舍五入。亚毫秒精度（剩余的微秒和 100 纳秒刻度）将被丢弃。

`time` 的负值在算术层面上是有效的，但在正常使用中不应出现，因为内核时间和用户时间始终为非负值。

## 要求

| 要求 | 值 |
|------|------|
| **模块** | `scheduler.rs` |
| **可见性** | 模块私有（`fn`，无 `pub`） |
| **调用方** | [`PrimeThreadScheduler::drop_process_by_pid`](PrimeThreadScheduler.md#drop_process_by_pid) |
| **依赖** | 无（纯计算） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| format_filetime | [format_filetime](format_filetime.md) |
| PrimeThreadScheduler::drop_process_by_pid | [PrimeThreadScheduler](PrimeThreadScheduler.md#drop_process_by_pid) |
| ThreadStats | [ThreadStats](ThreadStats.md) |
| SYSTEM_THREAD_INFORMATION | [Microsoft Learn — SYSTEM_THREAD_INFORMATION](https://learn.microsoft.com/en-us/windows/win32/api/winternl/nf-winternl-ntquerysysteminformation#system_thread_information) |