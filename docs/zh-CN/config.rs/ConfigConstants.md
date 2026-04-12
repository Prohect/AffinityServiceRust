# ConfigConstants 结构体 (config.rs)

控制主线程（Prime Thread）提升和降级阈值的调度器行为常量。这些值从配置文件的常量部分解析，并提供合理的默认值。

## 语法

```rust
pub struct ConfigConstants {
    pub min_active_streak: u8,
    pub keep_threshold: f64,
    pub entry_threshold: f64,
}
```

## 参数

`min_active_streak`

线程在有资格被提升为主线程之前，必须连续保持活跃的最少调度周期数。默认值为 `2`。

`keep_threshold`

已提升的线程必须维持的 CPU 周期占比阈值，以保持其主线程状态。如果线程的相对周期使用率低于此阈值，该线程将被降级。默认值为 `0.69`。

`entry_threshold`

线程被考虑提升为主线程所需达到的 CPU 周期占比阈值。只有超过此阈值的线程才是候选对象。默认值为 `0.42`。

## 返回值

不适用（结构体定义）。

## 备注

`ConfigConstants` 实现了 `Default` trait，提供以下默认值：

| 字段 | 默认值 |
| --- | --- |
| `min_active_streak` | `2` |
| `keep_threshold` | `0.69` |
| `entry_threshold` | `0.42` |

这些常量在配置文件中使用 `@` 前缀进行设置：

```
@MIN_ACTIVE_STREAK = 2
@KEEP_THRESHOLD = 0.69
@ENTRY_THRESHOLD = 0.42
```

常量由 [parse_constant](parse_constant.md) 在 [read_config](read_config.md) 过程中解析，并存储在 [ConfigResult](ConfigResult.md)`.constants` 字段中。随后传递给 `src/scheduler.rs` 中的 `PrimeThreadScheduler`，用于控制主线程调度决策。

- `keep_threshold` 应大于 `entry_threshold`，以提供滞后效应，防止频繁的提升/降级循环。
- 将 `min_active_streak` 设置为更高的值会使提升更加保守，要求线程证明其持续活跃性。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/config.rs |
| **派生宏** | `Debug`, `Clone` |
| **使用者** | [ConfigResult](ConfigResult.md), `PrimeThreadScheduler` (src/scheduler.rs) |
| **解析者** | [parse_constant](parse_constant.md) |