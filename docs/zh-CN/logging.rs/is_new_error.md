# is_new_error 函数 (logging.rs)

检查特定的 Windows API 操作失败是否已经为给定进程记录过，如果没有，则将其注册以便将来去重。当错误组合首次出现时返回 `true`，通知调用方应发出日志消息。在后续遇到相同失败时返回 `false`，抑制重复的日志输出。

## 语法

```logging.rs
pub fn is_new_error(pid: u32, tid: u32, process_name: &str, operation: Operation, error_code: u32) -> bool
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `pid` | `u32` | 发生失败的目标进程的进程标识符。用作 [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) 的外层键。 |
| `tid` | `u32` | 与失败关联的线程标识符。包含在 [ApplyFailEntry](ApplyFailEntry.md) 键中，使同一进程的不同线程上相同操作的失败可以被独立跟踪。 |
| `process_name` | `&str` | 目标进程的可执行文件名称（例如 `"notepad.exe"`）。既作为去重键的一部分，也用于 PID 重用一致性检查。 |
| `operation` | `Operation` | 标识哪个 Windows API 调用失败的 [Operation](Operation.md) 变体（例如 `Operation::SetPriorityClass`、`Operation::OpenProcess2processSetLimitedInformation`）。 |
| `error_code` | `u32` | 失败的 API 调用返回的 Win32 或 NTSTATUS 错误代码。如果没有可用的上下文错误代码，传入 `0` 或自定义的判别值。 |

## 返回值

| 值 | 含义 |
|------|------|
| `true` | 这是该 `(pid, tid, process_name, operation, error_code)` 精确组合的**首次**出现。调用方应记录该错误。 |
| `false` | 此 PID 已存在相同的失败条目。调用方应抑制重复的日志消息。 |

## 备注

### 去重算法

1. 该函数从提供的参数构造一个 [ApplyFailEntry](ApplyFailEntry.md)。
2. 通过 [get_pid_map_fail_entry_set!](get_pid_map_fail_entry_set.md) 宏锁定全局 [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md)。
3. **如果 `pid` 存在内层映射：**
   - 在内层 `HashMap<ApplyFailEntry, bool>` 中搜索匹配条目。
   - 如果找到，将条目标记为存活（`true`）并返回 `false`（重复）。
   - 如果未找到，执行 **PID 重用检查**（见下文）并插入新条目，标记为存活 = `true`，然后返回 `true`（新错误）。
4. **如果 `pid` 不存在内层映射：**
   - 创建一个包含该单个条目的新内层映射，并插入到外层映射中，然后返回 `true`。

### PID 重用安全性（不变量 A）

同一 PID 的内层映射中所有条目应共享相同的 `process_name`。在插入新条目时，函数会检查现有条目是否属于不同的进程名称。如果检测到不匹配——表明操作系统已将该 PID 重用给了一个新进程——内层映射将在插入新条目之前被**清空**。这防止了过时的去重状态抑制新进程的合法首次错误。

### 存活标志

每个条目的 `bool` 值跟踪 [purge_fail_map](purge_fail_map.md) 垃圾回收周期的存活状态：

- 当 `is_new_error` 遇到已存在的条目时，它将 `bool` 设置为 `true`，表示该进程在本次轮询迭代中仍然活跃。
- [purge_fail_map](purge_fail_map.md) 定期将所有标志重置为 `false`，然后重新标记当前运行进程的条目。在此遍历后仍为 `false` 的条目将被移除。

### 错误代码语义

当没有可用的上下文 Win32 错误代码时（例如，API 返回了布尔失败但没有调用 `GetLastError`），调用方应为 `error_code` 传入 `0`。如果调用方需要区分同一操作的多种逻辑失败模式，可以使用自定义的非零哨兵值作为 `error_code` 来维护独立的去重条目。

### 线程安全

该函数在整个查找和插入操作期间获取 [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) 互斥锁，确保原子性。在实际使用中，此函数仅从单线程服务循环中调用，因此不会发生竞争。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `logging` |
| 调用方 | [log_error_if_new](../apply.rs/log_error_if_new.md) |
| 被调用方 | [get_pid_map_fail_entry_set!](get_pid_map_fail_entry_set.md)（宏） |
| 数据结构 | [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md)、[ApplyFailEntry](ApplyFailEntry.md)、[Operation](Operation.md) |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 过期条目清理 | [purge_fail_map](purge_fail_map.md) |
| 失败条目复合键 | [ApplyFailEntry](ApplyFailEntry.md) |
| 操作标识符枚举 | [Operation](Operation.md) |
| 全局失败跟踪映射 | [PID_MAP_FAIL_ENTRY_SET](PID_MAP_FAIL_ENTRY_SET.md) |
| 日志消息的错误格式化 | [error_from_code_win32](../error_codes.rs/error_from_code_win32.md) |
| apply 模块错误日志辅助函数 | [log_error_if_new](../apply.rs/log_error_if_new.md) |
| logging 模块概述 | [logging 模块](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd