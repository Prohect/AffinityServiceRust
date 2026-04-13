# hotreload_config 函数 (config.rs)

检查配置文件是否自上次检查以来被修改，如果是则重新加载。

## 语法

```rust
pub fn hotreload_config(
    cli: &CliArgs,
    configs: &mut HashMap<u32, HashMap<String, ProcessConfig>>,
    last_config_mod_time: &mut Option<std::time::SystemTime>,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    process_level_applied: &mut HashSet<u32>,
)
```

## 参数

`cli`

指向 [`CliArgs`](../cli.rs/CliArgs.md) 的引用，包含配置文件路径。

`configs`

按 grade 分组的配置映射的可变引用。加载有效新配置时就地替换。

`last_config_mod_time`

跟踪配置文件的最后已知修改时间。

`prime_core_scheduler`

指向 [`PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md) 的可变引用。配置重载时更新其常量。

`process_level_applied`

已应用进程级设置的 PID 集合的可变引用。重载时清空，以便所有进程使用新配置重新应用。

## 备注

每次循环迭代中在休眠后由 [`main`](../main.rs/main.md) 调用。如果配置文件修改时间已变更：

1. 通过 [`read_config`](read_config.md) 解析新配置。
2. 如果解析成功（无错误），替换活动配置、更新调度器常量、清空 `process_level_applied` 并记录重载。
3. 如果解析失败，保持先前配置并记录错误。

此函数之前内联在 `main()` 中，现已提取为独立函数以提高可读性。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/config.rs |
| **调用方** | [`main`](../main.rs/main.md) |

## 另请参阅

- [hotreload_blacklist](hotreload_blacklist.md)
- [read_config](read_config.md)
- [config.rs 模块概述](README.md)