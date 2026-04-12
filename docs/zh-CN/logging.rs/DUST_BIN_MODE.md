# DUST_BIN_MODE 静态变量 (logging.rs)

控制是否抑制日志输出。启用时，通过 [`log_message`](log_message.md) 和 [`log_pure_message`](log_pure_message.md) 写入的所有消息将被静默丢弃，而不会写入日志文件或控制台。

## 语法

```rust
static DUST_BIN_MODE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::from(false));
```

## 成员

该静态变量在 `Mutex` 后持有一个 `bool`：

- `false`（默认） — 日志正常工作；消息写入日志文件，并可选地输出到控制台。
- `true` — 日志被抑制；所有输出被丢弃。

## 备注

"回收站模式"（dust bin mode）这个名称反映了日志消息被丢弃的概念——发送到垃圾箱。此模式的存在是为了处理 UAC 提升期间的特定时序问题。

### 用途

当应用程序在没有管理员权限的情况下启动，并确定需要 UAC 提升时，它将通过 [`request_uac_elevation`](../winapi.rs/request_uac_elevation.md) 以提升的进程重新启动自身。未提升的实例在重新启动之前产生的任何日志输出都会写入一个日志文件，而该文件在提升的实例开始写入其自己的日志文件后将立即被废弃。为了避免产生令人困惑的不完整日志文件，在提升尝试之前将 `DUST_BIN_MODE` 设置为 `true`，抑制短暂存活的未提升实例的所有输出。

### 生命周期

1. 应用程序启动时 `DUST_BIN_MODE` 为 `false`（正常日志记录）。
2. 如果 [`main`](../main.rs/main.md) 确定需要 UAC 提升，且 [`CliArgs`](../cli.rs/CliArgs.md) 中的 `skip_log_before_elevation` 标志已设置，则将 `DUST_BIN_MODE` 设置为 `true`。
3. 未提升实例中所有后续的日志调用不产生任何输出。
4. 提升的实例全新启动，`DUST_BIN_MODE` 处于其默认值 `false`。

### 线程安全

访问通过 `Mutex` 同步。每个日志函数短暂获取锁以检查当前模式，然后立即释放。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/logging.rs |
| **源码行** | L63 |
| **读取方** | [`log_message`](log_message.md)、[`log_pure_message`](log_pure_message.md) |
| **设置方** | [`main`](../main.rs/main.md) |

## 另请参阅

- [USE_CONSOLE 静态变量](USE_CONSOLE.md)
- [log_message 函数](log_message.md)
- [CliArgs](../cli.rs/CliArgs.md)（`skip_log_before_elevation` 标志）
- [request_uac_elevation](../winapi.rs/request_uac_elevation.md)
- [logging.rs 模块概述](README.md)