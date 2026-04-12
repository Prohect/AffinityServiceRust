# IOPriority 枚举 (priority.rs)

表示进程的 I/O 优先级级别。映射到 Windows 内核通过 `NtSetInformationProcess` 使用的 I/O 优先级提示值。

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

`None`

不更改 I/O 优先级。用作默认值，表示配置中未指定 I/O 优先级设置。

`VeryLow`

最低 I/O 优先级（Windows 常量值 `0`）。适用于后台任务，I/O 请求将在所有更高优先级请求之后处理。

`Low`

低 I/O 优先级（Windows 常量值 `1`）。I/O 请求优先于 `VeryLow`，但低于 `Normal`。

`Normal`

默认 I/O 优先级（Windows 常量值 `2`）。大多数进程的标准 I/O 优先级。

`High`

最高 I/O 优先级（Windows 常量值 `3`）。需要管理员权限和 `SeIncreaseBasePriorityPrivilege` 特权。

## 方法

| 方法 | 签名 | 描述 |
| --- | --- | --- |
| **as_str** | `pub fn as_str(&self) -> &'static str` | 返回变体的人类可读字符串名称（例如 `"very low"`、`"normal"`）。 |
| **as_win_const** | `pub fn as_win_const(&self) -> Option<u32>` | 返回对应的 Windows 常量值。`None` 变体返回 `Option::None`。 |
| **from_str** | `pub fn from_str(s: &str) -> Self` | 从字符串解析（不区分大小写）。无法识别的字符串返回 `IOPriority::None`。 |
| **from_win_const** | `pub fn from_win_const(val: u32) -> &'static str` | 从 Windows 常量值反查人类可读名称。未知值返回 `"unknown"`。 |

## 备注

### 静态查找表

`IOPriority` 使用与本模块中所有枚举相同的静态查找表模式。内部常量 `TABLE` 定义了变体、字符串名称和 Windows 常量值之间的三元映射：

```rust
const TABLE: &'static [(Self, &'static str, Option<u32>)] = &[
    (Self::None,    "none",     None),
    (Self::VeryLow, "very low", Some(0)),
    (Self::Low,     "low",      Some(1)),
    (Self::Normal,  "normal",   Some(2)),
    (Self::High,    "high",     Some(3)),
];
```

所有方法（`as_str`、`as_win_const`、`from_str`、`from_win_const`）均通过遍历此表实现双向转换。

### `None` 语义

`None` 变体表示"不更改"，在配置文件中省略 I/O 优先级设置或显式指定 `"none"` 时使用。当 `as_win_const()` 返回 `Option::None` 时，[apply_io_priority](../apply.rs/apply_io_priority.md) 会跳过对应的 Windows API 调用。

### High 优先级的权限要求

设置 `IOPriority::High` 需要满足两个前提条件：

1. **管理员权限** — 进程必须以提升的管理员身份运行。
2. **SeIncreaseBasePriorityPrivilege** — 必须启用此特权（通过 `--no-inc-base-priority` CLI 参数可跳过启用）。

如果条件不满足，`NtSetInformationProcess` 将返回 `STATUS_PRIVILEGE_NOT_HELD` (0xC0000061)。

### Windows API 映射

I/O 优先级通过 `NtSetInformationProcess` 和 `ProcessInformationClass = ProcessIoPriority` 设置。与 [ProcessPriority](ProcessPriority.md) 使用 `PROCESS_CREATION_FLAGS` 不同，I/O 优先级使用原始 `u32` 值，因为 Windows SDK 未为其提供类型化常量。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | `src/priority.rs` |
| **源码行** | L60–L107 |
| **依赖** | 无外部 crate 依赖（Windows 常量为原始整数） |
| **消费者** | [ProcessConfig](../config.rs/ProcessConfig.md)（`io_priority` 字段）、[apply_io_priority](../apply.rs/apply_io_priority.md) |

## 另请参阅

- [priority.rs 模块概述](README.md)
- [ProcessPriority 枚举](ProcessPriority.md)
- [MemoryPriority 枚举](MemoryPriority.md)
- [apply_io_priority 函数](../apply.rs/apply_io_priority.md)