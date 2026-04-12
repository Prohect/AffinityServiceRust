# main 函数 (main.rs)

应用程序入口点。解析命令行参数，处理帮助/工具模式，加载配置，管理特权，执行主循环以发现并配置进程。

## 语法

```rust
fn main() -> windows::core::Result<()>
```

## 返回值

返回 `windows::core::Result<()>`。成功完成时返回 `Ok(())`，出现 Windows API 错误时返回 `Err`。

## 备注

### 启动流程

函数按以下顺序执行初始化：

1. **解析 CLI** — 调用 [`parse_args`](../cli.rs/parse_args.md) 将命令行参数填充到 [`CliArgs`](../cli.rs/CliArgs.md) 结构体中。
2. **帮助模式** — 若启用 `-help` 或 `-helpall`，调用 [`print_help`](../cli.rs/print_help.md) 或 [`print_help_all`](../cli.rs/print_help_all.md) 后立即退出。
3. **工具模式** — 若启用 `-convert` 或 `-autogroup`，执行相应的配置转换函数后退出。
4. **加载配置** — 调用 [`read_config`](../config.rs/read_config.md) 加载并解析配置文件。若存在错误则打印报告并退出。
5. **验证模式** — 若启用 `-validate`，打印配置报告后退出，不执行任何应用逻辑。
6. **加载黑名单** — 若指定了 `-blacklist` 文件，通过 [`read_list`](../config.rs/read_list.md) 加载。
7. **处理日志模式** — 若启用 `-processlogs`，调用 [`process_logs`](process_logs.md) 处理 `.find.log` 文件后退出。
8. **空配置检查** — 若配置和黑名单均为空且未启用 `-find` 模式，打印提示后退出。

### 特权管理

9. **启用 SeDebugPrivilege** — 除非指定 `-noDebugPriv`，调用 [`enable_debug_privilege`](../winapi.rs/enable_debug_privilege.md)。
10. **启用 SeIncreaseBasePriorityPrivilege** — 除非指定 `-noIncBasePriority`，调用 [`enable_inc_base_priority_privilege`](../winapi.rs/enable_inc_base_priority_privilege.md)。
11. **设置计时器分辨率** — 若 `-resolution` 非零，调用 `NtSetTimerResolution` 设置系统计时器精度。
12. **UAC 提升** — 若当前未以管理员运行且未指定 `-noUAC`，调用 [`request_uac_elevation`](../winapi.rs/request_uac_elevation.md) 请求提升。

### 初始化

13. **清理子进程** — 调用 [`terminate_child_processes`](../winapi.rs/terminate_child_processes.md) 清理之前可能残留的子进程。
14. **初始化调度器** — 创建 [`PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md) 实例，使用配置中的常量参数。

### 主循环

主循环持续运行，直到满足退出条件（dry-run 模式完成首次迭代、达到 `-loop` 指定次数，或程序终止）：

1. **拍摄快照** — 调用 `ProcessSnapshot::take` 获取当前进程快照。
2. **清理错误映射** — 调用 [`purge_fail_map`](../logging.rs/purge_fail_map.md) 移除已不存在的进程的错误记录。
3. **重置存活标记** — 调用 `prime_core_scheduler.reset_alive()` 重置所有进程的存活状态。
4. **Grade 调度** — 遍历所有 grade 级别的配置：
   - Grade 1 的规则每次循环执行
   - Grade 2 的规则每 2 次循环执行
   - Grade 5 的规则每 5 次循环执行
   - 通过 `current_loop.is_multiple_of(grade)` 判断是否执行
5. **应用配置** — 对快照中匹配配置规则的每个进程，调用 [`apply_config`](apply_config.md)。
6. **记录更改** — 通过 [`log_message`](../logging.rs/log_message.md) 和 [`log_pure_message`](../logging.rs/log_pure_message.md) 记录更改。日志格式为 `"{pid:>5}::{name}::{change}"`。
7. **关闭死进程句柄** — 调用 `prime_core_scheduler.close_dead_process_handles()` 清理不再存活的进程资源。
8. **Find 模式** — 若启用 `-find`，使用 `CreateToolhelp32Snapshot` 枚举所有进程，对不在配置和黑名单中且亲和性为系统默认值的进程调用 [`log_process_find`](../logging.rs/log_process_find.md) 记录。
9. **Dry-run 退出** — 若为 dry-run 模式，打印摘要后退出循环。
10. **循环计数** — 递增计数器，若达到 `-loop` 指定次数则退出。
11. **休眠** — 调用 `thread::sleep` 等待 `interval_ms` 毫秒。

### 热重载

每次休眠后，函数检查配置文件和黑名单文件的修改时间：

- **配置文件热重载** — 若文件修改时间发生变化，调用 [`read_config`](../config.rs/read_config.md) 重新加载。若新配置无错误，则替换当前配置并更新 `prime_core_scheduler.constants`；若有错误，保留旧配置并记录错误详情。
- **黑名单文件热重载** — 若文件修改时间发生变化，重新加载黑名单列表。若文件不再可访问，清空黑名单。

### Grade 调度机制

配置规则按 grade 值分组，存储在 `HashMap<u32, HashMap<String, ProcessConfig>>` 中：

| Grade | 执行频率 | 典型用途 |
| --- | --- | --- |
| 1 | 每次循环 | 高优先级进程（如游戏、实时应用） |
| 2 | 每 2 次循环 | 中等优先级进程 |
| 5 | 每 5 次循环 | 低优先级或系统进程 |

这种分级机制减少了每次循环的 API 调用次数，同时保证关键进程得到及时处理。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/main.rs |
| **源码行** | L196–L456 |
| **平台** | Windows |
| **管理员权限** | 建议（可通过 `-noUAC` 禁用 UAC 提升） |
| **关键依赖** | [`CliArgs`](../cli.rs/CliArgs.md)、[`ProcessConfig`](../config.rs/ProcessConfig.md)、[`PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md)、[`ProcessSnapshot`](../process.rs/ProcessSnapshot.md) |
| **Windows API** | `NtSetTimerResolution`、`CreateToolhelp32Snapshot`、`Process32FirstW`、`Process32NextW`、`CloseHandle` |

## 另请参阅

- [main.rs 模块概述](README.md)
- [apply_config](apply_config.md)
- [process_logs](process_logs.md)
- [CliArgs 结构体](../cli.rs/CliArgs.md)
- [parse_args](../cli.rs/parse_args.md)
- [read_config](../config.rs/read_config.md)