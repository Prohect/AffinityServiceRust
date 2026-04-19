# format_100ns 函数 (scheduler.rs)

将以 100 纳秒为单位的时间值格式化为人类可读的 `"seconds.milliseconds s"` 字符串。用于进程退出报告中显示来自 `SYSTEM_THREAD_INFORMATION` 的内核时间、用户时间及其他计时字段。

## 语法

```rust
fn format_100ns(time: i64) -> String
```

## 参数

| 名称 | 类型 | 描述 |
|-----------|------|-------------|
| `time` | `i64` | 以 100 纳秒（100ns）为单位的时间持续值，由 Windows 内核结构（如 `SYSTEM_THREAD_INFORMATION.KernelTime` 和 `SYSTEM_THREAD_INFORMATION.UserTime`）报告。 |

## 返回值

`String` — 格式为 `"{seconds}.{milliseconds:03} s"` 的字符串，其中毫秒部分以零填充至三位数字。

### 示例

| 输入（100ns 单位） | 输出 |
|---------------------|--------|
| `0` | `"0.000 s"` |
| `10_000_000` | `"1.000 s"` |
| `156_250_000` | `"15.625 s"` |
| `10_000` | `"0.001 s"` |
| `99_999` | `"0.009 s"` |

## 备注

- 转换仅使用整数运算，不涉及浮点数舍入：
  - 秒：`time / 10_000_000`
  - 毫秒：`(time % 10_000_000) / 10_000`
- 亚毫秒精度（剩余的微秒和 100ns 分量）被截断而非四舍五入。
- 此函数为模块私有（`fn`，而非 `pub fn`），仅在 [PrimeThreadScheduler::drop_process_by_pid](PrimeThreadScheduler.md) 生成进程退出时的线程诊断报告时调用。
- `" s"` 后缀始终会被附加，以在日志输出中将其与原始数值区分开来。

### 单位参考

| Windows 单位 | = SI 等价 |
|---|---|
| 1 tick | 100 纳秒 |
| 10,000 ticks | 1 毫秒 |
| 10,000,000 ticks | 1 秒 |

## 要求

| | |
|---|---|
| **模块** | `src/scheduler.rs` |
| **调用方** | [PrimeThreadScheduler::drop_process_by_pid](PrimeThreadScheduler.md) |
| **依赖** | 无（纯格式化函数） |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 文件时间格式化器 | [format_filetime](format_filetime.md) |
| 进程退出报告 | [PrimeThreadScheduler::drop_process_by_pid](PrimeThreadScheduler.md) |
| 线程统计容器 | [ThreadStats](ThreadStats.md) |
| 模块概述 | [scheduler.rs](README.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*