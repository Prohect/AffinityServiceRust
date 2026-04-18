# HashMap 类型别名 (collections.rs)

`FxHashMap<K, V>` 的类型别名，来自 `rustc_hash` crate。它提供了标准库 `HashMap` 的直接替代品，使用 Fx (Firefox) 非加密哈希函数，针对速度进行了优化，而非抗哈希洪泛攻击。在 AffinityServiceRust 中，它用于所有哈希映射需求，包括 PID 到进程的查找、故障跟踪映射和模块缓存。

## 语法

```rust
pub type HashMap<K, V> = FxHashMap<K, V>;
```

## 参数

| 参数 | 类型 | 描述 |
|-----------|------|-------------|
| `K` | 泛型 | 键类型。必须实现 `Eq` 和 `Hash`。 |
| `V` | 泛型 | 值类型。 |

## 备注

- `FxHashMap` 使用 Fx 哈希算法，该算法最初为 Firefox 浏览器开发，后被 Rust 编译器 (`rustc`) 采用于内部使用。对于小型、类整数的键（如 `u32` PID 和 TID），它比 `std::collections::HashMap` 使用的默认 `SipHash` 快得多，代价是不提供抗哈希洪泛拒绝服务攻击的保护。

- 由于 AffinityServiceRust 操作的是受信任的、本地生成的数据（进程 ID、线程 ID、CPU 集合 ID），缺乏哈希洪泛抗性并不构成安全问题。在调度循环的热路径中，每个周期都需要创建、填充和查询映射，性能优势非常显著。

- 此别名允许整个项目通过更改 `collections.rs` 中的单行代码即可切换哈希映射实现，而无需修改任何调用点。

- 该别名在模块级别重新导出，因此消费者通过 `crate::collections::HashMap` 导入它。

- `HashMap::default()` 调用（在整个代码库中使用）创建一个带有默认哈希器的空映射，等同于 `FxHashMap::default()`。

### 项目中的常见用途

| 模块 | 键类型 | 值类型 | 用途 |
|--------|----------|------------|---------|
| `process.rs` | `u32` (PID) | `ProcessEntry` | `PID_TO_PROCESS_MAP` 中的 PID 到进程查找映射 |
| `process.rs` | `u32` (TID) | `SYSTEM_THREAD_INFORMATION` | `ProcessEntry::get_threads()` 返回的线程映射 |
| `winapi.rs` | `u32` (PID) | `Vec<(usize, usize, String)>` | `MODULE_CACHE` 每进程模块列表 |
| `logging.rs` | `u32` (PID) | `HashMap<ApplyFailEntry, bool>` | `PID_MAP_FAIL_ENTRY_SET` 故障跟踪 |
| `logging.rs` | `ApplyFailEntry` | `bool` | 内部故障条目映射，带存活标志 |

## 要求

| 要求 | 值 |
|-------------|-------|
| **模块** | `collections.rs` |
| **Crate 依赖** | `rustc_hash`（提供 `FxHashMap`） |
| **标准库等效项** | `std::collections::HashMap` |
| **平台** | 跨平台 |

## 另请参阅

| 主题 | 链接 |
|-------|------|
| HashSet 类型别名 | [HashSet](HashSet.md) |
| List 类型别名 | [List](List.md) |
| collections 模块概述 | [README](README.md) |
| process 模块（主要消费者） | [process.rs](../process.rs/README.md) |
| logging 模块（故障跟踪） | [logging.rs](../logging.rs/README.md) |
| winapi 模块（模块缓存） | [winapi.rs](../winapi.rs/README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
