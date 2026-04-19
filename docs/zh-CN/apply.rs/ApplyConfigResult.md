# ApplyConfigResult 结构体（apply.rs）

用于收集在配置应用过程中发生的变更和错误的累加器。模块中的每个 `apply_*` 函数都接收此结构体的可变引用，并附加人类可读的消息来描述发生了什么变更或什么操作失败了。

## 语法

```AffinityServiceRust/src/apply.rs#L31-35
#[derive(Debug, Default)]
pub struct ApplyConfigResult {
    pub changes: Vec<String>,
    pub errors: Vec<String>,
}
```

## 成员

| 成员 | 类型 | 描述 |
|--------|------|-------------|
| `changes` | `Vec<String>` | 成功应用的设置描述（或试运行模式下的设置）。格式：`"$operation 详情"`，由调用方自动以 `"{pid:>5}::{config.name}::"` 作为前缀。 |
| `errors` | `Vec<String>` | 应用过程中遇到的失败描述。格式：`"$函数名：[$operation][$error_message] 详情"`。仅记录唯一错误（参见 [log_error_if_new](log_error_if_new.md)）。 |

## 方法

### new

```AffinityServiceRust/src/apply.rs#L38-40
pub fn new() -> Self
```

创建一个新的 `ApplyConfigResult`，其中包含空的 `changes` 和 `errors` 向量。委托给 `Default::default()`。

### add_change

```AffinityServiceRust/src/apply.rs#L45-47
pub fn add_change(&mut self, change: String)
```

将变更描述附加到 `changes` 向量。

**参数**

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `change` | `String` | 人类可读的已应用（或试运行）变更描述。 |

### add_error

```AffinityServiceRust/src/apply.rs#L51-53
pub fn add_error(&mut self, error: String)
```

将错误描述附加到 `errors` 向量。

**参数**

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `error` | `String` | 人类可读的失败描述，包含操作标签和 Win32/NTSTATUS 错误文本。 |

### is_empty

```AffinityServiceRust/src/apply.rs#L55-57
pub fn is_empty(&self) -> bool
```

当 `changes` 和 `errors` 都为空时返回 `true`，表示在当前应用周期内此进程未发生可观察到的操作。

**返回值**

`bool` — 当未记录任何变更或错误时返回 `true`。

## 备注

- 所有 `apply_*` 函数都以 `&mut ApplyConfigResult` 作为最后一个参数，遵循模块中的一致约定。
- 调用方（在 `main.rs` 中）使用 `is_empty()` 跳过那些不需要任何变更的进程的日志输出。
- `add_change` 和 `add_error` 标记为 `#[inline(always)]`，因为它们在应用循环的热路径上被频繁调用。

## 需求

| | |
|---|---|
| **模块** | `src/apply.rs` |
| **调用方** | 所有 `apply_*` 函数，[log_error_if_new](log_error_if_new.md)，主应用循环 |
| **依赖项** | 无（普通数据结构） |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 模块概述 | [apply.rs](README.md) |
| 错误去重辅助函数 | [log_error_if_new](log_error_if_new.md) |

*Documented for Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*