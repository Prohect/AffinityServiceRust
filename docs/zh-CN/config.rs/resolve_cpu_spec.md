# resolve_cpu_spec 函数 (config.rs)

解析可能包含别名引用的 CPU 规格字符串。如果规格以 `*` 开头，则视为别名查找；否则直接作为 CPU 规格解析。

## 语法

```rust
fn resolve_cpu_spec(
    spec: &str,
    field_name: &str,
    line_number: usize,
    cpu_aliases: &HashMap<String, List<[u32; CONSUMER_CPUS]>>,
    errors: &mut Vec<String>,
) -> List<[u32; CONSUMER_CPUS]>
```

## 参数

`spec: &str`

要解析的 CPU 规格字符串。如果以 `*` 开头，则其余部分被视为别名名称（不区分大小写）。否则，字符串被直接传递给 [parse_cpu_spec](parse_cpu_spec.md) 进行直接解析。

`field_name: &str`

配置字段的易读名称（例如，`"affinity"`、`"cpuset"`、`"prime_cpus"`）。用于错误消息中指示哪个字段包含无效的别名。

`line_number: usize`

规格出现的配置文件中的基于 1 的行号。包含在错误消息中，用于用户诊断。

`cpu_aliases: &HashMap<String, List<[u32; CONSUMER_CPUS]>>`

当前定义的 CPU 别名映射，由之前的 [parse_alias](parse_alias.md) 调用填充。键是小写别名名称（不带 `*` 前缀）。

`errors: &mut Vec<String>`

**\[out\]** 错误消息累加器。如果别名引用（`*name`）指向未定义的别名，则追加错误。

## 返回值

`List<[u32; CONSUMER_CPUS]>` — 排序后的 CPU 索引列表。如果别名存在，返回别名的 CPU 列表；如果别名未定义，返回空列表；对于非别名规格，返回 [parse_cpu_spec](parse_cpu_spec.md) 的结果。

## 备注

此函数是原始配置字段值和存储在 [ProcessLevelConfig](ProcessLevelConfig.md) 和 [ThreadLevelConfig](ThreadLevelConfig.md) 中的 CPU 索引列表之间的中心间接层。它被 [parse_and_insert_rules](parse_and_insert_rules.md) 用于亲和性、cpuset 和 Prime CPU 字段调用。

### 别名解析

1. 输入 `spec` 被去除首尾空格。
2. 如果修剪后的字符串以 `*` 开头，则去除 `*` 前缀并将剩余部分转为小写以形成别名键。
3. 别名键在 `cpu_aliases` 中查找。如果找到，克隆关联的 CPU 列表并返回。
4. 如果未找到，则以格式 `"Line {N}: Undefined alias '*{name}' in {field_name} field"` 推送错误，并返回空列表。

### 直接解析

如果规格不以 `*` 开头，则直接传递给 [parse_cpu_spec](parse_cpu_spec.md)，后者处理范围（`0-7`）、单个 CPU（`0;4;8`）、十六进制掩码（`0xFF`）和特殊值 `"0"`（空列表/不更改）。

### 可见性

此函数是模块私有的（`fn`，而非 `pub fn`）。它仅在 `config.rs` 内部使用的内部辅助函数。

## 需求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **可见性** | 私有 |
| **调用方** | [parse_and_insert_rules](parse_and_insert_rules.md)（亲和性、cpuset、Prime CPU 字段） |
| **被调用方** | [parse_cpu_spec](parse_cpu_spec.md) |
| **权限** | 无 |

## 参见

| 主题 | 链接 |
|------|------|
| CPU 规格解析器 | [parse_cpu_spec](parse_cpu_spec.md) |
| 别名定义 | [parse_alias](parse_alias.md) |
| 规则插入 | [parse_and_insert_rules](parse_and_insert_rules.md) |
| 配置读取器 | [read_config](read_config.md) |
| 模块概述 | [config.rs](README.md) |

*文档针对 Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*