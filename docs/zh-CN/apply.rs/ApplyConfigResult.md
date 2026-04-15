# ApplyConfigResult 类型 (apply.rs)

`ApplyConfigResult` 结构体用于在单个进程的一次配置应用过程中，累积人类可读的变更描述和错误消息。`apply` 模块中的每个函数都接收一个 `ApplyConfigResult` 的可变引用，并向其中追加条目以记录已更改的内容或失败的操作。当所有 apply 函数执行完毕后，调用方检查该结果以输出日志或采取纠正措施。

## 语法

```AffinityServiceRust/src/apply.rs#L32-35
#[derive(Debug, Default)]
pub struct ApplyConfigResult {
    pub changes: Vec<String>,
    pub errors: Vec<String>,
}
```

## 成员

| 成员 | 类型 | 描述 |
|------|------|------|
| `changes` | `Vec<String>` | 一个人类可读字符串列表，描述每个成功应用（或在 dry-run 模式下将会应用）的配置变更。每个条目遵循 `"$operation details"` 的格式，调用方会在前面添加进程 ID 和配置名称作为前缀。 |
| `errors` | `Vec<String>` | 一个人类可读字符串列表，描述应用过程中发生的每个错误。每个条目遵循 `"$fn_name: [$operation][$error_message] details"` 的格式。错误通过 [`log_error_if_new`](log_error_if_new.md) 在调用端进行去重，因此同一 pid/operation/error-code 的重复失败不会被重复添加。 |

## 方法

| 方法 | 签名 | 描述 |
|------|------|------|
| `new` | `pub fn new() -> Self` | 创建一个新的空 `ApplyConfigResult`。委托给 `Default::default()`。 |
| `add_change` | `pub fn add_change(&mut self, change: String)` | 将一个变更描述字符串追加到 `changes` 向量中。标记为 `#[inline(always)]`。 |
| `add_error` | `pub fn add_error(&mut self, error: String)` | 将一个错误描述字符串追加到 `errors` 向量中。标记为 `#[inline(always)]`。 |
| `is_empty` | `pub fn is_empty(&self) -> bool` | 当 `changes` 和 `errors` 都为空时返回 `true`，表示没有执行任何操作且没有发生失败。 |

## 备注

- `ApplyConfigResult` 派生了 `Debug` 和 `Default`。`Default` 实现生成一个包含两个空 `Vec` 的实例，与调用 `new()` 完全相同。
- 该结构体本身不是线程安全的；调用方在单个进程的顺序 apply 流水线中以 `&mut ApplyConfigResult` 的形式传递它。
- 变更字符串设计为由调用方拼接进程标识前缀（例如 `"{pid:>5}::{config.name}::"`）。`apply` 函数本身**不**包含该前缀。
- 错误字符串是自包含的，包含产生错误的函数名、失败的 Windows API 操作以及解码后的错误码，可直接用于日志记录。
- 调用方使用 `is_empty` 方法来避免在进程已处于期望状态时输出空的日志条目。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `apply.rs` |
| Crate | `AffinityServiceRust` |
| 依赖 | 仅标准库 (`Vec<String>`) |
| 调用方 | 模块中所有 `apply_*` 函数；`scheduler.rs` 和 `main.rs` 中的编排代码 |
| 平台 | Windows（内容是平台特定的，但结构体本身与平台无关） |

## 另请参阅

| 参考 | 链接 |
|------|------|
| apply 模块概述 | [`README`](README.md) |
| log_error_if_new | [`log_error_if_new`](log_error_if_new.md) |
| apply_priority | [`apply_priority`](apply_priority.md) |
| apply_affinity | [`apply_affinity`](apply_affinity.md) |
| apply_io_priority | [`apply_io_priority`](apply_io_priority.md) |
| apply_memory_priority | [`apply_memory_priority`](apply_memory_priority.md) |
| ProcessLevelConfig | [`config.rs/ProcessLevelConfig`](../config.rs/ProcessLevelConfig.md) |

---
*Commit: 7221ea0694670265d4eb4975582d8ed2ae02439d*