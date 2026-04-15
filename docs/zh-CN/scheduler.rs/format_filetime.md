# format_filetime 函数 (scheduler.rs)

将 Windows `FILETIME` 值（表示为自 1601 年 1 月 1 日 UTC 以来的 100 纳秒间隔计数的 64 位整数）转换为格式为 `YYYY-MM-DD HH:MM:SS.mmm` 的人类可读本地日期时间字符串。此函数在进程退出时记录详细线程信息时使用。

## 语法

```AffinityServiceRust/src/scheduler.rs#L279-L286
fn format_filetime(time: i64) -> String
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `time` | `i64` | 表示 Windows `FILETIME` 值的 64 位有符号整数——自 1601 年 1 月 1 日 00:00:00 UTC 以来的 100 纳秒间隔数。通常来源于 `SYSTEM_THREAD_INFORMATION::CreateTime.QuadPart()`。 |

## 返回值

返回一个包含本地日期时间表示的 `String`，格式为 `YYYY-MM-DD HH:MM:SS.mmm`（例如 `"2025-01-15 14:32:07.456"`）。如果时间戳无法转换为有效的 `DateTime`（例如输入为负数或表示的日期超出可表示范围），则返回原始 `i64` 值的十进制字符串表示。

## 备注

### 转换算法

1. **FILETIME 到 Unix 纪元** — 函数首先通过除以 10,000,000 获取整数秒，然后减去常量 `11,644,473,600`（两个纪元之间的秒数），将 Windows 纪元（1601-01-01）转换为 Unix 纪元（1970-01-01）。
2. **亚秒精度** — 纳秒分量从 `time % 10_000_000` 的余数中提取，乘以 100 将 100 纳秒单位转换为纳秒。
3. **本地时间转换** — 生成的 UTC 时间戳使用 `chrono::Local` 时区转换为本地时间，并以 `"%Y-%m-%d %H:%M:%S%.3f"` 模式格式化，包含毫秒精度。

### 边界情况

- 如果 `DateTime::from_timestamp` 返回 `None`（无效或超出范围的时间戳），函数回退返回 `time.to_string()`，即原始 100 纳秒刻度计数的十进制字符串。
- `time` 值为 `0` 对应 Windows 纪元（1601-01-01 00:00:00 UTC），它转换为负的 Unix 时间戳，取决于平台可能可以表示也可能无法表示。
- 非常接近系统启动时间的线程创建时间可能会携带异常值，具体取决于 Windows 内核版本。

### 使用场景

此函数在 `PrimeThreadScheduler::drop_process_by_pid` 内部调用，用于在进程退出时生成 top-N 线程诊断报告时格式化 `SYSTEM_THREAD_INFORMATION` 的 `CreateTime` 字段。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `scheduler.rs` |
| 调用者 | [`PrimeThreadScheduler::drop_process_by_pid`](PrimeThreadScheduler.md) |
| 被调用者 | `chrono::DateTime::from_timestamp`、`chrono::DateTime::with_timezone`、`chrono::DateTime::format` |
| 依赖 | `chrono` crate（`DateTime`、`Local`） |
| 权限 | 无 |

## 另请参阅

| 参考 | 链接 |
|------|------|
| format_100ns | [format_100ns](format_100ns.md) |
| PrimeThreadScheduler | [PrimeThreadScheduler](PrimeThreadScheduler.md) |
| ThreadStats | [ThreadStats](ThreadStats.md) |
| scheduler 模块概述 | [README](README.md) |

---
> Commit SHA: `7221ea0694670265d4eb4975582d8ed2ae02439d`
