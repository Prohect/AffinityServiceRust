# PIDS 常量 (collections.rs)

用于存储进程 ID 的 `SmallVec` 数组的内联容量常量。此值决定了在 `SmallVec` 溢出到堆分配之前，可以在栈上存储多少个进程 ID。

## 语法

```rust
pub const PIDS: usize = 512;
```

## 值

`512`

## 备注

- 此常量用作应用程序中所有需要进程 ID 集合的 `List<[T; PIDS]>`（即 `SmallVec<[T; PIDS]>`）的内联容量参数。

- 选择 `512` 这个值是为了在桌面或工作站系统上容纳典型数量的匹配进程，而无需触发堆分配。在匹配 PID 超过 512 个的系统上，`SmallVec` 会透明地溢出到堆上，行为不会发生任何变化——仅会产生额外分配带来的性能开销。

- 由于 `SmallVec` 将其内联元素直接存储在结构体内部，因此 `List<[u32; PIDS]>` 在栈上占用 `512 * 4 = 2048` 字节。在深度递归函数中使用此容量，或同时存在多个此类集合时，调用者应注意栈帧大小。

- 此常量在模块级别定义，所有从 `collections` 导入的模块均可使用。

### 与其他容量常量的对比

| 常量 | 值 | 典型用途 |
|----------|-------|-------------|
| **PIDS** | `512` | 进程 ID 集合 |
| [TIDS_FULL](TIDS_FULL.md) | `128` | 完整线程 ID 集合 |
| [TIDS_CAPED](TIDS_CAPED.md) | `64` | 有上限（受限）的线程 ID 集合 |
| [CONSUMER_CPUS](CONSUMER_CPUS.md) | `32` | CPU 集合 ID 和 CPU 索引数组 |
| [PENDING](PENDING.md) | `16` | 待处理操作条目 |

## 要求

| 要求 | 值 |
|-------------|-------|
| **模块** | `collections.rs` |
| **类型** | `usize` |
| **使用方** | `apply.rs`、`scheduler.rs` —— 用于 PID 集合缓冲区 |
| **平台** | 与平台无关 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| TIDS_FULL 常量 | [TIDS_FULL](TIDS_FULL.md) |
| TIDS_CAPED 常量 | [TIDS_CAPED](TIDS_CAPED.md) |
| CONSUMER_CPUS 常量 | [CONSUMER_CPUS](CONSUMER_CPUS.md) |
| PENDING 常量 | [PENDING](PENDING.md) |
| List 类型别名 | [List](List.md) |
| collections 模块概述 | [README](README.md) |

---
> Commit SHA: `7221ea0694670265d4eb4975582d8ed2ae02439d`
