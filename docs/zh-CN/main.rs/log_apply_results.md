# log_apply_results 函数 (main.rs)

格式化并记录在单个进程的配置应用过程中，[`ApplyConfigResult`](../apply.rs/ApplyConfigResult.md) 所收集的变更和错误。错误被路由到 find 日志；变更则以右对齐的多行格式写入主日志。

## 语法

```rust
fn log_apply_results(pid: &u32, name: &String, result: ApplyConfigResult)
```

## 参数

`pid: &u32`

目标进程的进程标识符。在格式化输出中右对齐填充至 5 个字符。

`name: &String`

匹配此进程的配置规则中的进程名称（例如 `"game.exe"`）。用作日志行前缀的中间部分。

`result: ApplyConfigResult`

[`ApplyConfigResult`](../apply.rs/ApplyConfigResult.md) 累加器，包含当前应用周期的变更和错误。该值被此函数消费。

## 返回值

此函数不返回值。

## 备注

如果 `result.is_empty()` 返回 `true`，函数会立即退出，对于不需要变更且未遇到错误的进程不产生任何日志输出。

### 错误日志

`result.errors` 中的所有条目通过 `log_to_find` 写入 find 日志。这将错误输出与正常的变更跟踪分离，允许错误被独立审查（例如在 `-process_logs` 后处理期间）。

### 变更日志

变更以对齐的多行布局进行格式化：

1. **第一条**变更以单行格式记录：

   `{pid:>5}::{name}::{change}`

   例如：`" 1234::game.exe::Priority: normal -> high"`

2. **后续**变更以计算好的填充前缀记录，使其直接对齐到第一条变更文本的下方。填充考虑了日志基础设施预置的 `[HH:MM:SS]` 时间前缀（10 个字符）。

这种对齐确保单个进程的所有变更在日志文件中以视觉分组的形式呈现，在同时应用多个设置时提高可读性。

### 日志基础设施

- `log_message` 以带时间戳前缀的方式写入主日志文件。
- `log_pure_message` 写入主日志文件但不添加自己的时间戳，依赖调用方提供的填充进行对齐。
- `log_to_find` 写入 `.find.log` 文件，供 find 模式后处理使用。

## 要求

| | |
|---|---|
| **模块** | `src/main.rs` |
| **调用方** | [apply_config](apply_config.md)、[main](main.md) 中仅线程级别的应用路径 |
| **被调用方** | `log_to_find`、`log_message`、`log_pure_message` |
| **Win32 API** | 无 |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 结果累加器 | [ApplyConfigResult](../apply.rs/ApplyConfigResult.md) |
| 组合应用入口点 | [apply_config](apply_config.md) |
| 日志基础设施 | [logging.rs](../logging.rs/README.md) |

*为提交 [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf) 记录*