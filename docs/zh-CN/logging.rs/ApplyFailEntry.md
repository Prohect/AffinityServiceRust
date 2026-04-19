# ApplyFailEntry 结构体 (logging.rs)

表示进程/线程操作失败的复合键。用作每个 PID 错误去重映射表 (`PID_MAP_FAIL_ENTRY_SET`) 中的哈希表键，使得重复的相同失败只会被记录一次。

## 语法

```rust
#[derive(PartialEq, Eq, Hash)]
pub struct ApplyFailEntry {
    tid: u32,
    process_name: String,
    operation: Operation,
    error_code: u32,
}
```

## 成员

| 成员 | 类型 | 描述 |
|--------|------|-------------|
| `tid` | `u32` | 与失败关联的线程标识符。对于没有特定线程的进程级操作，调用方通常传递 `0`。 |
| `process_name` | `String` | 失败进程的可执行文件名（例如 `"chrome.exe"`）。也用作陈旧性指示器 — 当 PID 被不同的进程复用时，名称不匹配会触发清除该 PID 的所有先前条目。 |
| `operation` | [Operation](Operation.md) | 失败的 Windows API 操作（例如 `SetProcessAffinityMask`、`SetThreadPriority`）。 |
| `error_code` | `u32` | 失败调用返回的 Win32 或 NTSTATUS 错误代码。当没有可用的操作系统错误代码且调用方需要自定义哨兵值时，使用 `0`。 |

## 备注

- 该结构体派生了 `PartialEq`、`Eq` 和 `Hash`，因此可以用作 `HashMap<ApplyFailEntry, bool>` 的键。映射表中的 `bool` 值跟踪活性 — `true` 表示条目在当前应用周期中被看到，`false` 表示它是 [purge_fail_map](purge_fail_map.md) 期间删除的候选。
- `process_name` 字段具有双重用途：它既是相等性键的一部分，也被 [is_new_error](is_new_error.md) 用于检测 PID 复用。如果传入的 `process_name` 与该 PID 的任何现有条目的 `process_name` 不匹配，则在插入新条目之前会清除整个子映射表。
- 所有字段都是私有的；实例仅由 [is_new_error](is_new_error.md) 内部构造。

## 要求

| | |
|---|---|
| **模块** | `src/logging.rs` |
| **调用方** | [is_new_error](is_new_error.md)（唯一构造器） |
| **被调用方** | 无 |
| **依赖项** | [Operation](Operation.md) |
| **权限** | 无 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [logging.rs](README.md) |
| 错误去重逻辑 | [is_new_error](is_new_error.md) |
| 陈旧条目清理 | [purge_fail_map](purge_fail_map.md) |
| Operation 枚举 | [Operation](Operation.md) |
| 去重结果的消费者 | [log_error_if_new](../apply.rs/log_error_if_new.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*