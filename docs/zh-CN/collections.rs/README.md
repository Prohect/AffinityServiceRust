# collections 模块 (AffinityServiceRust)

`collections` 模块定义了贯穿整个项目的高性能类型别名和预调优容量常量。通过集中化这些定义，每个模块都可以共享相同的高性能哈希图、哈希集和小向量实现，无需重复 crate 级别的导入。

## 类型别名

| 名称 | 底层类型 | 描述 |
|------|---------|-------------|
| `HashMap<K, V>` | `FxHashMap<K, V>` ([rustc-hash](https://crates.io/crates/rustc-hash)) | `std::collections::HashMap` 的无缝替代品，使用 Fx 哈希算法。对于进程/线程 ID 查找中常见的小整数或指针大小的键，速度显著更快。不具备加密安全性——这是可接受的，因为所有键都来自可信的本地系统数据。 |
| `HashSet<V>` | `FxHashSet<V>` ([rustc-hash](https://crates.io/crates/rustc-hash)) | `std::collections::HashSet` 的无缝替代品，由相同的 Fx 哈希算法支持。用于 PIDs、TIDs 和错误跟踪集合的去重。 |
| `List<E>` | `SmallVec<E>` ([smallvec](https://crates.io/crates/smallvec)) | 栈分配向量，仅在元素数量超过类型参数 `E` 中编码的内联容量时才会溢出到堆（例如，`[u32; 32]` 可在栈上存储最多 32 个 `u32` 值）。消除了常见的小型 CPU 索引列表和线程 ID 向量的堆分配。 |

## 宏重导出

| 名称 | 底层宏 | 描述 |
|------|--------|-------------|
| `list!` | `smallvec::smallvec!` | 用于构造包含字面值的 `List<E>` 实例的便捷宏，类似于 `vec![]`。 |

## 常量

| 名称 | 类型 | 值 | 描述 |
|------|-----|-----|-------------|
| `PIDS` | `usize` | `256` | 进程 ID 列表的内联容量。大小设置为可覆盖典型的管理进程数量，无需堆分配。用作 `List<[u32; PIDS]>` 的类型参数。 |
| `TIDS_FULL` | `usize` | `96` | 每个进程的完整线程 ID 列表的内联容量。可容纳重量级应用（游戏、浏览器）可能启动的大量线程。 |
| `TIDS_CAPED` | `usize` | `32` | 限制（过滤）线程 ID 列表的内联容量。当只需要跟踪线程子集（如 Prime 线程候选）时使用。 |
| `CONSUMER_CPUS` | `usize` | `32` | 亲和性、CPU 集合和 Prime 线程规范中 CPU 索引数组的内联容量。覆盖消费级处理器多达 32 个逻辑核心，无需堆分配。 |
| `PENDING` | `usize` | `16` | 挂起操作队列的内联容量，例如单个应用周期内的延迟线程提升或降级操作。 |

## 语法

```rust
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;

pub type HashMap<K, V> = FxHashMap<K, V>;
pub type HashSet<V> = FxHashSet<V>;
pub type List<E> = SmallVec<E>;
pub use smallvec::smallvec as list;

pub const PIDS: usize = 256;
pub const TIDS_FULL: usize = 96;
pub const TIDS_CAPED: usize = 32;
pub const CONSUMER_CPUS: usize = 32;
pub const PENDING: usize = 16;
```

## 备注

- **为什么选择 `FxHashMap` 而不是 `std::HashMap`？** 默认的 `std::HashMap` 使用 SipHash-1-3，提供防止 HashDoS 攻击的能力，但以吞吐量作为代价。AffinityServiceRust 仅在紧密轮询循环中对本地可信数据（PIDs、TIDs、进程名）进行哈希，因此更快的 Fx 算法可带来可测量的延迟节省，且无安全妥协。
- **为什么选择 `SmallVec` 而不是 `Vec`？** 大多数亲和性规则针对少于 32 个 CPU，大多数进程拥有少于 96 个线程。`SmallVec` 将这些常见情况完全保留在栈上，避免在应用循环的热路径中产生分配器开销。上述容量常量针对消费级硬件进行了调优；拥有超过 32 个逻辑处理器的专业工作站或服务器拓扑将透明地溢出到堆。
- **命名约定：** 常量使用短大写名称（`PIDS`、`TIDS_FULL`），因为它们在类型签名中作为泛型数组大小参数出现（如 `List<[u32; CONSUMER_CPUS]>`），简洁性提高了调用点的可读性。

## 需求

| | |
|---|---|
| **模块** | `src/collections.rs` |
| **Crate 依赖** | [rustc-hash](https://crates.io/crates/rustc-hash)、[smallvec](https://crates.io/crates/smallvec) |
| **调用方** | 项目中的每个模块都从 `collections` 导入 |
| **权限** | 无 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 配置解析（主要消费者） | [config.rs](../config.rs/README.md) |
| 应用循环（热路径消费者） | [apply.rs](../apply.rs/README.md) |
| Prime 线程调度器 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| 进程快照数据 | [ProcessEntry](../process.rs/ProcessEntry.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*