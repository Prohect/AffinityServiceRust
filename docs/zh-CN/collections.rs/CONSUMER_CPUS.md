# CONSUMER_CPUS 常量 (collections.rs)

指定用于存储 CPU 集合 ID 或 CPU 索引的 `SmallVec` 数组的内联容量。此常量在 [`winapi`](../winapi.rs/README.md) 模块的 CPU 集合转换和过滤函数中作为 `List<[u32; CONSUMER_CPUS]>` 的类型级数组大小参数使用。

## 语法

```rust
pub const CONSUMER_CPUS: usize = 32;
```

## 值

`32`

## 备注

- 此常量定义了 `SmallVec<[u32; CONSUMER_CPUS]>`（别名为 [`List`](List.md)）在溢出到堆分配之前可以在栈上内联存储的元素数量。值为 32 时，每个内联的 `List<[u32; CONSUMER_CPUS]>` 在栈上占用 `32 × 4 = 128` 字节（加上 `SmallVec` 的额外开销）。

- 选择 32 这个值是为了适应消费级 CPU 的典型逻辑处理器数量（例如 16 核 / 32 线程处理器）。对于逻辑处理器超过 32 个的系统，`SmallVec` 会透明地溢出到堆上，不影响正确性——仅产生少量的分配性能开销。

- 此常量在 [`winapi.rs`](../winapi.rs/README.md) 中以下函数的返回类型中用作内联容量：
  - [`cpusetids_from_indices`](../winapi.rs/cpusetids_from_indices.md)
  - [`cpusetids_from_mask`](../winapi.rs/cpusetids_from_mask.md)
  - [`indices_from_cpusetids`](../winapi.rs/indices_from_cpusetids.md)
  - [`mask_from_cpusetids`](../winapi.rs/mask_from_cpusetids.md)
  - [`filter_indices_by_mask`](../winapi.rs/filter_indices_by_mask.md)

- 修改此值会影响每个 `List<[u32; CONSUMER_CPUS]>` 变量和返回值的栈占用空间。增大该值可减少高核心数系统上的堆分配；减小该值可降低面向较少核心系统的应用程序的栈使用量。

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `collections.rs` |
| **类型** | `usize` |
| **使用者** | [`winapi.rs`](../winapi.rs/README.md) CPU 集合转换函数、`apply.rs` |
| **依赖** | 无 |
| **平台** | 平台无关 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| List 类型别名 | [List](List.md) |
| PIDS 常量 | [PIDS](PIDS.md) |
| TIDS_FULL 常量 | [TIDS_FULL](TIDS_FULL.md) |
| TIDS_CAPED 常量 | [TIDS_CAPED](TIDS_CAPED.md) |
| PENDING 常量 | [PENDING](PENDING.md) |
| cpusetids_from_indices | [cpusetids_from_indices](../winapi.rs/cpusetids_from_indices.md) |
| collections 模块概述 | [README](README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
