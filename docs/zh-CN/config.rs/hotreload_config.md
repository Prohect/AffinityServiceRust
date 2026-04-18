# hotreload_config 函数 (config.rs)

监视配置文件的修改，通过将文件系统的修改时间戳与缓存值进行比较来检测变更，并在检测到变更时热重载配置。如果新解析的配置有效，则原子性地替换活跃配置、更新调度器常量并重置每 PID 的应用跟踪列表。如果新配置包含错误，则保留先前的配置并记录错误。

## 语法

```AffinityServiceRust/src/config.rs#L1305-1338
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

| 参数 | 类型 | 描述 |
|------|------|------|
| `cli` | `&CliArgs` | CLI 参数结构体的引用。`config_file_name` 字段用于在磁盘上定位配置文件。 |
| `configs` | `&mut ConfigResult` | 活跃配置结果的可变引用。如果重载的配置有效，则将其替换为新结果；否则保持不变。 |
| `last_config_mod_time` | `&mut Option<std::time::SystemTime>` | 上次成功检查时配置文件的缓存修改时间戳的可变引用。每当检测到变更时（无论新配置是否有效），都会更新为当前修改时间。首次调用时应为 `None`，以强制执行初始加载。 |
| `prime_core_scheduler` | `&mut PrimeThreadScheduler` | 主线程调度器的可变引用。当配置成功重载时，调度器的 `constants` 字段会更新为重载配置中新的 [`ConfigConstants`](ConfigConstants.md) 值。 |
| `process_level_applied` | `&mut List<[u32; PIDS]>` | 已应用进程级设置的进程 ID 列表的可变引用。成功重载后会被清空，以便在下次轮询迭代中使用新规则重新评估所有进程。 |
| `full_process_level_match` | `&mut bool` | 完整匹配标志的可变引用。在配置成功重载后设置为 `true`，以便下一次轮询迭代对所有进程执行完整规则匹配，不受 grade 调度限制，确保新规则立即应用于每个运行中的进程。 |

## 返回值

此函数不返回值。所有副作用通过对 `configs`、`last_config_mod_time`、`prime_core_scheduler`、`process_level_applied` 和 `full_process_level_match` 参数的修改来传达。

## 备注

### 变更检测算法

1. 函数对 `cli.config_file_name` 调用 `std::fs::metadata` 以获取文件的元数据。
2. 通过 `metadata.modified()` 提取修改时间戳。
3. 如果修改时间与 `*last_config_mod_time` 不同，则触发重载。如果时间匹配（或 metadata/modified 调用失败），函数立即返回且无副作用。
4. 在解析开始**之前**，将缓存的时间戳 `*last_config_mod_time` 更新为 `Some(mod_time)`。这可以防止在解析缓慢或文件正在被快速写入时反复重试重载。

### 重载流程

当检测到变更时：

1. 发出日志消息：`"Configuration file '{name}' changed, reloading..."`。
2. 通过 [`read_config`](read_config.md) 将文件完整解析为新的 `ConfigResult`。
3. **如果新配置有效**（`new_config_result.errors.is_empty()`）：
   - `*configs` 被替换为新的 `ConfigResult`。
   - 调用 `configs.print_report()` 以记录摘要。
   - 更新调度器常量：`prime_core_scheduler.constants = configs.constants.clone()`。
   - 记录总规则数。
   - 清空 `process_level_applied`，以便在下一次循环中重新评估所有进程。
   - 将 `*full_process_level_match` 设置为 `true`，以确保下一次轮询迭代对所有进程执行完整规则匹配。
4. **如果新配置有错误**：
   - 保留先前的 `*configs` 不变。
   - 发出日志消息：`"Configuration file '{name}' has errors, keeping previous configuration."`。
   - 每个错误以 `"  - "` 前缀单独记录。

### 完整匹配标志

成功重载后，`*full_process_level_match` 被设置为 `true`。此标志告知主轮询循环在下一次迭代中绕过 grade 调度限制，对所有运行中的进程执行完整的规则匹配。这确保了新加载的配置规则能够立即应用于每个进程，而不是等待其正常的 grade 调度周期。该标志在完成一次完整匹配后由主循环重置为 `false`。

### 原子替换

`*configs` 的替换使用简单赋值（`*configs = new_config_result`）。因为整个旧的 `ConfigResult` 在单条语句中被丢弃并替换，所以不存在部分更新的配置可见的中间状态。但是，这**不是**线程安全的——该函数假设对配置的单线程访问，这由主轮询循环的顺序执行模型保证。

### 清空 process_level_applied

成功重载后，会调用 `process_level_applied.clear()`。此列表跟踪哪些 PID 已应用了进程级设置（优先级、亲和性、CPU 集合、I/O 优先级、内存优先级）。清空它可确保新规则在下次轮询迭代中应用于所有运行中的进程，而不仅仅是新生成的进程。

### 调度器常量传播

调度器的 `constants` 字段与 `ConfigResult` 的替换分开更新，因为 `PrimeThreadScheduler` 出于性能原因（避免在每线程调度决策期间重复进行哈希映射查找）维护了自己的常量副本。

### 错误弹性

热重载设计是有意保守的：包含**任何**错误的配置文件都会被完全拒绝。这可以防止部分或不一致的规则集被应用。用户必须修复所有错误后重载才会生效。仅有警告不会阻止重载。

### 文件系统访问模式

该函数在每次调用时（通常每个轮询循环一次）调用 `std::fs::metadata`。这在 Windows 上是一个轻量级操作（在检测到变更之前不会读取文件内容），是不使用文件系统监视器进行文件变更检测的标准模式。

### 边界情况

| 场景 | 行为 |
|------|------|
| 运行时配置文件被删除 | `metadata()` 失败；函数返回且无副作用。保留先前的配置。 |
| 配置文件被替换为空文件 | 成功解析（无错误、无规则）；先前的配置被替换为空规则集。 |
| 配置文件保存时有语法错误 | 新配置被拒绝；保留先前的配置；错误被记录。 |
| 首次调用且 `last_config_mod_time = None` | 无条件触发重载（因为 `Some(mod_time) != None`）。 |
| 快速连续保存（编辑器自动保存） | 每个不同的修改时间戳都会触发一次重载尝试。 |

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `config.rs` |
| 可见性 | `pub` |
| 调用方 | `main.rs`（主轮询循环，每次迭代调用一次） |
| 被调用方 | [`read_config`](read_config.md)、[`ConfigResult::print_report`](ConfigResult.md)、[`ConfigResult::total_rules`](ConfigResult.md)、`std::fs::metadata`、`log!` |
| 依赖 | [`CliArgs`](../cli.rs/CliArgs.md)、[`ConfigResult`](ConfigResult.md)、[`ConfigConstants`](ConfigConstants.md)、[`scheduler.rs`](../scheduler.rs/README.md) 中的 `PrimeThreadScheduler`、[`collections.rs`](../collections.rs/README.md) 中的 `List` 和 `PIDS` |
| I/O | 对 `cli.config_file_name` 进行文件系统元数据读取；仅在检测到变更时通过 `read_config` 进行完整文件读取 |
| 权限 | 配置文件的文件系统读取权限 |

## 另请参阅

| 资源 | 链接 |
|------|------|
| hotreload_blacklist | [hotreload_blacklist](hotreload_blacklist.md) |
| read_config | [read_config](read_config.md) |
| ConfigResult | [ConfigResult](ConfigResult.md) |
| ConfigConstants | [ConfigConstants](ConfigConstants.md) |
| CliArgs | [CliArgs](../cli.rs/CliArgs.md) |
| scheduler 模块 | [scheduler.rs 概述](../scheduler.rs/README.md) |
| config 模块概述 | [README](README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*