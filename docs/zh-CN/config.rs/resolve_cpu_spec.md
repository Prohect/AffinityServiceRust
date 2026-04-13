# resolve_cpu_spec 函数 (config.rs)

解析一个 CPU 规格字符串，该字符串可能是字面 CPU 规格，也可能是对先前定义的 CPU 别名的引用。当规格以 `*` 开头时，其余部分被视为别名名称，并在提供的别名映射中查找。否则，规格将被转发给 [parse_cpu_spec](parse_cpu_spec.md) 进行直接解析。

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

| 参数 | 类型 | 描述 |
|------|------|------|
| `spec` | `&str` | 要解析的 CPU 规格字符串。前后空白会被裁剪。如果以 `*` 开头，其余部分被视为大小写不敏感的别名名称。否则由 [parse_cpu_spec](parse_cpu_spec.md) 作为字面 CPU 规格进行解析。 |
| `field_name` | `&str` | 正在解析的规则字段名称（例如 `"affinity"`、`"cpuset"`、`"prime_cpus"`）。仅用于错误消息，以帮助用户定位问题。 |
| `line_number` | `usize` | 配置文件中遇到此规格的基于 1 的行号。包含在错误消息中。 |
| `cpu_aliases` | `&HashMap<String, Vec<u32>>` | 已定义的 CPU 别名映射，键为小写的别名名称（不含前导 `*`），值为已解析的 `Vec<u32>` CPU 索引。由解析过程中先前的 [parse_alias](parse_alias.md) 调用构建。 |
| `errors` | `&mut Vec<String>` | 当前解析上下文中错误列表的可变引用。如果别名引用无法解析，会向此列表推入一条描述性错误字符串。 |

## 返回值

返回一个排序后的 CPU 索引 `Vec<u32>`。

- **别名路径 (`*name`)：** 如果别名存在，返回别名映射中 CPU 索引向量的克隆。如果别名未定义，返回空向量并推入一条错误。
- **字面路径：** 返回对裁剪后的规格字符串调用 [parse_cpu_spec](parse_cpu_spec.md) 的输出。

## 备注

### 别名解析

别名名称的匹配是大小写不敏感的。前导 `*` 被去除，其余部分转为小写，然后在 `cpu_aliases` 中查找。这意味着 `*Perf`、`*PERF` 和 `*perf` 都会解析到同一个别名条目。

当别名未找到时，错误消息遵循以下格式：

```text
Line {line_number}: Undefined alias '*{alias}' in {field_name} field
```

在这种情况下，函数仍然返回一个空向量，允许解析继续进行并累积更多错误，而不是在第一个失败时中止。

### 字面规格直通

当规格不以 `*` 开头时，它将直接转发给 [parse_cpu_spec](parse_cpu_spec.md)，不进行任何额外的验证或错误报告。字面规格中的任何问题（例如格式错误的范围）由 `parse_cpu_spec` 的宽容解析行为静默处理。

### 可见性

此函数具有 **crate 私有** 可见性（`fn`，而非 `pub fn`）。它由 [parse_and_insert_rules](parse_and_insert_rules.md) 在解析规则行的 `affinity`、`cpuset` 和 `prime_cpus` 字段时内部调用。

### 空白处理

`spec` 参数在函数开始时被裁剪。这确保了像 `" *perf "` 或 `" 0-7 "` 这样的规格无论配置文件中的空白如何都能被正确处理。

### 示例

假设别名映射为 `{ "perf": [0, 1, 2, 3], "eff": [4, 5, 6, 7] }`：

| 输入 `spec` | 结果 | 是否推入错误？ |
|-------------|------|---------------|
| `"*perf"` | `[0, 1, 2, 3]` | 否 |
| `"*EFF"` | `[4, 5, 6, 7]` | 否 |
| `"*unknown"` | `[]` | 是 — `"Line N: Undefined alias '*unknown' in {field_name} field"` |
| `"0-7"` | `[0, 1, 2, 3, 4, 5, 6, 7]` | 否 |
| `"0xFF"` | `[0, 1, 2, 3, 4, 5, 6, 7]` | 否 |
| `"0"` | `[]` | 否 |

## 要求

| | |
|---|---|
| **模块** | `config` (`src/config.rs`) |
| **可见性** | Crate 私有 |
| **调用者** | [parse_and_insert_rules](parse_and_insert_rules.md) |
| **调用** | [parse_cpu_spec](parse_cpu_spec.md) |
| **API** | 纯函数 - 无 I/O，无 Windows API 调用 |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| CPU 规格字符串解析器 | [parse_cpu_spec](parse_cpu_spec.md) |
| 别名定义解析 | [parse_alias](parse_alias.md) |
| 规则字段解析（主要调用者） | [parse_and_insert_rules](parse_and_insert_rules.md) |
| CPU 索引转位掩码 | [cpu_indices_to_mask](cpu_indices_to_mask.md) |
| 每进程配置（使用别名的字段） | [ProcessConfig](ProcessConfig.md) |
| 模块概述 | [config 模块](README.md) |