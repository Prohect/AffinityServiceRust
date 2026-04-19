# IOPriority 枚举 (priority.rs)

Windows NT IO 优先级级别的类型安全表示。在人类可读的字符串名称和 `NtSetInformationProcess`/`NtQueryInformationProcess` 配合 `ProcessIoPriority` 信息类使用的未文档化 `u32` IO 优先级值之间进行映射。

## 语法

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IOPriority {
    None,
    VeryLow,
    Low,
    Normal,
    High,
}
```

## 成员

| 变体 | 字符串名称 | Win32 常量 | 描述 |
|---------|-------------|----------------|-------------|
| `None` | `"none"` | *(无)* | 未配置 IO 优先级。`as_win_const()` 返回 `None`。 |
| `VeryLow` | `"very low"` | `0` | 最低 IO 优先级。后台级别 IO 调度。 |
| `Low` | `"low"` | `1` | 低 IO 优先级。减少 IO 带宽分配。 |
| `Normal` | `"normal"` | `2` | 大多数进程的默认 IO 优先级。 |
| `High` | `"high"` | `3` | 最高 IO 优先级。需要 `SeIncreaseBasePriorityPrivilege` 和管理员权限。 |

## 方法

### as_str

```rust
pub fn as_str(&self) -> &'static str
```

返回此 IO 优先级变体的人类可读字符串名称。如果变体未在内部表中找到（对于有效变体不会发生），返回 `"unknown"`。

### as_win_const

```rust
pub fn as_win_const(&self) -> Option<u32>
```

返回用于 `NtSetInformationProcess` 的 NT IO 优先级值，或者对于 `IOPriority::None` 变体返回 `None`。当此方法返回 `None` 时，调用方应跳过 API 调用。

### from_str

```rust
pub fn from_str(s: &str) -> Self
```

将不区分大小写的字符串解析为 `IOPriority` 变体。如果字符串不匹配任何已知的 IO 优先级名称，返回 `IOPriority::None`。输入在比较前会转换为小写。

**参数**

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `s` | `&str` | 要解析的字符串（例如，`"very low"`、`"Normal"`、`"HIGH"`）。 |

### from_win_const

```rust
pub fn from_win_const(val: u32) -> &'static str
```

将 NT IO 优先级值转换回其人类可读的字符串名称。如果值不匹配任何已知常量，返回 `"unknown"`。此方法用于记录当前 IO 优先级，以便与期望值进行比较。

**参数**

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `val` | `u32` | NT IO 优先级值（0-3）。 |

## 备注

### TABLE 驱动转换

与此模块中的所有优先级枚举一样，`IOPriority` 使用单个 `const TABLE` 数组来驱动所有四种转换方法。每个条目是 `(Self, &'static str, Option<u32>)` 元组，确保字符串名称和数值常量仅在一个地方定义。

### 未文档化的 API

与进程优先级（使用文档化的 `SetPriorityClass` API）不同，IO 优先级通过 NT 原生的 `NtSetInformationProcess` 配合 `ProcessIoPriority`（信息类 33）进行管理。数值 0-3 未由 Microsoft 正式文档化，但已通过逆向工程和社区文档得到广泛确认。

### 高 IO 优先级

设置 `IOPriority::High`（值 `3`）需要调用进程持有 `SeIncreaseBasePriorityPrivilege` 并以管理员权限运行。如果没有这些条件，`NtSetInformationProcess` 调用将以 `STATUS_PRIVILEGE_NOT_HELD` 失败。服务通过 [`enable_inc_base_priority_privilege`](../winapi.rs/enable_inc_base_priority_privilege.md) 在启动时获取此权限。

### 关于 from_str 的说明

此 `from_str` 是固有方法，而非 `std::str::FromStr` 特质实现。它不返回 `Result` — 未识别的字符串静默映射为 `IOPriority::None`。

## 要求

| | |
|---|---|
| **模块** | [`src/priority.rs`](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf/src/priority.rs) |
| **使用者** | [`apply_io_priority`](../apply.rs/apply_io_priority.md)，配置解析器 ([`read_config`](../config.rs/read_config.md)) |
| **Win32 API** | `NtQueryInformationProcess` / `NtSetInformationProcess` 配合 `ProcessIoPriority` |
| **权限** | `High` 变体需要 `SeIncreaseBasePriorityPrivilege` |

## 参见

| 主题 | 链接 |
|-------|------|
| 进程优先级枚举 | [ProcessPriority](ProcessPriority.md) |
| 内存优先级枚举 | [MemoryPriority](MemoryPriority.md) |
| 线程优先级枚举 | [ThreadPriority](ThreadPriority.md) |
| IO 优先级应用 | [apply_io_priority](../apply.rs/apply_io_priority.md) |
| 模块概览 | [priority.rs](README.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*