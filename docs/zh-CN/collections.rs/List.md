# List 类型别名 (collections.rs)

`SmallVec<E>` 的类型别名，来自 `smallvec` crate。`List` 是 `Vec` 的直接替代品，在溢出到堆分配之前，它会将固定数量的元素以内联方式存储（在栈上）。这种混合策略消除了小型常见集合的堆分配开销，同时保留了可增长向量对于较大工作负载的灵活性。

## 语法

```rust
pub type List<E> = SmallVec<E>;
```

## 参数

| 类型参数 | 描述 |
|----------------|-------------|
| `E` | 定义元素类型和内联容量的数组类型。例如，`[u32; 32]` 表示 `List` 存储 `u32` 元素，内联容量为 32。 |

## 备注

- `SmallVec<E>` 的参数是一个**数组类型**，而不是一个标量元素类型和一个单独的容量常量。数组的长度决定了内联容量，数组的元素类型决定了存储的元素类型。例如，`List<[u32; CONSUMER_CPUS]>` 在栈上最多存储 `CONSUMER_CPUS`（32）个 `u32` 值。

- 当元素数量超过内联容量时，`SmallVec` 会透明地分配堆内存并移动元素，其行为与 `Vec` 完全相同。这种转换对调用者是不可见的——其 API 是 `Vec` API 的超集。

- `list!` 宏（通过 `pub use smallvec::smallvec as list` 重新导出）提供了类似 `vec!` 的便捷语法，用于构造带有初始值的 `List` 实例：

  ```text
  let cpus: List<[u32; CONSUMER_CPUS]> = list![0, 1, 2, 3];
  ```

- 在整个 AffinityServiceRust 中，`List` 与同一模块中定义的以下内联容量常量一起使用：

  | 用途 | 数组类型 | 内联容量 |
  |-------|-----------|-----------------|
  | 进程 ID 集合 | `[u32; PIDS]` | 512 |
  | 完整线程 ID 集 | `[u32; TIDS_FULL]` | 128 |
  | 受限线程 ID 集 | `[u32; TIDS_CAPED]` | 64 |
  | CPU 集 ID / 索引 | `[u32; CONSUMER_CPUS]` | 32 |
  | 待处理操作条目 | 各种 | 16 (`PENDING`) |

- 内联容量值经过调优，以在无需堆分配的情况下覆盖常见情况。对于绝大多数消费级和服务器系统，CPU 核心数在 32 以内，每个进程的线程数在 128 以内，受监控的进程数在 512 以内。

### 性能特征

| 操作 | 小型（≤ 内联容量） | 大型（> 内联容量） |
|-----------|--------------------------|--------------------------|
| Push | O(1) 摊销，无分配 | O(1) 摊销，溢出时堆分配 |
| 索引访问 | O(1)，栈本地 | O(1)，堆指针解引用 |
| 迭代 | 缓存友好（连续栈内存） | 缓存友好（连续堆内存） |
| Drop | 无需释放 | 堆释放 |

### 与 `Vec` 的关系

`SmallVec` 实现了 `Deref<Target = [T]>`，因此可以在任何需要切片 `&[T]` 的地方使用。它还实现了与 `Vec` 相同的大多数 trait，包括 `IntoIterator`、`Extend`、`FromIterator`、`Index`、`Clone`、`Debug` 和 `PartialEq`。

## 要求

| 要求 | 值 |
|-------------|-------|
| **模块** | `collections.rs` |
| **底层类型** | `smallvec::SmallVec<E>` |
| **Crate 依赖** | [`smallvec`](https://crates.io/crates/smallvec) |
| **平台** | 跨平台 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| HashMap 类型别名 | [HashMap](HashMap.md) |
| HashSet 类型别名 | [HashSet](HashSet.md) |
| PIDS 常量 | [PIDS](PIDS.md) |
| TIDS_FULL 常量 | [TIDS_FULL](TIDS_FULL.md) |
| TIDS_CAPED 常量 | [TIDS_CAPED](TIDS_CAPED.md) |
| CONSUMER_CPUS 常量 | [CONSUMER_CPUS](CONSUMER_CPUS.md) |
| PENDING 常量 | [PENDING](PENDING.md) |
| collections 模块概述 | [README](README.md) |
| winapi 模块（主要使用者） | [winapi.rs](../winapi.rs/README.md) |

---
*提交：[29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
