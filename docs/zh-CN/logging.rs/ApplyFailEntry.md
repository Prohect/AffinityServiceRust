# ApplyFailEntry 结构体 (logging.rs)

复合键结构体，表示一个唯一的操作失败事件。每个 `ApplyFailEntry` 标识线程 ID、进程名称、Windows API 操作和错误代码的特定组合。它用作每个 PID 的失败跟踪映射（[`PID_MAP_FAIL_ENTRY_SET`](statics.md#pid_map_fail_entry_set)）中的键，以对错误日志消息进行去重——确保对同一线程上相同操作的重复失败仅记录一次。

## 语法

```rust
#[derive(PartialEq, Eq, Hash)]
pub struct ApplyFailEntry {
    tid: u32,
    process_name: String,
    error_code: u32,
    operation: Operation,
}
```

## 成员

| 字段 | 类型 | 可见性 | 描述 |
|------|------|--------|------|
| `tid` | `u32` | 私有 | 与失败关联的线程标识符。对于进程级操作（不涉及特定线程），通常为 `0`。 |
| `process_name` | `String` | 私有 | 尝试执行操作的进程名称。用于检测 PID 被不同进程重用的情况（参见备注）。 |
| `operation` | [`Operation`](Operation.md) | 私有 | 失败的 Windows API 操作（例如 `OpenProcess2processQueryLimitedInformation`、`SetPriorityClass`、`SetThreadIdealProcessorEx`）。 |
| `error_code` | `u32` | 私有 | 失败操作返回的 Win32 错误代码。当没有上下文错误代码可用时，将其设置为 `0` 或自定义区分值以区分不同的失败模式。 |

## 备注

### 派生特征

该结构体派生了 `PartialEq`、`Eq` 和 `Hash`，这些是在 `HashMap` 中用作键以及在 [`is_new_error`](is_new_error.md) 去重逻辑中进行相等性比较所必需的。当**所有四个字段**都匹配时，两个 `ApplyFailEntry` 实例被视为相等。

### PID 重用检测

`process_name` 字段具有双重用途：

1. **键标识** — 它是用于去重的复合键的一部分。
2. **PID 重用守卫** — 在 [`is_new_error`](is_new_error.md) 中，当给定 PID 的失败条目集包含的 `process_name` 与传入条目的 `process_name` 不同时，整个集合会在插入新条目之前被清空。这处理了操作系统将 PID 回收给新进程的情况——旧进程的过时失败条目被丢弃，以便新进程的失败能够被正确记录。

### 存活标志

在 `PID_MAP_FAIL_ENTRY_SET` 映射中，每个 `ApplyFailEntry` 与一个 `bool` 存活标志配对（`HashMap<ApplyFailEntry, bool>`）。此标志由 [`purge_fail_map`](purge_fail_map.md) 用于实现标记-清除垃圾回收：

- 每个清除周期开始时，所有条目被标记为已失效（`false`）。
- 匹配当前正在运行的进程的条目被重新标记为存活（`true`）。
- 已失效的条目（已退出的进程）从映射中移除。

### 字段可见性

所有字段均为**私有**（无 `pub` 修饰符）。`ApplyFailEntry` 实例仅在 `logging` 模块内部创建和检查——具体在 [`is_new_error`](is_new_error.md) 和 [`purge_fail_map`](purge_fail_map.md) 函数中。外部调用者仅通过这些函数与失败跟踪系统交互。

### 典型构造方式

```rust
let entry = ApplyFailEntry {
    tid,
    process_name: process_name.to_string(),
    operation,
    error_code,
};
```

`process_name` 被克隆为拥有所有权的 `String`，因为该条目必须比传递给 `is_new_error` 的借用 `&str` 存活更长时间。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `logging.rs` |
| **创建者** | [`is_new_error`](is_new_error.md) |
| **存储于** | [`PID_MAP_FAIL_ENTRY_SET`](statics.md#pid_map_fail_entry_set) |
| **依赖** | [`Operation`](Operation.md) 枚举 |
| **平台** | 平台无关的结构体；用于 Windows API 错误跟踪的上下文中 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| Operation 枚举 | [Operation](Operation.md) |
| is_new_error 函数 | [is_new_error](is_new_error.md) |
| purge_fail_map 函数 | [purge_fail_map](purge_fail_map.md) |
| PID_MAP_FAIL_ENTRY_SET 静态变量 | [statics](statics.md#pid_map_fail_entry_set) |
| logging 模块概述 | [README](README.md) |

---
> Commit SHA: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
