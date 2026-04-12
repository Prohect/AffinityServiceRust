# parse_args 函数 (cli.rs)

解析命令行参数字符串切片，填充 [`CliArgs`](CliArgs.md) 结构体的各个字段。

## 语法

```rust
pub fn parse_args(args: &[String], cli: &mut CliArgs) -> Result<()>
```

## 参数

`args`

命令行参数的字符串切片。索引 0 通常是可执行文件路径，实际参数从索引 1 开始解析。

`cli`

指向 [CliArgs](CliArgs.md) 的可变引用，解析结果将写入其中。应在调用前通过 `CliArgs::new()` 初始化以获取默认值。

## 返回值

返回 `windows::core::Result<()>`。当前实现始终返回 `Ok(())`。

## 备注

函数使用索引循环逐一遍历参数。对于需要附带值的参数（如 `-interval`、`-config`、`-loop` 等），会先检查 `i + 1 < args.len()` 以确保下一个参数存在，然后消费下一个位置作为该参数的值。

### 参数匹配表

| 参数 | 字段 | 说明 |
| --- | --- | --- |
| `-help` \| `--help` \| `-?` \| `/?` \| `?` | `help_mode = true` | 基本帮助模式 |
| `-helpall` \| `--helpall` | `help_all_mode = true` | 完整帮助模式 |
| `-console` | （全局 `USE_CONSOLE`） | 输出到控制台而非日志文件 |
| `-noUAC` \| `-nouac` | `no_uac = true` | 禁用 UAC 提升 |
| `-convert` | `convert_mode = true` | Process Lasso 配置转换模式 |
| `-autogroup` | `autogroup_mode = true` | 自动分组模式 |
| `-find` | `find_mode = true` | 发现使用默认亲和性的进程 |
| `-validate` | `validate_mode = true` | 配置文件语法校验模式（同时启用控制台输出） |
| `-processlogs` | `process_logs_mode = true` | 处理 find 日志模式 |
| `-dryrun` \| `-dry-run` \| `--dry-run` | `dry_run = true` | 模拟运行，不实际应用 |
| `-interval <ms>` | `interval_ms` | 检查间隔（毫秒），最小值 16ms |
| `-loop <count>` | `loop_count` | 循环次数，最小值 1 |
| `-resolution <t>` | `time_resolution` | 定时器分辨率，0 表示不设置 |
| `-logloop` | `log_loop = true` | 每次循环开始时记录日志 |
| `-config <file>` | `config_file_name` | 配置文件路径 |
| `-blacklist <file>` | `blacklist_file_name` | 黑名单文件路径 |
| `-in <file>` | `in_file_name` | 输入文件（转换/日志目录） |
| `-out <file>` | `out_file_name` | 输出文件 |
| `-skip_log_before_elevation` | `skip_log_before_elevation = true` | 提升前跳过日志 |
| `-noDebugPriv` \| `-nodebugpriv` | `no_debug_priv = true` | 不请求 SeDebugPrivilege |
| `-noIncBasePriority` \| `-noincbasepriority` | `no_inc_base_priority = true` | 不请求 SeIncreaseBasePriorityPrivilege |

### 未知参数处理

未知参数会被**静默忽略**（匹配到 `_ => {}` 分支），不会产生错误或警告。这使得程序对命令行中的拼写错误或过时参数具有宽容性。

### 间隔验证

`-interval` 参数通过 `.max(16)` 强制最小值为 16 毫秒，防止过于频繁的轮询。解析失败时回退到默认值 5000ms。

### 循环次数验证

`-loop` 参数通过 `.max(1)` 强制最小值为 1 次循环。解析失败时回退到 1。

### 带值参数的安全检查

所有需要附加值的参数（如 `-interval`、`-config`、`-loop` 等）在匹配模式中包含 `if i + 1 < args.len()` 守卫条件。如果参数出现在命令行末尾且没有后续值，该参数将不会匹配，从而落入未知参数分支被静默忽略。

## 要求

| 要求 | 值 |
| --- | --- |
| **模块** | src/cli.rs |
| **源码行** | L38–L119 |
| **调用者** | [`main`](../main.rs/main.md) |
| **依赖** | [CliArgs](CliArgs.md) |

## 另请参阅

- [CliArgs 结构体](CliArgs.md)
- [print_help](print_help.md) — `-help` 触发的帮助输出
- [print_help_all](print_help_all.md) — `-helpall` 触发的完整帮助输出
- [cli.rs 模块概述](README.md)