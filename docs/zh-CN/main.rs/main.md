# main 函数 (main.rs)

`main` 函数是 AffinityServiceRust 的程序入口点。它解析命令行参数、加载并验证配置、获取 Windows 权限、可选地启动 ETW（Windows 事件跟踪）进程监视器以实现响应式进程检测，然后进入主轮询循环，在该循环中将进程级和线程级调度策略应用于匹配的运行进程。该循环支持配置和黑名单文件的热重载、在没有线程级工作处于活动状态时使用 ETW 驱动的睡眠以提高功耗效率，以及优雅关闭。

## 语法

```AffinityServiceRust/src/main.rs#L302-302
fn main() -> windows::core::Result<()>
```

## 参数

无。命令行参数通过 `std::env::args()` 读取，并由 `parse_args` 解析为 `CliArgs` 结构体。

## 返回值

返回 `windows::core::Result<()>`。正常退出时返回 `Ok(())`，或在关键的 Win32 调用（如查找模式下的 `CreateToolhelp32Snapshot`）失败时传播 `windows::core::Error`。

## 备注

### 启动序列

1. **命令行解析** — 调用 `parse_args` 填充 `CliArgs` 结构体。提前退出模式（`-help`、`-helpAll`、`-convert`、`-autogroup`）在各自操作完成后立即返回。
2. **配置加载** — 调用 `read_config` 解析 TOML/自定义配置文件。如果存在错误，函数会打印错误信息并退出。如果指定了 `-validate`，则在报告后退出。
3. **黑名单加载** — 可选地加载要忽略的进程名称黑名单文件。
4. **处理日志模式** — 如果 `-processLogs` 处于活动状态，委托给 `process_logs` 并返回。
5. **权限获取** — 除非被 `-noDebugPriv` 或 `-noIncBasePriority` 命令行标志抑制，否则调用 `enable_debug_privilege` 和 `enable_inc_base_priority_privilege`。
6. **定时器分辨率** — 如果请求了自定义定时器分辨率，则通过 `set_timer_resolution` 应用。
7. **UAC 提升** — 如果进程未以管理员身份运行且未指定 `-noUAC`，则尝试 `request_uac_elevation`。
8. **子进程清理** — 调用 `terminate_child_processes` 清理上次运行遗留的子进程。
9. **ETW 监视器** — 除非设置了 `-no_etw`，否则启动 `EtwProcessMonitor`，通过通道接收器传递进程启动和停止事件。

### 主轮询循环

每次迭代：

1. 通过 NT 进程/线程信息 API 获取 `ProcessSnapshot`。
2. 重置 `PrimeThreadScheduler` 中的存活标志。
3. 遍历分级的进程级配置；对于每个匹配的 `(pid, name)` 对，调用 `apply_config`（内部调用 `apply_process_level` 和可选的 `apply_thread_level`）。
4. 处理迭代之间到达的所有 ETW 待处理 PID。
5. 对已在调度器中注册的进程应用独立的线程级配置。
6. 清理已死亡的进程（句柄关闭、模块缓存清除、可选的热门线程报告）。
7. 如果查找模式处于活动状态，运行 `process_find`。
8. 刷新日志记录器。
9. 睡眠 `interval_ms` 毫秒，或在没有线程级工作待处理时阻塞在 ETW 接收器通道上（节能等待）。
10. 唤醒后，清空 ETW 通道以更新 `process_level_pending` 和 `process_level_applied` 列表。
11. 调用 `hotreload_config` 和 `hotreload_blacklist` 以在无需重启的情况下获取文件变更。

### ETW 集成

当 ETW 监视器处于活动状态时，进程启动事件将 PID 推送到 `process_level_pending`，以便在下一次快照看到进程时立即应用规则。进程停止事件通过 `drop_process_by_pid` 触发立即清理，并从已应用 PID 列表中移除。

当 ETW 监视器**未**活动时（例如 `-no_etw`），后备轮询通过在每次快照扫描后比较调度器的存活标志来检测已死亡进程。

### 关闭

循环在以下情况下退出：
- `dry_run` 为 true（单次迭代）。
- 达到 `loop_count`。
- ETW 接收器通道断开连接（发送端被丢弃）。

循环结束后，如果 ETW 监视器已启动则停止它。

### 平台说明

- **仅限 Windows。** 使用 Win32 进程/线程 API、NT 内核信息类和 ETW。
- 需要**管理员**权限才能获得完整功能（调整优先级、为受保护进程设置亲和性）。除非被抑制，否则函数将尝试 UAC 自提升。
- 需要 `SeDebugPrivilege` 来打开其他用户拥有的进程的句柄。
- 需要 `SeIncreaseBasePriorityPrivilege` 来将 I/O 优先级设置为 High。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `main.rs` |
| 调用方 | 操作系统（程序入口点） |
| 被调用方 | `parse_args`、`read_config`、`read_list`、`process_logs`、`process_find`、`apply_config`、`enable_debug_privilege`、`enable_inc_base_priority_privilege`、`set_timer_resolution`、`is_running_as_admin`、`request_uac_elevation`、`terminate_child_processes`、`EtwProcessMonitor::start`、`ProcessSnapshot::take`、`hotreload_config`、`hotreload_blacklist`、`PrimeThreadScheduler::new` |
| Win32 API | `CreateToolhelp32Snapshot`、`Process32FirstW`、`Process32NextW`、`GetProcessAffinityMask`、`CloseHandle`、`GetConsoleOutputCP` |
| 权限 | `SeDebugPrivilege`、`SeIncreaseBasePriorityPrivilege`（可选，默认启用） |

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
Commit: `7221ea0694670265d4eb4975582d8ed2ae02439d`
