# format_filetime 函数 (scheduler.rs)

将 Windows FILETIME 格式的 100 纳秒间隔值转换为本地时区的人类可读日期时间字符串。

## 语法

```rust
fn format_filetime(time: i64) -> String
```

## 参数

`time`

以 100 纳秒间隔表示的 Windows FILETIME 时间戳。FILETIME 的纪元为 1601-01-01 00:00:00 UTC。该值通常来自 `SYSTEM_THREAD_INFORMATION` 中的 `CreateTime.QuadPart()`。

## 返回值

返回格式为 `"YYYY-MM-DD HH:MM:SS.mmm"` 的本地时区日期时间字符串。

如果时间戳无法成功转换为 `DateTime`（例如值超出有效范围），则直接返回原始 `i64` 值的字符串表示。

## 备注

转换过程分为两步：

1. **FILETIME → Unix 时间戳：** 将 100 纳秒间隔除以 10,000,000 得到秒数，再减去 11,644,473,600（即 1601-01-01 到 1970-01-01 之间的秒数差），得到 Unix 时间戳。
2. **Unix 时间戳 → 本地日期时间：** 使用 `chrono::DateTime::from_timestamp` 构造 UTC 时间，然后通过 `with_timezone(&Local)` 转换为本地时区，最后以 `%Y-%m-%d %H:%M:%S%.3f` 格式化输出。

亚秒精度保留到毫秒级别（3 位小数），通过 `time % 10_000_000 * 100` 计算纳秒余数传递给 `from_timestamp`。

此函数为模块私有（`fn`，非 `pub fn`），仅在 [`close_dead_process_handles`](PrimeThreadScheduler.md#close_dead_process_handles) 的进程退出报告中使用，用于格式化线程的 `CreateTime` 字段。

### 示例输出

| 输入（100ns 间隔） | 输出 |
| --- | --- |
| `133800000000000000` | `"2024-12-04 08:00:00.000"`（取决于本地时区） |
| `0` | `"1601-01-01 ..."` 或回退为 `"0"` |

### FILETIME 纪元

Windows FILETIME 以 1601 年 1 月 1 日 UTC 为纪元起点，而 Unix 时间以 1970 年 1 月 1 日 UTC 为起点。两者之间的差值为 11,644,473,600 秒（即代码中的魔术数字）。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/scheduler.rs |
| **源码行** | L309–L316 |
| **可见性** | 模块私有 |
| **调用者** | [`PrimeThreadScheduler::close_dead_process_handles`](PrimeThreadScheduler.md) |
| **依赖** | `chrono::DateTime`、`chrono::Local` |

## 另请参阅

- [format_100ns](format_100ns.md) — 格式化持续时间（非绝对时间点）
- [PrimeThreadScheduler](PrimeThreadScheduler.md) — 在进程退出报告中调用此函数
- [ThreadStats](ThreadStats.md) — `last_system_thread_info` 包含原始 FILETIME 值