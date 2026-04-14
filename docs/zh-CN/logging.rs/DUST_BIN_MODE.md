# DUST_BIN_MODE 静态变量 (logging.rs)

全局标志，当设置为 `true` 时抑制所有日志输出。此模式在 UAC 提升之前被激活，以防止未提权的（提升前的）进程写入它可能不拥有或无权创建的日志文件。一旦提升后的进程接管，`DUST_BIN_MODE` 将被设回 `false`，恢复正常日志记录。

## 语法

```logging.rs
pub static DUST_BIN_MODE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));
```

## 成员

| 组件 | 类型 | 描述 |
|------|------|------|
| 外层 | `Lazy<…>` | 通过 `once_cell::sync::Lazy` 延迟初始化。在首次访问时创建。 |
| 内层 | `Mutex<bool>` | 线程安全的内部可变性。`false` = 日志启用（默认），`true` = 所有日志输出被抑制。 |

## 备注

- 当 `DUST_BIN_MODE` 为 `true` 时，[log_message](log_message.md) 函数会立即返回，不向控制台或日志文件写入任何内容。这是硬性抑制——不会发生缓冲或延迟输出；消息会被静默丢弃。
- 该标志通过 [get_dust_bin_mod!](get_dust_bin_mod.md) 宏访问，该宏获取互斥锁并返回 `MutexGuard<bool>`。调用者可以解引用 guard 来读取当前值，或进行可变解引用来更改它。
- "dust bin"（垃圾箱）这个名称是一个比喻——日志消息实际上被丢弃了，就像扔进了废纸篓一样。
- [log_pure_message](log_pure_message.md) 和 [log_to_find](log_to_find.md) **不会**检查 `DUST_BIN_MODE`。只有 [log_message](log_message.md)（以及通过扩展的 [log!](log.md) 宏）遵守此标志。

### 典型生命周期

1. **服务启动（未提权）：** 如果进程检测到需要 UAC 提升，它会在执行任何日志记录之前通过 `*get_dust_bin_mod!() = true` 将 `DUST_BIN_MODE` 设置为 `true`。这由 `--skip-log-before-elevation` CLI 标志控制。
2. **UAC 重新启动：** 进程调用 `request_uac_elevation` 并退出。提升后的子进程以全新状态启动，`DUST_BIN_MODE` 默认为 `false`。
3. **正常运行：** `DUST_BIN_MODE` 在整个服务生命周期中保持为 `false`，所有日志输出正常进行。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `logging` |
| Crate 依赖 | `once_cell` (`Lazy`)、`std::sync::Mutex` |
| 写入方 | [main](../main.rs/README.md)（在提升之前设置为 `true`） |
| 读取方 | [log_message](log_message.md) |
| 访问宏 | [get_dust_bin_mod!](get_dust_bin_mod.md) |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 控制台与文件输出标志 | [USE_CONSOLE](USE_CONSOLE.md) |
| 带时间戳的日志输出函数 | [log_message](log_message.md) |
| 便捷日志宏 | [log!](log.md) |
| UAC 提升请求 | [request_uac_elevation](../winapi.rs/request_uac_elevation.md) |
| 控制日志行为的 CLI 参数 | [cli 模块](../cli.rs/README.md) |
| logging 模块概述 | [logging 模块](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd