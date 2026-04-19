# parse_args 函数 (cli.rs)

将命令行参数字符串解析为 [CliArgs](CliArgs.md) 结构体，根据识别的开关设置标志和值。

## 语法

```rust
pub fn parse_args(args: &[String], cli: &mut CliArgs) -> Result<()>
```

## 参数

`args: &[String]`

原始命令行参数，通常从 `std::env::args()` 获取。元素 `[0]` 是可执行文件路径，将被跳过；解析从索引 1 开始。

`cli: &mut CliArgs`

**\[in, out\]** 要填充的 [CliArgs](CliArgs.md) 结构体。应使用 `CliArgs::new()` 进行初始化，以便在应用覆盖之前设置默认值（例如，`interval_ms = 5000`，`config_file_name = "config.ini"`）。

## 返回值

`Result<()>` — 始终返回 `Ok(())`。保留 `Result` 包装器是为了与其他可能失败的初始化函数保持 API 一致性。

## 备注

### 参数格式

所有开关使用单个 `-` 前缀（例如，`-console`，`-interval`）。对于少数常见标志，还接受双破折号（`--help`，`--helpall`，`--dry-run`）和 Windows 风格（`/?`，`/? `）变体。参数匹配是**区分大小写**的，除非列出显式别名（例如，`-noUAC` 和 `-nouac` 都被接受）。

### 值参数

消耗后续值的开关（`-interval`，`-loop`，`-resolution`，`-config`，`-blacklist`，`-in`，`-out`）在读取下一个元素之前检查 `i + 1 < args.len()`。如果守卫失败（未提供值），则静默忽略该开关并继续解析。

### 数值范围限制

| 参数 | 类型 | 默认值 | 限制范围 |
|----------|------|---------|-------------|
| `-interval <ms>` | `u32` | `5000` | `[16, 86_400_000]`（16 毫秒到 24 小时） |
| `-loop <count>` | `u32` | `1` | `[1, u32::MAX]` |
| `-resolution <t>` | `u32` | `0` | 不限制；`0` 表示不设置 |

如果解析数值失败，则使用该字段的默认值（通过 `unwrap_or`）。

### 副作用

- **`-console`** 和 **`-validate`**：除了设置各自的 `CliArgs` 字段外，这些开关还通过 `get_use_console!()` 将全局 `USE_CONSOLE` 标志设置为 `true`，将日志输出重定向到 stdout，直到进程生命周期结束。

### 未识别的参数

未识别的开关被静默忽略（通过 `_ => {}` 分支匹配）。不发出警告。

### 识别的开关

| 开关 | 设置的字段 | 备注 |
|--------|-----------|-------|
| `-help`, `--help`, `-?`, `/?`, `?` | `help_mode = true` | |
| `-helpall`, `--helpall` | `help_all_mode = true` | |
| `-console` | 全局 `USE_CONSOLE` | 未存储在 `CliArgs` 中 |
| `-noUAC`, `-nouac` | `no_uac = true` | |
| `-convert` | `convert_mode = true` | |
| `-autogroup` | `autogroup_mode = true` | |
| `-find` | `find_mode = true` | |
| `-validate` | `validate_mode = true` | 同时设置 `USE_CONSOLE` |
| `-processlogs` | `process_logs_mode = true` | |
| `-dryrun`, `-dry-run`, `--dry-run` | `dry_run = true` | |
| `-interval <ms>` | `interval_ms` | 限制在 `[16, 86_400_000]` |
| `-loop <count>` | `loop_count = Some(n)` | 最小值为 1 |
| `-resolution <t>` | `time_resolution` | |
| `-logloop` | `log_loop = true` | |
| `-config <file>` | `config_file_name` | |
| `-blacklist <file>` | `blacklist_file_name = Some(…)` | |
| `-in <file>` | `in_file_name = Some(…)` | |
| `-out <file>` | `out_file_name = Some(…)` | |
| `-skip_log_before_elevation` | `skip_log_before_elevation = true` | |
| `-noDebugPriv`, `-nodebugpriv` | `no_debug_priv = true` | |
| `-noIncBasePriority`, `-noincbasepriority` | `no_inc_base_priority = true` | |
| `-no_etw`, `-noetw` | `no_etw = true` | |
| `-continuous_process_level_apply` | `continuous_process_level_apply = true` | |

## 需求

| | |
|---|---|
| **模块** | `src/cli.rs` |
| **调用方** | `main()` 启动期间 |
| **被调用方** | `get_use_console!()` 宏 |
| **Win32 API** | 无 |
| **权限** | 无 |

## 参见

| 主题 | 链接 |
|-------|------|
| 参数容器 | [CliArgs](CliArgs.md) |
| 模块概述 | [cli.rs](README.md) |
| 基本帮助输出 | [print_help](print_help.md) |
| 完整帮助输出 | [print_help_all](print_help_all.md) |
| 日志系统 | [logging.rs](../logging.rs/README.md) |

*文档记录于提交：[facc6e1](https://github.com/Prohect/AffinityServiceRust/tree/facc6e145992bd6a24dc7f5f21525085e10a7caf)*