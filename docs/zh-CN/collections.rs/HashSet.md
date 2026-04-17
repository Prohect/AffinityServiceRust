# HashSet 类型别名 (collections.rs)

`FxHashSet<V>` 的类型别名，来自 `rustc_hash` crate，提供使用 Fx (Firefox) 非加密哈希函数的高性能哈希集合。此别名在整个 AffinityServiceRust 中需要 `HashSet` 的地方使用，确保一致地使用更快的哈希实现，而非标准库基于 `SipHash` 的 `HashSet`。

## 语法

```rust
pub type HashSet<V> = FxHashSet<V>;
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `V` | 泛型 | 集合中存储的值类型。必须实现 `Eq` 和 `Hash`。 |

## 备注

- `FxHashSet` 是 `rustc_hash` crate 中定义的类型别名，定义为 `HashSet<V, FxBuildHasher>` —— 它是一个使用自定义哈希器的标准 `HashSet`。所有标准 `HashSet` 方法（`insert`、`contains`、`remove`、`iter` 等）均可使用。

- Fx 哈希函数**不是加密安全的**。它针对小键（整数、短字符串）的速度进行了优化，而非抵抗哈希洪泛攻击。这对 AffinityServiceRust 来说是合适的，因为被哈希的数据（进程名称、PID、操作键）不受对手控制。

- 性能特征：
  - 整数键（`u32`、`usize`）：Fx 哈希显著快于 `SipHash`，因为它避免了 `SipHash` 执行的多轮混合。
  - 字符串键：对于短字符串（< ~64 字节），Fx 哈希更快；对于较长字符串则性能相当。
  - 内存布局与 `std::collections::HashSet` 相同 —— 仅哈希函数不同。

- 该别名在 [`collections`](../collections.rs/README.md) 模块中定义，并重新导出供整个项目使用。调用者使用 `HashSet<V>` 而无需直接导入 `rustc_hash`。

### 项目中的使用

| 模块 | 用途 |
|------|------|
| `logging.rs` | [`FINDS_SET`](../logging.rs/statics.md#finds_set) —— 已发现进程名称的去重。 |
| `logging.rs` | [`FINDS_FAIL_SET`](../logging.rs/statics.md#finds_fail_set) —— 在查找模式下跟踪访问被拒绝的进程名称。 |

### 与标准库的比较

| 属性 | `std::collections::HashSet` | `collections::HashSet`（此别名） |
|------|-----------------------------|--------------------------------------|
| 哈希函数 | `SipHash-1-3` | `FxHash` |
| 抗 DoS 攻击 | 是 | 否 |
| 速度（整数键） | 中等 | 快 |
| 速度（字符串键） | 中等 | 快（短字符串） |
| Crate | `std` | `rustc_hash` |

## 要求

| 要求 | 值 |
|------|-----|
| **模块** | `collections.rs` |
| **Crate** | `rustc_hash` (`FxHashSet`) |
| **`V` 所需 trait** | `Eq + Hash` |
| **平台** | 跨平台 |

## 另请参阅

| 主题 | 链接 |
|------|------|
| HashMap 类型别名 | [HashMap](HashMap.md) |
| List 类型别名 | [List](List.md) |
| collections 模块概述 | [README](README.md) |
| logging 静态变量 (FINDS_SET, FINDS_FAIL_SET) | [statics](../logging.rs/statics.md) |

---
*提交：[37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
