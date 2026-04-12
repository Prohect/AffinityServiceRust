# format_100ns 函数 (scheduler.rs)

将以 100 纳秒为单位的时间间隔格式化为人类可读的持续时间字符串。

## 语法

```rust
fn format_100ns(time: i64) -> String
```

## 参数

`time`

以 100 纳秒（即 0.1 微秒）为单位的时间间隔值。此值通常来自 Windows 内核结构体中的时间字段，例如 `SYSTEM_THREAD_INFORMATION` 的 `KernelTime` 和 `UserTime`。

## 返回值

返回格式为 `"{seconds}.{milliseconds:03} s"` 的 `String`。毫秒部分始终为三位数，不足时补零。

### 示例输出

| 输入值 (100ns 单位) | 输出 |
| --- | --- |
| `0` | `"0.000 s"` |
| `10_000_000` | `"1.000 s"` |
| `156_250_000` | `"15.625 s"` |
| `50_000` | `"0.005 s"` |

## 备注

Windows 内核广泛使用 100 纳秒作为时间度量单位。`SYSTEM_THREAD_INFORMATION` 结构体中的 `KernelTime`（内核态时间）和 `UserTime`（用户态时间）字段均以此单位存储。此函数将这些原始值转换为便于阅读和日志输出的格式。

### 换算关系

- 1 秒 = 10,000,000 个 100ns 单位
- 1 毫秒 = 10,000 个 100ns 单位

### 精度

函数输出精度为毫秒级（3 位小数）。100ns 单位到毫秒之间的亚毫秒部分在整数除法中被截断，不进行四舍五入。

### 可见性

此函数为模块私有（`fn`，非 `pub fn`），仅在 `scheduler.rs` 内部被 [`PrimeThreadScheduler::close_dead_process_handles`](PrimeThreadScheduler.md) 调用，用于在进程退出时格式化线程的内核态和用户态时间。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/scheduler.rs |
| **源码行** | L303–L307 |
| **可见性** | 模块私有 |
| **调用者** | [`close_dead_process_handles`](PrimeThreadScheduler.md) |

## 另请参阅

- [format_filetime](format_filetime.md) — 将 FILETIME 转换为本地日期时间字符串
- [PrimeThreadScheduler](PrimeThreadScheduler.md) — 使用此函数进行日志格式化
- [ThreadStats](ThreadStats.md) — `last_system_thread_info` 字段包含需要格式化的时间值