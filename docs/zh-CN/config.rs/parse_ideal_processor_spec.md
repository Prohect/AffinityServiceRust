# parse_ideal_processor_spec 函数 (config.rs)

将理想处理器规格字符串解析为 [`IdealProcessorRule`](IdealProcessorRule.md) 条目列表。每条规则将一组 CPU 索引（从别名解析）映射到可选的模块名称前缀，这些前缀用于过滤哪些线程接收理想处理器分配。该规格支持链接多个段，以根据线程的启动模块将不同的 CPU 集合分配给不同的线程。

## 语法

```AffinityServiceRust/src/config.rs#L323-384
fn parse_ideal_processor_spec(
    spec: &str,
    line_number: usize,
    cpu_aliases: &HashMap<String, List<[u32; CONSUMER_CPUS]>>,
    errors: &mut Vec<String>,
) -> Vec<IdealProcessorRule>
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `spec` | `&str` | 理想处理器规格字符串。必须以 `*` 开头以表示基于别名的规则。特殊值 `"0"` 或空字符串表示不进行理想处理器分配。前后空白将被修剪。 |
| `line_number` | `usize` | 配置文件中从 1 开始的行号。用于错误消息，帮助用户定位问题。 |
| `cpu_aliases` | `&HashMap<String, List<[u32; CONSUMER_CPUS]>>` | 由先前的 `*name = cpu_spec` 行填充的别名查找表。键为不含前导 `*` 的小写别名名称。 |
| `errors` | `&mut Vec<String>` | 错误累加器的可变引用。当规格不以 `*` 开头、别名名称为空或引用的别名未定义时，会推送错误。 |

## 返回值

类型：`Vec<IdealProcessorRule>`

一个 [`IdealProcessorRule`](IdealProcessorRule.md) 条目的向量。每条规则包含：
- `cpus`：从别名解析的 CPU 索引列表。
- `prefixes`：限制规则适用于哪些线程的小写模块名称前缀列表。空向量表示规则适用于所有线程。

在以下情况下返回空向量：
- `spec` 为空或 `"0"`（未请求理想处理器分配）。
- `spec` 不以 `*` 开头（记录错误）。
- 所有段解析为空 CPU 集合（具有空 CPU 集合的段被静默跳过）。

## 备注

### 规格格式

一般格式为一个或多个 `*` 分隔的段：

```/dev/null/syntax.txt#L1-3
*alias[@prefix1;prefix2;...]
*alias1[@prefix1]*alias2[@prefix2;prefix3]
```

| 组成部分 | 是否必需 | 描述 |
|----------|----------|------|
| `*` | 是 | 前缀标记，标志每个规则段的开始。 |
| `alias` | 是 | CPU 别名名称（不区分大小写），必须在配置文件的 `[ALIAS]` 部分中定义。 |
| `@` | 否 | 别名名称与前缀过滤列表之间的分隔符。 |
| `prefix1;prefix2` | 否 | 以分号分隔的模块名称前缀列表。存在时，仅启动模块以这些字符串之一开头的线程有资格从此规则获得理想处理器分配。 |

### 解析算法

1. 对输入进行修剪。如果为空或 `"0"`，则立即返回空向量。
2. 如果字符串不以 `*` 开头，则推送错误并返回空向量。
3. 按 `*` 拆分字符串（跳过前导 `*` 产生的第一个空元素）。
4. 对每个非空段：
   a. 如果段包含 `@`，则拆分为别名部分（`@` 之前）和前缀部分（`@` 之后）。
   b. 如果没有 `@`，则整个段为别名名称，前缀列表为空。
   c. 别名名称被修剪、转为小写，并在 `cpu_aliases` 中查找。
   d. 如果别名为空，则推送错误并跳过该段。
   e. 如果在映射中找不到别名，则推送错误并使用空 CPU 列表。
   f. 如果解析的 CPU 列表为空，则静默跳过该段（不创建规则）。
   g. 前缀字符串按 `;` 拆分，修剪、转为小写，并过滤掉空条目。
   h. 创建 `IdealProcessorRule { cpus, prefixes }` 并推入结果向量。

### 多段链接

可以链接多个段，将不同的 CPU 集合分配给不同组的线程。例如：

```/dev/null/example.ini#L1
*p@engine.dll;render.dll*e@helper.dll
```

这会产生两条规则：
1. 启动模块以 `engine.dll` 或 `render.dll` 开头的线程 → 使用别名 `p` 的 CPU。
2. 启动模块以 `helper.dll` 开头的线程 → 使用别名 `e` 的 CPU。

### 通配规则

不带 `@` 前缀过滤的段会创建一条适用于所有线程的通配规则：

```/dev/null/example.ini#L1
*pN01
```

这会产生一条 `prefixes` 向量为空的规则，意味着进程中的所有线程均有资格。

### 边界情况

| 输入 | 结果 | 说明 |
|------|------|------|
| `""` 或 `"0"` | `[]` | 不进行理想处理器分配。 |
| `"7"`（无前导 `*`） | `[]` + 错误 | 规格必须以 `*` 开头。 |
| `"*undefined_alias"` | `[]` | 推送错误；别名未找到，空 CPU 集合，段被跳过。 |
| `"**"` | `[]` | 两个段的别名名称均为空；推送错误，段被跳过。 |
| `"*p@"` | 规则包含来自 `p` 的 `cpus`，空 `prefixes` | `@` 存在但后面没有前缀；前缀过滤实际上为空（通配）。 |

### 可见性

此函数是模块私有的（`fn`，非 `pub fn`），仅在 `config.rs` 内部被调用。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `config.rs` |
| 可见性 | 私有（crate 内部） |
| 调用方 | [`parse_and_insert_rules`](parse_and_insert_rules.md) |
| 被调用方 | `HashMap::get`、`str::split`、`str::find`、`str::trim`、`str::to_lowercase` |
| 依赖 | [`IdealProcessorRule`](IdealProcessorRule.md)、[`collections.rs`](../collections.rs/README.md) 中的 `List` 和 `CONSUMER_CPUS` |
| 权限 | 无 |

## 另请参阅

| 资源 | 链接 |
|------|------|
| IdealProcessorRule | [IdealProcessorRule](IdealProcessorRule.md) |
| parse_and_insert_rules | [parse_and_insert_rules](parse_and_insert_rules.md) |
| resolve_cpu_spec | [resolve_cpu_spec](resolve_cpu_spec.md) |
| parse_alias | [parse_alias](parse_alias.md) |
| ThreadLevelConfig | [ThreadLevelConfig](ThreadLevelConfig.md) |
| config 模块概述 | [README](README.md) |

---
*提交：[29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*