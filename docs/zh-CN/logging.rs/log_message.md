# log_message 函数 (logging.rs)

将带时间戳的日志行写入控制台或每日日志文件。这是服务中使用的主要日志函数，通常通过 [`log!`](README.md) 宏调用。

## 语法

```rust
pub fn log_message(args: &str)
```

## 参数

`args: &str`

要记录的日志消息主体。该字符串在 `[HH:MM:SS]` 时间戳前缀之后附加，无分隔空格 — 调用方负责根据需要包含任何前导空格或标点符号。

## 返回值

此函数不返回值。

## 备注

### 输出格式

每行日志格式如下：

```
[HH:MM:SS]此处为消息文本
```

时间戳从全局 [`LOCAL_TIME_BUFFER`](README.md) 静态变量通过 `get_local_time!()` 宏获取。由于此缓冲区由主循环外部更新，因此单个应用周期内的所有日志行共享相同的时间戳，这在视觉上对相关信息进行了分组。

### 禁用模式抑制

当 [`DUST_BIN_MODE`](README.md) 为 `true` 时，函数立即返回而不写入任何内容。此模式在 UAC 提升期间激活，以防止非提升实例在提升实例启动期间产生输出。

### 控制台与文件路由

目的地由 [`USE_CONSOLE`](README.md) 标志决定：

| `USE_CONSOLE` | 目标 |
|---------------|-------------|
| `true` | 通过 `writeln!` 写入 `stdout` |
| `false` | 通过 [`LOG_FILE`](README.md) 静态变量写入每日日志文件 `logs/YYYYMMDD.log` |

控制台模式用于交互式 CLI 执行（例如 `--find`、`--apply-once`）。文件模式用于作为 Windows 服务运行时。

### 错误处理

写入失败（例如磁盘已满、管道断裂）会被静默忽略 — `writeln!` 的结果使用 `let _ = ...` 丢弃。这可以防止日志记录失败导致服务崩溃。

### 通过宏的典型用法

`log!` 宏是调用此函数的首选方式，因为它支持 `format!` 风格的参数：

```rust
log!(" applied affinity mask 0x{:X} to pid {}", mask, pid);
```

这展开为：

```rust
crate::logging::log_message(format!(" applied affinity mask 0x{:X} to pid {}", mask, pid).as_str());
```

## 要求

| | |
|---|---|
| **模块** | `src/logging.rs` |
| **调用方** | `log!` 宏（在所有模块中使用） |
| **被调用方** | `get_dust_bin_mod!()`，`get_local_time!()`，`get_use_console!()`，`get_logger!()` |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 模块概述 | [logging.rs](README.md) |
| 无时间戳变体 | [log_pure_message](log_pure_message.md) |
| 查找模式日志 | [log_to_find](log_to_find.md) |

*记录于提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*