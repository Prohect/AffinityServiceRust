# parse_alias 函数 (config.rs)

解析配置文件中的 `*alias = cpu_spec` 行，将 CPU 规格解析为 CPU 索引列表，并将别名插入到 CPU 别名映射表中。别名为 CPU 集合提供符号名称，可在规则字段中通过 `*alias` 语法引用。

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

| 参数 | 类型 | 描述 |
|------|------|------|
| `name` | `&str` | 从前导 `*` 和 `=` 号之间提取的别名名称，已由调用方修剪空白并转换为小写。例如，对于 `*perf = 0-7`，`name` 为 `"perf"`。 |
| `value` | `&str` | `=` 号右侧的原始 CPU 规格字符串，已由调用方修剪空白。直接传递给 [parse_cpu_spec](parse_cpu_spec.md) 进行解析。 |
| `line_number` | `usize` | 别名定义在配置文件中的行号（从 1 开始）。用于错误和警告消息。 |
| `cpu_aliases` | `&mut HashMap<String, Vec<u32>>` | 在配置解析过程中构建的别名映射表的可变引用。解析成功时，已解析的别名将被插入（或覆盖同名的先前条目）。 |
| `result` | `&mut ConfigResult` | 当前 [ConfigResult](ConfigResult.md) 的可变引用。成功时 `aliases_count` 计数器递增。错误和警告分别推入 `result.errors` 和 `result.warnings`。 |

## 返回值

此函数没有返回值。结果通过对 `cpu_aliases` 和 `result` 的修改来传达。

## 备注

### 验证

- **空名称** — 如果 `name` 为空，将向 `result.errors` 推入一条错误消息 `"Line {N}: Empty alias name"`，函数直接返回，不插入任何内容。
- **空 CPU 集合** — 如果 [parse_cpu_spec](parse_cpu_spec.md) 返回空向量且 `value` 不是字面字符串 `"0"`，将推入一条警告，指出别名解析为空 CPU 集合。别名仍会以空向量被插入；这允许配置文件定义一个占位别名，以禁用引用它的功能。

### 覆盖

如果 `cpu_aliases` 中已存在同名的别名，它将被静默覆盖。重新定义不会生成警告或错误。这允许后面的别名行覆盖先前的定义。

### 配置文件语法

配置文件中的别名行遵循以下格式：

```
*alias_name = cpu_spec
```

其中：

- `*` 是标识该行为别名定义的必需前缀标记。
- `alias_name` 是不区分大小写的标识符（由 [read_config](read_config.md) 中的调用方在解析时转换为小写）。
- `cpu_spec` 是 [parse_cpu_spec](parse_cpu_spec.md) 接受的任意格式：十六进制掩码 (`0xFF`)、范围 (`0-7`)、分号分隔列表 (`0;4;8`) 或混合格式 (`0-3;8`)。

**示例：**

```
*perf = 0-7
*efficiency = 8-15
*all = 0-15
*gaming = 0;2;4;6
*legacy = 0xFF
```

### 别名用法

定义后，可通过在别名名称前加 `*` 在规则字段中引用别名：

- **CPU 亲和性 / CPU 集合 / 主线程 CPU** — 由 [resolve_cpu_spec](resolve_cpu_spec.md) 解析（例如 `process.exe:high:*perf:*perf`）。
- **理想处理器规则** — 由 [parse_ideal_processor_spec](parse_ideal_processor_spec.md) 解析（例如 `*perf@engine.dll`）。

### 解析顺序

别名必须在引用它们的规则行之前定义。[read_config](read_config.md) 从上到下处理文件，因此放在文件顶部的别名行对所有后续规则可用。

## 要求

| | |
|---|---|
| **模块** | `config` (`src/config.rs`) |
| **可见性** | 包内私有 |
| **调用方** | [read_config](read_config.md) |
| **被调用方** | [parse_cpu_spec](parse_cpu_spec.md) |
| **API** | 无 Windows API 调用 |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| CPU 规格解析器 | [parse_cpu_spec](parse_cpu_spec.md) |
| 支持别名的 CPU 规格解析 | [resolve_cpu_spec](resolve_cpu_spec.md) |
| 配置文件读取器（调用方） | [read_config](read_config.md) |
| 常量解析（类似的 `@` 行） | [parse_constant](parse_constant.md) |
| 理想处理器规格（别名消费者） | [parse_ideal_processor_spec](parse_ideal_processor_spec.md) |
| 解析输出聚合 | [ConfigResult](ConfigResult.md) |
| 模块概述 | [config 模块](README.md) |