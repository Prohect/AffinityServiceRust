# IdealProcessorRule 结构体 (config.rs)

定义从一组 CPU 索引到可选模块名前缀过滤器的映射，用于理想处理器分配。当服务将理想处理器规则应用于进程的线程时，会检查每个线程的起始地址模块名是否与 `prefixes` 列表匹配。如果列表为空（无过滤器），则所有线程都符合条件；否则，只有起始模块名与某个前缀匹配的线程才会从 `cpus` 集合中分配理想处理器。

## 语法

```rust
#[derive(Debug, Clone)]
pub struct IdealProcessorRule {
    pub cpus: Vec<u32>,
    pub prefixes: Vec<String>,
}
```

## 成员

| 成员 | 类型 | 描述 |
|------|------|------|
| `cpus` | `Vec<u32>` | 已排序的逻辑 CPU 索引列表，匹配此规则的线程应作为理想处理器分布在这些索引上。这些索引在解析时通过 [parse_ideal_processor_spec](parse_ideal_processor_spec.md) 从 CPU 别名解析而来。apply 模块以轮询方式在此集合上分配线程的理想处理器。 |
| `prefixes` | `Vec<String>` | 模块名前缀过滤器。每个条目是一个小写字符串，与线程起始地址的已解析模块名进行匹配（例如 `"engine.dll"`、`"ntdll"`）。空向量表示该规则无条件地应用于所有线程。非空时，只有起始地址模块名以其中一个前缀开头的线程才会从 `cpus` 中分配理想处理器。 |

## 备注

### 配置语法

理想处理器规则出现在规则行的第 7 个字段（`ideal_processor` 位置）。规范格式为：

```
*alias[@prefix1;prefix2]*alias2[@prefix3]
```

其中：

- 每个段以 `*` 开头，后跟一个 CPU 别名名称（必须事先通过 `*alias = cpu_spec` 定义）。
- 可选的 `@` 后缀提供分号分隔的模块名前缀，用于过滤哪些线程从该段的 CPU 集合中接收理想处理器分配。
- 可以链接多个段，为单个进程创建多个 `IdealProcessorRule` 条目。

**示例：**

| 规范字符串 | 结果 |
|------------|------|
| `*perf` | 一条规则：所有线程从别名 `perf` 的 CPU 获取理想处理器。 |
| `*perf@engine.dll` | 一条规则：只有在 `engine.dll` 中启动的线程从 `perf` 获取理想处理器。 |
| `*p@engine.dll;render.dll*e@audio.dll` | 两条规则：`engine.dll`/`render.dll` 线程 → 别名 `p` 的 CPU；`audio.dll` 线程 → 别名 `e` 的 CPU。 |
| `0` | 无规则（禁用理想处理器分配）。 |

### 分配行为

在运行时，apply 模块遍历进程的线程，对于每个 `IdealProcessorRule`，按顺序循环遍历 `cpus` 来分配理想处理器。这将线程调度提示分布到指定的核心上。分配使用 `SetThreadIdealProcessorEx`，仅应用于匹配前缀过滤器的线程。

### 空的 cpus

如果引用的别名解析为空的 CPU 集合，则在解析期间会静默跳过该规则，不会为该段生成 `IdealProcessorRule` 条目。

### 与 CPU 集合的交互

当进程同时配置了带 `@` 前缀的 `cpu_set_cpus`（即 `cpu_set_reset_ideal` 为 true）时，会先调用 `reset_thread_ideal_processors`，将理想处理器分布到 CPU 集合上。此结构体中的理想处理器规则会单独应用，并可能为特定线程覆盖这些分配。

## 要求

| | |
|---|---|
| **模块** | `config` (`src/config.rs`) |
| **构造者** | [parse_ideal_processor_spec](parse_ideal_processor_spec.md) |
| **使用者** | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |
| **父结构体** | [ProcessConfig](ProcessConfig.md)（字段 `ideal_processor_rules`） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 每进程配置记录 | [ProcessConfig](ProcessConfig.md) |
| 理想处理器规范解析器 | [parse_ideal_processor_spec](parse_ideal_processor_spec.md) |
| 理想处理器应用逻辑 | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |
| CPU 别名解析 | [parse_alias](parse_alias.md) |
| CPU 别名解析 | [resolve_cpu_spec](resolve_cpu_spec.md) |
| 模块概述 | [config 模块](README.md) |