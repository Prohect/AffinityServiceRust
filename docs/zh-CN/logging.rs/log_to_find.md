# log_to_find 函数 (logging.rs)

将带时间戳的消息写入 `.find` 日志文件或 stdout，具体取决于当前的控制台模式设置。此函数是 `-find` 模式输出和与进程发现及亲和性检查相关的诊断消息的专用日志通道。

## 语法

```rust
pub fn log_to_find(msg: &str)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `msg` | `&str` | 要写入的消息字符串。会自动添加 `[HH:MM:SS]` 格式的时间戳前缀。 |

## 返回值

此函数不返回值。

## 备注

该函数的操作流程如下：

1. 获取 `LOCAL_TIME_BUFFER` 互斥锁（通过 `get_local_time!()`），并将当前时间格式化为 `HH:MM:SS`。
2. 检查 `USE_CONSOLE` 标志（通过 `get_use_console!()`）：
   - 如果为 `true`，使用 `writeln!` 将带时间戳的消息写入 **stdout**。
   - 如果为 `false`，通过 `get_logger_find!()` 宏使用 `writeln!` 将带时间戳的消息写入 **find 日志文件**（`FIND_LOG_FILE`）。
3. 写入错误会被静默忽略（`let _ = writeln!(...)`）。

### 与相关日志函数的区别

| 函数 | 日志目标 | 时间戳 | 垃圾桶模式检查 |
|------|---------|--------|---------------|
| [`log_message`](log_message.md) | 主日志文件（`LOG_FILE`）或 stdout | 有（`[HH:MM:SS]`） | 有 — 启用时会抑制输出 |
| [`log_pure_message`](log_pure_message.md) | 主日志文件（`LOG_FILE`）或 stdout | 无 | 无 |
| **`log_to_find`** | Find 日志文件（`FIND_LOG_FILE`）或 stdout | 有（`[HH:MM:SS]`） | **无** — 无论垃圾桶模式如何始终写入 |

与 [`log_message`](log_message.md) 不同，此函数**不**检查 `DUST_BIN_MODE` 标志。Find 模式的诊断信息始终会被写入，即使主日志输出在垃圾桶模式下被抑制。这确保了进程发现结果和访问被拒绝的诊断信息永远不会被静默丢弃。

### 输出格式

```text
[14:32:07]find chrome.exe
[14:32:07]is_affinity_unset: [OPEN][ACCESS_DENIED]  1234-svchost.exe
```

### 日志文件

Find 日志文件是一个带日期戳且带有 `.find` 后缀的文件，创建在 `logs/` 目录下。其路径由 [`get_log_path`](get_log_path.md) 以 `".find"` 作为后缀参数确定（例如 `logs/20250101.find.log`）。文件句柄存储在 [`FIND_LOG_FILE`](statics.md#find_log_file) 静态变量中，在首次访问时以追加模式打开。

### 线程安全

该函数每次调用最多获取两个互斥锁（`LOCAL_TIME_BUFFER` 以及 `USE_CONSOLE` + stdout 或 `FIND_LOG_FILE`）。锁的获取顺序在所有日志函数中保持一致，防止死锁。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `logging.rs` |
| **调用方** | [`is_affinity_unset`](../winapi.rs/is_affinity_unset.md)、[`get_process_handle`](../winapi.rs/get_process_handle.md)、[`get_thread_handle`](../winapi.rs/get_thread_handle.md)、[`log_process_find`](log_process_find.md)、`CPU_SET_INFORMATION` 初始化器，以及 `winapi.rs` 和 `apply.rs` 中的其他 find 模式诊断 |
| **被调用方** | `get_local_time!()`、`get_use_console!()`、`get_logger_find!()`、`std::io::Write::write_fmt`（通过 `writeln!`） |
| **静态变量** | [`LOCAL_TIME_BUFFER`](statics.md#local_time_buffer)、[`USE_CONSOLE`](statics.md#use_console)、[`FIND_LOG_FILE`](statics.md#find_log_file) |
| **平台** | Windows（日志基础设施），但函数本身不包含平台特定代码 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| log_message 函数 | [log_message](log_message.md) |
| log_pure_message 函数 | [log_pure_message](log_pure_message.md) |
| log_process_find 函数 | [log_process_find](log_process_find.md) |
| get_log_path 函数 | [get_log_path](get_log_path.md) |
| 日志静态变量 | [statics](statics.md) |
| logging 模块概述 | [README](README.md) |

---
*提交：[37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
