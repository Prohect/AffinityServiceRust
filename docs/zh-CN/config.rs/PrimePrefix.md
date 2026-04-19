# PrimePrefix 结构体 (config.rs)

将模块名称前缀与 CPU 集合和可选的线程优先级提升关联，用于 Prime 线程匹配。当 Prime 线程调度器将线程识别为"Prime 线程"（高活动性）时，它会检查线程的启动模块与存储的 `PrimePrefix` 条目是否匹配，以确定该线程应绑定到哪些 CPU，以及是否应提升其优先级。

## 语法

```rust
#[derive(Debug, Clone)]
pub struct PrimePrefix {
    pub prefix: String,
    pub cpus: Option<List<[u32; CONSUMER_CPUS]>>,
    pub thread_priority: ThreadPriority,
}
```

## 成员

| 成员 | 类型 | 描述 |
|--------|------|-------------|
| `prefix` | `String` | 要与线程启动模块匹配的名称前缀（例如，`"engine.dll"`）。空字符串表示匹配任何线程，无论其启动模块是什么。比较不区分大小写。 |
| `cpus` | `Option<List<[u32; CONSUMER_CPUS]>>` | 用于绑定匹配的 Prime 线程的 CPU 索引。当为 `Some` 时，覆盖父级 [ThreadLevelConfig](ThreadLevelConfig.md) 中的 `prime_threads_cpus`，用于匹配此前缀的线程。当为 `None` 时，使用父级的 `prime_threads_cpus` 作为回退。 |
| `thread_priority` | [ThreadPriority](../priority.rs/ThreadPriority.md) | 线程提升为 Prime 线程状态时应用的可选优先级提升。`ThreadPriority::None` 表示无优先级更改（自动提升行为）。在配置中使用 `!priority` 后缀语法指定（例如，`engine.dll!above normal`）。 |

## 备注

### 配置语法

`PrimePrefix` 条目从规则行的 Prime 线程字段（字段 4）解析。该格式支持每前缀的 CPU 覆盖和优先级提升：

```
process.exe:normal:0:0:*pcore@engine.dll;helper.dll!above normal:none:none:0:1
```

在此示例中：
- `*pcore` 引用定义要分配哪些 CPU 的 CPU 别名。
- `@engine.dll;helper.dll!above normal` 定义两个前缀：`engine.dll`（无优先级提升）和 `helper.dll`（提升至高于正常）。

### 匹配行为

- 当 `prefix` 为空（`""`）时，该条目作为捕获所有规则，匹配任何线程，无论其启动模块是什么。
- 可以为单个进程规则存在多个 `PrimePrefix` 条目。调度器按顺序评估它们，线程匹配其字符串是线程启动模块名前缀的第一个前缀。
- `cpus` 字段允许将来自不同模块的线程引导到同一进程规则内的不同 CPU 核心。例如，渲染线程可以进入 P 核，而音频线程可以进入 E 核。

### 默认构造

当 Prime 线程字段中未使用 `@prefix` 语法时，会创建单个 `PrimePrefix`，其中 `prefix` 为空，`cpus` 为 `None`，`thread_priority` 为 `ThreadPriority::None`。这意味着该进程的所有 Prime 线程使用父级配置的 CPU 集合，无优先级提升。

## 要求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **使用者** | [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md), [apply_prime_threads_demote](../apply.rs/apply_prime_threads_demote.md) |
| **依赖项** | [ThreadPriority](../priority.rs/ThreadPriority.md), [List](../collections.rs/README.md) |
| **权限** | 无（仅数据结构） |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| 父级配置结构体 | [ThreadLevelConfig](ThreadLevelConfig.md) |
| Prime 线程提升 | [apply_prime_threads_promote](../apply.rs/apply_prime_threads_promote.md) |
| 线程优先级枚举 | [ThreadPriority](../priority.rs/ThreadPriority.md) |
| 规则解析器 | [parse_and_insert_rules](parse_and_insert_rules.md) |
| 模块概述 | [config.rs](README.md) |

*文档针对提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*