# logging 模块 (AffinityServiceRust)

`logging` 模块为 AffinityServiceRust 提供基于文件和基于控制台的日志记录功能。它管理每日轮转的日志文件、对进程发现日志条目进行去重，并跟踪每个 PID 的操作失败以抑制重复的错误消息。该模块公开全局静态状态用于日志配置（控制台模式、垃圾桶模式、时间缓冲区、文件句柄），并提供便捷宏以方便访问。

## 函数

| 函数 | 描述 |
|------|------|
| [is_new_error](is_new_error.md) | 按 PID/TID/进程名/操作/错误码元组跟踪操作失败；仅在首次出现时返回 `true`。 |
| [purge_fail_map](purge_fail_map.md) | 根据当前正在运行的进程列表，从失败跟踪映射中移除过时的条目。 |
| [get_log_path](get_log_path.md) | 在 `logs/` 目录下构建带日期戳的日志文件路径，支持可选后缀。 |
| [log_message](log_message.md) | 将带时间戳的消息写入主日志文件或标准输出，遵循垃圾桶模式设置。 |
| [log_pure_message](log_pure_message.md) | 将不带时间戳前缀的消息写入主日志文件或标准输出。 |
| [log_to_find](log_to_find.md) | 将带时间戳的消息写入 `.find` 日志文件或标准输出。 |
| [log_process_find](log_process_find.md) | 记录已发现的进程名称，通过 `FINDS_SET` 在每个会话中去重。 |

## 结构体 / 枚举

| 项目 | 描述 |
|------|------|
| [Operation](Operation.md) | 枚举由 `is_new_error` 跟踪失败的 Windows API 操作。 |
| [ApplyFailEntry](ApplyFailEntry.md) | 复合键结构体，表示唯一的失败事件（TID、进程名、操作、错误码）。 |

## 静态变量

| 静态变量 | 描述 |
|----------|------|
| [FINDS_SET](statics.md#finds_set) | `Lazy<Mutex<HashSet<String>>>` — 当前会话中已被 `log_process_find` 记录的进程名称集合。 |
| [USE_CONSOLE](statics.md#use_console) | `Lazy<Mutex<bool>>` — 当为 `true` 时，所有日志输出到标准输出而非文件。 |
| [DUST_BIN_MODE](statics.md#dust_bin_mode) | `Lazy<Mutex<bool>>` — 当为 `true` 时，`log_message` 静默丢弃输出。 |
| [LOCAL_TIME_BUFFER](statics.md#local_time_buffer) | `Lazy<Mutex<DateTime<Local>>>` — 用于时间戳格式化的缓存本地时间。 |
| [LOG_FILE](statics.md#log_file) | `Lazy<Mutex<File>>` — 主日志每日文件句柄（追加模式）。 |
| [FIND_LOG_FILE](statics.md#find_log_file) | `Lazy<Mutex<File>>` — `.find` 每日日志文件句柄（追加模式）。 |
| [FINDS_FAIL_SET](statics.md#finds_fail_set) | `Lazy<Mutex<HashSet<String>>>` — `-find` 模式访问检查失败的进程名称集合。 |
| [PID_MAP_FAIL_ENTRY_SET](statics.md#pid_map_fail_entry_set) | `Lazy<Mutex<HashMap<u32, HashMap<ApplyFailEntry, bool>>>>` — 带存活标志的每 PID 失败跟踪映射。 |

## 宏

| 宏 | 描述 |
|----|------|
| `log!` | `log_message` 的便捷包装器，接受 `format!` 风格的参数。 |
| `get_use_console!` | 锁定并返回 `USE_CONSOLE` 互斥锁守卫。 |
| `get_dust_bin_mod!` | 锁定并返回 `DUST_BIN_MODE` 互斥锁守卫。 |
| `get_local_time!` | 锁定并返回 `LOCAL_TIME_BUFFER` 互斥锁守卫。 |
| `get_logger!` | 锁定并返回 `LOG_FILE` 互斥锁守卫。 |
| `get_logger_find!` | 锁定并返回 `FIND_LOG_FILE` 互斥锁守卫。 |
| `get_fail_find_set!` | 锁定并返回 `FINDS_FAIL_SET` 互斥锁守卫。 |
| `get_pid_map_fail_entry_set!` | 锁定并返回 `PID_MAP_FAIL_ENTRY_SET` 互斥锁守卫。 |

## 另请参阅

| 链接 | 描述 |
|------|------|
| [collections 模块](../collections.rs/README.md) | 本模块使用的自定义 `HashMap` 和 `HashSet` 类型别名。 |
| [error_codes 模块](../error_codes.rs/README.md) | 与日志记录配合使用的 Win32/NTSTATUS 错误码翻译。 |
| [winapi 模块](../winapi.rs/README.md) | 调用日志记录进行错误报告的 Windows API 封装。 |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
