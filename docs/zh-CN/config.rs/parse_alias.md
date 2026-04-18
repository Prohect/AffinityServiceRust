# parse_alias 函数 (config.rs)

解析并注册来自配置文件的 `*name = cpu_spec` 别名定义。别名为 CPU 规格提供了命名快捷方式，可以在配置中的其他位置使用 `*name` 语法引用，允许用户一次定义 CPU 拓扑结构并在多个规则中重复使用。

## 语法

```AffinityServiceRust/src/config.rs#L293-313
fn parse_alias(
    name: &str,
    value: &str,
    line_number: usize,
    cpu_aliases: &mut HashMap<String, List<[u32; CONSUMER_CPUS]>>,
    result: &mut ConfigResult,
)
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `name` | `&str` | 别名名称（不含前导 `*`），已由调用方转为小写并修剪。例如，对于配置行 `*pcore = 0-7`，调用方传入 `"pcore"`。 |
| `value` | `&str` | 要与此别名关联的 CPU 规格字符串（例如 `"0-7"`、`"0;4;8"`、`"0xFF"`）。由 [`parse_cpu_spec`](parse_cpu_spec.md) 解析。 |
| `line_number` | `usize` | 此别名定义在配置文件中出现的从 1 开始的行号。用于错误和警告消息。 |
| `cpu_aliases` | `&mut HashMap<String, List<[u32; CONSUMER_CPUS]>>` | 可变的别名查找表。新别名作为 `(name, parsed_cpus)` 键值对插入（或覆盖同名的先前定义）。 |
| `result` | `&mut ConfigResult` | 可变的配置结果累加器。错误会被推送到 `result.errors`；警告推送到 `result.warnings`；成功时 `result.aliases_count` 递增。 |

## 返回值

此函数不返回值。副作用应用于 `cpu_aliases` 和 `result`。

## 备注

### 处理步骤

1. 如果 `name` 为空，则推送一条错误：`"Line {line_number}: Empty alias name"`。
2. 否则，通过 [`parse_cpu_spec`](parse_cpu_spec.md) 将 `value` 字符串解析为已排序的 CPU 索引列表。
3. 如果解析后的 CPU 列表为空**且**原始值不是字面字符串 `"0"`，则推送一条警告，指出别名解析为空 CPU 集合。这可区分有意的空操作别名（`*empty = 0`）和解析失败。
4. 别名以提供的 `name` 作为键插入到 `cpu_aliases` 中。如果已存在同名别名，则被静默覆盖。
5. `result.aliases_count` 递增 1。

### 配置文件语法

别名在配置文件中使用 `*` 前缀定义：

```/dev/null/example.ini#L1-3
*pcore = 0-7
*ecore = 8-19
*all = 0-19
```

然后可以在规则行中引用：

```/dev/null/example.ini#L1
game.exe:high:*pcore:*ecore:0:none:none:0:1
```

### 名称冲突

如果两个别名定义共享相同名称，后面的定义会覆盖前面的定义，不会产生警告或错误。这允许用户在有条件包含的配置文件部分中重新定义别名。

### 空别名警告

警告消息遵循以下格式：

```/dev/null/example.txt#L1
Line {line_number}: Alias '*{name}' has empty CPU set from '{value}'
```

这会提醒用户，当被引用时该别名实际上是空操作，这很可能是配置错误（例如，范围表达式中的拼写错误）。

### 可见性

此函数是模块私有的（`fn`，非 `pub fn`）。它仅由 [`read_config`](read_config.md) 在逐行解析过程中调用。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `config.rs` |
| 可见性 | 私有（包内部） |
| 调用方 | [`read_config`](read_config.md) |
| 被调用方 | [`parse_cpu_spec`](parse_cpu_spec.md) |
| 依赖 | [`collections.rs`](../collections.rs/README.md) 中的 `HashMap`、`List` 和 `CONSUMER_CPUS` |
| 权限 | 无 |

## 另请参阅

| 资源 | 链接 |
|------|------|
| parse_cpu_spec | [parse_cpu_spec](parse_cpu_spec.md) |
| resolve_cpu_spec | [resolve_cpu_spec](resolve_cpu_spec.md) |
| parse_constant | [parse_constant](parse_constant.md) |
| read_config | [read_config](read_config.md) |
| ConfigResult | [ConfigResult](ConfigResult.md) |
| config 模块概述 | [README](README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*