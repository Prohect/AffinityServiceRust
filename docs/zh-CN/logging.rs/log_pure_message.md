# log_pure_message 函数 (logging.rs)

将消息写入日志输出，**不带**时间戳前缀。用于续行、横幅或预格式化块，其中调用方管理自己的格式化。

## 语法

```rust
pub fn log_pure_message(args: &str)
```

## 参数

`args: &str`

要写入的消息字符串。通过 `writeln!` 自动追加换行符。

## 返回值

此函数不返回值。

## 备注

与 [log_message](log_message.md) 不同，此函数**不会**在前面添加 `[HH:MM:SS]` 时间戳，也**不会**检查 `DUST_BIN_MODE` 标志。无论禁用模式状态如何，总是输出内容。

### 输出路由

| `USE_CONSOLE` | 目标 |
|---------------|-------------|
| `true` | `stdout` |
| `false` | `LOG_FILE`（`logs/YYYYMMDD.log`） |

### 写入错误

`writeln!` 的错误被静默丢弃（`writeln!` 的返回值被赋值给 `_`）。这防止了当日志文件不可访问时的级联失败。

### 与 log_message 的比较

| 方面 | `log_message` | `log_pure_message` |
|--------|---------------|-------------------|
| 时间戳前缀 | `[HH:MM:SS]` | 无 |
| 遵循 `DUST_BIN_MODE` | 是 | 否 |
| 输出目标 | 控制台或文件 | 控制台或文件 |

## 要求

| | |
|---|---|
| **模块** | `src/logging.rs` |
| **调用方** | 主循环横幅输出、多行日志续行 |
| **被调用方** | `get_use_console!`、`get_logger!` |
| **访问的静态变量** | `USE_CONSOLE`、`LOG_FILE` |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 模块概述 | [logging.rs](README.md) |
| 带时间戳的日志记录 | [log_message](log_message.md) |
| 查找模式日志记录 | [log_to_find](log_to_find.md) |
| 日志文件路径计算 | [get_log_path](get_log_path.md) |

*记录于提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*