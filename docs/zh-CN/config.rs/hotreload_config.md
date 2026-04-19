# hotreload_config 函数 (config.rs)

检查配置文件是否在磁盘上被修改，如果是，则重新加载它。在成功重新加载且无解析错误的情况下，替换当前配置，更新 Prime 线程调度器的常量，并重置所有进程级应用状态，以便规则在下一轮轮询迭代中对运行的进程重新评估。

## 语法

```rust
pub fn hotreload_config(
    cli: &CliArgs,
    configs: &mut ConfigResult,
    last_config_mod_time: &mut Option<std::time::SystemTime>,
    prime_core_scheduler: &mut PrimeThreadScheduler,
    process_level_applied: &mut List<[u32; PIDS]>,
    full_process_level_match: &mut bool,
)
```

## 参数

`cli: &CliArgs`

已解析命令行参数的引用。`config_file_name` 字段提供要监控的配置文件的路径。

`configs: &mut ConfigResult`

**\[in, out\]** 当前活动的配置。如果重新加载成功且无错误，则就地替换为刚刚解析的 [ConfigResult](ConfigResult.md)。如果发现错误，则此值保持不变，之前的配置仍然生效。

`last_config_mod_time: &mut Option<std::time::SystemTime>`

**\[in, out\]** 跟踪配置文件的最后已知修改时间戳。每次调用时，函数查询文件的元数据，并将修改时间与此值进行比较。如果它们不同（或此值为 `None`），则触发重新加载并存储新的时间戳。调用方在轮询迭代之间持久化此值。

`prime_core_scheduler: &mut PrimeThreadScheduler`

**\[in, out\]** Prime 线程调度器实例。成功重新加载时，调度器的 `constants` 字段被替换为刚解析的配置中的新 [ConfigConstants](ConfigConstants.md)，以便更新的迟滞阈值立即生效。

`process_level_applied: &mut List<[u32; PIDS]>`

**\[in, out\]** 已经应用了进程级规则的进程 ID 列表。在成功重新加载后清空，以便所有运行的进程在下一个轮询周期中根据新规则重新评估。

`full_process_level_match: &mut bool`

**\[in, out\]** 标志，表示是否所有当前运行的进程都已与进程级规则匹配。在成功重新加载后重置为 `true`，通知主循环执行完整的匹配传递。

## 返回值

此函数不返回值。结果通过可变引用参数传达。

## 备注

### 重新加载决策逻辑

函数使用两步保护：

1. 查询 `std::fs::metadata` 以获取配置文件路径。如果元数据调用失败（例如，文件被删除、权限被拒绝），则不采取任何操作，之前的配置仍然有效。
2. 将 `metadata.modified()` 与 `*last_config_mod_time` 进行比较。如果时间戳相等，则文件未更改，函数立即返回。如果它们不同（包括 `last_config_mod_time` 为 `None` 的初始情况），则触发重新加载。

这意味着函数可以安全地在每次轮询迭代中调用，开销可忽略不计——仅在文件未更改时执行单次 `metadata()` 系统调用。

### 重新加载过程

当检测到修改时：

1. 更新 `*last_config_mod_time` 为新的修改时间。
2. 记录一条日志消息：`"Configuration file '{path}' changed, reloading..."`。
3. 调用 [read_config](read_config.md) 从磁盘重新解析文件。
4. 通过 `errors.is_empty()` 检查新的 [ConfigResult](ConfigResult.md)：
   - **无错误：** 替换活动的 `configs`，调用 [print_report](ConfigResult.md)，更新调度器的常量，清空 `process_level_applied`，将 `full_process_level_match` 设为 `true`，完成消息记录总规则计数。
   - **存在错误：** 保留旧的 `configs`，记录错误消息，并打印每个单独的错误。服务继续使用旧配置运行。

### 成功重新加载时的状态重置

清空 `process_level_applied` 强制主轮询循环重新应用进程级设置（优先级、亲和性、CPU 集合、IO 优先级、内存优先级）到所有当前运行的进程，即使这些进程之前已经根据旧规则配置过。这确保规则更改（例如，进程从 P 核移动到 E 核）在不需要重启进程的情况下生效。

将 `full_process_level_match` 设为 `true` 告诉主循环扫描所有运行进程，而不仅仅是新发现的进程。

### 线程安全

此函数不是线程安全的，必须仅从主轮询循环调用。所有可变参数是独占引用，由 Rust 借用检查器强制执行。

### 错误弹性

函数遵循安全失败模式：损坏的配置文件永远不会中断运行的服务。旧配置继续被应用，直到保存有效的配置文件。这允许用户在服务运行时编辑配置文件，而不会冒险经历未配置操作的时期。

## 需求

| | |
|---|---|
| **模块** | `src/config.rs` |
| **调用方** | `src/main.rs` 中的主轮询循环 |
| **被调用方** | [read_config](read_config.md), [ConfigResult::print_report](ConfigResult.md), `std::fs::metadata` |
| **依赖项** | [CliArgs](../cli.rs/CliArgs.md), [ConfigResult](ConfigResult.md), [ConfigConstants](ConfigConstants.md), [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md), [List](../collections.rs/README.md) |
| **权限** | 对配置文件路径的文件系统读取访问 |

## 参见

| 主题 | 链接 |
|-------|------|
| 模块概述 | [config.rs](README.md) |
| 黑名单热重载对应项 | [hotreload_blacklist](hotreload_blacklist.md) |
| 配置文件解析器 | [read_config](read_config.md) |
| 解析结果容器 | [ConfigResult](ConfigResult.md) |
| 传播到调度器的调优常量 | [ConfigConstants](ConfigConstants.md) |
| Prime 线程调度器 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| CLI 参数 | [CliArgs](../cli.rs/CliArgs.md) |

*文档针对 Commit：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*