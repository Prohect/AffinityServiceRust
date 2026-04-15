# parse_constant 函数 (config.rs)

解析 `@NAME = value` 常量定义并将其从配置文件应用到 `ConfigResult`。此函数验证常量名称，将值解析为适当的类型，更新 `ConfigResult.constants` 中的相应字段，并在名称无效或值无法解析时记录错误。

## 语法

```AffinityServiceRust/src/config.rs#L251-291
fn parse_constant(name: &str, value: &str, line_number: usize, result: &mut ConfigResult)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `name` | `&str` | 常量名称（已由调用方去除前导 `@` 并转换为大写）。必须匹配以下已识别的常量名称之一：`MIN_ACTIVE_STREAK`、`KEEP_THRESHOLD` 或 `ENTRY_THRESHOLD`。 |
| `value` | `&str` | 要解析的字符串值，已由调用方去除周围空白。预期的类型取决于常量名称（`MIN_ACTIVE_STREAK` 为 `u8`，阈值常量为 `f64`）。 |
| `line_number` | `usize` | 此常量定义在配置文件中出现的从 1 开始的行号。用于错误和警告消息。 |
| `result` | `&mut ConfigResult` | 正在构建的配置结果的可变引用。成功时，`result.constants` 中的相应字段会被更新，并且 `result.constants_count` 递增。失败时，错误会被推送到 `result.errors`。 |

## 返回值

此函数不返回值。结果通过对 `result` 参数的修改来传达。

## 备注

### 已识别的常量

| 常量名称 | 类型 | 目标字段 | 描述 |
|---------|------|---------|------|
| `MIN_ACTIVE_STREAK` | `u8` | `result.constants.min_active_streak` | 线程成为主线程提升候选之前所需的最小连续活跃轮询间隔数。 |
| `KEEP_THRESHOLD` | `f64` | `result.constants.keep_threshold` | 主线程必须维持的 CPU 利用率比例，以保持其主线程状态。 |
| `ENTRY_THRESHOLD` | `f64` | `result.constants.entry_threshold` | 非主线程必须超过的 CPU 利用率比例，才能被考虑提升。 |

### 错误处理

- 如果 `value` 字符串无法解析为预期的数值类型（`MIN_ACTIVE_STREAK` 为 `u8`，阈值为 `f64`），则会向 `result.errors` 推送一条错误消息，指明行号、无效值和预期类型。
- 如果 `name` 不匹配任何已识别的常量，则会向 `result.warnings` 推送一条**警告**（而非错误），指出该常量未知且将被忽略。这允许与未来的常量名称保持前向兼容性。

### 日志记录

成功解析后，函数调用 `log_message` 发出形如 `Config: NAME = value` 的诊断日志条目。这提供了在启动或热重载期间加载了哪些常量的可见性。

### 计数

每次成功解析都会将 `result.constants_count` 递增 1。此计数器用于诊断报告，显示从配置文件中加载了多少个常量。

### 配置文件语法

常量在配置文件中使用 `@` 前缀定义：

```/dev/null/example.ini#L1-3
@MIN_ACTIVE_STREAK = 3
@KEEP_THRESHOLD = 0.75
@ENTRY_THRESHOLD = 0.50
```

`@` 前缀和 `=` 符号由调用方（[`read_config`](read_config.md)）在调用 `parse_constant` 之前去除。名称由调用方转为大写，值由调用方修剪。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `config.rs` |
| 可见性 | 私有（`fn`，非 `pub fn`） |
| 调用方 | [`read_config`](read_config.md) |
| 被调用方 | `log_message`（来自 [`logging.rs`](../logging.rs/README.md)）、`str::parse` |
| API | 标准库解析（`u8::parse`、`f64::parse`） |
| 权限 | 无 |

## 另请参阅

| 资源 | 链接 |
|------|------|
| ConfigConstants | [ConfigConstants](ConfigConstants.md) |
| ConfigResult | [ConfigResult](ConfigResult.md) |
| read_config | [read_config](read_config.md) |
| parse_alias | [parse_alias](parse_alias.md) |
| config 模块概述 | [README](README.md) |

---
*Commit: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)*