# IdealProcessorRule 结构体 (config.rs)

理想处理器分配规则。根据模块前缀匹配，将一组 CPU 索引映射到可选的线程过滤前缀列表，用于控制哪些线程接受理想处理器分配。

## 语法

```rust
#[derive(Debug, Clone)]
pub struct IdealProcessorRule {
    pub cpus: Vec<u32>,
    pub prefixes: Vec<String>,
}
```

## 参数

`cpus`

类型：`Vec<u32>`

已排序的 CPU 索引列表，匹配此规则的线程将被分配到这些 CPU 作为其理想处理器。调度器以轮询（round-robin）方式将线程分配到这些 CPU 上。

`prefixes`

类型：`Vec<String>`

模块名称前缀列表，用于过滤此规则适用的线程。线程的起始地址会被解析为所属模块，只有模块名称以列表中某个前缀开头的线程才会被分配 `cpus` 中指定的理想处理器。当此向量为空时，规则适用于该进程的**所有**线程。

## 返回值

不适用（结构体定义）。

## 备注

`IdealProcessorRule` 是配置解析后理想处理器规格的最终解析形式，由 [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) 通过 Windows `SetThreadIdealProcessorEx` API 为每个线程设置理想处理器。

### 规则应用顺序

规则按照在配置中出现的顺序进行评估。线程会被**第一个**前缀列表包含匹配项的规则（或第一个前缀列表为空的规则）匹配。这意味着更具体的规则（带前缀）应排在通配规则（无前缀）之前。

### 配置语法

理想处理器规则在进程规则行的第 7 个字段中指定，格式如下：

`*alias[@prefix1;prefix2]*alias2[@prefix3]`

例如，`*p@engine.dll*e@helper.dll` 创建两条规则：
- 在 `engine.dll` 中启动的线程分配到别名 `*p` 对应的 CPU
- 在 `helper.dll` 中启动的线程分配到别名 `*e` 对应的 CPU

不带 `@` 前缀的规则（例如 `*p`）作为通配规则适用于所有线程。

### 与其他类型的关系

- 规则由 [parse_ideal_processor_spec](parse_ideal_processor_spec.md) 解析，并存储在 `ProcessConfig::ideal_processor_rules` 中。
- CPU 别名引用（如 `*p`）通过 [parse_alias](parse_alias.md) 定义的别名进行解析。

| 结构体 | 用途 |
| --- | --- |
| **IdealProcessorRule** | 将多个前缀归入同一组 CPU 的最终表示 |

## 要求

| 条目 | 值 |
| --- | --- |
| **所属模块** | src/config.rs |
| **派生宏** | `Debug`, `Clone` |
| **使用方** | [ProcessConfig](ProcessConfig.md)、[parse_ideal_processor_spec](parse_ideal_processor_spec.md)、[apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |

## 另请参阅

- [ProcessConfig](ProcessConfig.md) — 包含 `ideal_processor_rules` 字段
- [parse_ideal_processor_spec](parse_ideal_processor_spec.md) — 解析理想处理器规格的函数
- [parse_alias](parse_alias.md) — 定义 CPU 别名
