# parse_alias 函数 (config.rs)

从配置文件中解析 `*name = cpu_spec` 别名定义行，并将解析后的 CPU 索引列表插入别名映射。别名提供 CPU 规格的命名快捷方式，可以在配置的其他位置使用 `*name` 语法引用。

## 语法

```rust
fn parse_alias(
    name: &str,
    value: &str,
    line_number: usize,
    cpu_aliases: &mut HashMap<String, List<[u32; CONSUMER_CPUS]>>,
    result: &mut ConfigResult,
)
```

## 参数

`name: &str`

别名名称（不含前导 `*` 前缀），已由调用方转为小写。例如，给定配置行 `*PCore = 0-7`，调用方传递 `"pcore"`。空名称会触发错误。

`value: &str`

`=` 右侧的 CPU 规格字符串，已由调用方修剪。此内容转发给 [parse_cpu_spec](parse_cpu_spec.md) 解析为 CPU 索引列表。支持范围（`0-7`）、分号分隔的值（`0;4;8`）、十六进制掩码（`0xFF`）或 `"0"`（空集）。

`line_number: usize`

配置文件中此别名定义出现的基于 1 的行号。包含在错误和警告消息中。

`cpu_aliases: &mut HashMap<String, List<[u32; CONSUMER_CPUS]>>`

**\[in, out\]** 配置解析过程中构建的别名映射。成功时，插入新条目，`name` 为键，解析后的 CPU 列表为值。如果已存在同名的别名，则会被静默覆盖。

`result: &mut ConfigResult`

**\[in, out\]** 解析结果累加器。成功时，`result.aliases_count` 递增。失败时（空名称），错误被推入 `result.errors`。如果解析后的 CPU 集合对于非 `"0"` 值为空，则警告被推入 `result.warnings`。

## 返回值

此函数不返回值。结果通过 `cpu_aliases` 和 `result` 传达。

## 备注

### 验证

函数执行两个验证检查：

1. **空名称：** 如果 `name` 为空，则推入错误 `"Line {N}: Empty alias name"`，函数返回而不插入任何内容。
2. **非零值的空 CPU 集合：** 如果 [parse_cpu_spec](parse_cpu_spec.md) 返回空列表，但 `value` 不是字面量的 `"0"`，则会发出警告，因为用户可能在 CPU 规格中打错了字。别名仍会以空的 CPU 列表插入。

### 别名命名约定

- 别名名称不区分大小写。调用方在传递之前会先将名称转为小写。
- 除了配置解析器的行分割逻辑强加的限制外，别名名称字符没有限制（名称是 `*` 和 `=` 之间的所有内容）。
- 常见的约定包括 `pcore`、`ecore`、`perf`、`efficiency` 或特定应用的名称如 `game_cpus`。

### 覆盖行为

如果配置文件中多次定义相同的别名名称，后续定义会静默覆盖前面的定义。不会为重新定义发出警告——`cpu_aliases` 中只保留最终值。引用该别名的规则行将使用该别名在其行位置时的值（因为别名是按顺序解析的，在规则之前）。

### 配置语法示例

```
*pcore = 0-7
*ecore = 8-19
*all = 0-19
*fast = 0;2;4;6
```

### 与 resolve_cpu_spec 的交互

定义后，别名由 [resolve_cpu_spec](resolve_cpu_spec.md) 在 CPU 字段值以 `*` 开头时消费。例如，规则行中的亲和性字段 `*pcore` 会导致 `resolve_cpu_spec` 在别名映射中查找 `"pcore"` 并返回存储的 CPU 列表 `[0, 1, 2, 3, 4, 5, 6, 7]`。

## 需求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **可见性** | 私有 (`fn`) |
| **调用方** | [read_config](read_config.md)（在遇到每个 `*name = value` 行时） |
| **被调用方** | [parse_cpu_spec](parse_cpu_spec.md) |
| **权限** | 无 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [config.rs](README.md) |
| CPU 规格解析器 | [parse_cpu_spec](parse_cpu_spec.md) |
| 别名消费者 | [resolve_cpu_spec](resolve_cpu_spec.md) |
| 常量解析器（类似模式） | [parse_constant](parse_constant.md) |
| 配置文件读取器 | [read_config](read_config.md) |
| 聚合结果 | [ConfigResult](ConfigResult.md) |

*文档针对 Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*