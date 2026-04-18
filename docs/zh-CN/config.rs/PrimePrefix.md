# PrimePrefix 类型 (config.rs)

`PrimePrefix` 结构体将线程启动模块前缀字符串与可选的 CPU 索引集合和线程优先级覆盖值关联。它在 [`ThreadLevelConfig`](ThreadLevelConfig.md) 中使用，用于控制当主线程的启动模块与给定前缀匹配时，应分配哪些 CPU，以及可选地提升或降低这些线程的调度优先级。

## 语法

```AffinityServiceRust/src/config.rs#L17-21
#[derive(Debug, Clone)]
pub struct PrimePrefix {
    pub prefix: String,
    pub cpus: Option<List<[u32; CONSUMER_CPUS]>>,
    pub thread_priority: ThreadPriority,
}
```

## 成员

| 成员 | 类型 | 描述 |
|------|------|------|
| `prefix` | `String` | 不区分大小写的线程启动模块名称前缀，用于匹配（例如 `"engine.dll"`、`"render"`）。空字符串表示匹配所有线程，不考虑其启动模块。 |
| `cpus` | `Option<List<[u32; CONSUMER_CPUS]>>` | 可选的 CPU 索引列表，匹配此前缀的线程应被调度到这些 CPU 上。当为 `Some` 时，覆盖父规则的基础 `prime_threads_cpus`。当为 `None` 时，线程继承包含它的 [`ThreadLevelConfig`](ThreadLevelConfig.md) 的基础 CPU 集合。 |
| `thread_priority` | `ThreadPriority` | 应用于匹配线程的可选线程级优先级。`ThreadPriority::None` 表示不修改优先级（保留自动提升行为）。通过配置语法中的 `!priority` 后缀设置（例如 `engine.dll!above normal`）。 |

## 备注

### 配置语法

在配置文件中，`PrimePrefix` 值在规则行的主线程字段（字段 4）中指定。一般语法为：

```/dev/null/example.ini#L1-3
process.exe:normal:0:0:*alias@prefix1;prefix2!priority:none:none:0:1
```

`@prefix` 部分按 `;` 分割，为每个条目生成一个 `PrimePrefix`。`!priority` 后缀是可选的，通过 `ThreadPriority::from_str` 解析。

### 多段前缀规则

当多个 `*alias@prefix` 段被链接在一起时（例如 `*p@engine.dll*e@helper.dll`），每个段会生成自己的 `PrimePrefix` 实例集合，并将对应别名的 CPU 存储在 `cpus` 中。这允许不同组的线程根据其启动模块被调度到不同的 CPU 集合上。

### 默认行为

当规则指定了主线程 CPU 但没有任何 `@prefix` 过滤时，会创建一个 `PrimePrefix`，其 `prefix` 为空（匹配所有线程），`cpus` 设为 `None`（从父级继承），`thread_priority` 设为 `ThreadPriority::None`（不覆盖）。

### 生命周期

`PrimePrefix` 实例在配置解析期间由 [`parse_and_insert_rules`](parse_and_insert_rules.md) 创建，并存储在 [`ThreadLevelConfig`](ThreadLevelConfig.md) 中。它们在运行时被 [`PrimeThreadScheduler`](../scheduler.rs/README.md) 使用，用于决定哪些线程具有"主线程"资格以及如何调度它们。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `config.rs` |
| 构造方式 | [`parse_and_insert_rules`](parse_and_insert_rules.md) |
| 包含于 | [`ThreadLevelConfig`](ThreadLevelConfig.md)（字段 `prime_threads_prefixes`） |
| 使用方 | `PrimeThreadScheduler`（见 [scheduler.rs](../scheduler.rs/README.md)） |
| 派生 | `Debug`、`Clone` |

## 另请参阅

| 资源 | 链接 |
|------|------|
| ThreadLevelConfig | [ThreadLevelConfig](ThreadLevelConfig.md) |
| IdealProcessorRule | [IdealProcessorRule](IdealProcessorRule.md) |
| parse_and_insert_rules | [parse_and_insert_rules](parse_and_insert_rules.md) |
| priority 模块 | [priority.rs 概述](../priority.rs/README.md) |
| config 模块概述 | [README](README.md) |

---
*提交：[29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*