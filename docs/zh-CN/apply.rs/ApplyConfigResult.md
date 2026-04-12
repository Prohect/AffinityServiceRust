# ApplyConfigResult struct (apply.rs)

收集配置应用过程中的更改描述和错误消息。每次对单个进程执行配置应用时，所有 `apply_*` 函数将结果汇总到此结构体中。

## 语法

```rust
#[derive(Debug, Default)]
pub struct ApplyConfigResult {
    pub changes: Vec<String>,
    pub errors: Vec<String>,
}
```

## 成员

`changes`

一个包含人类可读字符串的向量，描述每个成功应用（或在 dry-run 模式下将要应用）的配置更改。每个条目格式为 `"$operation details"`，调用者稍后会添加 `"{pid:>5}::{config.name}::"` 前缀。

`errors`

一个包含人类可读字符串的向量，描述应用过程中遇到的错误。每个条目格式为 `"$fn_name: [$operation][$error_message] details"`。

## 方法

| 方法 | 签名 | 描述 |
| --- | --- | --- |
| **new** | `pub fn new() -> Self` | 通过 `Default` 创建一个空结果。 |
| **add_change** | `pub fn add_change(&mut self, change: String)` | 将一条更改描述追加到 `changes`。 |
| **add_error** | `pub fn add_error(&mut self, error: String)` | 将一条错误描述追加到 `errors`。 |
| **is_empty** | `pub fn is_empty(&self) -> bool` | 当 `changes` 和 `errors` 均为空时返回 `true`。 |

## 备注

`ApplyConfigResult` 是本模块中所有 `apply_*` 函数的主要反馈机制。顶层编排函数 [`apply_config`](../main.rs/apply_config.md)（位于 `main.rs`）在每次循环迭代中为每个进程创建一个实例，通过可变引用传递给整个应用链，然后检查结果以决定是否记录更改日志。

空结果（两个向量均为空）表示进程已处于期望状态，无需执行任何操作。调用者使用 [`is_empty`](#方法) 跳过不必要的日志输出。

该结构体派生了 `Default`，因此 `new()` 构造函数只是一个便捷别名。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/apply.rs |
| **行号** | L30–L56 |
| **返回者** | [`apply_config`](../main.rs/apply_config.md)（main.rs） |
| **消费者** | 本模块中所有 `apply_*` 函数（通过 `&mut ApplyConfigResult`） |

## 另请参阅

- [apply.rs 模块概述](README.md)
- [apply_priority](apply_priority.md)
- [apply_affinity](apply_affinity.md)
- [log_error_if_new](log_error_if_new.md)