# log_message 函数 (logging.rs)

向主日志文件或标准输出写入带时间戳的日志消息，具体取决于当前的控制台模式设置。如果垃圾箱模式处于活动状态，消息将被静默丢弃。此函数是主要的日志入口点，通常通过 `log!` 宏间接调用。

## 语法

```rust
pub fn log_message(args: &str)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `args` | `&str` | 要记录的消息字符串。通过 `log!` 宏调用时，通常是 `format!(...)` 的输出。 |

## 返回值

此函数不返回值。

## 备注

### 行为

1. **垃圾箱模式检查。** 函数首先通过 `get_dust_bin_mod!()` 获取 `DUST_BIN_MODE` 互斥锁。如果值为 `true`，函数立即返回，不写入任何内容。此模式用于在某些操作阶段抑制所有日志输出。

2. **时间戳格式化。** 通过 `get_local_time!()` 从 `LOCAL_TIME_BUFFER` 静态变量读取当前时间，并使用 `chrono` 的 `format` 方法格式化为 `HH:MM:SS`。

3. **输出路由。** 函数检查 `USE_CONSOLE` 静态变量（通过 `get_use_console!()`）：
   - 如果为 `true`，消息通过 `writeln!(stdout(), "[{}]{}", time_prefix, args)` 写入**标准输出**。
   - 如果为 `false`，消息通过锁定 `LOG_FILE` 静态变量（通过 `get_logger!()`）并调用 `writeln!` 写入**主日志文件**。

4. **错误抑制。** 写入错误通过 `let _ = writeln!(...)` 被静默忽略。这防止日志失败传播并导致应用程序崩溃。

### 输出格式

```text
[HH:MM:SS]<消息>
```

例如，如果 `args` 是 `" Starting scheduler loop"`，输出将为：

```text
[14:32:07] Starting scheduler loop
```

注意时间戳右括号 `]` 和消息开始之间**没有空格**。如果需要间距，调用者必须在 `args` 字符串中包含它。

### 与 `log!` 宏的关系

`log!` 宏是调用此函数的首选方式：

```rust
log!("Processing {} threads for PID {}", thread_count, pid);
```

这将展开为：

```rust
crate::logging::log_message(format!("Processing {} threads for PID {}", thread_count, pid).as_str())
```

宏处理 `format!` 风格的参数插值，并将结果 `String` 作为 `&str` 传递给 `log_message`。

### 加锁顺序

此函数按以下顺序获取最多三个互斥锁：

1. `DUST_BIN_MODE` — 首先检查，用于提前退出。
2. `LOCAL_TIME_BUFFER` — 读取用于时间戳格式化。
3. `USE_CONSOLE`，然后是标准输出或 `LOG_FILE` — 用于输出路由。

每个锁独立获取和释放（不会同时持有），因此死锁风险很小。但是，与其他日志函数（[`log_pure_message`](log_pure_message.md)、[`log_to_find`](log_to_find.md)）在并发线程上的交错可能导致日志行乱序。

### 平台说明

- 在 Windows 上，如果已附加控制台窗口，标准输出写入将发送到控制台；如果进程没有控制台，则被丢弃。
- 日志文件由 `LOG_FILE` 静态变量以追加模式打开，因此消息在会话期间累积，不会覆盖之前的条目。
- `LOCAL_TIME_BUFFER` 静态变量缓存当前时间，必须由调度循环在外部更新以使时间戳推进。如果未更新，同一周期内的所有日志消息将共享相同的时间戳。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `logging.rs` |
| **调用方** | 所有模块通过 `log!` 宏调用 |
| **被调用方** | `get_dust_bin_mod!`、`get_local_time!`、`get_use_console!`、`get_logger!`、`std::io::stdout`、`writeln!` |
| **依赖** | `chrono::DateTime`、`std::io::Write`、`std::io::stdout` |
| **访问的静态变量** | [`DUST_BIN_MODE`](statics.md#dust_bin_mode)、[`LOCAL_TIME_BUFFER`](statics.md#local_time_buffer)、[`USE_CONSOLE`](statics.md#use_console)、[`LOG_FILE`](statics.md#log_file) |
| **平台** | Windows（标准输出行为）、跨平台文件 I/O |

## 另请参阅

| 主题 | 链接 |
|------|------|
| log_pure_message 函数 | [log_pure_message](log_pure_message.md) |
| log_to_find 函数 | [log_to_find](log_to_find.md) |
| log_process_find 函数 | [log_process_find](log_process_find.md) |
| 日志静态变量 | [statics](statics.md) |
| logging 模块概述 | [README](README.md) |

---
*提交：[29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
