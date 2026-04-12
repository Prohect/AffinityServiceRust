# USE_CONSOLE 静态变量 (logging.rs)

控制日志输出是否同时写入控制台（stdout）。启用时，[`log_message`](log_message.md) 写入的所有消息都会回显到控制台以供交互式监控。

## 语法

```rust
static USE_CONSOLE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));
```

## 成员

该静态变量在 `Mutex` 后持有一个 `bool`，初始化为 `false`：

- `true` — 日志消息同时写入日志文件和控制台。
- `false` — 日志消息仅写入日志文件。

## 备注

`USE_CONSOLE` 默认为 `false`，当应用程序在交互式控制台会话中运行时，由 [`main`](../main.rs/main.md) 设置为 `true`。这允许用户在正常的交互式操作期间查看实时日志输出，同时在作为后台服务运行或 stdout 不可用的场景中抑制控制台输出。

该标志在每次日志调用时由 [`log_message`](log_message.md) 检查。启用时，消息在写入日志文件后通过 `println!` 打印到 stdout。控制台输出使用与文件输出相同的格式，包括时间戳前缀。

### 线程安全

对该标志的访问通过 `Mutex` 同步。由于该值通常在启动时设置一次，之后仅在运行期间读取，因此竞争极小。[`log_message`](log_message.md) 在每次日志调用时短暂获取锁。

### 与 DUST_BIN_MODE 的交互

当 [`DUST_BIN_MODE`](DUST_BIN_MODE.md) 处于活跃状态时，日志输出将被完全抑制——文件和控制台输出都会被跳过。`USE_CONSOLE` 仅在 `DUST_BIN_MODE` 为 `false` 时生效。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/logging.rs |
| **源码行** | L62 |
| **设置方** | [`main`](../main.rs/main.md) |
| **读取方** | [`log_message`](log_message.md) |

## 另请参阅

- [DUST_BIN_MODE 静态变量](DUST_BIN_MODE.md)
- [log_message 函数](log_message.md)
- [logging.rs 模块概述](README.md)