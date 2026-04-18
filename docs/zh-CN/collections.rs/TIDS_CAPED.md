# TIDS_CAPED 常量 (collections.rs)

定义用于存储有上限（受限）线程 ID 集合的 `SmallVec` 数组的内联容量。此常量在整个项目中用作 `List<[T; TIDS_CAPED]>` 类型标注中的数组大小参数，允许在向量溢出到堆分配之前在栈上存储最多 32 个线程 ID。

## 语法

```rust
pub const TIDS_CAPED: usize = 32;
```

## 值

`32`

## 备注

- 此常量用作存储线程 ID 子集的 `SmallVec`（`List`）数组的内联容量——通常用于应用程序对给定规则或操作处理的线程数量施加上限的场景。名称 "CAPED"（而非 "CAPPED"）沿用了源代码的命名约定。

- `32` 的值小于 [`TIDS_FULL`](TIDS_FULL.md)（`96`），反映了有上限的线程集合预期小于完整线程枚举的情况。当存储的线程 ID 少于 32 个时，`SmallVec` 完全避免堆分配，将数据内联保存在栈上的数组中。

- 如果向 `List<[T; TIDS_CAPED]>` 中推入超过 32 个条目，`SmallVec` 会透明地溢出到堆分配的 `Vec`，因此无论实际线程数量如何，正确性都会得到保证。

- 此常量与相关的容量常量 [`PIDS`](PIDS.md)、[`TIDS_FULL`](TIDS_FULL.md)、[`CONSUMER_CPUS`](CONSUMER_CPUS.md) 和 [`PENDING`](PENDING.md) 一起定义，它们分别为不同类别的 ID 和操作调整内联存储容量。

### 容量常量比较

| 常量 | 值 | 典型用途 |
|----------|-------|-------------|
| [`PIDS`](PIDS.md) | `256` | 进程 ID 数组 |
| [`TIDS_FULL`](TIDS_FULL.md) | `96` | 完整（无上限）线程 ID 数组 |
| **TIDS_CAPED** | `32` | 有上限/受限线程 ID 数组 |
| [`CONSUMER_CPUS`](CONSUMER_CPUS.md) | `32` | CPU 集合 ID / CPU 索引数组 |
| [`PENDING`](PENDING.md) | `16` | 待处理操作条目 |

## 要求

| 要求 | 值 |
|-------------|-------|
| **模块** | `collections.rs` |
| **类型** | `pub const usize` |
| **使用者** | `apply.rs`、`scheduler.rs` —— 具有上限迭代限制的线程处理 |
| **依赖** | 无 |
| **平台** | 跨平台 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| TIDS_FULL 常量 | [TIDS_FULL](TIDS_FULL.md) |
| PIDS 常量 | [PIDS](PIDS.md) |
| CONSUMER_CPUS 常量 | [CONSUMER_CPUS](CONSUMER_CPUS.md) |
| PENDING 常量 | [PENDING](PENDING.md) |
| List 类型别名 | [List](List.md) |
| collections 模块概述 | [README](README.md) |

---
*提交：[29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
