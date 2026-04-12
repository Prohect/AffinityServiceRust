# parse_alias 函数 (config.rs)

解析 CPU 别名定义行，并将别名名称及其解析后的 CPU 索引注册到别名映射表中。

## 语法

```rust
fn parse_alias(
    name: &str,
    value: &str,
    line_number: usize,
    cpu_aliases: &mut HashMap<String, Vec<u32>>,
    result: &mut ConfigResult,
)
```

## 参数

`name`

别名名称（不含前导 `*` 前缀）。不可为空。名称以小写形式存储在别名映射表中。

`value`

要与此别名关联的 CPU 规格字符串。通过 [parse_cpu_spec](parse_cpu_spec.md) 解析为 `Vec<u32>` CPU 索引列表。支持所有 CPU 规格格式（范围、单独索引、十六进制掩码）。

`line_number`

配置文件中此别名定义所在的行号（从 1 开始）。用于错误和警告消息。

`cpu_aliases`

对 `HashMap<String, Vec<u32>>` 的可变引用，用于存储所有已注册的 CPU 别名。新别名以小写名称为键插入（或覆盖已有条目）。

`result`

对 [ConfigResult](ConfigResult.md) 累加器的可变引用。成功时 `aliases_count` 递增。错误推送到 `result.errors`，警告推送到 `result.warnings`。

## 返回值

此函数无返回值。结果通过修改 `cpu_aliases` 和 `result` 来传递。

## 备注

别名定义在配置文件中的格式为：

```
*别名名称 = cpu_spec
```

前导 `*` 和 `=` 由调用方（[read_config](read_config.md)）在调用 `parse_alias` 之前剥离。`name` 参数仅接收标识符部分（例如 `*p = 0-7` 中的 `"p"`）。

### 验证规则

- 如果 `name` 为空，则推送错误：`"Line N: Empty alias name"`。
- 如果解析后的 CPU 集合为空且原始值不是 `"0"`，则发出警告，表明该别名解析为空集合。这可以在不产生致命错误的情况下捕获拼写错误或格式错误的规格。

### 在配置文件中的用法

别名在配置规则的其他位置通过 `*` 前缀引用：

```
*p = 0-7
*e = 8-19
chrome.exe:high:*p:@*e:?8x*p
```

别名由 [resolve_cpu_spec](resolve_cpu_spec.md)、[parse_ideal_processor_spec](parse_ideal_processor_spec.md) 以及 [parse_and_insert_rules](parse_and_insert_rules.md) 在解析 prime 线程和理想处理器字段时进行解析。

### 重复定义行为

如果同一别名名称被多次定义，后面的定义会静默覆盖前面的定义。无论是否覆盖，每次定义都会递增 `aliases_count`。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/config.rs |
| **可见性** | 私有 (`fn`) |
| **定义位置** | L294 |
| **调用方** | [read_config](read_config.md) |
| **调用** | [parse_cpu_spec](parse_cpu_spec.md) |

## 另请参阅

- [parse_constant](parse_constant.md) — 解析 `@CONSTANT = value` 定义
- [resolve_cpu_spec](resolve_cpu_spec.md) — 在规则解析时解析 `*alias` 引用
- [ConfigResult](ConfigResult.md) — 解析结果累加器