# main 函数 (main.rs)

AffinityServiceRust 服务的入口点。解析命令行参数，加载配置文件，处理模式分发（帮助、转换、自动分组、验证、处理日志），并运行主轮询循环以对托管进程强制执行进程级和线程级设置。集成了基于 ETW 的响应式进程检测、基于等级的调度以及配置和黑名单文件的热重载。

## 语法

```rust
fn main() -> windows::core::Result<()>
```

## 参数

此函数不接受参数。命令行参数通过 `std::env::args()` 从环境中读取。

## 返回值

成功完成时返回 `Ok(())`，或在关键 Win32 API 调用失败时返回 `windows::core::Error`（例如 [process_find](process_find.md) 中的 `CreateToolhelp32Snapshot`）。大多数错误会被记录并妥善处理，而非向上传播——即使在配置错误或提权失败时，函数也会返回 `Ok(())`。

## 备注

### 启动序列

函数执行以下初始化步骤：

1. **解析 CLI 参数** — 调用 [parse_args](../cli.rs/parse_args.md) 以使用默认值和用户提供的覆盖项填充 [CliArgs](../cli.rs/CliArgs.md) 结构体。

2. **模式分发** — 按优先级顺序检查模式标志，对于非服务模式提前退出：
   - `help_mode` → 调用 [print_help](../cli.rs/print_help.md) 并返回。
   - `help_all_mode` → 调用 [print_help_all](../cli.rs/print_help_all.md) 并返回。
   - `convert_mode` → 调用 [convert](../config.rs/convert.md) 并返回。
   - `autogroup_mode` → 调用 [sort_and_group_config](../config.rs/sort_and_group_config.md) 并返回。

3. **加载配置** — 调用 [read_config](../config.rs/read_config.md) 将配置文件解析为按等级索引的 `HashMap<u32, HashMap<String, ProcessConfig>>`。打印配置报告（规则数量、警告）。如果存在任何错误，函数会记录消息并在不进入轮询循环的情况下返回。

4. **验证模式** — 如果设置了 `validate_mode`，函数在打印配置报告后返回，不进入轮询循环。

5. **加载黑名单** — 如果指定了黑名单文件，调用 [read_list](../config.rs/read_list.md) 加载要排除在管理和发现之外的进程名称。

6. **处理日志模式** — 如果设置了 `process_logs_mode`，使用加载的配置和黑名单调用 [process_logs](process_logs.md)，然后返回。

7. **空配置检查** — 如果配置和黑名单都为空且 `-find` 模式未激活，函数记录消息并退出。如果 `-find` 处于激活状态，服务将使用空配置继续运行以发现未托管的进程。

8. **权限获取** — 除非被 `-noDebugPriv` 或 `-noIncBasePriority` 标志抑制，否则调用 [enable_debug_privilege](../winapi.rs/enable_debug_privilege.md) 和 [enable_inc_base_priority_privilege](../winapi.rs/enable_inc_base_priority_privilege.md)。

9. **定时器分辨率** — 如果 `-resolution` 指定了非零值，调用 [set_timer_resolution](../winapi.rs/set_timer_resolution.md) 调整系统定时器粒度。

10. **UAC 提权** — 如果进程未以管理员身份运行，调用 [request_uac_elevation](../winapi.rs/request_uac_elevation.md) 以提升权限重新启动（除非设置了 `-noUAC`）。注意，在提权后的会话中控制台输出不可见；应使用日志文件。

11. **子进程清理** — 调用 [terminate_child_processes](../winapi.rs/terminate_child_processes.md) 清理上次提权尝试遗留的孤立子进程。

12. **初始化调度器** — 使用解析后的 [ConfigConstants](../config.rs/ConfigConstants.md)（迟滞阈值和最小活跃连续次数）创建 [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md)。

13. **启动 ETW 监控** — 调用 [EtwProcessMonitor::start](../event_trace.rs/EtwProcessMonitor.md) 开始实时进程启动/停止事件跟踪（除非设置了 `-no_etw`）。如果 ETW 启动失败（例如权限不足或另一个跟踪会话正在运行），服务将回退到仅轮询模式。

### 主轮询循环

循环无限运行（或在指定 `-loop` 时运行固定次数），每次迭代之间休眠 `cli.interval_ms` 毫秒。每次迭代执行：

1. **获取进程快照** — 调用 [ProcessSnapshot::take](../process.rs/ProcessSnapshot.md)，通过 `NtQuerySystemInformation` 枚举所有进程及其线程的周期时间数据。如果快照失败，记录错误并跳过该迭代。

2. **重置存活标志** — 调用 `prime_core_scheduler.reset_alive()` 将所有跟踪的进程标记为可能已终止。在当前快照中匹配到的进程稍后会被重新标记为存活。

3. **处理 ETW 待处理队列** — 对于 `process_level_pending` 中的每个 PID（由迭代之间的 ETW 进程启动事件填充），函数尝试立即应用进程级设置，不受等级限制。这为新启动的进程提供了近乎即时的规则应用。成功应用的 PID 被移至 `process_level_applied` 并从待处理集合中移除。

4. **基于等级的迭代** — 遍历配置中的所有等级级别。等级 `N` 意味着该级别的规则仅在 `current_loop` 是 `N` 的倍数时才被评估。对于当前等级下的每个进程：
   - 如果该 PID 尚未应用进程级设置，调用 [apply_config_process_level](apply_config_process_level.md) 并将 PID 记录到 `process_level_applied` 中。
   - 调用 [apply_config_thread_level](apply_config_thread_level.md) 进行线程级设置（始终重新评估）。
   - 记录 [ApplyConfigResult](../apply.rs/ApplyConfigResult.md) 中的所有更改和错误。

5. **死进程清理** — 当 ETW 未激活时，未标记为存活的死进程会从 `PrimeThreadScheduler` 中移除，错误去重失败映射也会被清除。当 ETW 激活时，清理通过进程退出事件响应式处理。

6. **试运行退出** — 如果 `-dryrun` 处于激活状态，记录将要进行的更改总数并在第一次迭代后退出。

7. **发现模式** — 调用 [process_find](process_find.md) 发现未托管的进程（如果 `-find` 处于激活状态）。

8. **刷新日志** — 刷新主日志文件和发现日志文件。

9. **循环终止** — 递增循环计数器。如果指定了 `-loop <count>` 且已达到计数，设置 `should_continue = false`。

10. **休眠和 ETW 排空** — 休眠配置的间隔时间，更新本地时间缓存，并排空 ETW 事件接收通道。进程启动事件将 PID 添加到 `process_level_pending`；进程退出事件从 `process_level_pending`、`process_level_applied`、错误失败映射和主线程调度器中移除 PID。

11. **热重载** — 调用 [hotreload_config](../config.rs/hotreload_config.md) 和 [hotreload_blacklist](../config.rs/hotreload_blacklist.md) 检测文件修改并在不重启服务的情况下重新加载配置和黑名单。当配置被重新加载时，`process_level_applied` 被清空以强制重新应用进程级设置。

### ETW 集成

ETW 进程监控提供两个关键优势：

- **响应式应用** — 新进程在启动后的下一次轮询迭代中即可应用其设置，而无需等待基于等级的调度到达它们。`process_level_pending` 集合桥接了 ETW 事件接收（异步）和同步轮询循环之间的间隔。
- **及时清理** — 当进程退出时，其状态会立即从调度器、失败映射和已应用集合中移除，防止过期数据积累并确保 PID 重用得到正确处理。

如果 ETW 不可用，服务会优雅地降级为仅轮询模式，在每次迭代结束时通过将调度器跟踪的 PID 与活跃快照进行比较来清理死进程。

### 日志记录

更改以格式化布局记录：

```
[HH:MM:SS] <PID>::<process_name>::<first_change>
                                   <subsequent_changes>
```

来自应用操作的错误通过 `log_to_find` 转发到发现日志。多行更改条目的填充考虑了 10 个字符的时间前缀（例如 `[04:55:16]`）。

### 关闭

轮询循环退出后（由于 `-loop` 计数、`-dryrun` 或信号），函数通过调用 `event_trace_monitor.stop()` 停止 ETW 监控（如果激活）并返回 `Ok(())`。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `main` |
| 调用者 | Rust 运行时（程序入口点） |
| 被调用者 | [parse_args](../cli.rs/parse_args.md)、[print_help](../cli.rs/print_help.md)、[print_help_all](../cli.rs/print_help_all.md)、[read_config](../config.rs/read_config.md)、[read_list](../config.rs/read_list.md)、[convert](../config.rs/convert.md)、[sort_and_group_config](../config.rs/sort_and_group_config.md)、[hotreload_config](../config.rs/hotreload_config.md)、[hotreload_blacklist](../config.rs/hotreload_blacklist.md)、[enable_debug_privilege](../winapi.rs/enable_debug_privilege.md)、[enable_inc_base_priority_privilege](../winapi.rs/enable_inc_base_priority_privilege.md)、[set_timer_resolution](../winapi.rs/set_timer_resolution.md)、[is_running_as_admin](../winapi.rs/is_running_as_admin.md)、[request_uac_elevation](../winapi.rs/request_uac_elevation.md)、[terminate_child_processes](../winapi.rs/terminate_child_processes.md)、[EtwProcessMonitor::start](../event_trace.rs/EtwProcessMonitor.md)、[ProcessSnapshot::take](../process.rs/ProcessSnapshot.md)、[apply_config_process_level](apply_config_process_level.md)、[apply_config_thread_level](apply_config_thread_level.md)、[process_find](process_find.md)、[process_logs](process_logs.md) |
| API | `NtQuerySystemInformation`（通过 `ProcessSnapshot`）、`CreateToolhelp32Snapshot`（通过 `process_find`）、ETW（`StartTrace`/`ProcessTrace`/`ControlTrace`）、`GetProcessAffinityMask`、通过 apply 函数调用的各种 `Set*` API |
| 权限 | `SeDebugPrivilege`（推荐）、`SeIncreaseBasePriorityPrivilege`（用于高/实时优先级）、管理员（推荐，以获得完整的进程访问权限和 ETW） |

## 另请参阅

| 主题 | 链接 |
|------|------|
| CLI 参数解析 | [parse_args](../cli.rs/parse_args.md) |
| CLI 参数结构体 | [CliArgs](../cli.rs/CliArgs.md) |
| 进程级设置（一次性） | [apply_config_process_level](apply_config_process_level.md) |
| 线程级设置（每次迭代） | [apply_config_thread_level](apply_config_thread_level.md) |
| 配置文件解析 | [read_config](../config.rs/read_config.md) |
| 主线程调度器 | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| ETW 进程监控 | [EtwProcessMonitor](../event_trace.rs/EtwProcessMonitor.md) |
| 进程快照 | [ProcessSnapshot](../process.rs/ProcessSnapshot.md) |
| 发现模式（运行时发现） | [process_find](process_find.md) |
| 日志处理 | [process_logs](process_logs.md) |


## Documentation on Commit SHA

678734d5df2c1188fb1bd6e448aae0884fb174fd