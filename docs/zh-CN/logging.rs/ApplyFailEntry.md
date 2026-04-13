# ApplyFailEntry 结构体 (logging.rs)

用于在日志子系统中对 Windows API 操作失败进行去重的复合键。每个实例通过组合线程 ID、进程名称、操作类型和错误代码来唯一标识一个特定的失败场景。实例作为键存储在 [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) 的每 PID 内部映射中，当 [is_new_error](is_new_error.md) 检查某个失败是否已被记录时，会进行相等性比较。

## 语法

```logging.rs
#[derive(PartialEq, Eq, Hash)]
pub struct ApplyFailEntry {
    tid: u32,
    process_name: String,
    operation: Operation,
    error_code: u32,
}
```

## 成员

| 字段 | 类型 | 描述 |
|------|------|------|
| `tid` | `u32` | 与失败操作关联的 Windows 线程标识符。对于进程级操作（例如 `SetPriorityClass`），通常为 `0` 或主线程 ID。对于线程级操作（例如 `SetThreadPriority`），则为导致失败的具体线程 ID。 |
| `process_name` | `String` | 目标进程的小写可执行文件名称（例如 `"chrome.exe"`）。既用于日志消息的显示，也作为去重键的一部分。同时作为不变量检查——同一 PID 内部映射中的所有条目预期具有相同的 `process_name`。 |
| `operation` | [Operation](Operation.md) | 标识哪个 Windows API 调用失败的 [Operation](Operation.md) 枚举变体（例如 `SetPriorityClass`、`OpenProcess2processSetInformation`）。 |
| `error_code` | `u32` | 失败操作返回的 Win32 错误代码或 NTSTATUS 值。当 API 调用上下文中没有特定的错误代码可用时，使用 `0`；或者作为自定义哨兵值，用于区分同一操作内的不同失败模式。 |

## 备注

- 该结构体派生了 `PartialEq`、`Eq` 和 `Hash`，这是作为 `HashMap<ApplyFailEntry, bool>` 中键使用所必需的。当且仅当所有四个字段完全匹配时，两个条目才被认为相等。这意味着同一线程上的同一操作以不同的错误代码失败会被视为不同的失败，并将被分别记录。
- 该结构体的字段**不是** `pub` 的——它们是模块私有的。构造在 [is_new_error](is_new_error.md) 内部完成，该函数是唯一创建 `ApplyFailEntry` 实例的函数。
- `process_name` 字段具有双重用途：它是去重键的一部分，同时也充当 PID 重用检测机制。当 [is_new_error](is_new_error.md) 遇到某个 PID 的现有内部映射中的条目具有与新条目不同的 `process_name` 时，它会在插入之前清空整个内部映射。这防止了已终止进程的过期去重状态抑制新进程（继承了相同 PID）的错误。
- `tid` 字段允许线程级操作按线程独立去重。例如，如果进程 `foo.exe` 的线程 1234 上 `SetThreadPriority` 以 `ACCESS_DENIED` 失败，该失败与同一进程的线程 5678 上的相同错误分开跟踪。这确保了每个线程的首次失败都会被记录，提供完整的诊断覆盖。
- `ApplyFailEntry` 未实现 `Debug` 或 `Clone`。它被创建、插入映射并进行比较——不需要其他操作。

### 去重流程

1. [apply 模块](../apply.rs/README.md) 遇到一个 Win32 API 失败。
2. 它调用 [is_new_error](is_new_error.md)，传入 `pid`、`tid`、`process_name`、`operation` 和 `error_code`。
3. `is_new_error` 从后四个参数构造一个 `ApplyFailEntry`，并在给定 `pid` 的内部映射中查找它。
4. 如果未找到该条目，则插入并标记 `alive = true`，函数返回 `true` —— 调用者记录该错误。
5. 如果条目已存在，函数将其标记为存活并返回 `false` —— 调用者抑制重复的日志消息。
6. [purge_fail_map](purge_fail_map.md) 定期移除不再运行的进程的条目。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `logging` |
| Trait 实现 | `PartialEq`、`Eq`、`Hash`（派生） |
| 构造方 | [is_new_error](is_new_error.md) |
| 存储位置 | [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) |
| 比较方 | [is_new_error](is_new_error.md)、[purge_fail_map](purge_fail_map.md) |

## 另请参阅

| 主题 | 链接 |
|------|------|
| Windows API 操作标识符 | [Operation](Operation.md) |
| 错误去重逻辑 | [is_new_error](is_new_error.md) |
| 过期条目清理 | [purge_fail_map](purge_fail_map.md) |
| 全局失败跟踪映射 | [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) |
| Win32 错误代码翻译 | [error_from_code_win32](../error_codes.rs/error_from_code_win32.md) |
| NTSTATUS 代码翻译 | [error_from_ntstatus](../error_codes.rs/error_from_ntstatus.md) |
| logging 模块概述 | [logging 模块](README.md) |