# format_100ns 函数 (scheduler.rs)

将 Windows 100 纳秒时间间隔值转换为人类可读的字符串，格式为 `秒.毫秒 s`。这是一个模块私有的工具函数，用于在进程退出时记录线程诊断信息时格式化 `SYSTEM_THREAD_INFORMATION` 中的内核时间和用户时间值。

## 语法

```AffinityServiceRust/src/scheduler.rs#L265-267
fn format_100ns(time: i64) -> String
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `time` | `i64` | 以 100 纳秒为单位表示的时间持续值，由 Windows 内核时间字段返回，例如 `SYSTEM_THREAD_INFORMATION.KernelTime` 和 `SYSTEM_THREAD_INFORMATION.UserTime`。 |

## 返回值

返回格式为 `"<秒>.<毫秒> s"` 的 `String`，其中毫秒部分零填充至三位数字。

**示例：**

| 输入（100 纳秒单位） | 输出 |
|----------------------|------|
| `0` | `"0.000 s"` |
| `10_000_000` | `"1.000 s"` |
| `156_250_000` | `"15.625 s"` |
| `10_000` | `"0.001 s"` |

## 备注

- 转换过程将输入除以 10,000,000 得到整秒数，然后通过对 10,000,000 取余并除以 10,000 提取毫秒部分。
- 亚毫秒精度（剩余的微秒和 100 纳秒分量）会被截断而非四舍五入。
- 负输入值从除法角度看在技术上是有效的，但会产生与实现相关的格式化结果（Rust 的 `%` 运算符保留被除数的符号）。在实践中，Windows 内核时间值始终为非负数。
- 此函数**不是** `pub` 的，仅在 `scheduler` 模块内部可访问。
- 此函数不执行任何 Win32 API 调用，且没有副作用。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `scheduler.rs` |
| 可见性 | 私有（crate 内部，不导出） |
| 调用者 | [`PrimeThreadScheduler::drop_process_by_pid`](PrimeThreadScheduler.md) |
| 被调用者 | `std::format!` |
| API | 无 |
| 权限 | 无 |

## 另请参阅

| 参考 | 链接 |
|------|------|
| format_filetime | [format_filetime](format_filetime.md) |
| PrimeThreadScheduler | [PrimeThreadScheduler](PrimeThreadScheduler.md) |
| ThreadStats | [ThreadStats](ThreadStats.md) |
| scheduler 模块概述 | [README](README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
