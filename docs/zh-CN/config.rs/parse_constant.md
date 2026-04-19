# parse_constant 函数 (config.rs)

从配置文件中解析 `@NAME = value` 常量定义，并更新 [ConfigResult](ConfigResult.md) 中 [ConfigConstants](ConfigConstants.md) 的相应字段。识别的常量控制 Prime 线程调度的迟滞行为。

## 语法

```rust
fn parse_constant(name: &str, value: &str, line_number: usize, result: &mut ConfigResult)
```

## 参数

`name: &str`

从配置行中 `@` 和 `=` 之间部分提取的大写常量名称。调用方负责在传递前修剪空格并转换为大写。识别的名称包括：

| 名称 | 期望类型 | 描述 |
|------|---------------|-------------|
| `MIN_ACTIVE_STREAK` | `u8` | 线程必须在提升前超过入口阈值的最小连续迭代次数。 |
| `KEEP_THRESHOLD` | `f64` | Prime 线程被降级的 CPU 周期份额分数阈值。 |
| `ENTRY_THRESHOLD` | `f64` | 非 Prime 线程必须超过的 CPU 周期份额分数，以开始累积活动 streak。 |

`value: &str`

`=` 右侧的字符串值，由调用方修剪空白。解析为适合常量名称的类型（`MIN_ACTIVE_STREAK` 为 `u8`，阈值字段为 `f64`）。

`line_number: usize`

配置文件中常量定义所在的 1 -based 行号。包含在错误和日志消息中，用于诊断。

`result: &mut ConfigResult`

**\[in, out\]** 解析结果累加器。成功时，`result.constants` 中的相应字段被更新，`result.constants_count` 递增。失败时，错误消息被推入 `result.errors`。

## 返回值

此函数不返回值。结果通过 `result` 参数的修改进行通信。

## 备注

### 解析行为

函数使用 `match` 对 `name` 参数进行匹配，以确定正在设置哪个常量：

1. **`MIN_ACTIVE_STREAK`：** 值被解析为 `u8`。成功时，`result.constants.min_active_streak` 被设置，并记录一条日志消息。失败时，推送错误，指示无效的 `u8` 值。

2. **`KEEP_THRESHOLD` / `ENTRY_THRESHOLD`：** 值被解析为 `f64`。成功时，相应的字段（`keep_threshold` 或 `entry_threshold`）被设置并记录日志。失败时，推送错误。

3. **未知名称：** 任何无法识别的常量名称会产生警告（而非错误），表明它将被忽略。这允许与未来常量向前兼容，而不会破坏现有配置。

### 日志记录

每个成功解析的常量都会产生格式为 `"Config: NAME = value"` 的日志消息，通过 `log_message` 发出。这在启动和热重载期间提供即时反馈。

### 错误消息

错误消息遵循格式 `"Line {N}: Invalid constant value '{value}' for '{name}' (expected type)"` 用于类型不匹配，以及 `"Line {N}: Unknown constant '{name}' - will be ignored"` 用于无法识别的名称。

### 与配置文件的关系

常量出现在配置文件顶部，使用 `@` 前缀：

```
@MIN_ACTIVE_STREAK = 3
@KEEP_THRESHOLD = 0.05
@ENTRY_THRESHOLD = 0.1
```

[read_config](read_config.md) 函数检测以 `@` 开头的行，提取 `=` 两侧的 name 和 value，将 name 转换为大写，修剪 value，并委托给 `parse_constant`。

### 幂等性

如果同一常量在配置文件中定义多次，最后定义生效。每个定义都会增加 `constants_count`，因此如果存在重复项，计数可能会超过 3（识别的常量数量）。

## 需求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **可见性** | 私有 (`fn`) |
| **调用方** | [read_config](read_config.md) |
| **被调用方** | `log_message`（日志记录） |
| **依赖项** | [ConfigResult](ConfigResult.md), [ConfigConstants](ConfigConstants.md) |
| **权限** | 无 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [config.rs](README.md) |
| 常量结构体 | [ConfigConstants](ConfigConstants.md) |
| 配置文件读取器 | [read_config](read_config.md) |
| 别名定义解析器 | [parse_alias](parse_alias.md) |
| 传播常量的热重载 | [hotreload_config](hotreload_config.md) |
| 解析结果容器 | [ConfigResult](ConfigResult.md) |

*文档针对 Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*