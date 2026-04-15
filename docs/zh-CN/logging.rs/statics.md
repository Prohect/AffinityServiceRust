# 日志模块静态变量 (logging.rs)

本页面记录了 `logging` 模块中定义的所有全局静态变量。这些静态变量管理日志文件句柄、控制台模式标志、时间缓冲区和失败跟踪数据结构。每个静态变量通过 `once_cell::sync::Lazy` 进行延迟初始化，并由 `std::sync::Mutex` 保护以实现线程安全访问。

> **注意：** 每个静态变量都有一个对应的便捷宏（例如 `get_logger!()`）来锁定互斥量并返回守卫。为了一致性，建议优先使用宏而非直接调用 `.lock().unwrap()`。

---

## FINDS_SET

跟踪当前会话中已被 [`log_process_find`](log_process_find.md) 记录的进程名称，防止对同一进程名称产生重复的日志条目。

### 语法

```rust
pub static FINDS_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::default()));
```

### 类型

`once_cell::sync::Lazy<std::sync::Mutex<HashSet<String>>>`

### 备注

- 由 [`log_process_find`](log_process_find.md) 填充：如果 `FINDS_SET.lock().unwrap().insert(process_name)` 返回 `true`，则该进程名称在本次会话中尚未出现过，会被记录到日志。如果返回 `false`，则该条目已存在，跳过日志记录。
- 该集合在运行时不会被显式清除。它在进程重新启动时自然重置。
- 此 `HashSet` 是项目中 [`HashSet`](../collections.rs/HashSet.md) 的类型别名（`FxHashSet`），使用快速的非加密哈希。

---

## USE_CONSOLE

控制日志输出是写入 stdout（控制台模式）还是写入日志文件（文件模式）。

### 语法

```rust
pub static USE_CONSOLE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));
```

### 类型

`once_cell::sync::Lazy<std::sync::Mutex<bool>>`

### 备注

- 默认值：`false`（文件模式）。
- 当应用程序使用 `console` CLI 标志启动时设置为 `true`。
- 当为 `true` 时，[`log_message`](log_message.md)、[`log_pure_message`](log_pure_message.md) 和 [`log_to_find`](log_to_find.md) 都会写入 `stdout` 而非各自的日志文件。
- 通过 `get_use_console!()` 宏访问。

---

## DUST_BIN_MODE

启用时，使 [`log_message`](log_message.md) 静默丢弃所有输出。用于启动阶段有意抑制日志记录的场景（例如权限提升之前）。

### 语法

```rust
pub static DUST_BIN_MODE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));
```

### 类型

`once_cell::sync::Lazy<std::sync::Mutex<bool>>`

### 备注

- 默认值：`false`（日志记录已启用）。
- 当为 `true` 时，`log_message` 立即返回，不写入任何内容。
- **不影响** [`log_pure_message`](log_pure_message.md) 或 [`log_to_find`](log_to_find.md) — 仅 `log_message` 检查此标志。
- 通过 `get_dust_bin_mod!()` 宏访问。

---

## LOCAL_TIME_BUFFER

所有日志函数用于时间戳格式化的缓存 `DateTime<Local>` 值。调用者负责在每个调度周期开始时刷新此值（通过赋值 `Local::now()`），以便周期内的日志时间戳共享一致的时间，无需重复查询系统时钟。

### 语法

```rust
pub static LOCAL_TIME_BUFFER: Lazy<Mutex<DateTime<Local>>> = Lazy::new(|| Mutex::new(Local::now()));
```

### 类型

`once_cell::sync::Lazy<std::sync::Mutex<chrono::DateTime<chrono::Local>>>`

### 备注

- 首次访问时初始化为 `Local::now()`。
- 日志函数通过 `time.format("%H:%M:%S")` 将时间戳格式化为 `HH:MM:SS`。
- 该缓冲区也被 [`get_log_path`](get_log_path.md) 用于确定当前日期，以实现每日日志文件轮换（`YYYYMMDD.log`）。
- 通过 `get_local_time!()` 宏访问。

---

## LOG_FILE

主每日日志文件的文件句柄。首次访问时以追加-创建模式打开，目标路径类似 `logs/YYYYMMDD.log`。

### 语法

```rust
pub static LOG_FILE: Lazy<Mutex<File>> =
    Lazy::new(|| Mutex::new(
        OpenOptions::new().append(true).create(true).open(get_log_path("")).unwrap()
    ));
```

### 类型

`once_cell::sync::Lazy<std::sync::Mutex<std::fs::File>>`

### 备注

- 文件路径在初始化时由 [`get_log_path("")`](get_log_path.md) 计算，使用 [`LOCAL_TIME_BUFFER`](#local_time_buffer) 中的日期。
- 使用 `append(true).create(true)` 打开，意味着文件不存在时会被创建，所有写入都追加到文件末尾。
- 当 `USE_CONSOLE` 为 `false` 时，由 [`log_message`](log_message.md) 和 [`log_pure_message`](log_pure_message.md) 使用。
- 通过 `get_logger!()` 宏访问。
- `logs/` 目录由 `get_log_path` 在不存在时创建。

---

## FIND_LOG_FILE

`.find` 每日日志文件的文件句柄。首次访问时以追加-创建模式打开，目标路径类似 `logs/YYYYMMDD.find.log`。

### 语法

```rust
pub static FIND_LOG_FILE: Lazy<Mutex<File>> =
    Lazy::new(|| Mutex::new(
        OpenOptions::new().append(true).create(true).open(get_log_path(".find")).unwrap()
    ));
```

### 类型

`once_cell::sync::Lazy<std::sync::Mutex<std::fs::File>>`

### 备注

- 文件路径由 [`get_log_path(".find")`](get_log_path.md) 计算，生成类似 `logs/YYYYMMDD.find.log` 的路径。
- 当 `USE_CONSOLE` 为 `false` 时，由 [`log_to_find`](log_to_find.md) 和间接由 [`log_process_find`](log_process_find.md) 使用。
- 通过 `get_logger_find!()` 宏访问。
- 与主日志文件分离，以便独立查看进程发现输出。

---

## FINDS_FAIL_SET

跟踪在 `-find` 模式期间访问检查失败的进程名称（特别是 `ACCESS_DENIED` 错误）。这些进程在后续的 find 模式迭代中被排除，以避免重复的失败尝试和日志噪声。

### 语法

```rust
pub static FINDS_FAIL_SET: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::default()));
```

### 类型

`once_cell::sync::Lazy<std::sync::Mutex<HashSet<String>>>`

### 备注

- 当 `OpenProcess` 或 `GetProcessAffinityMask` 返回错误代码 `5`（`ACCESS_DENIED`）时，由 [`is_affinity_unset`](../winapi.rs/is_affinity_unset.md) 填充。
- 被调度器中的 find 模式逻辑查阅，以跳过已知无法访问的进程（例如反作弊服务、受保护进程）。
- 此 `HashSet` 是项目中 [`HashSet`](../collections.rs/HashSet.md) 的类型别名（`FxHashSet`）。
- 通过 `get_fail_find_set!()` 宏访问。
- 运行时不会被显式清除。进程重启时重置。

---

## PID_MAP_FAIL_ENTRY_SET

按 PID 跟踪失败操作的映射，记录给定进程中哪些操作已经失败，防止冗余的错误日志消息。每个 PID 映射到一个以 [`ApplyFailEntry`](ApplyFailEntry.md) 为键、`bool` 存活标志为值的二级 `HashMap`。

### 语法

```rust
pub static PID_MAP_FAIL_ENTRY_SET: Lazy<Mutex<HashMap<u32, HashMap<ApplyFailEntry, bool>>>> =
    Lazy::new(|| Mutex::new(HashMap::default()));
```

### 类型

`once_cell::sync::Lazy<std::sync::Mutex<HashMap<u32, HashMap<ApplyFailEntry, bool>>>>`

### 备注

- **结构**：外层映射以 PID（`u32`）为键。每个值是一个以 [`ApplyFailEntry`](ApplyFailEntry.md)（由 TID、进程名称、操作和错误代码组成的复合键）为键、`bool` 值指示该条目是否"存活"（仍与运行中的进程相关）的内层映射。

- **填充者**：[`is_new_error`](is_new_error.md)，它插入新的失败条目，并仅在给定失败元组首次出现时返回 `true`。如果条目已存在，则将其标记为存活并返回 `false`。

- **清除者**：[`purge_fail_map`](purge_fail_map.md)，它实现标记-清除算法：
  1. 将所有条目标记为死亡（`alive = false`）。
  2. 将其 PID 和进程名称出现在当前运行进程列表中的条目重新标记为存活（`alive = true`）。
  3. 移除所有条目均已死亡的映射条目，防止无限增长。

- **进程名称一致性**：如果新条目的进程名称与同一 PID 的现有条目的进程名称不匹配，内层映射将被清除。这处理了 PID 重用的情况——当新进程获得与已终止进程相同的 PID 时，过时的失败条目会被丢弃。

- 外层和内层的 `HashMap` 类型均为项目中 [`HashMap`](../collections.rs/HashMap.md) 的类型别名（`FxHashMap`）。

- 通过 `get_pid_map_fail_entry_set!()` 宏访问。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `logging.rs` |
| **依赖** | `once_cell::sync::Lazy`、`std::sync::Mutex`、`std::fs::File`、`chrono::{DateTime, Local}`、[`HashMap`](../collections.rs/HashMap.md)、[`HashSet`](../collections.rs/HashSet.md)、[`ApplyFailEntry`](ApplyFailEntry.md) |
| **平台** | Windows（文件路径使用 Windows 约定） |
| **初始化方式** | 首次访问时延迟初始化 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| is_new_error 函数 | [is_new_error](is_new_error.md) |
| purge_fail_map 函数 | [purge_fail_map](purge_fail_map.md) |
| log_message 函数 | [log_message](log_message.md) |
| log_to_find 函数 | [log_to_find](log_to_find.md) |
| log_process_find 函数 | [log_process_find](log_process_find.md) |
| get_log_path 函数 | [get_log_path](get_log_path.md) |
| ApplyFailEntry 结构体 | [ApplyFailEntry](ApplyFailEntry.md) |
| Operation 枚举 | [Operation](Operation.md) |
| collections 模块 | [collections.rs](../collections.rs/README.md) |
| logging 模块概述 | [README](README.md) |

---
> Commit SHA: `7221ea0694670265d4eb4975582d8ed2ae02439d`
