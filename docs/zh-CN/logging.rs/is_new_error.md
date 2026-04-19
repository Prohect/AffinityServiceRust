# is_new_error 函数 (logging.rs)

确定对于给定进程是否首次看到特定操作失败，使调用方能够仅记录新颖错误并抑制重复的相同失败。

## 语法

```rust
pub fn is_new_error(
    pid: u32,
    tid: u32,
    process_name: &str,
    operation: Operation,
    error_code: u32,
) -> bool
```

## 参数

`pid: u32`

经历失败的进程标识符。用作全局 `PID_MAP_FAIL_ENTRY_SET` 映射中的顶层键。

`tid: u32`

与失败关联的线程标识符。对于不涉及特定线程的进程级操作，调用方通常传递 `0`。

`process_name: &str`

进程的可执行文件名（例如 `"explorer.exe"`）。既用作去重键的一部分，也用于检测 PID 重用 — 如果 PID 的现有条目有不同的进程名称，则在插入新条目之前清除该 PID 的整个条目集。

`operation: Operation`

[Operation](Operation.md) 变体，标识哪个 Windows API 调用失败。

`error_code: u32`

失败 API 调用返回的 Win32 或 NTSTATUS 错误代码。如果上下文中没有可用错误代码，调用方传递 `0` 或自定义区分符以区分不同的失败模式。

## 返回值

`bool` — 如果这是第一次记录确切的 `(pid, tid, process_name, operation, error_code)` 组合，则返回 `true`，意味着调用方应该记录它。如果失败已经被跟踪，则返回 `false`，意味着调用方应该抑制日志。

## 备注

### 去重策略

该函数通过全局静态 `PID_MAP_FAIL_ENTRY_SET` 维护两级映射：

```
HashMap<u32, HashMap<ApplyFailEntry, bool>>
  ^pid         ^(tid, process_name, operation, error_code) -> 存活标志
```

每个 [ApplyFailEntry](ApplyFailEntry.md) 通过所有四个字段（`tid`、`process_name`、`operation`、`error_code`）进行相等比较。`bool` 值是由 [purge_fail_map](purge_fail_map.md) 在垃圾回收期间使用的"存活"标志。

### PID 重用检测

**不变量：** 期望 PID 的失败条目集中的所有条目共享相同的 `process_name`。当函数发现 PID 的现有条目的进程名称与提供的不同时，它在插入新条目之前清除该 PID 的整个条目集。这处理了 Windows PID 重用场景，其中终止进程的 PID 被重新分配给新的无关进程。

### 存活标志管理

当找到现有条目（重复）时，其存活标志设置为 `true`。这确保条目在下一个 [purge_fail_map](purge_fail_map.md) 周期中存活，该周期将所有条目标记为死，然后重新标记活动的条目。

### 线程安全

该函数通过 `get_pid_map_fail_entry_set!()` 宏获取全局 `PID_MAP_FAIL_ENTRY_SET` 互斥锁。锁在查找和潜在插入期间保持。

## 要求

| | |
|---|---|
| **模块** | `src/logging.rs` |
| **调用方** | [log_error_if_new](../apply.rs/log_error_if_new.md)，[apply.rs](../apply.rs/README.md) 中的应用函数 |
| **被调用方** | `get_pid_map_fail_entry_set!()` 宏 |
| **Win32 API** | 无 |
| **权限** | 无 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [logging.rs](README.md) |
| 失败键结构体 | [ApplyFailEntry](ApplyFailEntry.md) |
| 操作枚举 | [Operation](Operation.md) |
| 陈旧条目的垃圾回收 | [purge_fail_map](purge_fail_map.md) |
| 调用方侧错误日志记录辅助函数 | [log_error_if_new](../apply.rs/log_error_if_new.md) |

*记录于提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*