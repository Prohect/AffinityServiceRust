# resolve_cpu_spec 函数 (config.rs)

解析可能包含 CPU 别名引用的 CPU 规格字符串，返回对应的 CPU 索引列表。

## 语法

```rust
fn resolve_cpu_spec(
    spec: &str,
    field_name: &str,
    line_number: usize,
    cpu_aliases: &HashMap<String, Vec<u32>>,
    errors: &mut Vec<String>,
) -> Vec<u32>
```

## 参数

`spec`

待解析的 CPU 规格字符串。若以 `*` 开头，则视为 CPU 别名引用（例如 `*p`）；否则转发至 [parse_cpu_spec](parse_cpu_spec.md) 进行直接解析。

`field_name`

当前正在解析的配置字段名称（例如 `"affinity"`、`"cpuset"`、`"prime_cpus"`）。用于错误消息中帮助用户定位问题。

`line_number`

该规格在配置文件中的行号（从 1 开始计数），包含在错误消息中用于诊断。

`cpu_aliases`

别名名称（不含 `*` 前缀）到已解析 CPU 索引向量的映射表。由配置文件中通过 [parse_alias](parse_alias.md) 解析的 `*name = cpu_spec` 行构建。

`errors`

指向 [ConfigResult](ConfigResult.md) 中错误列表的可变引用。若引用了未定义的别名，将向此列表追加错误消息。

## 返回值

返回 `Vec<u32>`，包含已排序的 CPU 索引。若规格为别名引用，则返回该别名对应的 CPU 列表；若别名未定义，则返回空向量并记录错误；若规格不是别名，则委托给 [parse_cpu_spec](parse_cpu_spec.md) 处理。

## 备注

此函数是配置规则字段中解析 CPU 规格的主要入口点。它将别名系统与原始 CPU 规格解析器桥接起来，为所有规则解析代码提供统一接口。

### 别名解析

当 `spec` 以 `*` 开头时，去除前缀后将剩余部分进行裁剪和小写化，然后在 `cpu_aliases` 映射中查找。`*` 前缀在查找前被去除，因此 `*P` 和 `*p` 都会解析到别名键 `"p"`。

若别名未找到，函数将推送如下格式的错误：

> Line {line_number}: Undefined alias '\*{alias}' in {field_name} field

并返回空的 `Vec<u32>`。

### 直接规格透传

当 `spec` 不以 `*` 开头时，将直接传递给 [parse_cpu_spec](parse_cpu_spec.md)，后者处理所有支持的 CPU 规格格式，包括范围（`0-7`）、分号分隔的索引（`0;4;8`）以及十六进制掩码（`0xFF`）。

### 在规则解析中的用法

此函数由 [parse_and_insert_rules](parse_and_insert_rules.md) 在处理每条进程规则的 affinity、cpuset 和 prime CPU 字段时调用。[parse_ideal_processor_spec](parse_ideal_processor_spec.md) 内部也会进行理想处理器别名查找（但该函数自行处理 `*` 前缀拆分）。

## 要求

| 要求 | 值 |
| --- | --- |
| **所属模块** | src/config.rs |
| **可见性** | 私有 (`fn`) |
| **源码行** | L221–L241 |
| **调用方** | [parse_and_insert_rules](parse_and_insert_rules.md) |
| **调用** | [parse_cpu_spec](parse_cpu_spec.md) |

## 另请参阅

- [parse_cpu_spec](parse_cpu_spec.md) — 将 CPU 规格字符串解析为索引向量
- [parse_alias](parse_alias.md) — 解析 `*alias = cpu_spec` 定义行
- [parse_and_insert_rules](parse_and_insert_rules.md) — 规则解析入口，调用本函数
- [ConfigResult](ConfigResult.md) — 解析结果累加器