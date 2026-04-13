# format_filetime 函数 (scheduler.rs)

将 Windows `FILETIME` 值（以自 1601 年 1 月 1 日起的 100 纳秒间隔表示的 `i64`）格式化为具有毫秒精度的本地日期时间字符串。

## 语法

```rust
fn format_filetime(time: i64) -> String
```

## 参数

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `time` | `i64` | 表示 Windows `FILETIME` 值的 64 位有符号整数——自 1601 年 1 月 1 日（UTC）以来的 100 纳秒间隔数。 |

## 返回值

返回一个包含格式化本地日期时间的 `String`。格式为 `YYYY-MM-DD HH:MM:SS.mmm`，其中 `mmm` 为毫秒。

如果时间戳无法转换为有效的 `DateTime`（例如，表示的日期超出可表示范围），则返回原始 `i64` 值的字符串形式。

## 备注

此函数通过减去已知常量 **11,644,473,600 秒**，将 Windows `FILETIME` 纪元（1601 年 1 月 1 日 UTC）转换为 Unix 纪元（1970 年 1 月 1 日 UTC）。转换过程如下：

1. 将 `time` 除以 10,000,000 得到自 Windows 纪元以来的整秒数。
2. 减去 11,644,473,600 以重新对齐到 Unix 纪元。
3. 提取亚秒余数作为纳秒：`(time % 10_000_000) * 100`。
4. 通过 `DateTime::from_timestamp` 构造 UTC `DateTime`，然后转换为本地时区。

输出使用 `chrono` 的 `%Y-%m-%d %H:%M:%S%.3f` 格式说明符进行格式化，始终生成恰好三位小数（毫秒）。

### 可见性

此函数为模块私有（`fn`，非 `pub fn`）。它在 [`PrimeThreadScheduler::drop_process_by_pid`](PrimeThreadScheduler.md) 内部调用，用于在线程诊断报告中格式化 `SYSTEM_THREAD_INFORMATION` 的 `CreateTime` 字段。

### 边界情况

| 场景 | 行为 |
|----------|----------|
| `time` 为 `0` | 转换为 `1601-01-01` 减去纪元偏移量，即负的 Unix 时间戳。如果超出范围则回退为打印 `"0"`。 |
| `time` 为负数 | 表示 Windows 纪元之前的日期。Unix 转换将进一步下溢；回退为打印原始值。 |
| 本地时区不可用 | `chrono` 在无法检测本地时区的平台上以 UTC 作为回退。 |

### 输出示例

对于表示本地时间 2025 年 7 月 4 日 14:30:00.123 的 `FILETIME` 值：

```text
2025-07-04 14:30:00.123
```

## 要求

| &nbsp; | &nbsp; |
|--------|--------|
| **模块** | `scheduler.rs` |
| **Crate 依赖** | `chrono`（用于 `DateTime`、`Local`） |
| **调用方** | [`PrimeThreadScheduler::drop_process_by_pid`](PrimeThreadScheduler.md) |
| **被调用方** | `chrono::DateTime::from_timestamp`、`DateTime::with_timezone`、`DateTime::format` |
| **平台** | Windows（值来源于 Windows `FILETIME` 结构） |

## 另请参阅

| 链接 | 描述 |
|------|-------------|
| [`format_100ns`](format_100ns.md) | 将 100 纳秒间隔格式化为 秒.毫秒 的持续时间字符串。 |
| [`PrimeThreadScheduler::drop_process_by_pid`](PrimeThreadScheduler.md) | 在进程退出时记录线程诊断信息时调用 `format_filetime` 的方法。 |
| [`ThreadStats`](ThreadStats.md) | 线程统计结构体，其 `last_system_thread_info` 包含由此函数格式化的 `FILETIME` 值。 |