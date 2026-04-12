# main.rs 模块 (main.rs)

`main` 模块是 AffinityServiceRust 的应用程序入口点，负责命令行解析、特权管理、配置热重载、grade 调度、进程发现以及各种工具模式的分发。

## 概述

本模块包含以下核心职责：

- **CLI 解析与模式分发** — 解析命令行参数，根据模式标志分发到帮助、转换、验证、日志处理等子流程
- **特权管理** — 请求 `SeDebugPrivilege` 和 `SeIncreaseBasePriorityPrivilege`，处理 UAC 提升
- **配置加载与热重载** — 首次加载配置文件，主循环中通过文件修改时间检测变更并自动重载
- **Grade 调度** — 按 grade 值控制规则执行频率（grade=1 每次循环执行，grade=2 每 2 次，grade=5 每 5 次）
- **进程快照与配置应用** — 每次循环获取系统进程快照，匹配配置规则并应用
- **进程发现（find 模式）** — 扫描所有进程，查找亲和性为系统默认值且不在配置/黑名单中的进程
- **工具模式** — 包括 convert（Process Lasso 配置转换）、autogroup（自动分组）、validate（语法验证）、processlogs（日志处理）

## 项目

### 函数

| 名称 | 描述 |
| --- | --- |
| [apply_config](apply_config.md) | 编排所有配置设置应用到目标进程。按顺序执行优先级、亲和性、CPU 集、I/O 优先级、内存优先级和 Prime 线程调度。 |
| [process_logs](process_logs.md) | 处理 `-find` 模式生成的 `.find.log` 文件，发现新进程并通过 `es.exe` 搜索其可执行路径。 |
| [main](main.md) | 应用程序入口点。完成 CLI 解析、模式分发、配置加载、特权获取、主循环及热重载。 |

## 执行流程

`main` 函数的整体执行流程：

1. **CLI 解析** — 通过 [`parse_args`](../cli.rs/parse_args.md) 解析命令行参数到 [`CliArgs`](../cli.rs/CliArgs.md)
2. **模式分发** — 检查帮助/转换/自动分组/验证/日志处理等模式标志，执行对应逻辑后提前退出
3. **配置加载** — 通过 [`read_config`](../config.rs/read_config.md) 加载并验证配置文件
4. **特权获取** — 请求 [`SeDebugPrivilege`](../winapi.rs/enable_debug_privilege.md) 和 [`SeIncreaseBasePriorityPrivilege`](../winapi.rs/enable_inc_base_priority_privilege.md)
5. **计时器分辨率** — 可选设置系统计时器分辨率（`NtSetTimerResolution`）
6. **UAC 提升** — 非管理员时请求 [`request_uac_elevation`](../winapi.rs/request_uac_elevation.md)
7. **清理子进程** — 调用 [`terminate_child_processes`](../winapi.rs/terminate_child_processes.md)
8. **初始化调度器** — 创建 [`PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md)
9. **主循环**：
   - 获取 [`ProcessSnapshot`](../process.rs/ProcessSnapshot.md)
   - 清理失败映射（[`purge_fail_map`](../logging.rs/purge_fail_map.md)）
   - 按 grade 遍历配置，匹配进程名并调用 [`apply_config`](apply_config.md)
   - find 模式下扫描进程
   - 休眠指定间隔
   - 检查配置文件和黑名单文件修改时间，按需热重载

## Grade 调度机制

配置规则按 grade 值分组存储在 `HashMap<u32, HashMap<String, ProcessConfig>>` 中：

| Grade | 执行频率 | 典型用途 |
| --- | --- | --- |
| 1 | 每次循环 | 需要频繁调整的关键进程 |
| 2 | 每 2 次循环 | 中等频率的进程 |
| 5 | 每 5 次循环 | 低频率或不常变化的进程 |

判断逻辑为 `current_loop.is_multiple_of(grade)`，即当循环计数是 grade 值的倍数时执行该 grade 组的规则。

## 热重载

主循环每次休眠后检查配置文件的 `modified()` 时间戳：

- **配置文件** — 若修改时间变化，重新调用 `read_config` 加载。仅当无错误时替换当前配置，否则保留旧配置并记录错误。
- **黑名单文件** — 若修改时间变化，重新调用 `read_list` 加载。若文件不可访问则清空黑名单。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | `src/main.rs` |
| **关键依赖** | [`CliArgs`](../cli.rs/CliArgs.md)、[`ProcessConfig`](../config.rs/ProcessConfig.md)、[`PrimeThreadScheduler`](../scheduler.rs/PrimeThreadScheduler.md)、[`ProcessSnapshot`](../process.rs/ProcessSnapshot.md) |
| **Windows API** | `NtSetTimerResolution`、`CreateToolhelp32Snapshot`、`Process32FirstW`/`Process32NextW` |