# USE_CONSOLE 静态变量 (logging.rs)

全局标志，控制日志输出是定向到控制台（标准输出）还是磁盘上的日志文件。当为 `true` 时，所有日志函数写入 `stdout`；当为 `false`（默认值）时，它们写入 [LOG_FILE](LOG_FILE.md) 或 [FIND_LOG_FILE](FIND_LOG_FILE.md) 句柄。

## 语法

```logging.rs
pub static USE_CONSOLE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));
```

## 成员

| 组件 | 类型 | 描述 |
|------|------|------|
| 外层 | `Lazy<…>` | 通过 `once_cell::sync::Lazy` 延迟初始化。在首次访问时创建。 |
| 内层 | `Mutex<bool>` | 线程安全的内部可变性。保护一个 `bool` 值。 |
| 默认值 | `false` | 默认情况下日志输出定向到文件系统。 |

## 备注

- 该标志通常在 `main` 中较早的位置被设置为 `true`，此时服务检测到自身以交互方式运行（连接到控制台）而非作为后台 Windows 服务运行。一旦设置，它在进程的整个生命周期内保持不变。
- 所有日志函数——[log_message](log_message.md)、[log_pure_message](log_pure_message.md) 和 [log_to_find](log_to_find.md)——都会检查此标志以决定其输出目标。当为 `true` 时，它们调用 `writeln!(stdout(), …)` 而非写入文件句柄。
- 该标志通过 [get_use_console!](get_use_console.md) 宏访问，该宏锁定互斥锁并返回 `MutexGuard<bool>`。调用者解引用该守卫以读取当前值。
- 由于该标志在每次日志调用时都会被读取，但仅在启动时写入一次，因此初始化后互斥锁竞争实际上为零。使用 `Mutex` 是为了安全的内部可变性，而非为了保护并发写入。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `logging` |
| Crate 依赖 | `once_cell` (`Lazy`)、`std::sync::Mutex` |
| 写入方 | [main](../main.rs/README.md)（启动期间） |
| 读取方 | [log_message](log_message.md)、[log_pure_message](log_pure_message.md)、[log_to_find](log_to_find.md) |
| 访问宏 | [get_use_console!](get_use_console.md) |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 提升前日志抑制标志 | [DUST_BIN_MODE](DUST_BIN_MODE.md) |
| 主日志文件句柄（标志为 `false` 时使用） | [LOG_FILE](LOG_FILE.md) |
| 查找模式日志文件句柄 | [FIND_LOG_FILE](FIND_LOG_FILE.md) |
| 带时间戳的日志输出函数 | [log_message](log_message.md) |
| logging 模块概述 | [logging 模块](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd