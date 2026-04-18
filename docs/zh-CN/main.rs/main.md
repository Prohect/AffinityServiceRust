# main 函数 (main.rs)

`main` 函数是 AffinityServiceRust 的程序入口点。它解析命令行参数、加载并验证配置、获取 Windows 特权，可选地启动 ETW（Windows 事件跟踪）进程监视器以实现响应式进程检测，然后进入主轮询循环，将进程级和线程级调度策略应用于匹配的运行中 Windows 进程。该循环支持配置文件和黑名单文件的热重载、在无线程级工作活跃时使用 ETW 驱动的睡眠以提高电源效率，以及优雅关闭。

## 语法

```AffinityServiceRust/src/main.rs#L302-302
fn main() -> windows::core::Result<()>
```

## 参数

无。命令行参数通过 `std::env::args()` 读取，并由 `parse_args` 解析为 `CliArgs` 结构体。

## 返回值

返回 `windows::core::Result<()>`。正常退出时返回 `Ok(())`，或在关键 Win32 调用（例如 find 模式下的 `CreateToolhelp32Snapshot`）失败时传播 `windows::core::Error`。

## 备注

### 启动序列

1. **CLI 解析** — 调用 `parse_args` 填充 `CliArgs` 结构体。早期退出模式（`-help`、`-helpAll`、`-convert`、`-autogroup`）在各自操作完成后立即返回。
2. **设置 `DUST_BIN_MODE`** — 在配置加载之前设置。
3. **配置加载** — 调用 `read_config` 将 TOML/自定义配置文件解析为 `ConfigResult` 结构体（包含 `process_level_configs`、`thread_level_configs`、`constants` 和 `errors`）。配置错误和 `-validate` 模式合并为单一检查：`if !configs.errors.is_empty() || cli.validate_mode { return Ok(()); }`。
4. **黑名单加载** — 可选地加载要忽略的进程名称黑名单文件（`read_bleack_list`）。加载函数内部已记录计数，不再单独记录。
5. **处理日志模式** — 如果 `-processLogs` 处于活动状态，则委托给 `process_logs` 并返回。
6. **特权获取** — 调用 `enable_debug_privilege(cli.no_debug_priv)` 和 `enable_inc_base_priority_privilege(cli.no_inc_base_priority)`。每个函数接受一个布尔参数，在内部处理禁用的情况。
7. **定时器分辨率** — 无条件调用 `set_timer_resolution`（函数在内部处理零值情况）。
8. **空配置检查** — 如果没有配置且 find 模式未启用，记录 `"not config, find mode not enabled, exiting"` 并返回。
9. **UAC 提升** — 如果进程未以管理员身份运行且未指定 `-noUAC`，则尝试 `request_uac_elevation`。
10. **子进程清理** — 调用 `terminate_child_processes` 清理上次运行遗留的过期子进程。
11. **ETW 监视器** — 除非设置了 `-no_etw`，否则启动 `EtwProcessMonitor`，通过通道接收器传递进程启动和进程停止事件。

### 主轮询循环

循环维护三个 PID 跟踪列表和一个布尔标志：

- `process_level_applied` — 已应用进程级设置的 PID（每次迭代去重，跨迭代保留）。
- `thread_level_applied` — 在**当前**迭代中已应用线程级设置的 PID（每次迭代结束时清除，以便每次轮询重新应用线程级配置）。
- `process_level_pending` — 由 ETW 事件排队的 PID，用于即时应用，不受 grade 限制。
- `full_process_level_match: bool` — 初始化为 `true`。在首次循环和配置重载后强制对所有进程进行完整规则评估。

每次迭代：

1. 通过 NT 进程/线程信息 API 获取 `ProcessSnapshot`。
2. 构建 `pids_and_names` 作为 `List<[(u32, &str); PIDS]>`（从快照借用的 PID/名称对的栈分配 small-vec）。
3. 重置 `PrimeThreadScheduler` 中的存活标志。
4. **ETW 监视器启动** — ETW 监视器现在在 `prime_core_scheduler`、`current_loop` 等之前启动。
5. **进程级配置遍历** — 按 grade 迭代 `configs.process_level_configs`：
   - 首先，通过 `process_level_pending.retain(...)` 处理所有 ETW 待处理的 PID，为每个匹配项调用 `apply_config`（即时应用，与 grade 无关）。retain 逻辑已反转——现使用 `!pids_and_names.iter().any(...)` 判断。
   - 然后，grade 跳过条件现在首先检查 `full_process_level_match`：`if !full_process_level_match && (!current_loop.is_multiple_of(*grade) || ...)`。
   - 对于匹配 grade 调度的 PID（`current_loop.is_multiple_of(*grade)`），为每个匹配 `ProcessLevelConfig` 条目的 `(pid, name)` 调用 `apply_config`——前提是该 PID 尚未被应用（或设置了 `continuous_process_level_apply`）。
   - 等级循环现在还会在 `prime_core_scheduler.pid_to_process_stats` 为空且 ETW 处于活动状态时跳过。这避免了在没有线程级跟踪工作时进行不必要的迭代。
   - `apply_config` 内部还会查找并应用同一 grade 和名称的任何匹配 `ThreadLevelConfig`，将 PID 推入 `process_level_applied` 和 `thread_level_applied`。此"双级应用"路径的存在是为了减少 `get_threads()` 调用并合并同一进程的日志输出。
6. 处理后 `full_process_level_match` 设为 `false`。
7. **独立线程级配置遍历** — 如果调度器有任何被跟踪的进程（`pid_to_process_stats` 非空），则按 grade 迭代 `configs.thread_level_configs`。对于每个匹配的 `(pid, name)`，如果该 PID **未**在步骤 5 的双级应用中被处理（即不在 `thread_level_applied` 中），则创建自己的基于 `OnceCell` 的线程缓存并直接调用 `apply_thread_level`，然后调用 `log_apply_results`。
8. 清理已终止的进程（关闭句柄、清除模块缓存、可选的顶级线程报告）。清理现在在ETW未激活（`event_trace_receiver.is_none()`）或主调度器有活跃条目（`!pid_to_process_stats.is_empty()`）时运行。之前仅在非 ETW 模式下运行。
9. 如果 find 模式处于活动状态，则运行 `process_find`。
10. 在 `dry_run` 模式下，设置 `should_continue = false` 以在单次迭代后退出（不再记录总变更计数）。
11. `process_level_pending.clear()` 现在在快照块之外（之后）调用，而不是在其内部。这防止了短生命周期进程在 retain 中被发现但很快退出时在 pending 中累积。
12. 在睡眠前显式丢弃 `pids_and_names` 和 `processes`。
13. 刷新日志记录器。
14. **睡眠** — 当没有线程级工作待处理且 `!cli.continuous_process_level_apply` 时，阻塞在 ETW 接收器通道上（节能等待）；否则回退到 `thread::sleep(interval_ms)`。
15. 唤醒后，排空 ETW 通道以更新 `process_level_pending` 和 `process_level_applied` 列表。
16. 调用 `hotreload_config` 和 `hotreload_blacklist` 以在不重启的情况下获取文件更改。`hotreload_config` 现在接受 `&mut full_process_level_match` 作为额外参数，重载时将其设回 `true`。
17. 调用 `process_level_applied.dedup()` 和 `process_level_pending.dedup()` 进行压缩；清除 `thread_level_applied` 以便在下次迭代中重新评估线程级规则。

### ETW 驱动的睡眠

当 ETW 活跃、`pid_to_process_stats` 为空且 `!cli.continuous_process_level_apply` 时，循环在 ETW 通道上使用 `recv_timeout`。事件现在在休眠阶段内联处理：
- `RecvTimeoutError::Disconnected`：设置 `should_continue = false` 并跳出。
- `RecvTimeoutError::Timeout`：仅在 `process_level_pending` 非空时跳出。
- 成功接收事件：进程启动事件推入 `process_level_pending`；进程停止事件从 pending、applied、fail map 和调度器中清理。循环使用智能节流跳出：如果 pending 已非空，则在半个间隔减去 16ms 后跳出；如果 pending 之前为空且有新事件到达，则在完整间隔后跳出。

当 ETW 监视器**未**活动时（例如 `-no_etw`），回退轮询通过在每次快照遍历后比较调度器的存活标志来检测已终止的进程，且循环始终使用 `thread::sleep`。

### 关闭

循环在以下情况下退出：
- `dry_run` 为 true（单次迭代）。
- 达到 `loop_count`。
- ETW 接收器通道断开连接（发送端被丢弃）。

循环结束后，如果已启动 ETW 监视器则将其停止。

### 平台说明

- **仅限 Windows。** 使用 Win32 进程/线程 API、NT 内核信息类和 ETW。
- 需要**管理员**权限才能获得完整功能（调整优先级、为受保护进程设置亲和性）。除非被抑制，否则函数将尝试 UAC 自提升。
- 需要 `SeDebugPrivilege` 才能打开其他用户拥有的进程的句柄。
- 需要 `SeIncreaseBasePriorityPrivilege` 才能将 I/O 优先级设置为高。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `main.rs` |
| 调用者 | 操作系统（程序入口点） |
| 被调用函数 | `parse_args`、`read_config`、`read_bleack_list`、`process_logs`、`process_find`、`apply_config`、`apply_thread_level`、`log_apply_results`、`enable_debug_privilege`、`enable_inc_base_priority_privilege`、`set_timer_resolution`、`is_running_as_admin`、`request_uac_elevation`、`terminate_child_processes`、`EtwProcessMonitor::start`、`ProcessSnapshot::take`、`hotreload_config`、`hotreload_blacklist`、`PrimeThreadScheduler::new` |
| Win32 API | `CreateToolhelp32Snapshot`、`Process32FirstW`、`Process32NextW`、`GetProcessAffinityMask`、`CloseHandle`、`GetConsoleOutputCP` |
| 特权 | `SeDebugPrivilege`、`SeIncreaseBasePriorityPrivilege`（可选，默认启用） |

## 另请参阅

| 参考 | 链接 |
|------|------|
| apply_config | [apply_config](apply_config.md) |
| apply_process_level | [apply_process_level](apply_process_level.md) |
| apply_thread_level | [apply_thread_level](apply_thread_level.md) |
| process_find | [process_find](process_find.md) |
| process_logs | [process_logs](process_logs.md) |
| log_apply_results | [log_apply_results](log_apply_results.md) |
| PrimeThreadScheduler | [PrimeThreadScheduler](../scheduler.rs/PrimeThreadScheduler.md) |
| cli 模块 | [cli.rs README](../cli.rs/README.md) |
| config 模块 | [config.rs README](../config.rs/README.md) |
| event_trace 模块 | [event_trace.rs README](../event_trace.rs/README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*
