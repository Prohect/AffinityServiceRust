# TIDS_FULL 常量 (collections.rs)

指定用于存放完整（未限制）线程 ID 集合的 `SmallVec` 数组的内联容量。此常量在整个应用程序中作为 `List<[T; TIDS_FULL]>` 的类型级数组大小参数使用，允许在向量溢出到堆分配之前，在栈上存储最多 96 个线程 ID。

## 语法

```rust
pub const TIDS_FULL: usize = 96;
```

## 值

`96`

## 备注

- 此常量决定了用于存放线程 ID 集合（未施加上限）的 `SmallVec`（`List`）数组的内联（栈分配）容量。当集合中的线程 ID 数量超过 96 时，`SmallVec` 会自动溢出到堆分配的 `Vec`，因此正确性不受影响——仅性能特征会发生变化。

- 选择 96 这个值是为了覆盖绝大多数实际进程的情况。大多数应用程序的线程数少于 96 个，因此栈分配的快速路径可以在没有任何堆分配开销的情况下处理常见场景。

- 对于预期线程 ID 集合较小或栈空间紧张的场景，伴随常量 [`TIDS_CAPED`](TIDS_CAPED.md)（值为 `32`）提供了较小的内联容量。

- `SmallVec` 类型在 [collections](README.md) 模块中被重导出为 [`List`](List.md)，而 `TIDS_FULL` 常量用作泛型数组大小参数：`List<[u32; TIDS_FULL]>`。

### 容量常量族

| 常量 | 值 | 典型用途 |
|----------|-------|-------------|
| [`PIDS`](PIDS.md) | `256` | 进程 ID 集合 |
| **TIDS_FULL** | `96` | 完整（未限制）线程 ID 集合 |
| [`TIDS_CAPED`](TIDS_CAPED.md) | `32` | 受限（有上限）线程 ID 集合 |
| [`CONSUMER_CPUS`](CONSUMER_CPUS.md) | `32` | CPU 集合 ID 和 CPU 索引集合 |
| [`PENDING`](PENDING.md) | `16` | 待处理操作条目集合 |

## 要求

| 要求 | 值 |
|-------------|-------|
| **模块** | `collections.rs` |
| **类型** | `usize`（编译时常量） |
| **使用者** | `apply.rs`、`scheduler.rs` —— 线程枚举与处理 |
| **平台** | 平台无关 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| TIDS_CAPED 常量 | [TIDS_CAPED](TIDS_CAPED.md) |
| PIDS 常量 | [PIDS](PIDS.md) |
| CONSUMER_CPUS 常量 | [CONSUMER_CPUS](CONSUMER_CPUS.md) |
| PENDING 常量 | [PENDING](PENDING.md) |
| List 类型别名 | [List](List.md) |
| collections 模块概述 | [README](README.md) |

---
*提交：[37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
