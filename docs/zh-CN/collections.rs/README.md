# collections 模块 (AffinityServiceRust)

`collections` 模块提供了项目范围的高性能集合类型别名和用于栈分配小向量的容量常量。它从 `rustc_hash` crate 中重新导出 `FxHashMap` 和 `FxHashSet`（使用快速的非加密哈希）分别作为 `HashMap` 和 `HashSet`，并从 `smallvec` crate 中重新导出 `SmallVec` 作为 `List`。该模块还定义了一组 `usize` 常量，用于指定应用程序中 `SmallVec` 数组的内联容量，这些容量针对进程 ID、线程 ID、CPU 集合和待处理操作的典型工作负载大小进行了调优。

## 类型别名

| 类型 | 描述 |
|------|------|
| [HashMap](HashMap.md) | `FxHashMap<K, V>` 的别名，一种使用 Fx (Firefox) 非加密哈希函数进行快速查找的哈希映射。 |
| [HashSet](HashSet.md) | `FxHashSet<V>` 的别名，一种使用 Fx 非加密哈希函数的哈希集合。 |
| [List](List.md) | `SmallVec<E>` 的别名，一种在溢出到堆之前将元素内联存储到固定容量的向量。 |

## 常量

| 常量 | 值 | 描述 |
|------|-----|------|
| [PIDS](PIDS.md) | `256` | 用于存储进程 ID 的 `SmallVec` 数组的内联容量。 |
| [TIDS_FULL](TIDS_FULL.md) | `96` | 用于存储完整线程 ID 集合的 `SmallVec` 数组的内联容量。 |
| [TIDS_CAPED](TIDS_CAPED.md) | `32` | 用于存储有上限（受限）线程 ID 集合的 `SmallVec` 数组的内联容量。 |
| [CONSUMER_CPUS](CONSUMER_CPUS.md) | `32` | 用于存储 CPU 集合 ID 或 CPU 索引的 `SmallVec` 数组的内联容量。 |
| [PENDING](PENDING.md) | `16` | 用于存储待处理操作条目的 `SmallVec` 数组的内联容量。 |

## 另请参阅

| 链接 | 描述 |
|------|------|
| [winapi.rs 模块](../winapi.rs/README.md) | `List` 和 `CONSUMER_CPUS` 在 CPU 集合操作中的主要使用者。 |
| [process.rs 模块](../process.rs/README.md) | 使用 `HashMap` 进行 PID 到进程的映射。 |
| [logging.rs 模块](../logging.rs/README.md) | 使用 `HashMap` 和 `HashSet` 进行失败跟踪和去重。 |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
