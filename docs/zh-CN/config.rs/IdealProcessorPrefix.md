# IdealProcessorPrefix 结构体 (config.rs)

内部解析辅助结构体，用于理想处理器规格解析过程中将模块名称前缀字符串与特定的 CPU 索引集关联。

## 语法

```rust
#[derive(Debug, Clone)]
pub struct IdealProcessorPrefix {
    pub prefix: String,
    pub cpus: Vec<u32>,
}
```

## 参数

`prefix`

模块名称前缀字符串，用于匹配线程起始地址所属的模块名称。例如，`"engine.dll"` 将匹配起始地址解析到该模块的线程。比较时采用不区分大小写的匹配方式。

`cpus`

CPU 索引向量，指定匹配此前缀的线程应被分配的理想处理器。与 [PrimePrefix](PrimePrefix.md) 中 `cpus` 为可选项不同，此字段始终为必填，因为该结构体的用途就是将前缀绑定到特定的 CPU。

## 返回值

不适用（结构体定义）。

## 备注

`IdealProcessorPrefix` 是 [parse_ideal_processor_spec](parse_ideal_processor_spec.md) 在解析阶段内部使用的中间表示。解析器将原始规格字符串转换为 `IdealProcessorPrefix` 实例，随后合并为 [IdealProcessorRule](IdealProcessorRule.md) 条目并存储在 [ProcessConfig](ProcessConfig.md) 中。

在最终的 [IdealProcessorRule](IdealProcessorRule.md) 结构体中，`cpus` 和 `prefixes` 字段是分离的——多个前缀可以共享同一组 CPU。`IdealProcessorPrefix` 在合并之前的解析阶段保持它们的配对关系。

该结构体在源代码中标记了 `#[allow(dead_code)]`，因为其使用范围仅限于内部解析逻辑，不会出现在面向外部的配置输出中。

### 与 IdealProcessorRule 的关系

| 结构体 | 用途 |
| --- | --- |
| **IdealProcessorPrefix** | 解析阶段的中间产物，将一个前缀与一组 CPU 配对 |
| [IdealProcessorRule](IdealProcessorRule.md) | 最终表示形式，将多个前缀归入同一组 CPU |

## 要求

| 项目 | 值 |
| --- | --- |
| **所属模块** | src/config.rs |
| **源码行** | L24–L27 |
| **派生宏** | `Debug`, `Clone` |
| **使用者** | [parse_ideal_processor_spec](parse_ideal_processor_spec.md) |

## 另请参阅

- [IdealProcessorRule](IdealProcessorRule.md) — 存储在进程配置中的最终规则结构体
- [ProcessConfig](ProcessConfig.md) — 包含 `ideal_processor_rules` 字段
- [parse_ideal_processor_spec](parse_ideal_processor_spec.md) — 生成此结构体实例的解析函数