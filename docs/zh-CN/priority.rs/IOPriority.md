# IOPriority 类型 (priority.rs)

表示可通过 `NtSetInformationProcess` 应用于进程的 Windows I/O 优先级提示级别。每个变体映射到未公开的 `ProcessIoPriority` 信息类所使用的原始 `u32` 常量。`None` 哨兵值表示未配置 I/O 优先级覆盖。

## 语法

```AffinityServiceRust/src/priority.rs#L63-69
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IOPriority {
    None,
    VeryLow,
    Low,
    Normal,
    High, // Requires SeIncreaseBasePriorityPrivilege + admin
}
```

## 成员

| 变体 | Win32 值 | 描述 |
|------|----------|------|
| `None` | *（无值）* | 哨兵变体，表示不应对 I/O 优先级进行更改。`as_win_const()` 返回 `None`。 |
| `VeryLow` | `0` | 后台 I/O 优先级。该进程发出的操作由 I/O 调度器以最低优先级处理，适用于不应干扰交互式工作负载的维护或索引任务。 |
| `Low` | `1` | 低 I/O 优先级。比 `VeryLow` 高一级，适用于执行非紧急磁盘操作的进程。 |
| `Normal` | `2` | 大多数进程的默认 I/O 优先级。除非显式更改，否则这是 Windows 内核分配的级别。 |
| `High` | `3` | 提升的 I/O 优先级。设置此级别需要 `SeIncreaseBasePriorityPrivilege` 权限和管理员权限。仅适用于其 I/O 操作应优先于正常优先级流量处理的延迟敏感应用程序。 |

## 方法

### `as_str`

```AffinityServiceRust/src/priority.rs#L80-85
pub fn as_str(&self) -> &'static str
```

返回变体的人类可读小写字符串表示（例如 `"very low"`、`"normal"`）。如果变体未在内部查找表中找到则返回 `"unknown"`（对于格式正确的值不应出现）。

### `as_win_const`

```AffinityServiceRust/src/priority.rs#L87-89
pub fn as_win_const(&self) -> Option<u32>
```

返回适合传递给 `NtSetInformationProcess`（使用 `ProcessIoPriority` 信息类）的原始 `u32` 值。对于 `IOPriority::None` 哨兵返回 `None`，表示不应进行 API 调用。

### `from_str`

```AffinityServiceRust/src/priority.rs#L91-98
pub fn from_str(s: &str) -> Self
```

将不区分大小写的字符串解析为对应的 `IOPriority` 变体。可识别的字符串为 `"none"`、`"very low"`、`"low"`、`"normal"` 和 `"high"`。无法识别的输入回退为 `IOPriority::None`。

**参数：**

| 参数 | 类型 | 描述 |
|------|------|------|
| `s` | `&str` | 要解析的字符串。在匹配前输入将被转换为小写进行比较（不区分大小写）。 |

### `from_win_const`

```AffinityServiceRust/src/priority.rs#L100-106
pub fn from_win_const(val: u32) -> &'static str
```

将原始 `u32` I/O 优先级常量转换回其人类可读的字符串名称。如果值不匹配任何已知常量则返回 `"unknown"`。用于读取进程当前 I/O 优先级时的显示和日志记录。

**参数：**

| 参数 | 类型 | 描述 |
|------|------|------|
| `val` | `u32` | 要查找的原始 Win32 I/O 优先级值。 |

## 备注

- I/O 优先级常量（`0`–`3`）不属于公开的 Windows SDK；它们与未公开的 `NtSetInformationProcess` / `NtQueryInformationProcess` 信息类 `ProcessIoPriority`（值为 `33`）一起使用。这些值在所有现代 Windows 版本（Vista 到 Windows 11）上都是稳定的。
- 对非调用方所拥有的进程设置 `IOPriority::High`，或未启用 `SeIncreaseBasePriorityPrivilege` 权限时，将因 `STATUS_PRIVILEGE_NOT_HELD` 而失败。
- 所有转换方法使用编译时常量查找表（`TABLE`），该表将每个变体与其字符串名称和可选原始值配对。这确保了零堆分配和对固定大小数组（n ≤ 5）的 O(n) 查找。
- `from_str` 方法由配置解析器用于反序列化配置文件中用户提供的字符串。`as_win_const` 方法由应用引擎用于获取传递给 Win32 API 的值。
- `IOPriority` 派生了 `Clone`、`Copy`、`PartialEq` 和 `Eq`，使其适合低成本比较和存储在配置结构体中。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `priority.rs` |
| 调用者 | 配置解析器（`config.rs`）、应用引擎（`apply.rs`）、[apply_process_level](../main.rs/apply_process_level.md) |
| Win32 API | `NtSetInformationProcess`（`ProcessIoPriority`，信息类 33） |
| 权限 | `SeIncreaseBasePriorityPrivilege` + 管理员（仅 `High` 变体） |

## 另请参阅

| 参考 | 链接 |
|------|------|
| ProcessPriority | [ProcessPriority](ProcessPriority.md) |
| MemoryPriority | [MemoryPriority](MemoryPriority.md) |
| ThreadPriority | [ThreadPriority](ThreadPriority.md) |
| MemoryPriorityInformation | [MemoryPriorityInformation](MemoryPriorityInformation.md) |
| priority 模块概述 | [README](README.md) |
| apply_process_level | [apply_process_level](../main.rs/apply_process_level.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*