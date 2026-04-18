# ETW_ACTIVE 静态变量 (event_trace.rs)

一个 `AtomicBool` 标志，指示 ETW（Windows 事件跟踪）会话当前是否处于活动状态。此标志用于防止冗余的停止操作，并协调由 [`EtwProcessMonitor`](EtwProcessMonitor.md) 管理的 ETW 跟踪会话的生命周期。

## 语法

```rust
static ETW_ACTIVE: AtomicBool = AtomicBool::new(false);
```

## 类型

`std::sync::atomic::AtomicBool`

## 备注

### 状态转换

| 转换 | 触发条件 | 内存排序 |
|------|---------|----------|
| `false` → `true` | [`EtwProcessMonitor::start`](EtwProcessMonitor.md) 成功生成后台处理线程。 | `SeqCst`（store） |
| `true` → `false` | [`EtwProcessMonitor::stop`](EtwProcessMonitor.md) 被调用（显式调用或通过 `Drop` 触发）。 | `SeqCst`（store） |

### 使用模式

- **在 `start()` 中**：在 ETW 会话完全初始化之后（跟踪已启动、提供程序已启用、跟踪已打开且处理线程已生成），`ETW_ACTIVE` 通过 `store(true, Ordering::SeqCst)` 设置为 `true`。

- **在 `stop()` 中**：该方法首先通过 `load(Ordering::SeqCst)` 检查 `ETW_ACTIVE`。如果标志已经为 `false`，则停止操作立即返回——这可以防止重复关闭跟踪句柄和冗余清理。如果为 `true`，则在继续清理（关闭跟踪、停止会话、加入线程和清除全局发送器）之前将标志设置为 `false`。

### 线程安全

`ETW_ACTIVE` 对所有操作使用带有 `SeqCst` 排序的 `AtomicBool`，提供最强的内存排序保证。这确保了：

- `stop()` 中的标志更新对任何并发读取者立即可见。
- `stop()` 顶部的守卫检查正确地防止并发或重复的停止调用发生竞争。

### 可见性

此静态变量为**模块私有**（没有 `pub` 修饰符）。它仅在 `EtwProcessMonitor::start` 和 `EtwProcessMonitor::stop` 方法内部被访问。

### 初始化

与此模块中的其他静态变量（使用 `once_cell::sync::Lazy`）不同，`ETW_ACTIVE` 是一个在编译时初始化为 `false` 的普通 `AtomicBool`。不需要延迟初始化，因为 `AtomicBool::new` 是一个 `const fn`。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `event_trace.rs` |
| **可见性** | 私有（模块内部） |
| **访问者** | [`EtwProcessMonitor::start`](EtwProcessMonitor.md)、[`EtwProcessMonitor::stop`](EtwProcessMonitor.md) |
| **依赖** | `std::sync::atomic::{AtomicBool, Ordering}` |
| **平台** | Windows |

## 另请参阅

| 主题 | 链接 |
|------|------|
| EtwProcessMonitor 结构体 | [EtwProcessMonitor](EtwProcessMonitor.md) |
| ETW_SENDER 静态变量 | [ETW_SENDER](ETW_SENDER.md) |
| EtwProcessEvent 结构体 | [EtwProcessEvent](EtwProcessEvent.md) |
| event_trace 模块概述 | [README](README.md) |

---
*提交：[29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
