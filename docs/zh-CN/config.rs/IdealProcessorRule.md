# IdealProcessorRule 结构体 (config.rs)

将一组 CPU 索引映射到线程起始模块前缀列表，形成理想处理器分配的单个规则。当服务遍历进程的线程时，任何起始模块匹配 `prefixes` 中任一前缀的线程都会获得从 `cpus` 中选取的理想处理器提示。如果 `prefixes` 为空，则规则无条件适用于所有线程。

## 语法

```rust
#[derive(Debug, Clone)]
pub struct IdealProcessorRule {
    pub cpus: List<[u32; CONSUMER_CPUS]>,
    pub prefixes: Vec<String>,
}
```

## 成员

| 成员 | 类型 | 描述 |
|--------|------|-------------|
| `cpus` | `List<[u32; CONSUMER_CPUS]>` | 用于分配理想处理器提示的逻辑处理器索引排序列表。在解析时从 CPU 别名解析。必须非空，规则才能生效。 |
| `prefixes` | `Vec<String>` | 用于筛选线程的小写模块名前缀。如果线程的起始模块以任一条目开头，则该线程匹配。空向量意味着规则匹配进程中的每个线程。 |

## 备注

`IdealProcessorRule` 由 [parse_ideal_processor_spec](parse_ideal_processor_spec.md) 从配置规则行的理想处理器字段（字段 7）生成。该规格格式使用 `*alias@prefix1;prefix2` 段，其中每个段成为单个 `IdealProcessorRule` 实例。多个段可以链接，以便在同一进程中为来自不同模块的线程分配不同的 CPU 集合。

### 理想处理器与亲和性

理想处理器提示是一种**软偏好**——Windows 调度器会偏好提示的核，但在高负载下可能将线程调度到其他位置。这与硬亲和性形成对比，硬亲和性将线程严格限制在一组 CPU 上。因此，理想处理器规则可用于指导线程放置，而不会引入饥饿风险。

### 线程匹配

线程到规则的匹配由 [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) 在每次轮询迭代期间执行。对于每个线程，服务通过缓存的事件追踪 for Windows (ETW) 子系统数据查询线程的起始地址模块。`prefixes` 列表包含匹配前缀的第一个规则（或其列表为空）决定理想 CPU。

### CPU 分布

当多个线程匹配同一规则时，服务通过 `cpus` 列表轮询分配理想处理器分配，以均匀分散负载。

### 配置语法示例

```
*pcore = 0-7
*ecore = 8-19
game.exe:normal:0:0:0:none:none:*pcore@engine.dll;render.dll*ecore@audio.dll
```

这产生两个 `IdealProcessorRule` 条目：

1. `cpus: [0..7], prefixes: ["engine.dll", "render.dll"]`
2. `cpus: [8..19], prefixes: ["audio.dll"]`

## 要求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **由...生成** | [parse_ideal_processor_spec](parse_ideal_processor_spec.md), [parse_and_insert_rules](parse_and_insert_rules.md) |
| **存储在** | [ThreadLevelConfig](ThreadLevelConfig.md)`.ideal_processor_rules` |
| **由...使用** | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |
| **集合类型** | [List](../collections.rs/README.md) (`SmallVec<[u32; CONSUMER_CPUS]>`) |
| **权限** | 无（数据结构） |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [config.rs](README.md) |
| 父配置结构 | [ThreadLevelConfig](ThreadLevelConfig.md) |
| 规格解析器 | [parse_ideal_processor_spec](parse_ideal_processor_spec.md) |
| 运行时应用 | [apply_ideal_processors](../apply.rs/apply_ideal_processors.md) |
| CPU 别名解析 | [parse_alias](parse_alias.md) |
| Prime 线程前缀（相关概念） | [PrimePrefix](PrimePrefix.md) |

*文档针对 Commit：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*