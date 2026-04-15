# resolve_cpu_spec 函数 (config.rs)

解析可能是字面 CPU 规格（范围、十六进制掩码、单独索引）或别名引用（`*name`）的 CPU 规格字符串，将其转换为已排序的 CPU 索引列表。这是位于原始配置字段值与存储在配置结构体中的已解析 CPU 索引列表之间的内部调度器。

## 语法

```AffinityServiceRust/src/config.rs#L220-240
fn resolve_cpu_spec(
    spec: &str,
    field_name: &str,
    line_number: usize,
    cpu_aliases: &HashMap<String, List<[u32; CONSUMER_CPUS]>>,
    errors: &mut Vec<String>,
) -> List<[u32; CONSUMER_CPUS]>
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `spec` | `&str` | 要解析的 CPU 规格字符串。前后空白会被修剪。如果字符串以 `*` 开头，则被视为别名引用；否则通过 [`parse_cpu_spec`](parse_cpu_spec.md) 作为字面 CPU 规格解析。 |
| `field_name` | `&str` | 标识正在解析哪个配置字段的人类可读名称（例如 `"affinity"`、`"cpuset"`、`"prime_cpus"`）。仅在错误消息中使用，用于提供上下文。 |
| `line_number` | `usize` | 此规格在配置文件中出现的从 1 开始的行号。用于错误消息，帮助用户定位问题。 |
| `cpu_aliases` | `&HashMap<String, List<[u32; CONSUMER_CPUS]>>` | 由配置文件中先前的 `*name = cpu_spec` 行填充的别名查找表。键为不含前导 `*` 的小写别名名称。 |
| `errors` | `&mut Vec<String>` | 错误累加器的可变引用。当别名引用在 `cpu_aliases` 中找不到时，会推送错误。 |

## 返回值

类型：`List<[u32; CONSUMER_CPUS]>`

已排序的 CPU 索引列表。在以下情况下返回空列表：
- 规格为 `"0"` 或空（传递给 [`parse_cpu_spec`](parse_cpu_spec.md)）。
- 规格引用了未定义的别名（同时记录错误）。

## 备注

### 别名解析

当修剪后的 `spec` 以 `*` 开头时，函数：

1. 去除前导 `*` 字符。
2. 将其余部分转换为小写。
3. 在 `cpu_aliases` 中查找结果。
4. 如果别名存在，返回存储的 CPU 列表的克隆。
5. 如果别名不存在，向 `errors` 推送描述性错误消息并返回空默认列表。

错误消息遵循以下格式：

```/dev/null/example.txt#L1
Line {line_number}: Undefined alias '*{alias}' in {field_name} field
```

### 字面规格传递

当规格不以 `*` 开头时，函数直接委托给 [`parse_cpu_spec`](parse_cpu_spec.md)，后者处理范围（`0-7`）、单独索引（`0;4;8`）、十六进制位掩码（`0xFF`）和特殊值 `"0"`（不更改）。

### 可见性

此函数是模块私有的（`fn`，非 `pub fn`）。它由 [`parse_and_insert_rules`](parse_and_insert_rules.md) 在处理亲和性和 CPU 集合字段时调用，在 `config` 模块外部不可访问。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `config.rs` |
| 可见性 | 私有（crate 内部） |
| 调用方 | [`parse_and_insert_rules`](parse_and_insert_rules.md) |
| 被调用方 | [`parse_cpu_spec`](parse_cpu_spec.md)、`HashMap::get`、`HashMap::contains_key` |
| API | 无 |
| 权限 | 无 |

## 另请参阅

| 资源 | 链接 |
|------|------|
| parse_cpu_spec | [parse_cpu_spec](parse_cpu_spec.md) |
| parse_alias | [parse_alias](parse_alias.md) |
| parse_and_insert_rules | [parse_and_insert_rules](parse_and_insert_rules.md) |
| ConfigResult | [ConfigResult](ConfigResult.md) |
| config 模块概述 | [README](README.md) |

---
*Commit: 7221ea0694670265d4eb4975582d8ed2ae02439d*