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
2. **配置加载** — 调用 `read_config` 将 TOML/自定义配置文件解析为 `ConfigResult` 结构体（包含 `process_level_configs`、`thread_level_configs`、`constants` 和 `errors`）。如果存在错误，函数将打印并退出。如果指定了 `-validate`，则在报告后退出。
3. **黑名单加载** — 可选地加载要忽略的进程名称黑名单文件。
4. **处理日志模式** — 如果 `-processLogs` 处于活动状态，则委托给 `process_logs` 并返回。
5. **特权获取** — 调用 `enable_debug_privilege` 和 `enable_inc_base_priority_privilege`，除非通过 `-noDebugPriv` 或 `-noIncBasePriority` CLI 标志抑制。
6. **定时器分辨率** — 如果请求了自定义定时器分辨率，则通过 `set_timer_resolution` 应用。
7. **UAC 提升** — 如果进程未以管理员身份运行且未指定 `-noUAC`，则尝试 `request_uac_elevation`。
8. **子进程清理** — 调用 `terminate_child_processes` 清理上次运行遗留的过期子进程。
9. **ETW 监视器** — 除非设置了 `-no_etw`，否则启动 `EtwProcessMonitor`，通过通道接收器传递进程启动和进程停止事件。

### 主轮询循环

循环维护三个 PID 跟踪列表：

- `process_level_applied` — 已应用进程级设置的 PID（每次迭代去重，跨迭代保留）。
- `thread_level_applied` — 在**当前**迭代中已应用线程级设置的 PID（每次迭代结束时清除，以便每次轮询重新应用线程级配置）。
- `process_level_pending` — 由 ETW 事件排队的 PID，用于即时应用，不受 grade 限制。

每次迭代：

1. 通过 NT 进程/线程信息 API 获取 `ProcessSnapshot`。
2. 构建 `pids_and_names` 作为 `List<[(u32, &str); PIDS]>`（从快照借用的 PID/名称对的栈分配 small-vec）。
3. 重置 `PrimeThreadScheduler` 中的存活标志。
4. **进程级配置遍历** — 按 grade 迭代 `configs.process_level_configs`：
   - 首先，通过 `process_level_pending.retain(...)` 处理所有 ETW 待处理的 PID，为每个匹配项调用 `apply_config`（即时应用，与 grade 无关）。
   - 然后，对于匹配 grade 调度的 PID（`current_loop.is_multiple_of(*grade)`），为每个匹配 `ProcessLevelConfig` 条目的 `(pid, name)` 调用 `apply_config`——前提是该 PID 尚未被应用（或设置了 `continuous_process_level_apply`）。
   - `apply_config` 内部还会查找并应用同一 grade 和名称的任何匹配 `ThreadLevelConfig`，将 PID 推入 `process_level_applied` 和 `thread_level_applied`。此"双级应用"路径的存在是为了减少 `get_threads()` 调用并合并同一进程的日志输出。
5. **独立线程级配置遍历** — 如果调度器有任何被跟踪的进程（`pid_to_process_stats` 非空），则按 grade 迭代 `configs.thread_level_configs`。对于每个匹配的 `(pid, name)`，如果该 PID **未**在步骤 4 的双级应用中被处理（即不在 `thread_level_applied` 中），则创建自己的基于 `OnceCell` 的线程缓存并直接调用 `apply_thread_level`，然后调用 `log_apply_results`。
6. 当 ETW 未活动时，清理已终止的进程（关闭句柄、清除模块缓存、可选的顶级线程报告）。
7. 如果 find 模式处于活动状态，则运行 `process_find`。
8. 在 `dry_run` 模式下，设置 `should_continue = false` 以在单次迭代后退出（不再记录总变更计数）。
9. 在睡眠前显式丢弃 `pids_and_names` 和 `processes`。
10. 刷新日志记录器。
11. **睡眠** — 当没有线程级工作待处理时，阻塞在 ETW 接收器通道上（节能等待）；否则回退到 `thread::sleep(interval_ms)`。
12. 唤醒后，排空 ETW 通道以更新 `process_level_pending` 和 `process_level_applied` 列表。
13. 调用 `hotreload_config` 和 `hotreload_blacklist` 以在不重启的情况下获取文件更改。
14. 调用 `process_level_applied.dedup()` 和 `process_level_pending.dedup()` 进行压缩；清除 `thread_level_applied` 以便在下次迭代中重新评估线程级规则。

### ETW 驱动的睡眠

当 ETW 监视器处于活动状态且主线程调度器没有被跟踪的进程（`pid_to_process_stats` 为空）时，循环进入节能等待而非固定的 `thread::sleep`。它在循环中调用 `event_trace_receiver.recv_timeout(...)`，超时时间为 `(interval_ms + 16) / 2` 毫秒。循环在以下情况之一时中断：
- 接收器断开连接（设置 `should_continue = false` 以触发关闭）。
- 已经过足够的墙钟时间（通过 `Local::now() - last_time > TimeDelta::milliseconds(interval_ms)` 检查）。

当服务没有线程级工作要执行且仅等待通过 ETW 事件出现的新进程时，这可以减少 CPU 唤醒次数。

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
| 被调用函数 | `parse_args`、`read_config`、`read_list`、`process_logs`、`process_find`、`apply_config`、`apply_thread_level`、`log_apply_results`、`enable_debug_privilege`、`enable_inc_base_priority_privilege`、`set_timer_resolution`、`is_running_as_admin`、`request_uac_elevation`、`terminate_child_processes`、`EtwProcessMonitor::start`、`ProcessSnapshot::take`、`hotreload_config`、`hotreload_blacklist`、`PrimeThreadScheduler::new` |
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
Commit: [b0df9da](https://github.com/Prohect/AffinityServiceRust/tree/b0df9da35213b050501fab02c3020ad4dbd6c4e0)
