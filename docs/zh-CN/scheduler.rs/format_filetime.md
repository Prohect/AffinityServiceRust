# format_filetime 函数 (scheduler.rs)

将 Windows FILETIME 值（自 1601 年 1 月 1 日 UTC 起的 100 纳秒间隔数）转换为人类可读的本地日期时间字符串。用于诊断报告中记录线程创建时间和其他内核时间戳。

## 语法

```rust
fn format_filetime(time: i64) -> String
```

## 参数

| 名称 | 类型 | 描述 |
|-----------|------|-------------|
| `time` | `i64` | Windows FILETIME 值，表示自 Windows 纪元（1601 年 1 月 1 日 00:00:00 UTC）以来的 100 纳秒间隔数。这与 `SYSTEM_THREAD_INFORMATION.CreateTime` 及其他以 `LARGE_INTEGER` 存储的内核时间字段使用相同的单位。 |

## 返回值

`String` — 格式化的本地日期时间字符串，模式为 `"YYYY-MM-DD HH:MM:SS.mmm"`（例如 `"2025-01-15 14:32:07.123"`）。如果时间戳无法转换为有效的 `DateTime`，则返回原始 `i64` 值的十进制字符串表示。

## 备注

### 转换算法

1. **FILETIME 转 Unix 纪元：** 将输入除以 `10,000,000` 从 100ns 刻度转换为整秒数，然后减去 Windows 到 Unix 纪元的偏移量（`11,644,473,600` 秒）。这弥合了 Windows 纪元（1601-01-01）与 Unix 纪元（1970-01-01）之间的差距。
2. **亚秒精度：** 小数部分 `(time % 10_000_000) * 100` 作为纳秒分量传递给 `DateTime::from_timestamp`。
3. **时区转换：** 生成的 UTC `DateTime` 通过 `dt.with_timezone(&Local)` 转换为本地时区。
4. **格式化：** `chrono` 格式字符串 `"%Y-%m-%d %H:%M:%S%.3f"` 产生毫秒精度的输出。

### 回退行为

如果 `DateTime::from_timestamp` 返回 `None`（例如输入值为负数或表示的日期超出可表示范围），函数将回退返回 `time.to_string()` — 即原始 100ns 刻度计数的纯十进制数字。

### 转换示例

| 输入（100ns 刻度） | 含义 | 输出（示例，UTC+8） |
|---------------------|---------|-------------------------|
| `133500000000000000` | ~2024-01-15 | `"2024-01-15 08:00:00.000"` |
| `0` | Windows 纪元（1601-01-01） | `"1601-01-01 08:00:00.000"` 或本地等效值 |
| `-1` | 无效 | `"-1"` |

### 与 format_100ns 的关系

[format_100ns](format_100ns.md) 将*持续时间*（经过的 100ns 间隔）格式化为 `"seconds.milliseconds s"`，而 `format_filetime` 将*绝对时间戳*（自 1601 年起的 100ns 间隔）格式化为日历日期时间字符串。两者都操作相同 100ns 单位的 `i64` 值，但服务于不同的语义用途。

## 要求

| | |
|---|---|
| **模块** | `src/scheduler.rs` |
| **调用方** | [PrimeThreadScheduler::drop_process_by_pid](PrimeThreadScheduler.md)（在最活跃线程报告中格式化 `CreateTime`） |
| **依赖项** | `chrono::DateTime`、`chrono::Local` |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 模块概述 | [scheduler.rs](README.md) |
| 持续时间格式化 | [format_100ns](format_100ns.md) |
| 调度器清理逻辑 | [PrimeThreadScheduler](PrimeThreadScheduler.md) |
| 线程统计 | [ThreadStats](ThreadStats.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*