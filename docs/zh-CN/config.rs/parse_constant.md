# parse_constant 函数 (config.rs)

解析配置文件中的 `@CONSTANT = value` 行，并更新 [ConfigResult](ConfigResult.md) 中对应的字段。已识别的常量通过 [ConfigConstants](ConfigConstants.md) 控制主线程调度器的滞后行为。未知的常量名称会产生警告；无效的值会产生错误。

## 语法

```rust
fn parse_constant(name: &str, value: &str, line_number: usize, result: &mut ConfigResult)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `name` | `&str` | 去除前导 `@` 并转换为大写后的常量名称。例如，如果配置行为 `@keep_threshold = 0.75`，调用方传入 `"KEEP_THRESHOLD"`。 |
| `value` | `&str` | `=` 号之后的原始值字符串，调用方已去除周围空白。 |
| `line_number` | `usize` | 配置文件中从 1 开始的行号，用于错误和警告消息。 |
| `result` | `&mut ConfigResult` | 可变的解析结果累加器。成功时，`result.constants` 中匹配的字段会被更新，`result.constants_count` 会递增。失败时，条目会被推入 `result.errors` 或 `result.warnings`。 |

## 返回值

此函数没有返回值。所有输出通过副作用写入 `result`。

## 备注

### 已识别的常量

| 常量名称 | 类型 | 存储字段 | 描述 |
|----------|------|----------|------|
| `MIN_ACTIVE_STREAK` | `u8` | `result.constants.min_active_streak` | 线程在被提升之前必须连续出现在 top-N 中的最小调度周期数。使用 `u8::parse` 解析；超出 0–255 范围的值会被拒绝。 |
| `KEEP_THRESHOLD` | `f64` | `result.constants.keep_threshold` | 已提升线程必须维持的、占最热线程周期增量的比例，低于此比例则被降级。使用 `f64::parse` 解析。 |
| `ENTRY_THRESHOLD` | `f64` | `result.constants.entry_threshold` | 未提升线程必须达到的、占最热线程周期增量的比例，达到后才被考虑提升。使用 `f64::parse` 解析。 |

### 成功行为

当已识别的常量名称匹配且其值成功解析时：

1. `result.constants` 中对应的字段会被更新。
2. `result.constants_count` 递增 1。
3. 通过 `log_message` 输出日志消息（例如 `"Config: KEEP_THRESHOLD = 0.75"`）。

### 错误行为

- **已知常量的无效值** — 错误会被推入 `result.errors`，格式为：
  `"Line {N}: Invalid constant value '{value}' for '{name}' (expected u8)"`（针对 `MIN_ACTIVE_STREAK`）或
  `"Line {N}: Invalid constant value '{value}' for '{name}'"`（针对阈值常量）。
  `constants_count` **不会**递增。

- **未知常量名称** — 警告会被推入 `result.warnings`，格式为：
  `"Line {N}: Unknown constant '{name}' - will be ignored"`。
  这是非致命的；配置仍然有效。

### 调用上下文

当 [read_config](read_config.md) 的主解析循环遇到以 `@` 开头的行时，会调用 `parse_constant`。调用方按 `=` 拆分，提取名称（去除空白、转大写）和值（去除空白），然后委托给此函数。没有 `=` 号的行在到达 `parse_constant` 之前就会被调用方拒绝。

### 配置文件示例

```text
# 设置更长的连续周期要求以提高提升稳定性
@MIN_ACTIVE_STREAK = 4

# 收紧入口门槛，仅提升真正热门的线程
@ENTRY_THRESHOLD = 0.55

# 放宽保持门槛以减少降级抖动
@KEEP_THRESHOLD = 0.60

# 未知常量会产生警告但不会阻止解析
@SOME_FUTURE_SETTING = 42
```

### 幂等性

如果同一常量在配置文件中被多次定义，每次出现都会覆盖前一个值并再次递增 `constants_count`。重复定义常量不会发出警告——以最后一次定义为准。

## 要求

| | |
|---|---|
| **模块** | `config` (`src/config.rs`) |
| **可见性** | 包内私有 |
| **调用方** | [read_config](read_config.md) |
| **修改** | [ConfigResult](ConfigResult.md)（字段 `constants`、`constants_count`、`errors`、`warnings`） |
| **依赖** | `log_message` 用于诊断输出 |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 滞后调优常量结构体 | [ConfigConstants](ConfigConstants.md) |
| 解析配置输出 | [ConfigResult](ConfigResult.md) |
| 主配置文件读取器 | [read_config](read_config.md) |
| CPU 别名行解析器 | [parse_alias](parse_alias.md) |
| 主线程调度器 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| 模块概述 | [config 模块](README.md) |