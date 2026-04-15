# CliArgs 类型 (cli.rs)

`CliArgs` 结构体保存控制 AffinityServiceRust Windows 服务运行时行为的所有命令行选项和标志。它由 [`parse_args`](parse_args.md) 填充，并在整个应用程序中使用，以确定操作模式、轮询间隔、文件路径、权限请求和调试设置。

## 语法

```AffinityServiceRust/src/cli.rs#L5-28
pub struct CliArgs {
    pub interval_ms: u32,
    pub help_mode: bool,
    pub help_all_mode: bool,
    pub convert_mode: bool,
    pub autogroup_mode: bool,
    pub find_mode: bool,
    pub validate_mode: bool,
    pub process_logs_mode: bool,
    pub dry_run: bool,
    pub config_file_name: String,
    pub blacklist_file_name: Option<String>,
    pub in_file_name: Option<String>,
    pub out_file_name: Option<String>,
    pub no_uac: bool,
    pub loop_count: Option<u32>,
    pub time_resolution: u32,
    pub log_loop: bool,
    pub skip_log_before_elevation: bool,
    pub no_debug_priv: bool,
    pub no_inc_base_priority: bool,
    pub no_etw: bool,
    pub continuous_process_level_apply: bool,
}
```

## 成员

| 成员 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| `interval_ms` | `u32` | `5000` | 进程扫描循环之间的轮询间隔（毫秒）。由 [`parse_args`](parse_args.md) 限制在 16–86 400 000 范围内。 |
| `help_mode` | `bool` | `false` | 当为 `true` 时，服务打印简洁的帮助消息并退出。由 `-help`、`--help`、`-?`、`/?` 或 `?` 设置。 |
| `help_all_mode` | `bool` | `false` | 当为 `true` 时，服务打印详细帮助（CLI + 配置参考）并退出。由 `-helpall` 或 `--helpall` 设置。 |
| `convert_mode` | `bool` | `false` | 启用 Process Lasso 配置转换模式。需要 `-in` 和 `-out`。由 `-convert` 设置。 |
| `autogroup_mode` | `bool` | `false` | 启用具有相同设置的规则自动分组。需要 `-in` 和 `-out`。由 `-autogroup` 设置。 |
| `find_mode` | `bool` | `false` | 启用进程发现模式，记录以默认（全核）亲和性运行的进程。由 `-find` 设置。 |
| `validate_mode` | `bool` | `false` | 验证配置文件的语法错误和未定义别名，然后退出。同时强制控制台输出。由 `-validate` 设置。 |
| `process_logs_mode` | `bool` | `false` | 启用日志处理模式，扫描查找模式的日志以发现新进程。由 `-processlogs` 设置。 |
| `dry_run` | `bool` | `false` | 当为 `true` 时，服务模拟变更而不实际应用。由 `-dryrun`、`-dry-run` 或 `--dry-run` 设置。 |
| `config_file_name` | `String` | `"config.ini"` | 配置文件的路径。由 `-config <file>` 设置。 |
| `blacklist_file_name` | `Option<String>` | `None` | 查找模式使用的黑名单文件的可选路径。由 `-blacklist <file>` 设置。 |
| `in_file_name` | `Option<String>` | `None` | `-convert` 的输入文件路径，或 `-processlogs` 的日志目录。由 `-in <file>` 设置。 |
| `out_file_name` | `Option<String>` | `None` | `-convert`、`-autogroup` 或 `-processlogs` 的输出文件路径。由 `-out <file>` 设置。 |
| `no_uac` | `bool` | `false` | 禁用启动时的 UAC 提升请求。由 `-noUAC` 或 `-nouac` 设置。 |
| `loop_count` | `Option<u32>` | `None` | 设置后，限制轮询循环次数（最小为 1）。`None` 表示无限循环。由 `-loop <count>` 设置。 |
| `time_resolution` | `u32` | `0` | Windows 计时器分辨率，以 100 纳秒为单位（例如 `5210` → 0.5210 毫秒）。`0` 表示不修改。由 `-resolution <t>` 设置。 |
| `log_loop` | `bool` | `false` | 在每次轮询循环开始时记录诊断消息。由 `-logloop` 设置。 |
| `skip_log_before_elevation` | `bool` | `false` | 在 UAC 提升完成之前抑制日志输出。由 `-skip_log_before_elevation` 设置。 |
| `no_debug_priv` | `bool` | `false` | 跳过在启动时请求 `SeDebugPrivilege`。由 `-noDebugPriv` 或 `-nodebugpriv` 设置。 |
| `no_inc_base_priority` | `bool` | `false` | 跳过在启动时请求 `SeIncreaseBasePriorityPrivilege`。由 `-noIncBasePriority` 或 `-noincbasepriority` 设置。 |
| `no_etw` | `bool` | `false` | 禁用 ETW（Windows 事件跟踪）跟踪。由 `-no_etw` 或 `-noetw` 设置。 |
| `continuous_process_level_apply` | `bool` | `false` | 在每次轮询迭代中重新应用进程级设置（优先级、亲和性、CPU 集合、I/O 优先级、内存优先级），而非每个 PID 仅应用一次。由 `-continuous_process_level_apply` 设置。 |

## 备注

- `CliArgs` 派生了 `Debug` 和 `Default`。`Default` 派生会对所有字段进行零初始化；使用 `CliArgs::new()` 可获得具有生产默认值的实例（`interval_ms = 5000`、`config_file_name = "config.ini"`）。
- 操作模式（`convert_mode`、`autogroup_mode`、`find_mode`、`validate_mode`、`process_logs_mode`）按约定互斥。同时设置多个会在应用程序级别产生未定义行为；解析器不强制互斥性。
- 当设置 `validate_mode` 时，无论是否同时指定了 `-console`，都会强制启用控制台输出，以确保验证结果可见。
- `loop_count` 字段对集成测试很有用；与 `-logloop` 和 `-interval` 结合使用，可以限定测试会话的运行时间。

## 要求

| 要求 | 值 |
|------|-----|
| 模块 | `cli.rs` |
| 构造方式 | `CliArgs::new()` |
| 填充方 | [`parse_args`](parse_args.md) |
| 使用方 | `main.rs`、[`read_config`](../config.rs/read_config.md)、[`hotreload_config`](../config.rs/hotreload_config.md)、[`hotreload_blacklist`](../config.rs/hotreload_blacklist.md)、[`convert`](../config.rs/convert.md)、[`sort_and_group_config`](../config.rs/sort_and_group_config.md) |

## 另请参阅

| 资源 | 链接 |
|------|------|
| parse_args | [parse_args](parse_args.md) |
| config 模块 | [config.rs 概述](../config.rs/README.md) |
| cli 模块概述 | [README](README.md) |

---
> Commit SHA: `7221ea0694670265d4eb4975582d8ed2ae02439d`
