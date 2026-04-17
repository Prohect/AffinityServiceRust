# ConfigConstants 类型 (config.rs)

`ConfigConstants` 结构体保存控制主线程调度器行为的可调数值常量。这些值通过配置文件中的 `@NAME = value` 语法读取，影响线程被提升到主线程集合或从中降级的激进程度。

## 语法

```AffinityServiceRust/src/config.rs#L49-63
#[derive(Debug, Clone)]
pub struct ConfigConstants {
    pub min_active_streak: u8,
    pub keep_threshold: f64,
    pub entry_threshold: f64,
}

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

## 成员

| 成员 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| `min_active_streak` | `u8` | `2` | 线程在成为主线程提升候选之前，必须在连续轮询间隔内保持活跃（消耗 CPU）的最小次数。较高的值使调度器更加保守，减少瞬时峰值导致的频繁切换。 |
| `keep_threshold` | `f64` | `0.69` | 已在主线程集合中的线程必须维持的 CPU 利用率比例（0.0–1.0），以保持其主线程状态。当主线程的利用率低于此阈值时，它将成为降级候选。 |
| `entry_threshold` | `f64` | `0.42` | 非主线程必须超过的 CPU 利用率比例（0.0–1.0），才能被考虑提升到主线程集合。此值应低于 `keep_threshold`，以提供滞后效应并防止快速切换。 |

## 备注

- **滞后设计**：默认值建立了一个滞后带（`entry_threshold = 0.42` < `keep_threshold = 0.69`）。线程必须超过 42% 的利用率才能进入主线程集合，但必须降到 69% 以下才会离开。这可以防止利用率在单一阈值附近徘徊的线程发生快速提升/降级循环。

- **配置文件语法**：常量在配置文件中使用 `@` 前缀定义：
  ```/dev/null/example.ini#L1-3
  @MIN_ACTIVE_STREAK = 3
  @KEEP_THRESHOLD = 0.75
  @ENTRY_THRESHOLD = 0.50
  ```
  解析由 [parse_constant](parse_constant.md) 处理，它会验证类型并对无效值报告错误。

- **派生**：该结构体派生了 `Debug` 和 `Clone`。`Default` 实现提供上述生产环境调优的默认值，而非 Rust 的零初始化。

- **验证**：`min_active_streak` 被解析为 `u8`（范围 0–255）。阈值字段被解析为 `f64`，没有显式的范围裁剪；[0.0, 1.0] 范围外的值在技术上会被接受，但会产生未定义的调度行为。

- **热重载**：当配置文件在运行时被修改时，[hotreload_config](hotreload_config.md) 会用重新加载的 [ConfigResult](ConfigResult.md) 中新解析的值替换调度器的常量。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `config.rs` |
| 构造方式 | `Default::default()`，[read_config](read_config.md) 通过 [parse_constant](parse_constant.md) |
| 使用方 | `scheduler::PrimeThreadScheduler`，[hotreload_config](hotreload_config.md) |
| 存储位置 | [ConfigResult](ConfigResult.md)`.constants` |
| 权限 | 无 |

## 另请参阅

| 资源 | 链接 |
|------|------|
| ConfigResult | [ConfigResult](ConfigResult.md) |
| parse_constant | [parse_constant](parse_constant.md) |
| hotreload_config | [hotreload_config](hotreload_config.md) |
| read_config | [read_config](read_config.md) |
| scheduler 模块 | [scheduler.rs 概述](../scheduler.rs/README.md) |
| config 模块概述 | [README](README.md) |

---
*提交：[37fbbc5](https://github.com/Prohect/AffinityServiceRust/tree/37fbbc5135cec7c7ace9ffdacdcfc27b5865c30f)*
