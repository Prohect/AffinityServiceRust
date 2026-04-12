# parse_ideal_processor_spec 函数 (config.rs)

解析理想处理器规格字符串，生成 [IdealProcessorRule](IdealProcessorRule.md) 条目列表，支持 CPU 别名解析和可选的模块前缀过滤。

## 语法

```rust
fn parse_ideal_processor_spec(
    spec: &str,
    line_number: usize,
    cpu_aliases: &HashMap<String, Vec<u32>>,
    errors: &mut Vec<String>,
) -> Vec<IdealProcessorRule>
```

## 参数

`spec`

待解析的理想处理器规格字符串。每个规则段必须以 `*` 开头。格式为 `*alias[@prefix1;prefix2]`，可链式拼接多个段：`*p@engine.dll*e@helper.dll`。值为 `"0"` 或空字符串表示不设置理想处理器规则。

`line_number`

该规格在配置文件中的行号（从 1 开始），用于错误报告。

`cpu_aliases`

别名名称（不含 `*` 前缀）到已解析 CPU 索引向量的映射。规格中引用的每个别名都必须存在于此映射中，否则会记录错误。

`errors`

可变向量引用，当规格格式错误或引用了未定义的别名时，解析错误消息将追加到此向量。

## 返回值

返回 `Vec<`[IdealProcessorRule](IdealProcessorRule.md)`>`，包含已解析的规则。每条规则具有 `cpus` 字段（从别名解析得到）和 `prefixes` 字段（从 `@` 部分解析的模块名过滤条件）。当规格为 `"0"`、空字符串或完全无效时，返回空向量。

## 备注

该规格采用分段格式，每个段以 `*` 开头：

| 标记 | 含义 |
| --- | --- |
| `*` | 段分隔符兼别名标记 |
| `alias` | CPU 别名名称（通过 `cpu_aliases` 解析） |
| `@` | 别名与前缀过滤列表之间的分隔符 |
| `;` | 多个前缀过滤条件之间的分隔符 |

### 解析算法

1. 若 `spec` 为空或等于 `"0"`，立即返回空向量。
2. 若 `spec` 不以 `*` 开头，推送错误并返回空向量。
3. 以 `*` 分割 `spec`，跳过第一个空段。
4. 对每个段：
   - 以 `@` 分割，将别名部分与可选前缀部分分离。
   - 在 `cpu_aliases` 中查找别名（小写化后）。若未找到，推送错误并继续下一段。
   - 若解析出的 CPU 列表为空，跳过该段。
   - 以 `;` 分割前缀部分，对每个条目进行修剪和小写化处理，过滤空条目。
   - 使用解析得到的 CPU 列表和前缀列表构造 [IdealProcessorRule](IdealProcessorRule.md)。

### 示例

| 规格 | 结果 |
| --- | --- |
| `*p` | 一条规则：别名 `p` 的 CPU，无前缀过滤（适用于所有线程） |
| `*p@engine.dll` | 一条规则：别名 `p` 的 CPU，仅适用于 `engine.dll` 中启动的线程 |
| `*p@engine.dll;render.dll` | 一条规则：别名 `p` 的 CPU，适用于 `engine.dll` 或 `render.dll` 中的线程 |
| `*p@engine.dll*e@helper.dll` | 两条规则：别名 `p` 对应 `engine.dll`，别名 `e` 对应 `helper.dll` |

当规则的 `prefixes` 向量为空时，该规则充当兜底规则，适用于未被更具体前缀规则匹配的所有线程。[apply_ideal_processors](../apply.rs/apply_ideal_processors.md) 在线程调度时使用此机制。

本函数由 [parse_and_insert_rules](parse_and_insert_rules.md) 在处理配置规则行的理想处理器字段（字段索引 6）时内部调用。

## 要求

| 要求 | 值 |
| --- | --- |
| **所属模块** | src/config.rs |
| **可见性** | 私有 (`fn`) |
| **源码行** | L310–L379 |
| **调用方** | [parse_and_insert_rules](parse_and_insert_rules.md) |
| **依赖** | [IdealProcessorRule](IdealProcessorRule.md)，[resolve_cpu_spec](resolve_cpu_spec.md)（通过别名映射间接依赖） |

## 另请参阅

- [IdealProcessorRule](IdealProcessorRule.md) — 本函数生成的结构体
- [parse_and_insert_rules](parse_and_insert_rules.md) — 在规则解析过程中调用本函数
- [ProcessConfig](ProcessConfig.md) — 在 `ideal_processor_rules` 字段中存储已解析的规则
