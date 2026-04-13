# log! 宏 (logging.rs)

便捷宏，使用 `format!` 语法格式化其参数，并委托给 [log_message](log_message.md) 进行带时间戳的输出。此宏是 AffinityServiceRust 中使用的主要日志入口点；它生成 `[HH:MM:SS]` 前缀的日志行，根据 [USE_CONSOLE](USE_CONSOLE.md) 标志定向到控制台或主日志文件。当 [DUST_BIN_MODE](DUST_BIN_MODE.md) 处于活动状态时，输出将被抑制。

## 语法

```logging.rs
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        $crate::logging::log_message(format!($($arg)*).as_str())
    };
}
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `$($arg:tt)*` | Token tree（可变参数） | `std::format!` 接受的任何 token 序列。通常是一个格式化字符串字面量，后跟零个或多个表达式。 |

## 备注

- 该宏标注了 `#[macro_export]`，将其放置在 crate 根级别。调用者以 `crate::log!` 或在 crate 内简写为 `log!` 来导入使用。
- 在内部，宏调用 `format!($($arg)*)` 生成一个堆分配的 `String`，然后将 `&str` 引用传递给 [log_message](log_message.md)。这意味着无论日志是否因 [DUST_BIN_MODE](DUST_BIN_MODE.md) 而被抑制，格式化字符串都会在每次调用时被分配。抑制检查发生在 `log_message` 内部，而不是在宏展开中。
- 该宏名称遮蔽了 `log` crate 的 `log!` 宏。AffinityServiceRust 不使用 `log` crate，因此不存在冲突。
- 由于该宏完全委托给 [log_message](log_message.md)，其输出行为完全相同：使用 [LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md) 中缓存的时间添加 `[HH:MM:SS]` 时间戳前缀，输出目标（控制台或文件）由 [USE_CONSOLE](USE_CONSOLE.md) 决定。

### 展开示例

如下调用：

```/dev/null/example.rs#L1-1
log!("Applied {} rules to PID {}", count, pid);
```

展开为：

```/dev/null/example.rs#L1-1
crate::logging::log_message(format!("Applied {} rules to PID {}", count, pid).as_str());
```

将产生如下日志行：

```/dev/null/example.log#L1-1
[14:32:07]Applied 3 rules to PID 1234
```

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `logging`（通过 `#[macro_export]` 导出到 crate 根级别） |
| 委托给 | [log_message](log_message.md) |
| 遵从 | [DUST_BIN_MODE](DUST_BIN_MODE.md)、[USE_CONSOLE](USE_CONSOLE.md)、[LOCAL_TIME_BUFFER](LOCAL_TIME_BUFFER.md)、[LOG_FILE](LOG_FILE.md) |
| 标准库依赖 | `std::format!` |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 底层日志函数 | [log_message](log_message.md) |
| 无时间戳的原始日志 | [log_pure_message](log_pure_message.md) |
| 查找模式日志 | [log_to_find](log_to_find.md) |
| 日志抑制标志 | [DUST_BIN_MODE](DUST_BIN_MODE.md) |
| 控制台输出标志 | [USE_CONSOLE](USE_CONSOLE.md) |
| logging 模块概述 | [logging 模块](README.md) |