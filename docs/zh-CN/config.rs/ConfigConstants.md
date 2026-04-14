# ConfigConstants 结构体 (config.rs)

保存可调节的迟滞常量，用于控制主线程调度器的提升和降级决策。这些值控制线程被提升到性能核心的积极程度以及被降级回效率核心的保守程度，以防止线程到核心分配的快速振荡（"抖动"）。

## 语法

```rust
#[derive(Debug, Clone)]
pub struct ConfigConstants {
    pub min_active_streak: u8,
    pub keep_threshold: f64,
    pub entry_threshold: f64,
}
```

## 成员

| 成员 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| `min_active_streak` | `u8` | `2` | 线程在有资格被提升到性能核心之前，必须连续出现在前 N 个最热线程中的最小调度周期数。较高的值会增加提升延迟，但可以减少由瞬时 CPU 峰值引起的误判。 |
| `keep_threshold` | `f64` | `0.69` | 当前已提升线程必须维持的最高线程周期增量的分数阈值（0.0–1.0），以保留其主线程状态。如果已提升线程的增量降至 `top_delta * keep_threshold` 以下，则会被降级。较高的值使降级更加积极。 |
| `entry_threshold` | `f64` | `0.42` | 未提升线程必须达到的最高线程周期增量的分数阈值（0.0–1.0），才能被考虑提升。只有超过 `top_delta * entry_threshold` 且满足 `min_active_streak` 要求的线程才会被提升。较低的值会扩大提升窗口。 |

## 备注

### 默认值

`Default` 实现提供了针对典型混合核心桌面工作负载调优的值：

```rust
impl Default for ConfigConstants {
    fn default() -> Self {
        ConfigConstants {
            min_active_streak: 2,
            keep_threshold: 0.69,
            entry_threshold: 0.42,
        }
    }
}
```

这些默认值使调度器保持响应性（两个周期的连续要求），同时确保只有消耗了有意义的 CPU 周期份额的线程才会被提升（`entry_threshold = 0.42`），并且已提升的线程在降级之前有合理的余量（`keep_threshold = 0.69`）。

### 配置文件语法

常量在配置文件中使用 `@CONSTANT = value` 行进行设置，由 [parse_constant](parse_constant.md) 解析：

```
@MIN_ACTIVE_STREAK = 3
@KEEP_THRESHOLD = 0.75
@ENTRY_THRESHOLD = 0.50
```

未知的常量名称会产生警告但不会导致解析错误。无效的值（例如非数字字符串）会作为错误记录在 [ConfigResult](ConfigResult.md) 中。

### 与调度器的交互

在 [read_config](read_config.md) 返回 [ConfigResult](ConfigResult.md) 之后，`constants` 字段会被复制到 [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) 中。在每个调度周期中，调度器使用这些阈值来决定提升或降级哪些线程：

1. **连续门控** — 线程的 `active_streak` 计数器（每个周期中出现在前 N 个时递增）必须达到 `min_active_streak` 才会考虑提升。
2. **进入门控** — 线程的周期增量必须超过 `top_delta * entry_threshold`。
3. **保持门控** — 如果当前已提升线程的周期增量降至 `top_delta * keep_threshold` 以下，则会被降级。

由于默认情况下 `keep_threshold > entry_threshold`，刚刚达到提升资格的线程在被降级之前有一个缓冲区，从而减少振荡。

### 热重载行为

当 [hotreload_config](hotreload_config.md) 检测到配置文件被修改时，它会重新解析常量并将新值复制到活动调度器中。更改的阈值会在下一个调度周期生效，无需重启服务。

## 要求

| | |
|---|---|
| **模块** | `config` (`src/config.rs`) |
| **构造者** | [read_config](read_config.md)（通过 [parse_constant](parse_constant.md) 和 `Default::default`） |
| **消费者** | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md)、[hotreload_config](hotreload_config.md) |
| **存储位置** | [ConfigResult](ConfigResult.md)（字段 `constants`） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| 配置解析输出 | [ConfigResult](ConfigResult.md) |
| 常量行解析器 | [parse_constant](parse_constant.md) |
| 主线程调度器 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| 配置热重载 | [hotreload_config](hotreload_config.md) |
| 模块概述 | [config 模块](README.md) |

## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd