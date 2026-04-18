# parse_args 函数 (cli.rs)

将命令行参数字符串切片解析到可变的 `CliArgs` 实例中，根据识别的参数标记设置标志、模式和值。未识别的参数会被静默忽略。

## 语法

```AffinityServiceRust/src/cli.rs#L42-43
pub fn parse_args(args: &[String], cli: &mut CliArgs) -> Result<()>
```

## 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `args` | `&[String]` | 完整的命令行参数列表。元素 `[0]`（可执行文件路径）会被跳过；解析从索引 1 开始。 |
| `cli` | `&mut CliArgs` | 将被填充解析值的 `CliArgs` 结构体的可变引用。调用前应通过 `CliArgs::new()` 初始化。 |

## 返回值

返回 `windows::core::Result<()>`。当前始终返回 `Ok(())`——参数解析错误通过回退到默认值（例如 `unwrap_or`）来处理，而非传播错误。

## 备注

### 已识别的参数

| 参数 | 效果 | 说明 |
|------|------|------|
| `-help`、`--help`、`-?`、`/?`、`?` | 设置 `help_mode = true` | |
| `-helpall`、`--helpall` | 设置 `help_all_mode = true` | |
| `-console` | 通过 `get_use_console!()` 启用控制台输出 | 覆盖日志文件输出 |
| `-noUAC`、`-nouac` | 设置 `no_uac = true` | 不区分大小写的变体 |
| `-convert` | 设置 `convert_mode = true` | 需要 `-in` 和 `-out` |
| `-autogroup` | 设置 `autogroup_mode = true` | 需要 `-in` 和 `-out` |
| `-find` | 设置 `find_mode = true` | |
| `-validate` | 设置 `validate_mode = true` | 同时强制启用控制台输出 |
| `-processlogs` | 设置 `process_logs_mode = true` | |
| `-dryrun`、`-dry-run`、`--dry-run` | 设置 `dry_run = true` | |
| `-interval <ms>` | 设置 `interval_ms` | 限制在 `[16, 86400000]` 范围内；解析失败时默认为 `5000` |
| `-loop <count>` | 设置 `loop_count = Some(n)` | 最小值为 `1` |
| `-resolution <t>` | 设置 `time_resolution` | `0` 表示不更改系统计时器分辨率 |
| `-logloop` | 设置 `log_loop = true` | |
| `-config <file>` | 设置 `config_file_name` | |
| `-blacklist <file>` | 设置 `blacklist_file_name = Some(...)` | |
| `-in <file>` | 设置 `in_file_name = Some(...)` | |
| `-out <file>` | 设置 `out_file_name = Some(...)` | |
| `-skip_log_before_elevation` | 设置 `skip_log_before_elevation = true` | |
| `-noDebugPriv`、`-nodebugpriv` | 设置 `no_debug_priv = true` | |
| `-noIncBasePriority`、`-noincbasepriority` | 设置 `no_inc_base_priority = true` | |
| `-no_etw`、`-noetw` | 设置 `no_etw = true` | |
| `-continuous_process_level_apply` | 设置 `continuous_process_level_apply = true` | |

### 参数消费

需要值的参数（例如 `-interval`、`-config`）会消费 `args` 中的*下一个*元素。边界检查（`i + 1 < args.len()`）可防止越界访问；如果值参数缺失，则静默跳过该标志。

### 未识别的参数

任何不匹配已知标记的参数字符串都会被忽略，不会发出警告或错误。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `cli.rs` |
| 调用方 | `main`（入口点） |
| 被调用方 | `CliArgs` 字段、`get_use_console!()` 宏 |
| API | `windows::core::Result` |
| 权限 | 无 |

## 另请参阅

| 资源 | 链接 |
|------|------|
| CliArgs 结构体 | [CliArgs](CliArgs.md) |
| print_help | [print_help](print_help.md) |
| print_help_all | [print_help_all](print_help_all.md) |
| config 模块 | [config.rs 概述](../config.rs/README.md) |

---
*Documented for Commit: [29c0140](https://github.com/Prohect/AffinityServiceRust/tree/29c0140cfc5ad80a5ee53fea0ce61fedb90783aa)*