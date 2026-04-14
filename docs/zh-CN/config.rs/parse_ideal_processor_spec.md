# parse_ideal_processor_spec 函数 (config.rs)

将理想处理器规格字符串解析为 [IdealProcessorRule](IdealProcessorRule.md) 条目的向量。规格格式使用 `*alias` 段来引用 CPU 别名，并可选地使用 `@prefix` 后缀根据线程起始地址的模块名称来过滤哪些线程接受理想处理器分配。

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

| 参数 | 类型 | 描述 |
|------|------|------|
| `spec` | `&str` | 规则行第 7 个字段中的理想处理器规格字符串。除非值为 `"0"` 或空字符串，否则必须以 `*` 开头。前后空白会被裁剪。 |
| `line_number` | `usize` | 配置文件中的 1 基行号。用于错误消息，帮助用户定位问题。 |
| `cpu_aliases` | `&HashMap<String, Vec<u32>>` | 先前定义的 CPU 别名映射，键为小写的别名名称（不含 `*` 前缀）。由解析过程中更早的 [parse_alias](parse_alias.md) 调用构建。 |
| `errors` | `&mut Vec<String>` | 错误累加器的可变引用。未定义的别名、空别名名称或不以 `*` 开头的规格所产生的错误会被推入此列表。 |

## 返回值

返回 `Vec<IdealProcessorRule>`。每个元素将一组 CPU 索引映射到可选的模块名称前缀过滤器列表。

在以下情况返回空向量：

- `spec` 为空或仅包含空白字符。
- `spec` 为 `"0"`（标准的"禁用"哨兵值）。
- `spec` 不以 `*` 开头（同时会推入一条错误）。
- 所有引用的别名解析为空 CPU 集合（CPU 列表为空的段会被静默跳过）。

## 备注

### 规格格式

规格字符串由一个或多个以 `*` 分隔的段组成：

```
*alias[@prefix1;prefix2]*alias2[@prefix3;prefix4]
```

每个段具有以下结构：

| 组件 | 必需 | 描述 |
|------|------|------|
| `*` | 是 | 段分隔符和别名标记。规格必须以 `*` 开头。 |
| `alias` | 是 | CPU 别名名称（不区分大小写）。小写化后在 `cpu_aliases` 中查找。必须在配置文件中先通过 `*alias = cpu_spec` 定义。 |
| `@` | 否 | 别名名称与前缀过滤器列表之间的分隔符。当缺省时，规则适用于所有线程。 |
| `prefix1;prefix2` | 否 | 以分号分隔的模块名称前缀列表。每个前缀被裁剪、小写化，并存储在结果 [IdealProcessorRule](IdealProcessorRule.md)`.prefixes` 向量中。 |

### 解析算法

1. **裁剪和提前退出** — 规格被裁剪。如果为空或等于 `"0"`，立即返回空向量。
2. **前缀验证** — 如果规格不以 `*` 开头，推入错误并返回空向量。
3. **段分割** — 按 `*` 分割，跳过由前导 `*` 产生的第一个（空）元素。
4. **逐段处理：**
   a. 跳过空段（由连续的 `**` 产生）。
   b. 按 `@` 分割，将别名部分与可选的前缀部分分离。
   c. 将别名名称小写化并在 `cpu_aliases` 中查找。
   d. 如果别名未知，推入错误并使用空 CPU 向量。
   e. 如果解析后的 CPU 向量为空，跳过该段（不生成规则）。
   f. 将前缀部分按 `;` 分割，裁剪并小写化每个条目，过滤掉空字符串。
   g. 使用解析后的 `cpus` 和 `prefixes` 构造一个 [IdealProcessorRule](IdealProcessorRule.md)，并推入结果向量。

### 错误条件

| 条件 | 错误消息格式 | 行为 |
|------|-------------|------|
| 规格不以 `*` 开头 | `"Line {N}: Ideal processor spec must start with '*', got '{spec}'"` | 返回空向量。 |
| 段中别名名称为空 | `"Line {N}: Empty alias in ideal processor rule '*{segment}'"` | 跳过该段；继续解析。 |
| 未定义的别名名称 | `"Line {N}: Unknown CPU alias '*{alias}' in ideal processor specification"` | 该段产生空 CPU 向量并被跳过；继续解析。 |

### 示例

给定别名 `{ "p": [0, 1, 2, 3], "e": [4, 5, 6, 7] }`：

| 规格字符串 | 结果 | 说明 |
|-----------|------|------|
| `"0"` | `[]` | 已禁用。 |
| `""` | `[]` | 已禁用。 |
| `"*p"` | `[IdealProcessorRule { cpus: [0,1,2,3], prefixes: [] }]` | 所有线程 → CPU 0–3。 |
| `"*p@engine.dll"` | `[IdealProcessorRule { cpus: [0,1,2,3], prefixes: ["engine.dll"] }]` | 仅 `engine.dll` 线程 → CPU 0–3。 |
| `"*p@engine.dll;render.dll*e@audio.dll"` | 两条规则：`engine.dll`/`render.dll` → `p`，`audio.dll` → `e`。 | 链接多个段。 |
| `"*p@*e"` | `[{ cpus: [0..3], prefixes: [] }, { cpus: [4..7], prefixes: [] }]` | 第二个段中没有 `@` — `e` 是别名名称；无前缀过滤器。 |
| `"*unknown"` | `[]` | 为未定义别名推入错误；空 CPU 导致段被跳过。 |
| `"0-7"` | `[]` | 推入错误（不以 `*` 开头）。 |

### 与规则解析的交互

`parse_ideal_processor_spec` 在处理规则行第 7 个字段时由 [parse_and_insert_rules](parse_and_insert_rules.md) 调用。返回的向量直接存储在 [ProcessConfig](ProcessConfig.md)`.ideal_processor_rules` 中。如果第 7 个字段可以解析为纯整数（等级），则不会被解释为理想处理器规格——该函数仅在字段以 `*` 开头或无法解析为等级时才被调用。

### 运行时应用

在应用时，[apply_ideal_processors](../apply.rs/apply_ideal_processors.md) 函数遍历返回的规则。对于每条规则，它遍历进程的线程，检查每个线程的起始地址模块名称是否匹配 `prefixes`（如果 `prefixes` 为空则匹配所有线程），并通过 `SetThreadIdealProcessorEx` 在 `cpus` 中轮询分配理想处理器。

### 空前缀与无前缀

[IdealProcessorRule](IdealProcessorRule.md) 的 `prefixes` 向量为空意味着"匹配所有线程"。这与完全被跳过的规则（在向量中不产生任何条目）不同。空前缀的情况出现在段没有 `@` 分隔符时——例如，`*p` 创建一条适用于进程中每个线程的规则。

## 要求

| | |
|---|---|
| **模块** | `config` (`src/config.rs`) |
| **可见性** | 包内私有 |
| **调用者** | [parse_and_insert_rules](parse_and_insert_rules.md) |
| **被调用者** | 无（通过 `HashMap::get` 查找别名） |
| **API** | 纯函数 — 无 I/O，无 Windows API 调用 |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 理想处理器规则结构体 | [IdealProcessorRule](IdealProcessorRule.md) |
| 规则字段解析（调用者） | [parse_and_insert_rules](parse_and_insert_rules.md) |
| CPU 别名定义 | [parse_alias](parse_alias.md) |
| 别名感知的 CPU 规格解析 | [resolve_cpu_spec](resolve_cpu_spec.md) |
| 进程级配置 | [ProcessConfig](ProcessConfig.md) |
| 运行时理想处理器应用 | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |
| 模块概览 | [config 模块](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd