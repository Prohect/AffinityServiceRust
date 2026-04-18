# IdealProcessorRule 类型 (config.rs)

`IdealProcessorRule` 结构体将一组 CPU 索引映射到可选的线程启动模块名称前缀列表，用于理想处理器分配。当调度器为进程的线程分配理想处理器时，它使用这些规则来确定应优先使用哪些 CPU，并且可以选择性地将分配限制为启动模块与指定前缀之一匹配的线程。

## 语法

```AffinityServiceRust/src/config.rs#L24-27
#[derive(Debug, Clone)]
pub struct IdealProcessorRule {
    pub cpus: List<[u32; CONSUMER_CPUS]>,
    pub prefixes: Vec<String>,
}
```

## 成员

| 成员 | 类型 | 描述 |
|------|------|------|
| `cpus` | `List<[u32; CONSUMER_CPUS]>` | 调度器应分配为理想处理器的已排序 CPU 索引列表。列表中的 CPU 数量决定了有多少个排名靠前的线程（按总 CPU 时间排名）会从此规则获得理想处理器分配。 |
| `prefixes` | `Vec<String>` | 用于过滤此规则适用于哪些线程的小写模块名称前缀列表。当为空时，规则适用于进程的**所有**线程。当非空时，仅考虑启动模块名称以这些前缀之一开头的线程。 |

## 备注

### 与理想处理器规格的关系

`IdealProcessorRule` 实例由 [`parse_ideal_processor_spec`](parse_ideal_processor_spec.md) 在配置文件包含形如 `*alias[@prefix1;prefix2]` 的理想处理器字段时生成。当存在多个 `*` 分隔的段时（例如 `*p@engine.dll*e@helper.dll`），单个规格字符串可以产生多条规则。

### 线程选择算法

调度器使用 `cpus` 列表来确定应从此规则获得理想处理器分配的排名靠前的线程数量（N = `cpus.len()`）。线程按其累计 CPU 时间排名。当线程跌出前 N 名时，它会回退到之前的理想处理器值。

### 前缀匹配

前缀字符串以小写存储，并与线程启动模块名称的小写形式进行比较。这允许匹配部分模块名称——例如，前缀 `engine` 可匹配 `engine.dll`、`engine_worker.dll` 等。

### 边界情况

- 如果 `cpus` 为空（例如，因为引用的别名解析为空 CPU 集合），则该规则在解析期间会被**跳过**，不会出现在最终的 `Vec<IdealProcessorRule>` 中。
- 如果 `prefixes` 为空，则该规则是一个通配规则，适用于进程中的每个线程。
- 如果前缀过滤存在重叠，多条规则可以定位到同一个线程；调度器按顺序处理规则。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `config.rs` |
| 构造方式 | [`parse_ideal_processor_spec`](parse_ideal_processor_spec.md) |
| 存储位置 | [`ThreadLevelConfig.ideal_processor_rules`](ThreadLevelConfig.md) |
| 使用方 | `scheduler.rs`（主线程调度器理想处理器分配） |
| 依赖 | `collections.rs` 中的 `List`、`CONSUMER_CPUS` 常量 |

## 另请参阅

| 资源 | 链接 |
|------|------|
| ThreadLevelConfig | [ThreadLevelConfig](ThreadLevelConfig.md) |
| parse_ideal_processor_spec | [parse_ideal_processor_spec](parse_ideal_processor_spec.md) |
| PrimePrefix | [PrimePrefix](PrimePrefix.md) |
| parse_and_insert_rules | [parse_and_insert_rules](parse_and_insert_rules.md) |
| scheduler 模块 | [scheduler.rs 概述](../scheduler.rs/README.md) |
| config 模块概述 | [README](README.md) |

---
*提交：[29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*