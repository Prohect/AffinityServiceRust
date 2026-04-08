# Main 模块文档

应用程序入口点和主循环。

## 概述

本模块实现：
- CLI 参数解析委托
- 特权管理
- 配置文件监控和热重载
- 基于 grade 的调度主服务循环
- 进程发现 ([`-find` 模式](#-find-flag))
- 工具模式 ([`-convert`](#-convert), [`-validate`](#-validate), [`-processlogs`](#-processlogs))

## 调用者

- 操作系统 - 程序入口点
- 用户 - 使用各种 CLI 标志直接执行

## 数据结构

### 模块导入

```rust
mod apply;      // 配置应用
mod cli;        // CLI 解析
mod config;     // 配置文件解析
mod error_codes;// 错误转换
mod logging;    // 日志基础设施
mod priority;   // 优先级枚举
mod process;    // 进程枚举
mod scheduler;  // Prime 线程调度器
mod winapi;     // Windows API 包装器
```

## 入口点

### main

应用程序入口点。

```rust
fn main() -> windows::core::Result<()>
```

**流程：**
1. 解析 CLI 参数 ([`parse_args()`](cli.md#parse_args))
2. 处理帮助模式 (`-help`, `-helpall`)
3. 处理工具模式 ([`-convert`](#-convert), [`-autogroup`](#-autogroup))
4. 加载配置 ([`read_config()`](config.md#config-file-format))
5. 启用特权 ([`enable_debug_privilege()`](winapi.md#enable_debug_privilege), [`enable_inc_base_priority_privilege()`](winapi.md#enable_inc_base_priority_privilege))
6. 设置计时器分辨率（如果指定）
7. 请求 UAC 提升 ([`request_uac_elevation()`](winapi.md#request_uac_elevation))
8. 清理子进程 ([`terminate_child_processes()`](winapi.md#terminate_child_processes))
9. 初始化 prime 线程调度器
10. **主循环：**
    - 获取进程快照 ([`ProcessSnapshot::take()`](process.md#processsnapshottake) 在 [process.rs](process.md))
    - 按 grade 应用配置
    - 处理 [`-find` 模式](#-find-flag)
    - 休眠间隔时间
    - 检查配置文件更改（热重载）

## 主循环

### 基于 Grade 的调度

规则按 "grade" 组织 - 应用频率：

```rust
for (grade, grade_configs) in &configs {
    if !current_loop.is_multiple_of(*grade) {
        continue; // 本次循环跳过此 grade
    }
    // 应用此 grade 的配置...
}
```

**Grade 值：**
| Grade | 频率 | 用例 |
|-------|-----------|----------|
| 1 | 每次循环 | 关键进程（游戏） |
| 2 | 每第 2 次循环 | 半关键 |
| 5 | 每第 5 次循环 | 后台工具 |
| 10 | 每第 10 次循环 | 很少变化 |

### 配置应用

对于每个匹配配置的进程：

```rust
let result = apply_config(pid, config, &mut scheduler, &mut processes, cli.dry_run);
```

参见：[apply_config-函数](main.md#apply_config-函数)

**结果处理：**
- 错误通过 [`log_to_find()`](logging.md#log_to_find) 记录到 `.find.log`
- 更改记录到主日志，格式：
  ```
  [HH:MM:SS] 12345::process.exe::Change 1
             Change 2
             Change 3
  ```

### 试运行模式

使用 `-dryrun` 标志：
- 计算但不应用更改
- 显示报告：`[DRY RUN] N change(s) would be made`
- 一次迭代后退出

### 循环控制

**终止条件：**
- 达到 `-loop <count>`
- 试运行完成
- 错误（已记录，大多数错误继续）

**间隔：**通过 `-interval <ms>` 配置（最小 16ms）

## 配置热重载

### 文件监控

每次循环检查修改时间：

```rust
if metadata(&config_file)?.modified()? != last_config_mod_time {
    // 重新加载配置...
}
```

### 重载流程

1. 记录更改检测
2. 解析新配置
3. 如果有效：
   - 更新 `configs` HashMap
   - 更新调度器常量
   - 记录成功
4. 如果无效：
   - 记录错误
   - 保留之前的配置

### 黑名单重载

对可选黑名单文件进行类似的监控。

## 进程发现模式

### -find 标志

启用时，扫描未管理的进程：

```rust
if cli.find_mode {
    // 通过 ToolHelp32 枚举所有进程
    for each process {
        if !in_configs && !in_blacklist && is_affinity_unset(pid, name) {
            log_process_find(&name);
        }
    }
}
```

**条件：**
- 进程名不在任何配置中
- 进程名不在黑名单中
- 进程具有默认系统亲和性

**输出：**`logs/YYYYMMDD.find.log`

**去重：**每个进程每个会话通过 [`FINDS_SET`](logging.md#finds_set) 只记录一次

## 工具模式

### -validate

验证配置语法而不运行：

```rust
if cli.validate_mode {
    config_result.print_report();
    return Ok(());
}
```

输出错误/警告到控制台。

### -convert

转换 Process Lasso 配置格式：

```rust
if cli.convert_mode {
    convert(cli.in_file_name, cli.out_file_name);
    return Ok(());
}
```

有关转换详情，请参见 [config.md](config.md#process-lasso-conversion)。

### -autogroup

分组相同规则：

```rust
if cli.autogroup_mode {
    sort_and_group_config(cli.in_file_name, cli.out_file_name);
    return Ok(());
}
```

有关分组算法，请参见 [config.md](config.md#auto-grouping)。

### -processlogs

处理 `.find.log` 文件以发现新进程：

```rust
if cli.process_logs_mode {
    process_logs(&configs, &blacklist, cli.in_file_name.as_deref(), cli.out_file_name.as_deref());
    return Ok(());
}
```

**算法：**
1. 扫描 `logs/*.find.log` 文件
2. 提取发现的进程名
3. 过滤掉已知配置和黑名单
4. 通过 `es.exe` (Everything) 搜索可执行路径
5. 将结果写入文件以供手动审查

## apply_config 函数

将配置应用到单个进程。

```rust
fn apply_config(
    pid: u32,
    config: &ProcessConfig,
    dry_run: bool,
    process: &mut ProcessEntry,
    prime_core_scheduler: &mut PrimeThreadScheduler,
) -> ApplyConfigResult
```

**参数：**
- `config`: [`ProcessConfig`](config.md#processconfig)
- `prime_core_scheduler`: [`PrimeThreadScheduler`](scheduler.md#primethreadscheduler)
- `process`: [`ProcessEntry`](process.md#processentry)
- 返回: [`ApplyConfigResult`](apply.md#applyconfigresult)

**操作（按顺序）：**
1. **优先级** - [`apply_priority()`](apply.md#apply_priority)
2. **亲和性** - [`apply_affinity()`](apply.md#apply_affinity)
3. **CPU 集** - [`apply_process_default_cpuset()`](apply.md#apply_process_default_cpuset)
4. **I/O 优先级** - [`apply_io_priority()`](apply.md#apply_io_priority)
5. **内存优先级** - [`apply_memory_priority()`](apply.md#apply_memory_priority)
6. **Prime 调度**（如果配置）：
   - 释放模块缓存 ([`drop_module_cache()`](winapi.md#drop_module_cache))
   - 在调度器中设置存活 ([`set_alive()`](scheduler.md#reset_alive--set_alive))
   - 预取周期 ([`prefetch_all_thread_cycles()`](apply.md#prefetch_all_thread_cycles))
   - 应用 prime 线程 ([`apply_prime_threads()`](apply.md#apply_prime_threads) 在 [apply.rs](apply.md))
   - 应用理想处理器 ([`apply_ideal_processors()`](apply.md#apply_ideal_processors))
   - 更新线程统计 ([`update_thread_stats()`](apply.md#update_thread_stats))

**提前退出：**如果无法获取进程句柄，立即返回空结果。

## 特权管理

### 请求的特权

1. **SeDebugPrivilege** - 访问受保护进程（用于线程起始地址）
2. **SeIncreaseBasePriorityPrivilege** - I/O 优先级 "high" 设置

### 禁用标志

| 标志 | 效果 |
|------|------|
| `-noDebugPriv` | 不请求 `SeDebugPrivilege` |
| `-noIncBasePriority` | 不请求 `SeIncreaseBasePriorityPrivilege` |
| `-noUAC` | 不请求提升 |

### UAC 提升

如果不是管理员且未设置 `-noUAC`：

```rust
match request_uac_elevation(*use_console().lock().unwrap()) {
    Ok(_) => { /* 进程在这里退出，提升的实例接管 */ }
    Err(e) => { log!("Failed to elevate: {}", e); }
}
```

**注意：**原始进程退出；通过 PowerShell 生成提升的副本。

## 计时器分辨率

可选的系统计时器分辨率调整：

```rust
if cli.time_resolution != 0 {
    NtSetTimerResolution(cli.time_resolution, true, &mut current_resolution);
}
```

**单位：**100 纳秒间隔（10000 = 1ms）

**警告：**非常低的值（<1ms）可能影响系统稳定性。

## 依赖

- 所有子模块（`apply`, `cli`, `config` 等）
- `chrono::Local` - 日志时间戳
- `encoding_rs` - `-processlogs` 的字符编码
- `std::collections` - 配置的 HashMap
- `std::fs` - 配置文件监控
- `std::thread` - 循环之间休眠
- `windows` - Win32 API（ToolHelp 用于 `-find`）

## 性能特征

| 操作 | 成本 | 注意 |
|-----------|------|-------|
| 进程快照 | O(P + T) | 单系统调用 |
| 配置应用 | O(C × O_a) | C=配置, O_a=应用成本 |
| Prime 调度 | O(T_p log T_p) | 排序线程 |
| 热重载检查 | O(1) | 元数据检查 |
| Find 模式 | O(P) | 额外迭代 |

**典型循环：**取决于线程数和主要是配置，从 1.6ms 到 10ms。
