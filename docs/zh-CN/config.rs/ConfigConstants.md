# ConfigConstants 结构体 (config.rs)

控制 Prime 线程选择算法迟滞行为的调优常量。这些值决定了线程被提升或降级为 Prime 线程状态的积极程度，防止当线程利用率徘徊在决策边界附近时发生快速振荡。

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
|--------|------|---------|-------------|
| `min_active_streak` | `u8` | `2` | 线程必须超过 `entry_threshold` 的连续轮询迭代次数，之后才会被提升为 Prime 线程状态。较高的值会增加稳定性，但会延迟响应性。 |
| `keep_threshold` | `f64` | `0.69` | 当前 Prime 线程被降级的总 CPU 周期份额分数。必须小于或等于 `entry_threshold`，以防止提升/降级振荡。 |
| `entry_threshold` | `f64` | `0.42` | 非 Prime 线程必须超过的总 CPU 周期份额分数（持续 `min_active_streak` 次连续迭代），之后会被提升为 Prime 线程状态。 |

## 默认实现

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

`Default` 实现提供保守的值，适用于大多数游戏和实时工作负载。`keep_threshold`（0.69）和 `entry_threshold`（0.42）之间的间隙创建了迟滞带，防止在单个阈值附近振荡的线程被反复提升和降级。

## 备注

### 迟滞机制

Prime 线程调度器使用双阈值迟滞系统：

1. **进入：** 线程必须在 `min_active_streak` 次连续迭代中持续超过 `entry_threshold` 的 CPU 周期份额，然后才能被提升。
2. **保留：** 一旦被提升，只要线程的周期份额保持在 `keep_threshold` 以上，它将保持 Prime 线程状态。
3. **降级：** 当 Prime 线程的周期份额降至 `keep_threshold` 以下时，它会立即被降级。

因为 `entry_threshold` 高于 `keep_threshold`（在默认配置中，`keep_threshold` 是较大的值，作为保留门槛，而 `entry_threshold` 是较低的初始资格门槛），所以刚刚符合 Prime 线程资格的线程不会立即面临降级风险。

### 配置语法

常量在配置文件的顶部使用 `@NAME = value` 语法定义：

```
@MIN_ACTIVE_STREAK = 3
@KEEP_THRESHOLD = 0.05
@ENTRY_THRESHOLD = 0.1
```

解析由 [parse_constant](parse_constant.md) 处理。未知的常量名称会产生警告，但不会导致解析错误。

### 热重载时的传播

当通过 [hotreload_config](hotreload_config.md) 热重载配置文件时，新的 `ConfigConstants` 会被复制到 [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) 实例中，以便在下一个轮询迭代时生效更新的阈值。

## 需求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **调用方** | [read_config](read_config.md)（通过 [parse_constant](parse_constant.md) 填充），[hotreload_config](hotreload_config.md)（传播到调度器） |
| **消费者** | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| **依赖项** | 无（纯数据结构） |
| **权限** | 无 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [config.rs](README.md) |
| 常量解析器 | [parse_constant](parse_constant.md) |
| Prime 线程调度器 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| 线程级配置 | [ThreadLevelConfig](ThreadLevelConfig.md) |
| 热重载机制 | [hotreload_config](hotreload_config.md) |
| 配置结果容器 | [ConfigResult](ConfigResult.md) |

*文档为 Commit: [facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*