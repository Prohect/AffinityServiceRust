# parse_constant 函数 (config.rs)

解析配置文件中的调度器常量定义，并将结果存储到 [ConfigResult](ConfigResult.md) 中。

## 语法

```rust
fn parse_constant(
    name: &str,
    value: &str,
    line_number: usize,
    result: &mut ConfigResult,
)
```

## 参数

`name`

常量的大写名称，已去除前导 `@` 符号。可识别的名称包括 `MIN_ACTIVE_STREAK`、`KEEP_THRESHOLD` 和 `ENTRY_THRESHOLD`。无法识别的名称会产生警告。

`value`

`=` 号右侧的字符串值，已去除首尾空白。对于 `MIN_ACTIVE_STREAK` 解析为 `u8` 类型，对于阈值常量解析为 `f64` 类型。

`line_number`

配置文件中定义该常量的行号（从 1 开始）。用于错误和警告消息的定位。

`result`

正在构建的 [ConfigResult](ConfigResult.md) 的可变引用。解析成功时更新 `result.constants` 中的对应字段，并递增 `result.constants_count`。解析失败时将错误推入 `result.errors`。

## 返回值

此函数无返回值。结果通过修改 `result` 参数来传递。

## 备注

该函数处理三个已识别的常量，用于控制主线程调度器的行为（参见 [ConfigConstants](ConfigConstants.md)）：

| 常量 | 类型 | 默认值 | 用途 |
| --- | --- | --- | --- |
| `MIN_ACTIVE_STREAK` | `u8` | 2 | 线程在获得主线程提升资格前，必须连续活跃的最小调度周期数 |
| `KEEP_THRESHOLD` | `f64` | 0.69 | 已提升线程维持主线程状态所需的 CPU 周期占比阈值 |
| `ENTRY_THRESHOLD` | `f64` | 0.42 | 线程进入主线程提升候选所需的 CPU 周期占比阈值 |

常量在配置文件中使用以下语法定义：

```
@MIN_ACTIVE_STREAK = 3
@KEEP_THRESHOLD = 0.75
@ENTRY_THRESHOLD = 0.50
```

如果值无法解析为预期类型，将向 `result.errors` 推入错误。如果常量名称不是上述三个已识别名称之一，将向 `result.warnings` 推入警告并忽略该值。

每次成功解析后，会通过 `log_message` 记录新值。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/config.rs |
| **可见性** | 私有 (`fn`) |
| **调用方** | [read_config](read_config.md) |
| **修改** | [ConfigResult](ConfigResult.md)、[ConfigConstants](ConfigConstants.md) |