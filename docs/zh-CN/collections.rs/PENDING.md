# PENDING 常量 (collections.rs)

用于存放待处理操作条目的 `SmallVec` 数组的内联容量常量。值 `16` 指定了在 `SmallVec` 溢出到堆分配之前可以存储在栈上的元素数量。

## 语法

```rust
pub const PENDING: usize = 16;
```

## 值

`16`

## 备注

- 此常量用作 `List<[T; PENDING]>`（即 `SmallVec<[T; PENDING]>`）数组的内联容量参数，在应用程序中需要临时收集待处理或排队项目的场景中使用。

- 值 `16` 是为短期存在的待处理工作项集合选择的实用默认值——例如在单个调度周期中等待规则应用的进程或线程。在正常工作负载下，大多数待处理队列预计包含少于 16 个条目，从而使其能够完全保留在栈上分配。

- 如果向 `SmallVec<[T; PENDING]>` 中推入超过 16 个元素，向量会透明地溢出到堆上。此溢出对调用者不可见，但会产生一次性的分配开销。栈到堆的转换由 `smallvec` crate 内部处理。

- 这是 [`collections`](README.md) 模块中定义的五个容量常量中最小的一个，反映了待处理操作缓冲区通常较短的预期。

### 容量常量对比

| 常量 | 值 | 典型用途 |
|----------|-------|-------------|
| [`PIDS`](PIDS.md) | `512` | 进程 ID 列表 |
| [`TIDS_FULL`](TIDS_FULL.md) | `128` | 完整线程 ID 集合 |
| [`TIDS_CAPED`](TIDS_CAPED.md) | `64` | 受限线程 ID 集合 |
| [`CONSUMER_CPUS`](CONSUMER_CPUS.md) | `32` | CPU 集合 ID / CPU 索引 |
| **PENDING** | **16** | **待处理操作条目** |

## 要求

| 要求 | 值 |
|-------------|-------|
| **模块** | `collections.rs` |
| **类型** | `usize` |
| **使用者** | 各模块中用于待处理工作缓冲区的 `SmallVec` 内联容量 |
| **依赖** | 无——编译时常量 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| PIDS 常量 | [PIDS](PIDS.md) |
| TIDS_FULL 常量 | [TIDS_FULL](TIDS_FULL.md) |
| TIDS_CAPED 常量 | [TIDS_CAPED](TIDS_CAPED.md) |
| CONSUMER_CPUS 常量 | [CONSUMER_CPUS](CONSUMER_CPUS.md) |
| List 类型别名 | [List](List.md) |
| collections 模块概述 | [README](README.md) |

---
*提交：[37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
