# ApplyFailEntry 结构体 (logging.rs)

错误去重映射的复合键结构体。将进程 ID、线程 ID、进程名称、操作类型和错误代码组合为一个唯一键，用于在 [`PID_MAP_FAIL_ENTRY_SET`](PID_MAP_FAIL_ENTRY_SET.md) 中跟踪已记录的错误，防止同一错误被反复记录。

## 语法

```rust
struct ApplyFailEntry {
    pid: u32,
    tid: u32,
    process_name: String,
    operation: Operation,
    error_code: u32,
}
```

## 成员

| 字段 | 类型 | 说明 |
| --- | --- | --- |
| `pid` | `u32` | 产生错误的进程 ID。 |
| `tid` | `u32` | 产生错误的线程 ID。启用每线程级别的去重，使同一进程中不同线程的相同操作错误能够分别记录。 |
| `process_name` | `String` | 进程的可执行文件名称（例如 `"game.exe"`）。用于检测 PID 重用——当同一 PID 对应的进程名称发生变化时，表示原进程已退出且 PID 已被新进程重用。 |
| `operation` | [`Operation`](Operation.md) | 产生错误的 Windows API 操作类型。 |
| `error_code` | `u32` | Windows API 返回的错误代码。 |

## 备注

`ApplyFailEntry` 作为 [`PID_MAP_FAIL_ENTRY_SET`](PID_MAP_FAIL_ENTRY_SET.md) 两级映射中内层 `HashMap` 的键。外层映射按 `pid` 索引，内层映射按完整的 `ApplyFailEntry` 索引，值为 `bool` 类型的存活标志。

### 相等性判断

所有五个字段（`pid`、`tid`、`process_name`、`operation`、`error_code`）都参与相等性比较和哈希计算。只有当两个 `ApplyFailEntry` 的所有字段完全匹配时，才被视为相等。这确保了：

- 同一进程中不同线程的相同操作错误被分别跟踪。
- 同一线程的不同操作错误被分别跟踪。
- 同一操作产生不同错误代码时被分别跟踪。

### PID 重用检测

`process_name` 字段在去重之外还承担另一个重要职责：当 [`is_new_error`](is_new_error.md) 发现某个 PID 对应的已记录进程名称与当前进程名称不同时，会清除该 PID 下的所有旧条目。这是因为 Windows 可能将已退出进程的 PID 分配给新进程，旧进程的错误记录不应阻止新进程的错误被记录。

### 派生特征

该结构体派生了 `Hash` 和 `Eq`（以及 `PartialEq`）特征，以满足作为 `HashMap` 键的要求。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/logging.rs |
| **源码行** | L97–L103 |
| **使用方** | [`is_new_error`](is_new_error.md)、[`purge_fail_map`](purge_fail_map.md) |
| **存储于** | [`PID_MAP_FAIL_ENTRY_SET`](PID_MAP_FAIL_ENTRY_SET.md) |

## 另请参阅

- [Operation 枚举](Operation.md)
- [PID_MAP_FAIL_ENTRY_SET 静态变量](PID_MAP_FAIL_ENTRY_SET.md)
- [is_new_error 函数](is_new_error.md)
- [purge_fail_map 函数](purge_fail_map.md)
- [logging.rs 模块概述](README.md)