# ApplyConfigResult 结构体 (apply.rs)

在对进程执行单次配置应用过程中，累积人类可读的变更描述和错误消息。[apply](README.md) 模块中的每个 `apply_*` 函数都接收一个 `&mut ApplyConfigResult`，并将条目推送到其中，而不是直接记录日志，从而为 [main.rs](../main.rs/README.md) 中的调用方提供统一的操作结果视图。

## 语法

```AffinityServiceRust/src/apply.rs#L29-33
#[derive(Debug, Default)]
pub struct ApplyConfigResult {
    pub changes: Vec<String>,
    pub errors: Vec<String>,
}
```

## 成员

| 成员 | 类型 | 描述 |
|------|------|------|
| `changes` | `Vec<String>` | 成功应用于进程或其线程的修改。每个条目是一段简短的、人类可读的描述，例如 `"Priority: Normal -> High"` 或 `"Thread 1234 -> (promoted, [4,5], cycles=98000, start=ntdll.dll)"`。调用方在将这些内容写入日志之前，会加上进程 ID 和名称前缀。 |
| `errors` | `Vec<String>` | 应用过程中遇到的错误。条目格式为 `"fn_name: [OPERATION][error_message] details"`。只有*新*错误（即之前未针对相同 pid/操作/错误码三元组记录过的错误）才会被添加，因为所有 `apply_*` 函数在调用 `add_error` 之前都会经过 [log_error_if_new](log_error_if_new.md) 过滤。 |

## 方法

| 方法 | 签名 | 描述 |
|------|------|------|
| `new` | `pub fn new() -> Self` | 创建一个空的结果。等价于 `Self::default()`。 |
| `add_change` | `pub fn add_change(&mut self, change: String)` | 将一条变更描述推入 `changes` 向量。 |
| `add_error` | `pub fn add_error(&mut self, error: String)` | 将一条错误描述推入 `errors` 向量。 |
| `is_empty` | `pub fn is_empty(&self) -> bool` | 当 `changes` 和 `errors` 都为空时返回 `true`，允许调用方在没有任何变更时跳过日志记录。 |

## 备注

`ApplyConfigResult` 在 [apply_config_process_level](../main.rs/apply_config_process_level.md) 和 [apply_config_thread_level](../main.rs/apply_config_thread_level.md) 中每次应用周期为每个进程创建一次。当所有 `apply_*` 调用返回后，调用方通过检查 `is_empty()` 来决定是否输出日志行。变更和错误会一起打印，为运维人员提供每个进程每个周期的单条汇总摘要。

该结构体刻意使用 `String` 而非结构化的错误类型。这使得 apply 函数保持简洁——它们在调用点格式化上下文信息（pid、线程 ID、操作、Win32 错误消息）——并避免日志层与特定的错误枚举产生耦合。

`#[derive(Default)]` 实现会生成一个包含两个空 `Vec` 的实例，因此 `new()` 只是一个为可读性而提供的薄包装。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `apply` |
| 调用方 | [apply_config_process_level](../main.rs/apply_config_process_level.md)、[apply_config_thread_level](../main.rs/apply_config_thread_level.md) |
| 传递给 | [apply](README.md) 中的每个 `apply_*` 函数 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| apply 模块概述 | [apply](README.md) |
| 错误去重辅助函数 | [log_error_if_new](log_error_if_new.md) |
| 进程级编排 | [apply_config_process_level](../main.rs/apply_config_process_level.md) |
| 线程级编排 | [apply_config_thread_level](../main.rs/apply_config_thread_level.md) |