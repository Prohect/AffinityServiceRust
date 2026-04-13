# IOPriority 枚举 (priority.rs)

表示通过 `NtSetInformationProcess` 及 I/O 优先级对应的 `ProcessInformationClass` 值分配给进程的 I/O 优先级级别。每个变体映射到 NT 原生 API 所使用的数值常量。`None` 变体作为哨兵值，表示配置中未请求更改 I/O 优先级。

设置 `IOPriority::High` 需要 `SeIncreaseBasePriorityPrivilege` 特权和提升的（管理员）令牌。

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

| 变体 | Win32 值 | 字符串名称 | 描述 |
|---------|-------------|-------------|-------------|
| `None` | *(无)* | `"none"` | 哨兵值 — 未请求更改 I/O 优先级。`as_win_const` 返回 `None`。 |
| `VeryLow` | `0` | `"very low"` | 最低 I/O 优先级。后台类 I/O。 |
| `Low` | `1` | `"low"` | 低 I/O 优先级。 |
| `Normal` | `2` | `"normal"` | 大多数进程的默认 I/O 优先级。 |
| `High` | `3` | `"high"` | 最高 I/O 优先级。需要 `SeIncreaseBasePriorityPrivilege` 特权和管理员提升。 |

## 方法

### as_str

```rust
pub fn as_str(&self) -> &'static str
```

返回此变体的可读字符串名称（例如 `"very low"`、`"normal"`）。如果变体不在内部查找表中，则返回 `"unknown"`。

### as_win_const

```rust
pub fn as_win_const(&self) -> Option<u32>
```

返回用于 `NtSetInformationProcess` 的数值 I/O 优先级值，对于 `IOPriority::None` 哨兵值返回 `None`。

### from_str

```rust
pub fn from_str(s: &str) -> Self
```

将不区分大小写的字符串解析为 `IOPriority` 变体。无法识别的字符串返回 `IOPriority::None`。这**不是** `std::str::FromStr` trait；而是一个固有方法。

### from_win_const

```rust
pub fn from_win_const(val: u32) -> &'static str
```

将原始数值 I/O 优先级值映射回其可读字符串名称。对于无法识别的值返回 `"unknown"`。注意此方法返回 `&'static str`，而非 `IOPriority` 变体。

## 备注

Win32 常量值（`0` 到 `3`）对应于 `ntddk.h` 中定义的 `IO_PRIORITY_HINT` 枚举。AffinityServiceRust 使用 `NtSetInformationProcess` / `NtQueryInformationProcess` 原生 API，而非文档化的 Win32 `SetProcessInformation` 接口，因为后者不直接暴露 I/O 优先级。

查找由一个 `const TABLE` 数组驱动，该数组包含 `(Self, &str, Option<u32>)` 元组。所有四个公开方法都对此表执行线性扫描；对于五个变体，开销可以忽略不计。

`from_str` 在匹配前将输入转为小写，以实现不区分大小写的比较。

## 要求

| | |
|---|---|
| **模块** | `priority` |
| **调用者** | [parse_and_insert_rules](../config.rs/parse_and_insert_rules.md)、[apply_io_priority](../apply.rs/apply_io_priority.md) |
| **被调用者** | *(无 — 纯数据映射)* |
| **Win32 API** | `NtSetInformationProcess`（`ProcessInformationClass` = I/O 优先级）、`NtQueryInformationProcess` |
| **特权** | `High` 变体需要 `SeIncreaseBasePriorityPrivilege` |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 进程优先级类枚举 | [ProcessPriority](ProcessPriority.md) |
| 内存优先级级别枚举 | [MemoryPriority](MemoryPriority.md) |
| 线程优先级级别枚举 | [ThreadPriority](ThreadPriority.md) |
| I/O 优先级应用逻辑 | [apply_io_priority](../apply.rs/apply_io_priority.md) |
| priority 模块概述 | [README](README.md) |